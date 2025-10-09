use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufReader, Read, Seek},
    process::ExitCode,
    time::Duration,
};

use anyhow::{Context as AnyhowContext, Result, format_err};
use chrono::DateTime;
use crossterm::style::Stylize;
use flate2::{Compression, write::GzEncoder};
use pbjson_types::Timestamp;
use reqwest::header::{CONTENT_ENCODING, CONTENT_TYPE};
use sift_rs::{
    common::r#type::v1::{ChannelConfig, ChannelDataType},
    data_imports::v2::{
        CreateDataImportFromUploadRequest, CreateDataImportFromUploadResponse, CsvConfig,
        CsvTimeColumn, TimeFormat as PbTimeFormat,
        data_import_service_client::DataImportServiceClient,
    },
    jobs::v1::{JobStatus, JobType},
};
use tokio::time::sleep;

use crate::{
    cli::{CsvArgs, time::TimeFormat},
    cmd::{
        Context,
        utils::{try_parse_bit_field_config, try_parse_enum_config},
    },
    util::{
        api::{create_grpc_channel, create_rest_client},
        job::JobServiceWrapper,
        progress::Spinner,
        tty::Output,
        user::get_user_id,
    },
};

pub async fn import(ctx: Context, args: CsvArgs) -> Result<ExitCode> {
    let mut csv_file = File::open(&args.path)?;
    let csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&csv_file);

    let grpc_channel = create_grpc_channel(&ctx)?;
    let create_data_import_req = create_data_import_request(csv_reader, &args)?;
    let mut data_imports_client = DataImportServiceClient::new(grpc_channel.clone());

    let CreateDataImportFromUploadResponse { upload_url, .. } = data_imports_client
        .create_data_import_from_upload(create_data_import_req)
        .await
        .context("error creating data import")?
        .into_inner();

    csv_file.rewind()?;
    let mut reader = BufReader::new(csv_file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    io::copy(&mut buffer.as_slice(), &mut encoder)?;
    let compressed_data = encoder.finish()?;

    let rest_client = create_rest_client(&ctx)?;
    let res = rest_client
        .post(upload_url)
        .header(CONTENT_ENCODING, "gzip")
        .header(CONTENT_TYPE, "text/csv")
        .body(compressed_data)
        .send()
        .await
        .context("failed to upload CSV file")?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res
            .text()
            .await
            .unwrap_or_else(|_| "<failed to read body>".into());
        return Err(format_err!(
            "failed to upload CSV with http status {status}: {text}"
        ));
    }

    let location = args.run.as_ref().map_or_else(
        || format!("asset '{}'", args.asset.cyan()),
        |r| format!("run '{}'", r.clone().cyan()),
    );

    if !args.wait {
        Output::new()
            .line("Successfully uploaded CSV for processing.")
            .tip(format!(
                "Once processing is complete the data will be available on the {location}."
            ))
            .print();

        return Ok(ExitCode::SUCCESS);
    }

    let spinner = Spinner::new();
    spinner.set_message("CSV has been successfully uploaded and is waiting to be processed.");

    let user_id = get_user_id(grpc_channel.clone()).await?;
    let mut job_service = JobServiceWrapper::new(grpc_channel.clone());

    let Some(mut job) = job_service
        .get_latest_job_for_user(&user_id, JobType::DataImport)
        .await?
    else {
        spinner.finish_and_clear();

        Output::new()
            .line("The CSV was successfully uploaded but the job was unexpectedly not found")
            .tip("Please notify Sift about this bug.")
            .eprint();
        return Ok(ExitCode::FAILURE);
    };

    loop {
        sleep(Duration::from_secs(3)).await;

        let Some(updated_job) = job_service.get_job(&job.job_id).await? else {
            spinner.finish_and_clear();
            Output::new()
                .line("The CSV was successfully uploaded but the job was unexpectedly not found")
                .tip("Please notify Sift about this bug.")
                .eprint();
            return Ok(ExitCode::FAILURE);
        };
        job = updated_job;

        match job.job_status() {
            JobStatus::Created => (),
            JobStatus::Running => {
                spinner.set_message("CSV is currently being processed");
            }
            JobStatus::CancelRequested => {
                spinner.set_message("A cancellation was requested but the job may still finish");
            }
            JobStatus::Cancelled => {
                spinner.finish_and_clear();
                Output::new().line("CSV job was cancelled").print();
                break;
            }
            JobStatus::Failed => {
                spinner.finish_and_clear();
                Output::new()
                    .line("CSV processing failed.")
                    .tip("Please check the Sift jobs manage page for further details.")
                    .eprint();
                return Ok(ExitCode::FAILURE);
            }
            JobStatus::Finished => {
                spinner.finish_and_clear();
                Output::new()
                    .line("CSV import completed")
                    .tip(format!("The data should be available on the {location}."))
                    .print();
                break;
            }
            _ => (),
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn create_data_import_request<R: io::Read>(
    csv_reader: csv::Reader<R>,
    args: &CsvArgs,
) -> Result<CreateDataImportFromUploadRequest> {
    let num_overrides = args.channel_column.len();

    if ![
        args.data_type.len(),
        args.unit.len(),
        args.description.len(),
    ]
    .iter()
    .all(|n| *n == num_overrides)
    {
        return Err(format_err!(
            "occurrences of --data-type, --units, and --descriptions must equal --channel-column"
        ))
        .context("keep in mind that --units and --descriptions can be empty strings");
    }

    match args.time_format {
        TimeFormat::RelativeNanoseconds
        | TimeFormat::RelativeMicroseconds
        | TimeFormat::RelativeMilliseconds
        | TimeFormat::RelativeSeconds
        | TimeFormat::RelativeMinutes
        | TimeFormat::RelativeHours => {
            if args.relative_start_time.is_none() {
                return Err(format_err!(
                    "--relative-start-time is required if time format is relative"
                ));
            }
        }
        _ => (),
    }

    let relative_start_time = match &args.relative_start_time {
        Some(start) => {
            let rs = DateTime::parse_from_rfc3339(start)
                .context("--relative-start-time is not valid RFC3339")?;
            let utc = rs.to_utc();
            Some(Timestamp::from(utc))
        }
        None => None,
    };

    if args.header_row == 0 {
        return Err(format_err!(
            "--header-row cannot be 0 due to 1-based indexing"
        ));
    }
    if args.first_data_row == 0 {
        return Err(format_err!(
            "--first-data-row cannot be 0 due to 1-based indexing"
        ));
    }
    if args.header_row >= args.first_data_row {
        return Err(format_err!(
            "--header-row must come before --first-data-row"
        ));
    }

    let data_types = args
        .data_type
        .iter()
        .map(|dt| dt.clone().into())
        .collect::<Vec<ChannelDataType>>();

    let mut enum_configs_iter = {
        let mut parsed_enum_configs = Vec::with_capacity(args.enum_config.len());

        for config in &args.enum_config {
            let parsed = try_parse_enum_config(config)?;
            parsed_enum_configs.push(parsed);
        }
        parsed_enum_configs.into_iter()
    };

    let mut bit_field_configs_iter = {
        let mut parsed_bit_field_configs = Vec::with_capacity(args.bit_field_config.len());

        for config in &args.bit_field_config {
            let parsed = try_parse_bit_field_config(config)?;
            parsed_bit_field_configs.push(parsed);
        }
        parsed_bit_field_configs.into_iter()
    };
    let mut records_iter = csv_reader.into_records().enumerate();
    let mut current_row = 1;

    // Find the header row
    let headers = {
        let mut values = Vec::new();

        while current_row < args.header_row {
            current_row += 1;
            records_iter.next();
        }

        let Some((idx, header_row)) = records_iter.next() else {
            return Err(format_err!(
                "CSV prematurely reached EOF while looking for header row"
            ))
            .context("double check --header-row");
        };
        current_row += 1;
        let row_num = idx + 1;

        let parsed_record = header_row.context(format_err!("failed to parse row {row_num}"))?;

        for col in &parsed_record {
            values.push(col.to_string());
        }
        values
    };
    if headers.is_empty() {
        return Err(format_err!("no headers were found given the --header-row"));
    }
    if headers.len() < 2 {
        return Err(format_err!(
            "expected at least two columns: a timestamp column and a channel column"
        ));
    }
    let num_columns = headers.len();

    let mut channel_columns_set = HashSet::new();

    let data_columns = {
        let mut values = HashMap::<u32, ChannelConfig>::new();

        while current_row < args.first_data_row {
            if records_iter.next().is_none() {
                return Err(format_err!(
                    "CSV reached EOF with the provided --first-data-row"
                ));
            }
            current_row += 1;
        }

        // Create a config for every single column
        for (i, record) in records_iter {
            // All data columns have been accounted for
            if values.len() == num_columns - 1 {
                break;
            }
            let row_num = i + 1;

            let parsed_record = record.context(format_err!("failed to parse row {row_num}"))?;

            for (j, col) in parsed_record.iter().enumerate() {
                let col_num = j + 1;

                if col_num == args.time_column {
                    continue;
                }
                let name = headers.get(j).unwrap().to_string();

                if values.contains_key(&(col_num as u32)) {
                    continue;
                }

                // Is there an override specified for a particular column?
                if let Some((idx, col)) = args
                    .channel_column
                    .iter()
                    .enumerate()
                    .find(|(_, col)| **col == col_num)
                {
                    if !channel_columns_set.insert(col) {
                        return Err(format_err!(
                            "cannot have redundant values '{col}' for --channel-column"
                        ));
                    }

                    // Safe to unwrap all these because of top-level validation ensuring all
                    // vectors are of equal length with channel_columns; enum and bit filed configs
                    // follow other validation rules.
                    let data_type: i32 = (*data_types.get(idx).unwrap()).into();
                    let unit = args.unit.get(idx).unwrap().clone();
                    let description = args.description.get(idx).unwrap().clone();

                    let mut enum_configs = Vec::new();
                    let mut bit_field_configs = Vec::new();

                    if data_type == ChannelDataType::Enum.into() {
                        let Some(configs) = enum_configs_iter.next() else {
                            return Err(format_err!(
                                "'{name}' was declared as type enum but --enum-config was not specified"
                            ));
                        };
                        enum_configs = configs;
                    } else if data_type == ChannelDataType::BitField.into() {
                        let Some(configs) = bit_field_configs_iter.next() else {
                            return Err(format_err!(
                                "'{name}' was declared as type bit-field but --bit-field-config was not specified"
                            ));
                        };
                        bit_field_configs = configs;
                    }
                    values.insert(
                        *col as u32,
                        ChannelConfig {
                            name,
                            description,
                            data_type,
                            units: unit,
                            bit_field_elements: bit_field_configs,
                            enum_types: enum_configs,
                            ..Default::default()
                        },
                    );
                } else if col.parse::<f64>().is_ok() {
                    values.insert(
                        col_num as u32,
                        ChannelConfig {
                            name,
                            data_type: ChannelDataType::Double.into(),
                            ..Default::default()
                        },
                    );
                } else {
                    values.insert(
                        col_num as u32,
                        ChannelConfig {
                            name,
                            data_type: ChannelDataType::String.into(),
                            ..Default::default()
                        },
                    );
                }
            }
        }
        values
    };

    for col_num in &args.channel_column {
        if !channel_columns_set.contains(col_num) {
            return Err(format_err!(
                "an override was specified for column {col_num} but it doesn't refer to a channel"
            ));
        }
    }

    Ok(CreateDataImportFromUploadRequest {
        csv_config: Some(CsvConfig {
            asset_name: args.asset.clone(),
            data_columns,
            first_data_row: args.first_data_row as u32,
            run_name: args.run.clone().unwrap_or_default(),
            time_column: Some(CsvTimeColumn {
                relative_start_time,
                column_number: args.time_column as u32,
                format: PbTimeFormat::from(args.time_format.clone()).into(),
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod test_create_data_import_request {
    use std::path::PathBuf;

    use crate::cli::{CsvArgs, channel::DataType, time::TimeFormat};
    use indoc::indoc;
    use sift_rs::{
        common::r#type::v1::ChannelDataType, data_imports::v2::TimeFormat as PbTimeFormat,
    };

    use super::create_data_import_request;

    #[test]
    fn simple_case() {
        let test_csv = indoc! {"
            time,channel
            2025-10-04T21:58:13Z,1.0
        "};
        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(test_csv.as_bytes());

        let req = create_data_import_request(
            csv_reader,
            &CsvArgs {
                path: PathBuf::default(),
                asset: "test_asset".into(),
                run: None,
                header_row: 1,
                first_data_row: 2,
                channel_column: Vec::default(),
                data_type: Vec::default(),
                unit: Vec::default(),
                description: Vec::default(),
                enum_config: Vec::default(),
                bit_field_config: Vec::default(),
                time_column: 1,
                time_format: TimeFormat::default(),
                relative_start_time: None,
                wait: false,
            },
        )
        .expect("expected Result::Ok");

        let csv_config = req.csv_config.expect("expected Option::Some");
        assert_eq!(String::from("test_asset"), csv_config.asset_name);
        assert!(csv_config.run_id.is_empty());
        assert!(csv_config.run_name.is_empty());
        assert_eq!(2, csv_config.first_data_row);

        let time_config = csv_config.time_column.unwrap();
        assert_eq!(1, time_config.column_number);
        assert_eq!(PbTimeFormat::AbsoluteRfc3339 as i32, time_config.format);
        assert!(time_config.relative_start_time.is_none());

        assert_eq!(1, csv_config.data_columns.len());
        let config = csv_config.data_columns.get(&2).unwrap();
        assert_eq!(ChannelDataType::Double, config.data_type());
        assert_eq!(String::from("channel"), config.name);
    }

    #[test]
    fn simple_type_override() {
        let test_csv = indoc! {"
            time,channel
            2025-10-04T21:58:13Z,1.0
        "};
        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(test_csv.as_bytes());

        let req = create_data_import_request(
            csv_reader,
            &CsvArgs {
                path: PathBuf::default(),
                asset: "test_asset".into(),
                run: Some("test_run".into()),
                header_row: 1,
                first_data_row: 2,
                channel_column: vec![2],
                data_type: vec![DataType::Float],
                unit: vec!["km/hr".into()],
                description: vec!["some_description".into()],
                enum_config: Vec::default(),
                bit_field_config: Vec::default(),
                time_column: 1,
                time_format: TimeFormat::default(),
                relative_start_time: None,
                wait: false,
            },
        )
        .expect("expected Result::Ok");

        let csv_config = req.csv_config.expect("expected Option::Some");
        assert_eq!(1, csv_config.data_columns.len());
        let config = csv_config.data_columns.get(&2).unwrap();
        assert_eq!(String::from("test_run"), csv_config.run_name);
        assert_eq!(ChannelDataType::Float, config.data_type());
        assert_eq!(String::from("channel"), config.name);
        assert_eq!(String::from("km/hr"), config.units);
        assert_eq!(String::from("some_description"), config.description);
    }

    #[test]
    fn enum_type_override() {
        let test_csv = indoc! {"
            time,channel
            2025-10-04T21:58:13Z,0
            2025-10-04T21:58:13Z,1
        "};
        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(test_csv.as_bytes());

        let req = create_data_import_request(
            csv_reader,
            &CsvArgs {
                path: PathBuf::default(),
                asset: "test_asset".into(),
                run: Some("test_run".into()),
                header_row: 1,
                first_data_row: 2,
                channel_column: vec![2],
                data_type: vec![DataType::Enum],
                unit: vec![String::new()],
                description: vec![String::new()],
                enum_config: vec!["0,stop,1,go".into()],
                bit_field_config: Vec::default(),
                time_column: 1,
                time_format: TimeFormat::default(),
                relative_start_time: None,
                wait: false,
            },
        )
        .expect("expected Result::Ok");

        let csv_config = req.csv_config.expect("expected Option::Some");
        assert_eq!(1, csv_config.data_columns.len());
        let config = csv_config.data_columns.get(&2).unwrap();
        assert_eq!(ChannelDataType::Enum, config.data_type());
        assert_eq!(String::from("channel"), config.name);
        assert!(config.units.is_empty());
        assert!(config.description.is_empty());
        assert_eq!(2, config.enum_types.len());
        assert!(
            config
                .enum_types
                .iter()
                .find(|c| c.name == "stop" && c.key == 0)
                .is_some()
        );
        assert!(
            config
                .enum_types
                .iter()
                .find(|c| c.name == "go" && c.key == 1)
                .is_some()
        );
    }

    #[test]
    fn multi_channel_with_overrides_and_empty_cells() {
        // string_channel will have no override and will be inferred
        let test_csv = indoc! {"
            time,float_channel,enum_channel,string_channel
            2025-10-04T21:58:13Z,1.0,,
            2025-10-04T21:58:14Z,1.2,0,
            2025-10-04T21:58:14Z,,1,cthulhu
        "};
        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(test_csv.as_bytes());

        let req = create_data_import_request(
            csv_reader,
            &CsvArgs {
                path: PathBuf::default(),
                asset: "test_asset".into(),
                run: Some("test_run".into()),
                header_row: 1,
                first_data_row: 2,
                channel_column: vec![2, 3],
                data_type: vec![DataType::Float, DataType::Enum],
                unit: vec!["km/hr".into(), String::new()],
                description: vec!["float channel".into(), "enum channel".into()],
                enum_config: vec!["0,stop,1,go".into()],
                bit_field_config: Vec::default(),
                time_column: 1,
                time_format: TimeFormat::default(),
                relative_start_time: None,
                wait: false,
            },
        )
        .expect("expected Result::Ok");

        let csv_config = req.csv_config.expect("expected Option::Some");
        assert_eq!(3, csv_config.data_columns.len());

        // enum channel
        let config = csv_config.data_columns.get(&3).unwrap();
        assert_eq!(ChannelDataType::Enum, config.data_type());
        assert_eq!(String::from("enum_channel"), config.name);
        assert!(config.units.is_empty());
        assert_eq!("enum channel".to_string(), config.description);
        assert_eq!(2, config.enum_types.len());
        assert!(
            config
                .enum_types
                .iter()
                .find(|c| c.name == "stop" && c.key == 0)
                .is_some()
        );
        assert!(
            config
                .enum_types
                .iter()
                .find(|c| c.name == "go" && c.key == 1)
                .is_some()
        );

        // float channel
        let config = csv_config.data_columns.get(&2).unwrap();
        assert_eq!(ChannelDataType::Float, config.data_type());
        assert_eq!(String::from("float_channel"), config.name);
        assert_eq!("km/hr".to_string(), config.units);
        assert_eq!("float channel".to_string(), config.description);

        // string channel
        let config = csv_config.data_columns.get(&4).unwrap();
        assert_eq!(ChannelDataType::String, config.data_type());
        assert_eq!(String::from("string_channel"), config.name);
        assert!(config.units.is_empty());
        assert!(config.description.is_empty());
    }
}
