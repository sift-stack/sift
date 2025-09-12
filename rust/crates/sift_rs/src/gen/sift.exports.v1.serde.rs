// @generated
impl serde::Serialize for AssetsAndTimeRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_ids.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.stop_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.AssetsAndTimeRange", len)?;
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.stop_time.as_ref() {
            struct_ser.serialize_field("stopTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetsAndTimeRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_ids",
            "assetIds",
            "start_time",
            "startTime",
            "stop_time",
            "stopTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetIds,
            StartTime,
            StopTime,
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
                            "assetIds" | "asset_ids" => Ok(GeneratedField::AssetIds),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "stopTime" | "stop_time" => Ok(GeneratedField::StopTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetsAndTimeRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.AssetsAndTimeRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetsAndTimeRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_ids__ = None;
                let mut start_time__ = None;
                let mut stop_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::StopTime => {
                            if stop_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTime"));
                            }
                            stop_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AssetsAndTimeRange {
                    asset_ids: asset_ids__.unwrap_or_default(),
                    start_time: start_time__,
                    stop_time: stop_time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.AssetsAndTimeRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannelConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.channel_references.is_empty() {
            len += 1;
        }
        if self.units.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.CalculatedChannelConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.channel_references.is_empty() {
            struct_ser.serialize_field("channelReferences", &self.channel_references)?;
        }
        if let Some(v) = self.units.as_ref() {
            struct_ser.serialize_field("units", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannelConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "expression",
            "channel_references",
            "channelReferences",
            "units",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Expression,
            ChannelReferences,
            Units,
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
                            "name" => Ok(GeneratedField::Name),
                            "expression" => Ok(GeneratedField::Expression),
                            "channelReferences" | "channel_references" => Ok(GeneratedField::ChannelReferences),
                            "units" => Ok(GeneratedField::Units),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalculatedChannelConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.CalculatedChannelConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut expression__ = None;
                let mut channel_references__ = None;
                let mut units__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelReferences => {
                            if channel_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelReferences"));
                            }
                            channel_references__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Units => {
                            if units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("units"));
                            }
                            units__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CalculatedChannelConfig {
                    name: name__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                    channel_references: channel_references__.unwrap_or_default(),
                    units: units__,
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.CalculatedChannelConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportDataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_ids.is_empty() {
            len += 1;
        }
        if !self.calculated_channel_configs.is_empty() {
            len += 1;
        }
        if self.output_format != 0 {
            len += 1;
        }
        if self.export_options.is_some() {
            len += 1;
        }
        if self.time_selection.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.ExportDataRequest", len)?;
        if !self.channel_ids.is_empty() {
            struct_ser.serialize_field("channelIds", &self.channel_ids)?;
        }
        if !self.calculated_channel_configs.is_empty() {
            struct_ser.serialize_field("calculatedChannelConfigs", &self.calculated_channel_configs)?;
        }
        if self.output_format != 0 {
            let v = ExportOutputFormat::try_from(self.output_format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.output_format)))?;
            struct_ser.serialize_field("outputFormat", &v)?;
        }
        if let Some(v) = self.export_options.as_ref() {
            struct_ser.serialize_field("exportOptions", v)?;
        }
        if let Some(v) = self.time_selection.as_ref() {
            match v {
                export_data_request::TimeSelection::TimeRange(v) => {
                    struct_ser.serialize_field("timeRange", v)?;
                }
                export_data_request::TimeSelection::RunsAndTimeRange(v) => {
                    struct_ser.serialize_field("runsAndTimeRange", v)?;
                }
                export_data_request::TimeSelection::AssetsAndTimeRange(v) => {
                    struct_ser.serialize_field("assetsAndTimeRange", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportDataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_ids",
            "channelIds",
            "calculated_channel_configs",
            "calculatedChannelConfigs",
            "output_format",
            "outputFormat",
            "export_options",
            "exportOptions",
            "time_range",
            "timeRange",
            "runs_and_time_range",
            "runsAndTimeRange",
            "assets_and_time_range",
            "assetsAndTimeRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelIds,
            CalculatedChannelConfigs,
            OutputFormat,
            ExportOptions,
            TimeRange,
            RunsAndTimeRange,
            AssetsAndTimeRange,
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
                            "channelIds" | "channel_ids" => Ok(GeneratedField::ChannelIds),
                            "calculatedChannelConfigs" | "calculated_channel_configs" => Ok(GeneratedField::CalculatedChannelConfigs),
                            "outputFormat" | "output_format" => Ok(GeneratedField::OutputFormat),
                            "exportOptions" | "export_options" => Ok(GeneratedField::ExportOptions),
                            "timeRange" | "time_range" => Ok(GeneratedField::TimeRange),
                            "runsAndTimeRange" | "runs_and_time_range" => Ok(GeneratedField::RunsAndTimeRange),
                            "assetsAndTimeRange" | "assets_and_time_range" => Ok(GeneratedField::AssetsAndTimeRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportDataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.ExportDataRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportDataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_ids__ = None;
                let mut calculated_channel_configs__ = None;
                let mut output_format__ = None;
                let mut export_options__ = None;
                let mut time_selection__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelIds => {
                            if channel_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelIds"));
                            }
                            channel_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CalculatedChannelConfigs => {
                            if calculated_channel_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelConfigs"));
                            }
                            calculated_channel_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputFormat => {
                            if output_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputFormat"));
                            }
                            output_format__ = Some(map_.next_value::<ExportOutputFormat>()? as i32);
                        }
                        GeneratedField::ExportOptions => {
                            if export_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exportOptions"));
                            }
                            export_options__ = map_.next_value()?;
                        }
                        GeneratedField::TimeRange => {
                            if time_selection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeRange"));
                            }
                            time_selection__ = map_.next_value::<::std::option::Option<_>>()?.map(export_data_request::TimeSelection::TimeRange)
;
                        }
                        GeneratedField::RunsAndTimeRange => {
                            if time_selection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runsAndTimeRange"));
                            }
                            time_selection__ = map_.next_value::<::std::option::Option<_>>()?.map(export_data_request::TimeSelection::RunsAndTimeRange)
;
                        }
                        GeneratedField::AssetsAndTimeRange => {
                            if time_selection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetsAndTimeRange"));
                            }
                            time_selection__ = map_.next_value::<::std::option::Option<_>>()?.map(export_data_request::TimeSelection::AssetsAndTimeRange)
;
                        }
                    }
                }
                Ok(ExportDataRequest {
                    channel_ids: channel_ids__.unwrap_or_default(),
                    calculated_channel_configs: calculated_channel_configs__.unwrap_or_default(),
                    output_format: output_format__.unwrap_or_default(),
                    export_options: export_options__,
                    time_selection: time_selection__,
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.ExportDataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportDataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.presigned_url.is_empty() {
            len += 1;
        }
        if !self.job_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.ExportDataResponse", len)?;
        if !self.presigned_url.is_empty() {
            struct_ser.serialize_field("presignedUrl", &self.presigned_url)?;
        }
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportDataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "presigned_url",
            "presignedUrl",
            "job_id",
            "jobId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PresignedUrl,
            JobId,
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
                            "presignedUrl" | "presigned_url" => Ok(GeneratedField::PresignedUrl),
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportDataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.ExportDataResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportDataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut presigned_url__ = None;
                let mut job_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PresignedUrl => {
                            if presigned_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presignedUrl"));
                            }
                            presigned_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExportDataResponse {
                    presigned_url: presigned_url__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.ExportDataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_legacy_format {
            len += 1;
        }
        if self.simplify_channel_names {
            len += 1;
        }
        if self.combine_runs {
            len += 1;
        }
        if self.split_export_by_asset {
            len += 1;
        }
        if self.split_export_by_run {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.ExportOptions", len)?;
        if self.use_legacy_format {
            struct_ser.serialize_field("useLegacyFormat", &self.use_legacy_format)?;
        }
        if self.simplify_channel_names {
            struct_ser.serialize_field("simplifyChannelNames", &self.simplify_channel_names)?;
        }
        if self.combine_runs {
            struct_ser.serialize_field("combineRuns", &self.combine_runs)?;
        }
        if self.split_export_by_asset {
            struct_ser.serialize_field("splitExportByAsset", &self.split_export_by_asset)?;
        }
        if self.split_export_by_run {
            struct_ser.serialize_field("splitExportByRun", &self.split_export_by_run)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_legacy_format",
            "useLegacyFormat",
            "simplify_channel_names",
            "simplifyChannelNames",
            "combine_runs",
            "combineRuns",
            "split_export_by_asset",
            "splitExportByAsset",
            "split_export_by_run",
            "splitExportByRun",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseLegacyFormat,
            SimplifyChannelNames,
            CombineRuns,
            SplitExportByAsset,
            SplitExportByRun,
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
                            "useLegacyFormat" | "use_legacy_format" => Ok(GeneratedField::UseLegacyFormat),
                            "simplifyChannelNames" | "simplify_channel_names" => Ok(GeneratedField::SimplifyChannelNames),
                            "combineRuns" | "combine_runs" => Ok(GeneratedField::CombineRuns),
                            "splitExportByAsset" | "split_export_by_asset" => Ok(GeneratedField::SplitExportByAsset),
                            "splitExportByRun" | "split_export_by_run" => Ok(GeneratedField::SplitExportByRun),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.ExportOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_legacy_format__ = None;
                let mut simplify_channel_names__ = None;
                let mut combine_runs__ = None;
                let mut split_export_by_asset__ = None;
                let mut split_export_by_run__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UseLegacyFormat => {
                            if use_legacy_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useLegacyFormat"));
                            }
                            use_legacy_format__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SimplifyChannelNames => {
                            if simplify_channel_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("simplifyChannelNames"));
                            }
                            simplify_channel_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CombineRuns => {
                            if combine_runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("combineRuns"));
                            }
                            combine_runs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SplitExportByAsset => {
                            if split_export_by_asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splitExportByAsset"));
                            }
                            split_export_by_asset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SplitExportByRun => {
                            if split_export_by_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splitExportByRun"));
                            }
                            split_export_by_run__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExportOptions {
                    use_legacy_format: use_legacy_format__.unwrap_or_default(),
                    simplify_channel_names: simplify_channel_names__.unwrap_or_default(),
                    combine_runs: combine_runs__.unwrap_or_default(),
                    split_export_by_asset: split_export_by_asset__.unwrap_or_default(),
                    split_export_by_run: split_export_by_run__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.ExportOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportOutputFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXPORT_OUTPUT_FORMAT_UNSPECIFIED",
            Self::Csv => "EXPORT_OUTPUT_FORMAT_CSV",
            Self::Sun => "EXPORT_OUTPUT_FORMAT_SUN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExportOutputFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXPORT_OUTPUT_FORMAT_UNSPECIFIED",
            "EXPORT_OUTPUT_FORMAT_CSV",
            "EXPORT_OUTPUT_FORMAT_SUN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportOutputFormat;

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
                    "EXPORT_OUTPUT_FORMAT_UNSPECIFIED" => Ok(ExportOutputFormat::Unspecified),
                    "EXPORT_OUTPUT_FORMAT_CSV" => Ok(ExportOutputFormat::Csv),
                    "EXPORT_OUTPUT_FORMAT_SUN" => Ok(ExportOutputFormat::Sun),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetDownloadUrlRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.GetDownloadUrlRequest", len)?;
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDownloadUrlRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_id",
            "jobId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobId,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDownloadUrlRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.GetDownloadUrlRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDownloadUrlRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetDownloadUrlRequest {
                    job_id: job_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.GetDownloadUrlRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetDownloadUrlResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.presigned_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.GetDownloadUrlResponse", len)?;
        if !self.presigned_url.is_empty() {
            struct_ser.serialize_field("presignedUrl", &self.presigned_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetDownloadUrlResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "presigned_url",
            "presignedUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PresignedUrl,
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
                            "presignedUrl" | "presigned_url" => Ok(GeneratedField::PresignedUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetDownloadUrlResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.GetDownloadUrlResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetDownloadUrlResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut presigned_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PresignedUrl => {
                            if presigned_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presignedUrl"));
                            }
                            presigned_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetDownloadUrlResponse {
                    presigned_url: presigned_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.GetDownloadUrlResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunsAndTimeRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.run_ids.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.stop_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.RunsAndTimeRange", len)?;
        if !self.run_ids.is_empty() {
            struct_ser.serialize_field("runIds", &self.run_ids)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.stop_time.as_ref() {
            struct_ser.serialize_field("stopTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunsAndTimeRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_ids",
            "runIds",
            "start_time",
            "startTime",
            "stop_time",
            "stopTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunIds,
            StartTime,
            StopTime,
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
                            "runIds" | "run_ids" => Ok(GeneratedField::RunIds),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "stopTime" | "stop_time" => Ok(GeneratedField::StopTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunsAndTimeRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.RunsAndTimeRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RunsAndTimeRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_ids__ = None;
                let mut start_time__ = None;
                let mut stop_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RunIds => {
                            if run_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runIds"));
                            }
                            run_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::StopTime => {
                            if stop_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTime"));
                            }
                            stop_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RunsAndTimeRange {
                    run_ids: run_ids__.unwrap_or_default(),
                    start_time: start_time__,
                    stop_time: stop_time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.RunsAndTimeRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start_time.is_some() {
            len += 1;
        }
        if self.stop_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.exports.v1.TimeRange", len)?;
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.stop_time.as_ref() {
            struct_ser.serialize_field("stopTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_time",
            "startTime",
            "stop_time",
            "stopTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StartTime,
            StopTime,
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
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "stopTime" | "stop_time" => Ok(GeneratedField::StopTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.exports.v1.TimeRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimeRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_time__ = None;
                let mut stop_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::StopTime => {
                            if stop_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopTime"));
                            }
                            stop_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TimeRange {
                    start_time: start_time__,
                    stop_time: stop_time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.exports.v1.TimeRange", FIELDS, GeneratedVisitor)
    }
}
