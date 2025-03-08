// @generated
impl serde::Serialize for Ch10Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.run_name.is_empty() {
            len += 1;
        }
        if self.scale_values {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.Ch10Config", len)?;
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.run_name.is_empty() {
            struct_ser.serialize_field("runName", &self.run_name)?;
        }
        if self.scale_values {
            struct_ser.serialize_field("scaleValues", &self.scale_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ch10Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_name",
            "assetName",
            "run_name",
            "runName",
            "scale_values",
            "scaleValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetName,
            RunName,
            ScaleValues,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "runName" | "run_name" => Ok(GeneratedField::RunName),
                            "scaleValues" | "scale_values" => Ok(GeneratedField::ScaleValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ch10Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.Ch10Config")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Ch10Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_name__ = None;
                let mut run_name__ = None;
                let mut scale_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunName => {
                            if run_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runName"));
                            }
                            run_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ScaleValues => {
                            if scale_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scaleValues"));
                            }
                            scale_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Ch10Config {
                    asset_name: asset_name__.unwrap_or_default(),
                    run_name: run_name__.unwrap_or_default(),
                    scale_values: scale_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.Ch10Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDataImportFromUploadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.csv_config.is_some() {
            len += 1;
        }
        if self.ch10_config.is_some() {
            len += 1;
        }
        if self.tdms_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.CreateDataImportFromUploadRequest", len)?;
        if let Some(v) = self.csv_config.as_ref() {
            struct_ser.serialize_field("csvConfig", v)?;
        }
        if let Some(v) = self.ch10_config.as_ref() {
            struct_ser.serialize_field("ch10Config", v)?;
        }
        if let Some(v) = self.tdms_config.as_ref() {
            struct_ser.serialize_field("tdmsConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDataImportFromUploadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "csv_config",
            "csvConfig",
            "ch10_config",
            "ch10Config",
            "tdms_config",
            "tdmsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CsvConfig,
            Ch10Config,
            TdmsConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "csvConfig" | "csv_config" => Ok(GeneratedField::CsvConfig),
                            "ch10Config" | "ch10_config" => Ok(GeneratedField::Ch10Config),
                            "tdmsConfig" | "tdms_config" => Ok(GeneratedField::TdmsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDataImportFromUploadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.CreateDataImportFromUploadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDataImportFromUploadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut csv_config__ = None;
                let mut ch10_config__ = None;
                let mut tdms_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CsvConfig => {
                            if csv_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("csvConfig"));
                            }
                            csv_config__ = map_.next_value()?;
                        }
                        GeneratedField::Ch10Config => {
                            if ch10_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ch10Config"));
                            }
                            ch10_config__ = map_.next_value()?;
                        }
                        GeneratedField::TdmsConfig => {
                            if tdms_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tdmsConfig"));
                            }
                            tdms_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateDataImportFromUploadRequest {
                    csv_config: csv_config__,
                    ch10_config: ch10_config__,
                    tdms_config: tdms_config__,
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.CreateDataImportFromUploadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDataImportFromUploadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.upload_url.is_empty() {
            len += 1;
        }
        if !self.data_import_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.CreateDataImportFromUploadResponse", len)?;
        if !self.upload_url.is_empty() {
            struct_ser.serialize_field("uploadUrl", &self.upload_url)?;
        }
        if !self.data_import_id.is_empty() {
            struct_ser.serialize_field("dataImportId", &self.data_import_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDataImportFromUploadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upload_url",
            "uploadUrl",
            "data_import_id",
            "dataImportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UploadUrl,
            DataImportId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uploadUrl" | "upload_url" => Ok(GeneratedField::UploadUrl),
                            "dataImportId" | "data_import_id" => Ok(GeneratedField::DataImportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDataImportFromUploadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.CreateDataImportFromUploadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDataImportFromUploadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upload_url__ = None;
                let mut data_import_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UploadUrl => {
                            if upload_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uploadUrl"));
                            }
                            upload_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataImportId => {
                            if data_import_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImportId"));
                            }
                            data_import_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateDataImportFromUploadResponse {
                    upload_url: upload_url__.unwrap_or_default(),
                    data_import_id: data_import_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.CreateDataImportFromUploadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDataImportFromUrlRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        if self.csv_config.is_some() {
            len += 1;
        }
        if self.ch10_config.is_some() {
            len += 1;
        }
        if self.tdms_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.CreateDataImportFromUrlRequest", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if let Some(v) = self.csv_config.as_ref() {
            struct_ser.serialize_field("csvConfig", v)?;
        }
        if let Some(v) = self.ch10_config.as_ref() {
            struct_ser.serialize_field("ch10Config", v)?;
        }
        if let Some(v) = self.tdms_config.as_ref() {
            struct_ser.serialize_field("tdmsConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDataImportFromUrlRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
            "csv_config",
            "csvConfig",
            "ch10_config",
            "ch10Config",
            "tdms_config",
            "tdmsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
            CsvConfig,
            Ch10Config,
            TdmsConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "url" => Ok(GeneratedField::Url),
                            "csvConfig" | "csv_config" => Ok(GeneratedField::CsvConfig),
                            "ch10Config" | "ch10_config" => Ok(GeneratedField::Ch10Config),
                            "tdmsConfig" | "tdms_config" => Ok(GeneratedField::TdmsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDataImportFromUrlRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.CreateDataImportFromUrlRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDataImportFromUrlRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                let mut csv_config__ = None;
                let mut ch10_config__ = None;
                let mut tdms_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CsvConfig => {
                            if csv_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("csvConfig"));
                            }
                            csv_config__ = map_.next_value()?;
                        }
                        GeneratedField::Ch10Config => {
                            if ch10_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ch10Config"));
                            }
                            ch10_config__ = map_.next_value()?;
                        }
                        GeneratedField::TdmsConfig => {
                            if tdms_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tdmsConfig"));
                            }
                            tdms_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateDataImportFromUrlRequest {
                    url: url__.unwrap_or_default(),
                    csv_config: csv_config__,
                    ch10_config: ch10_config__,
                    tdms_config: tdms_config__,
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.CreateDataImportFromUrlRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateDataImportFromUrlResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data_import_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.CreateDataImportFromUrlResponse", len)?;
        if !self.data_import_id.is_empty() {
            struct_ser.serialize_field("dataImportId", &self.data_import_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateDataImportFromUrlResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_import_id",
            "dataImportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataImportId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dataImportId" | "data_import_id" => Ok(GeneratedField::DataImportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateDataImportFromUrlResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.CreateDataImportFromUrlResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateDataImportFromUrlResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_import_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataImportId => {
                            if data_import_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImportId"));
                            }
                            data_import_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateDataImportFromUrlResponse {
                    data_import_id: data_import_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.CreateDataImportFromUrlResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CsvConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.run_name.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        if self.first_data_row != 0 {
            len += 1;
        }
        if self.time_column.is_some() {
            len += 1;
        }
        if !self.data_columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.CsvConfig", len)?;
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.run_name.is_empty() {
            struct_ser.serialize_field("runName", &self.run_name)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if self.first_data_row != 0 {
            struct_ser.serialize_field("firstDataRow", &self.first_data_row)?;
        }
        if let Some(v) = self.time_column.as_ref() {
            struct_ser.serialize_field("timeColumn", v)?;
        }
        if !self.data_columns.is_empty() {
            struct_ser.serialize_field("dataColumns", &self.data_columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CsvConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_name",
            "assetName",
            "run_name",
            "runName",
            "run_id",
            "runId",
            "first_data_row",
            "firstDataRow",
            "time_column",
            "timeColumn",
            "data_columns",
            "dataColumns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetName,
            RunName,
            RunId,
            FirstDataRow,
            TimeColumn,
            DataColumns,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "runName" | "run_name" => Ok(GeneratedField::RunName),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "firstDataRow" | "first_data_row" => Ok(GeneratedField::FirstDataRow),
                            "timeColumn" | "time_column" => Ok(GeneratedField::TimeColumn),
                            "dataColumns" | "data_columns" => Ok(GeneratedField::DataColumns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CsvConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.CsvConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CsvConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_name__ = None;
                let mut run_name__ = None;
                let mut run_id__ = None;
                let mut first_data_row__ = None;
                let mut time_column__ = None;
                let mut data_columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunName => {
                            if run_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runName"));
                            }
                            run_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FirstDataRow => {
                            if first_data_row__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstDataRow"));
                            }
                            first_data_row__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TimeColumn => {
                            if time_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeColumn"));
                            }
                            time_column__ = map_.next_value()?;
                        }
                        GeneratedField::DataColumns => {
                            if data_columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataColumns"));
                            }
                            data_columns__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u32>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                    }
                }
                Ok(CsvConfig {
                    asset_name: asset_name__.unwrap_or_default(),
                    run_name: run_name__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    first_data_row: first_data_row__.unwrap_or_default(),
                    time_column: time_column__,
                    data_columns: data_columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.CsvConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CsvTimeColumn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.column_number != 0 {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        if self.relative_start_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.CsvTimeColumn", len)?;
        if self.column_number != 0 {
            struct_ser.serialize_field("columnNumber", &self.column_number)?;
        }
        if self.format != 0 {
            let v = TimeFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        if let Some(v) = self.relative_start_time.as_ref() {
            struct_ser.serialize_field("relativeStartTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CsvTimeColumn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "column_number",
            "columnNumber",
            "format",
            "relative_start_time",
            "relativeStartTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ColumnNumber,
            Format,
            RelativeStartTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "columnNumber" | "column_number" => Ok(GeneratedField::ColumnNumber),
                            "format" => Ok(GeneratedField::Format),
                            "relativeStartTime" | "relative_start_time" => Ok(GeneratedField::RelativeStartTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CsvTimeColumn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.CsvTimeColumn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CsvTimeColumn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut column_number__ = None;
                let mut format__ = None;
                let mut relative_start_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ColumnNumber => {
                            if column_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnNumber"));
                            }
                            column_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<TimeFormat>()? as i32);
                        }
                        GeneratedField::RelativeStartTime => {
                            if relative_start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relativeStartTime"));
                            }
                            relative_start_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CsvTimeColumn {
                    column_number: column_number__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                    relative_start_time: relative_start_time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.CsvTimeColumn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataImport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data_import_id.is_empty() {
            len += 1;
        }
        if !self.source_url.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.error_message.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if self.csv_config.is_some() {
            len += 1;
        }
        if self.ch10_config.is_some() {
            len += 1;
        }
        if self.tdms_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.DataImport", len)?;
        if !self.data_import_id.is_empty() {
            struct_ser.serialize_field("dataImportId", &self.data_import_id)?;
        }
        if !self.source_url.is_empty() {
            struct_ser.serialize_field("sourceUrl", &self.source_url)?;
        }
        if self.status != 0 {
            let v = DataImportStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if let Some(v) = self.csv_config.as_ref() {
            struct_ser.serialize_field("csvConfig", v)?;
        }
        if let Some(v) = self.ch10_config.as_ref() {
            struct_ser.serialize_field("ch10Config", v)?;
        }
        if let Some(v) = self.tdms_config.as_ref() {
            struct_ser.serialize_field("tdmsConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataImport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_import_id",
            "dataImportId",
            "source_url",
            "sourceUrl",
            "status",
            "error_message",
            "errorMessage",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "csv_config",
            "csvConfig",
            "ch10_config",
            "ch10Config",
            "tdms_config",
            "tdmsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataImportId,
            SourceUrl,
            Status,
            ErrorMessage,
            CreatedDate,
            ModifiedDate,
            CsvConfig,
            Ch10Config,
            TdmsConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dataImportId" | "data_import_id" => Ok(GeneratedField::DataImportId),
                            "sourceUrl" | "source_url" => Ok(GeneratedField::SourceUrl),
                            "status" => Ok(GeneratedField::Status),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "csvConfig" | "csv_config" => Ok(GeneratedField::CsvConfig),
                            "ch10Config" | "ch10_config" => Ok(GeneratedField::Ch10Config),
                            "tdmsConfig" | "tdms_config" => Ok(GeneratedField::TdmsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataImport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.DataImport")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataImport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_import_id__ = None;
                let mut source_url__ = None;
                let mut status__ = None;
                let mut error_message__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut csv_config__ = None;
                let mut ch10_config__ = None;
                let mut tdms_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataImportId => {
                            if data_import_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImportId"));
                            }
                            data_import_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceUrl => {
                            if source_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceUrl"));
                            }
                            source_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<DataImportStatus>()? as i32);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedDate => {
                            if created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdDate"));
                            }
                            created_date__ = map_.next_value()?;
                        }
                        GeneratedField::ModifiedDate => {
                            if modified_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedDate"));
                            }
                            modified_date__ = map_.next_value()?;
                        }
                        GeneratedField::CsvConfig => {
                            if csv_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("csvConfig"));
                            }
                            csv_config__ = map_.next_value()?;
                        }
                        GeneratedField::Ch10Config => {
                            if ch10_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ch10Config"));
                            }
                            ch10_config__ = map_.next_value()?;
                        }
                        GeneratedField::TdmsConfig => {
                            if tdms_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tdmsConfig"));
                            }
                            tdms_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DataImport {
                    data_import_id: data_import_id__.unwrap_or_default(),
                    source_url: source_url__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    error_message: error_message__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    csv_config: csv_config__,
                    ch10_config: ch10_config__,
                    tdms_config: tdms_config__,
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.DataImport", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataImportStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "DATA_IMPORT_STATUS_UNSPECIFIED",
            Self::Pending => "DATA_IMPORT_STATUS_PENDING",
            Self::InProgress => "DATA_IMPORT_STATUS_IN_PROGRESS",
            Self::Succeeded => "DATA_IMPORT_STATUS_SUCCEEDED",
            Self::Failed => "DATA_IMPORT_STATUS_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for DataImportStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DATA_IMPORT_STATUS_UNSPECIFIED",
            "DATA_IMPORT_STATUS_PENDING",
            "DATA_IMPORT_STATUS_IN_PROGRESS",
            "DATA_IMPORT_STATUS_SUCCEEDED",
            "DATA_IMPORT_STATUS_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataImportStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DATA_IMPORT_STATUS_UNSPECIFIED" => Ok(DataImportStatus::Unspecified),
                    "DATA_IMPORT_STATUS_PENDING" => Ok(DataImportStatus::Pending),
                    "DATA_IMPORT_STATUS_IN_PROGRESS" => Ok(DataImportStatus::InProgress),
                    "DATA_IMPORT_STATUS_SUCCEEDED" => Ok(DataImportStatus::Succeeded),
                    "DATA_IMPORT_STATUS_FAILED" => Ok(DataImportStatus::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DetectConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.DetectConfigRequest", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DetectConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DetectConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.DetectConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DetectConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DetectConfigRequest {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.DetectConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DetectConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.csv_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.DetectConfigResponse", len)?;
        if let Some(v) = self.csv_config.as_ref() {
            struct_ser.serialize_field("csvConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DetectConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "csv_config",
            "csvConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CsvConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "csvConfig" | "csv_config" => Ok(GeneratedField::CsvConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DetectConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.DetectConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DetectConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut csv_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CsvConfig => {
                            if csv_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("csvConfig"));
                            }
                            csv_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DetectConfigResponse {
                    csv_config: csv_config__,
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.DetectConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDataImportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data_import_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.GetDataImportRequest", len)?;
        if !self.data_import_id.is_empty() {
            struct_ser.serialize_field("dataImportId", &self.data_import_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDataImportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_import_id",
            "dataImportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataImportId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dataImportId" | "data_import_id" => Ok(GeneratedField::DataImportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDataImportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.GetDataImportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDataImportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_import_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataImportId => {
                            if data_import_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImportId"));
                            }
                            data_import_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetDataImportRequest {
                    data_import_id: data_import_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.GetDataImportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDataImportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data_import.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.GetDataImportResponse", len)?;
        if let Some(v) = self.data_import.as_ref() {
            struct_ser.serialize_field("dataImport", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDataImportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_import",
            "dataImport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataImport,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dataImport" | "data_import" => Ok(GeneratedField::DataImport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDataImportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.GetDataImportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDataImportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_import__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataImport => {
                            if data_import__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImport"));
                            }
                            data_import__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetDataImportResponse {
                    data_import: data_import__,
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.GetDataImportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDataImportsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.ListDataImportsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDataImportsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrderBy,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDataImportsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.ListDataImportsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDataImportsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDataImportsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.ListDataImportsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDataImportsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data_imports.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.ListDataImportsResponse", len)?;
        if !self.data_imports.is_empty() {
            struct_ser.serialize_field("dataImports", &self.data_imports)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDataImportsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_imports",
            "dataImports",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataImports,
            NextPageToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dataImports" | "data_imports" => Ok(GeneratedField::DataImports),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDataImportsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.ListDataImportsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDataImportsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_imports__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataImports => {
                            if data_imports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImports"));
                            }
                            data_imports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDataImportsResponse {
                    data_imports: data_imports__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.ListDataImportsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryDataImportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data_import_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.RetryDataImportRequest", len)?;
        if !self.data_import_id.is_empty() {
            struct_ser.serialize_field("dataImportId", &self.data_import_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryDataImportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_import_id",
            "dataImportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataImportId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dataImportId" | "data_import_id" => Ok(GeneratedField::DataImportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryDataImportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.RetryDataImportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryDataImportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_import_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataImportId => {
                            if data_import_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImportId"));
                            }
                            data_import_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RetryDataImportRequest {
                    data_import_id: data_import_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.RetryDataImportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryDataImportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.data_imports.v2.RetryDataImportResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryDataImportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryDataImportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.RetryDataImportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryDataImportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RetryDataImportResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.RetryDataImportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TdmsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.run_name.is_empty() {
            len += 1;
        }
        if self.start_time_override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.data_imports.v2.TDMSConfig", len)?;
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.run_name.is_empty() {
            struct_ser.serialize_field("runName", &self.run_name)?;
        }
        if let Some(v) = self.start_time_override.as_ref() {
            struct_ser.serialize_field("startTimeOverride", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TdmsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_name",
            "assetName",
            "run_name",
            "runName",
            "start_time_override",
            "startTimeOverride",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetName,
            RunName,
            StartTimeOverride,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "runName" | "run_name" => Ok(GeneratedField::RunName),
                            "startTimeOverride" | "start_time_override" => Ok(GeneratedField::StartTimeOverride),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TdmsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.data_imports.v2.TDMSConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TdmsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_name__ = None;
                let mut run_name__ = None;
                let mut start_time_override__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunName => {
                            if run_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runName"));
                            }
                            run_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTimeOverride => {
                            if start_time_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTimeOverride"));
                            }
                            start_time_override__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TdmsConfig {
                    asset_name: asset_name__.unwrap_or_default(),
                    run_name: run_name__.unwrap_or_default(),
                    start_time_override: start_time_override__,
                })
            }
        }
        deserializer.deserialize_struct("sift.data_imports.v2.TDMSConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TIME_FORMAT_UNSPECIFIED",
            Self::RelativeNanoseconds => "TIME_FORMAT_RELATIVE_NANOSECONDS",
            Self::RelativeMicroseconds => "TIME_FORMAT_RELATIVE_MICROSECONDS",
            Self::RelativeMilliseconds => "TIME_FORMAT_RELATIVE_MILLISECONDS",
            Self::RelativeSeconds => "TIME_FORMAT_RELATIVE_SECONDS",
            Self::RelativeMinutes => "TIME_FORMAT_RELATIVE_MINUTES",
            Self::RelativeHours => "TIME_FORMAT_RELATIVE_HOURS",
            Self::AbsoluteRfc3339 => "TIME_FORMAT_ABSOLUTE_RFC3339",
            Self::AbsoluteDatetime => "TIME_FORMAT_ABSOLUTE_DATETIME",
            Self::AbsoluteUnixSeconds => "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS",
            Self::AbsoluteUnixMilliseconds => "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS",
            Self::AbsoluteUnixMicroseconds => "TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS",
            Self::AbsoluteUnixNanoseconds => "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TimeFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TIME_FORMAT_UNSPECIFIED",
            "TIME_FORMAT_RELATIVE_NANOSECONDS",
            "TIME_FORMAT_RELATIVE_MICROSECONDS",
            "TIME_FORMAT_RELATIVE_MILLISECONDS",
            "TIME_FORMAT_RELATIVE_SECONDS",
            "TIME_FORMAT_RELATIVE_MINUTES",
            "TIME_FORMAT_RELATIVE_HOURS",
            "TIME_FORMAT_ABSOLUTE_RFC3339",
            "TIME_FORMAT_ABSOLUTE_DATETIME",
            "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS",
            "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS",
            "TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS",
            "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TIME_FORMAT_UNSPECIFIED" => Ok(TimeFormat::Unspecified),
                    "TIME_FORMAT_RELATIVE_NANOSECONDS" => Ok(TimeFormat::RelativeNanoseconds),
                    "TIME_FORMAT_RELATIVE_MICROSECONDS" => Ok(TimeFormat::RelativeMicroseconds),
                    "TIME_FORMAT_RELATIVE_MILLISECONDS" => Ok(TimeFormat::RelativeMilliseconds),
                    "TIME_FORMAT_RELATIVE_SECONDS" => Ok(TimeFormat::RelativeSeconds),
                    "TIME_FORMAT_RELATIVE_MINUTES" => Ok(TimeFormat::RelativeMinutes),
                    "TIME_FORMAT_RELATIVE_HOURS" => Ok(TimeFormat::RelativeHours),
                    "TIME_FORMAT_ABSOLUTE_RFC3339" => Ok(TimeFormat::AbsoluteRfc3339),
                    "TIME_FORMAT_ABSOLUTE_DATETIME" => Ok(TimeFormat::AbsoluteDatetime),
                    "TIME_FORMAT_ABSOLUTE_UNIX_SECONDS" => Ok(TimeFormat::AbsoluteUnixSeconds),
                    "TIME_FORMAT_ABSOLUTE_UNIX_MILLISECONDS" => Ok(TimeFormat::AbsoluteUnixMilliseconds),
                    "TIME_FORMAT_ABSOLUTE_UNIX_MICROSECONDS" => Ok(TimeFormat::AbsoluteUnixMicroseconds),
                    "TIME_FORMAT_ABSOLUTE_UNIX_NANOSECONDS" => Ok(TimeFormat::AbsoluteUnixNanoseconds),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
