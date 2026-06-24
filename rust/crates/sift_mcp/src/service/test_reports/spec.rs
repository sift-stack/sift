//! Author-facing JSON spec for `create_test_report` and its mapping to proto requests.
//!
//! The caller describes a report as a nested tree: each step owns measurements and child
//! steps. [`build`] validates that tree and lowers it to the flat proto types, computing
//! `step_path` and recording each step's parent so the service can link `parent_step_id`
//! once the server assigns ids. The layout mirrors the Python pytest plugin
//! (`util/test_results/context_manager.py`): roots are `"1"`, `"2"`, …; a child at index `i`
//! under parent path `P` is `"P.i"`.

use std::collections::BTreeMap;

use anyhow::{Result, anyhow, bail};
use pbjson_types::Timestamp;
use serde::Deserialize;
use sift_rs::{
    metadata::v1::{
        MetadataKey, MetadataKeyType, MetadataValue, metadata_value::Value as MetaInner,
    },
    test_reports::v1::{
        CreateTestReportRequest, ErrorInfo, NumericBounds, StringBounds, TestMeasurement,
        TestMeasurementType, TestStatus, TestStep, TestStepType, test_measurement,
    },
    unit::v2::Unit,
};

/// A scalar metadata value. Untagged so the author writes `"k": "v"`, `"k": 1`, or `"k": true`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MetadataScalar {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportSpec {
    pub name: String,
    pub test_system_name: String,
    pub test_case: String,
    pub status: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub serial_number: Option<String>,
    pub part_number: Option<String>,
    pub system_operator: Option<String>,
    pub run_id: Option<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, MetadataScalar>,
    #[serde(default)]
    pub steps: Vec<StepSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepSpec {
    pub name: String,
    pub status: Option<String>,
    pub step_type: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub error_info: Option<ErrorInfoSpec>,
    #[serde(default)]
    pub metadata: BTreeMap<String, MetadataScalar>,
    #[serde(default)]
    pub measurements: Vec<MeasurementSpec>,
    #[serde(default)]
    pub steps: Vec<StepSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorInfoSpec {
    pub error_code: i32,
    pub error_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementSpec {
    pub name: String,
    pub numeric_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub numeric_bounds: Option<NumericBoundsSpec>,
    pub string_expected: Option<String>,
    pub unit: Option<String>,
    pub passed: Option<bool>,
    pub measurement_type: Option<String>,
    pub timestamp: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub channel_names: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, MetadataScalar>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumericBoundsSpec {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// A fully validated report, ready for the service to write. `request` is the report create
/// request; `steps` is in pre-order so a parent is always created before its children.
pub struct BuiltReport {
    pub request: CreateTestReportRequest,
    pub steps: Vec<BuiltStep>,
}

/// One step plus its measurements, with ids left unset. The service fills `test_report_id`,
/// `parent_step_id` (resolved from `parent_path`), and the measurements' `test_step_id` once
/// the server returns real ids.
pub struct BuiltStep {
    pub step_path: String,
    pub parent_path: Option<String>,
    pub step: TestStep,
    pub measurements: Vec<TestMeasurement>,
}

/// Validate the author spec and lower it to proto messages. Returns an error (mapped to
/// `INVALID_PARAMS` by the caller) when a required field is empty, an enum name is unknown, a
/// timestamp is not RFC3339, or a measurement's value/bounds are inconsistent.
pub fn build(spec: ReportSpec) -> Result<BuiltReport> {
    if spec.name.trim().is_empty() {
        bail!("report `name` must not be empty");
    }
    if spec.test_system_name.trim().is_empty() {
        bail!("report `test_system_name` must not be empty");
    }
    if spec.test_case.trim().is_empty() {
        bail!("report `test_case` must not be empty");
    }

    let start = parse_ts(spec.start_time.as_deref(), "report start_time")?
        .unwrap_or_else(now_timestamp);
    let end = parse_ts(spec.end_time.as_deref(), "report end_time")?.unwrap_or_else(|| start.clone());

    let request = CreateTestReportRequest {
        status: parse_enum::<_>(spec.status.as_deref(), "TEST_STATUS_", "status", |n| {
            TestStatus::from_str_name(n).map(|e| e as i32)
        })?
        .unwrap_or(TestStatus::Passed as i32),
        name: spec.name,
        test_system_name: spec.test_system_name,
        test_case: spec.test_case,
        start_time: Some(start.clone()),
        end_time: Some(end.clone()),
        metadata: metadata_values(spec.metadata),
        serial_number: spec.serial_number.unwrap_or_default(),
        part_number: spec.part_number.unwrap_or_default(),
        system_operator: spec.system_operator.unwrap_or_default(),
        run_id: spec.run_id.unwrap_or_default(),
    };

    let mut steps = Vec::new();
    build_steps(spec.steps, None, &start, &end, &mut steps)?;

    Ok(BuiltReport { request, steps })
}

fn build_steps(
    specs: Vec<StepSpec>,
    parent_path: Option<&str>,
    report_start: &Timestamp,
    report_end: &Timestamp,
    out: &mut Vec<BuiltStep>,
) -> Result<()> {
    for (i, spec) in specs.into_iter().enumerate() {
        let index = i + 1;
        let step_path = match parent_path {
            Some(parent) => format!("{parent}.{index}"),
            None => index.to_string(),
        };

        if spec.name.trim().is_empty() {
            bail!("step `name` must not be empty (at step_path `{step_path}`)");
        }

        let status = parse_enum(spec.status.as_deref(), "TEST_STATUS_", "step status", |n| {
            TestStatus::from_str_name(n).map(|e| e as i32)
        })?
        .unwrap_or(TestStatus::Passed as i32);
        let step_type = parse_enum(
            spec.step_type.as_deref(),
            "TEST_STEP_TYPE_",
            "step_type",
            |n| TestStepType::from_str_name(n).map(|e| e as i32),
        )?
        .unwrap_or(TestStepType::Action as i32);

        let start = parse_ts(spec.start_time.as_deref(), "step start_time")?
            .unwrap_or_else(|| report_start.clone());
        let end = parse_ts(spec.end_time.as_deref(), "step end_time")?
            .unwrap_or_else(|| report_end.clone());

        let step = TestStep {
            name: spec.name,
            description: spec.description.unwrap_or_default(),
            step_type,
            step_path: step_path.clone(),
            status,
            start_time: Some(start.clone()),
            end_time: Some(end),
            error_info: spec.error_info.map(|e| ErrorInfo {
                error_code: e.error_code,
                error_message: e.error_message,
            }),
            metadata: metadata_values(spec.metadata),
            ..Default::default()
        };

        let measurements = spec
            .measurements
            .into_iter()
            .map(|m| build_measurement(m, &start, &step_path))
            .collect::<Result<Vec<_>>>()?;

        out.push(BuiltStep {
            step_path: step_path.clone(),
            parent_path: parent_path.map(str::to_string),
            step,
            measurements,
        });

        build_steps(spec.steps, Some(&step_path), report_start, report_end, out)?;
    }
    Ok(())
}

/// Build measurements to append to an existing step. Validation and value/bounds/`passed`
/// derivation match `create_test_report`; timestamps default to now (there is no parent step to
/// inherit from). `test_step_id`/`test_report_id` are left unset for the service to fill.
pub fn build_measurements(specs: Vec<MeasurementSpec>) -> Result<Vec<TestMeasurement>> {
    let default_ts = now_timestamp();
    specs
        .into_iter()
        .map(|spec| build_measurement(spec, &default_ts, "(appended)"))
        .collect()
}

fn build_measurement(
    spec: MeasurementSpec,
    step_start: &Timestamp,
    step_path: &str,
) -> Result<TestMeasurement> {
    if spec.name.trim().is_empty() {
        bail!("measurement `name` must not be empty (under step_path `{step_path}`)");
    }

    // Exactly one value variant.
    let value_count = spec.numeric_value.is_some() as u8
        + spec.string_value.is_some() as u8
        + spec.boolean_value.is_some() as u8;
    if value_count != 1 {
        bail!(
            "measurement `{}` must set exactly one of `numeric_value`, `string_value`, or \
             `boolean_value` (got {value_count})",
            spec.name
        );
    }

    // Bounds must match the value kind.
    if spec.numeric_bounds.is_some() && spec.numeric_value.is_none() {
        bail!(
            "measurement `{}` has `numeric_bounds` but no `numeric_value`",
            spec.name
        );
    }
    if spec.string_expected.is_some() && spec.string_value.is_none() {
        bail!(
            "measurement `{}` has `string_expected` but no `string_value`",
            spec.name
        );
    }

    let (value, default_type, passed) = if let Some(v) = spec.numeric_value {
        let passed = match (spec.passed, &spec.numeric_bounds) {
            (Some(p), _) => p,
            (None, Some(b)) => passes_numeric(v, b),
            (None, None) => true,
        };
        (
            test_measurement::Value::NumericValue(v),
            TestMeasurementType::Double,
            passed,
        )
    } else if let Some(s) = spec.string_value.clone() {
        let passed = match (spec.passed, &spec.string_expected) {
            (Some(p), _) => p,
            (None, Some(expected)) => &s == expected,
            (None, None) => true,
        };
        (
            test_measurement::Value::StringValue(s),
            TestMeasurementType::String,
            passed,
        )
    } else {
        let b = spec.boolean_value.unwrap();
        (
            test_measurement::Value::BooleanValue(b),
            TestMeasurementType::Boolean,
            spec.passed.unwrap_or(true),
        )
    };

    let bounds = match (spec.numeric_bounds, spec.string_expected) {
        (Some(b), _) => Some(test_measurement::Bounds::NumericBounds(NumericBounds {
            min: b.min,
            max: b.max,
        })),
        (None, Some(expected)) => Some(test_measurement::Bounds::StringBounds(StringBounds {
            expected_value: expected,
        })),
        (None, None) => None,
    };

    let measurement_type = parse_enum(
        spec.measurement_type.as_deref(),
        "TEST_MEASUREMENT_TYPE_",
        "measurement_type",
        |n| TestMeasurementType::from_str_name(n).map(|e| e as i32),
    )?
    .unwrap_or(default_type as i32);

    let timestamp = parse_ts(spec.timestamp.as_deref(), "measurement timestamp")?
        .unwrap_or_else(|| step_start.clone());

    Ok(TestMeasurement {
        measurement_type,
        name: spec.name,
        unit: spec.unit.map(|abbreviated_name| Unit {
            abbreviated_name,
            ..Default::default()
        }),
        passed,
        timestamp: Some(timestamp),
        description: spec.description.unwrap_or_default(),
        channel_names: spec.channel_names,
        metadata: metadata_values(spec.metadata),
        value: Some(value),
        bounds,
        ..Default::default()
    })
}

/// Numeric pass/fail, replicating `util/test_results/bounds.py`: both ends inclusive, an
/// absent bound is unbounded on that side.
fn passes_numeric(value: f64, bounds: &NumericBoundsSpec) -> bool {
    if let Some(min) = bounds.min {
        if value < min {
            return false;
        }
    }
    if let Some(max) = bounds.max {
        if value > max {
            return false;
        }
    }
    true
}

/// Resolve an optional enum string to its `i32` tag. Accepts a bare name (`"PASSED"`) or the
/// fully-qualified one (`"TEST_STATUS_PASSED"`), case-insensitively. `None` input → `Ok(None)`.
fn parse_enum<F>(raw: Option<&str>, prefix: &str, field: &str, lookup: F) -> Result<Option<i32>>
where
    F: Fn(&str) -> Option<i32>,
{
    let Some(raw) = raw else {
        return Ok(None);
    };
    let upper = raw.trim().to_uppercase();
    let qualified = if upper.starts_with(prefix) {
        upper
    } else {
        format!("{prefix}{upper}")
    };
    lookup(&qualified)
        .map(Some)
        .ok_or_else(|| anyhow!("unknown `{field}` value `{raw}`"))
}

fn parse_ts(raw: Option<&str>, field: &str) -> Result<Option<Timestamp>> {
    let Some(raw) = raw else {
        return Ok(None);
    };
    let dt = chrono::DateTime::parse_from_rfc3339(raw.trim())
        .map_err(|e| anyhow!("`{field}` is not a valid RFC3339 timestamp: {e}"))?;
    Ok(Some(Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }))
}

fn now_timestamp() -> Timestamp {
    let now = chrono::Utc::now();
    Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

fn metadata_values(map: BTreeMap<String, MetadataScalar>) -> Vec<MetadataValue> {
    map.into_iter()
        .map(|(name, scalar)| {
            let (key_type, value) = match scalar {
                MetadataScalar::String(s) => (MetadataKeyType::String, MetaInner::StringValue(s)),
                MetadataScalar::Number(n) => (MetadataKeyType::Number, MetaInner::NumberValue(n)),
                MetadataScalar::Boolean(b) => {
                    (MetadataKeyType::Boolean, MetaInner::BooleanValue(b))
                }
            };
            MetadataValue {
                key: Some(MetadataKey {
                    name,
                    r#type: key_type.into(),
                    ..Default::default()
                }),
                value: Some(value),
                ..Default::default()
            }
        })
        .collect()
}
