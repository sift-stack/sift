// @generated
impl serde::Serialize for AssetsTimeRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.assets.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.AssetsTimeRange", len)?;
        if let Some(v) = self.assets.as_ref() {
            struct_ser.serialize_field("assets", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetsTimeRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assets",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assets,
            StartTime,
            EndTime,
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
                            "assets" => Ok(GeneratedField::Assets),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetsTimeRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.AssetsTimeRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetsTimeRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assets__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AssetsTimeRange {
                    assets: assets__,
                    start_time: start_time__,
                    end_time: end_time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.AssetsTimeRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesAnnotationOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tags.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesAnnotationOptions", len)?;
        if let Some(v) = self.tags.as_ref() {
            struct_ser.serialize_field("tags", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesAnnotationOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tags,
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
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesAnnotationOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesAnnotationOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesAnnotationOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tags__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EvaluateRulesAnnotationOptions {
                    tags: tags__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesAnnotationOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesFromCurrentRuleVersions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromCurrentRuleVersions", len)?;
        if let Some(v) = self.rules.as_ref() {
            struct_ser.serialize_field("rules", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesFromCurrentRuleVersions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesFromCurrentRuleVersions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesFromCurrentRuleVersions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesFromCurrentRuleVersions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EvaluateRulesFromCurrentRuleVersions {
                    rules: rules__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromCurrentRuleVersions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesFromReportTemplate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report_template.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromReportTemplate", len)?;
        if let Some(v) = self.report_template.as_ref() {
            struct_ser.serialize_field("reportTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesFromReportTemplate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_template",
            "reportTemplate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportTemplate,
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
                            "reportTemplate" | "report_template" => Ok(GeneratedField::ReportTemplate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesFromReportTemplate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesFromReportTemplate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesFromReportTemplate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_template__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportTemplate => {
                            if report_template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplate"));
                            }
                            report_template__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EvaluateRulesFromReportTemplate {
                    report_template: report_template__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromReportTemplate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesFromRuleConfigs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromRuleConfigs", len)?;
        if !self.configs.is_empty() {
            struct_ser.serialize_field("configs", &self.configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesFromRuleConfigs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "configs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Configs,
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
                            "configs" => Ok(GeneratedField::Configs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesFromRuleConfigs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesFromRuleConfigs")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesFromRuleConfigs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Configs => {
                            if configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configs"));
                            }
                            configs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvaluateRulesFromRuleConfigs {
                    configs: configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromRuleConfigs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesFromRuleVersions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_version_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromRuleVersions", len)?;
        if !self.rule_version_ids.is_empty() {
            struct_ser.serialize_field("ruleVersionIds", &self.rule_version_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesFromRuleVersions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_version_ids",
            "ruleVersionIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleVersionIds,
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
                            "ruleVersionIds" | "rule_version_ids" => Ok(GeneratedField::RuleVersionIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesFromRuleVersions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesFromRuleVersions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesFromRuleVersions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_version_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleVersionIds => {
                            if rule_version_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersionIds"));
                            }
                            rule_version_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvaluateRulesFromRuleVersions {
                    rule_version_ids: rule_version_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesFromRuleVersions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesPreviewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.mode.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesPreviewRequest", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.time.as_ref() {
            match v {
                evaluate_rules_preview_request::Time::Run(v) => {
                    struct_ser.serialize_field("run", v)?;
                }
                evaluate_rules_preview_request::Time::RunTimeRange(v) => {
                    struct_ser.serialize_field("runTimeRange", v)?;
                }
            }
        }
        if let Some(v) = self.mode.as_ref() {
            match v {
                evaluate_rules_preview_request::Mode::Rules(v) => {
                    struct_ser.serialize_field("rules", v)?;
                }
                evaluate_rules_preview_request::Mode::RuleVersions(v) => {
                    struct_ser.serialize_field("ruleVersions", v)?;
                }
                evaluate_rules_preview_request::Mode::ReportTemplate(v) => {
                    struct_ser.serialize_field("reportTemplate", v)?;
                }
                evaluate_rules_preview_request::Mode::RuleConfigs(v) => {
                    struct_ser.serialize_field("ruleConfigs", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesPreviewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
            "run",
            "run_time_range",
            "runTimeRange",
            "rules",
            "rule_versions",
            "ruleVersions",
            "report_template",
            "reportTemplate",
            "rule_configs",
            "ruleConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
            Run,
            RunTimeRange,
            Rules,
            RuleVersions,
            ReportTemplate,
            RuleConfigs,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "run" => Ok(GeneratedField::Run),
                            "runTimeRange" | "run_time_range" => Ok(GeneratedField::RunTimeRange),
                            "rules" => Ok(GeneratedField::Rules),
                            "ruleVersions" | "rule_versions" => Ok(GeneratedField::RuleVersions),
                            "reportTemplate" | "report_template" => Ok(GeneratedField::ReportTemplate),
                            "ruleConfigs" | "rule_configs" => Ok(GeneratedField::RuleConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesPreviewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesPreviewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesPreviewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                let mut time__ = None;
                let mut mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Run => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            time__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_preview_request::Time::Run)
;
                        }
                        GeneratedField::RunTimeRange => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runTimeRange"));
                            }
                            time__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_preview_request::Time::RunTimeRange)
;
                        }
                        GeneratedField::Rules => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_preview_request::Mode::Rules)
;
                        }
                        GeneratedField::RuleVersions => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersions"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_preview_request::Mode::RuleVersions)
;
                        }
                        GeneratedField::ReportTemplate => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplate"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_preview_request::Mode::ReportTemplate)
;
                        }
                        GeneratedField::RuleConfigs => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleConfigs"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_preview_request::Mode::RuleConfigs)
;
                        }
                    }
                }
                Ok(EvaluateRulesPreviewRequest {
                    organization_id: organization_id__.unwrap_or_default(),
                    time: time__,
                    mode: mode__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesPreviewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesPreviewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.created_annotation_count != 0 {
            len += 1;
        }
        if !self.dry_run_annotations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesPreviewResponse", len)?;
        if self.created_annotation_count != 0 {
            struct_ser.serialize_field("createdAnnotationCount", &self.created_annotation_count)?;
        }
        if !self.dry_run_annotations.is_empty() {
            struct_ser.serialize_field("dryRunAnnotations", &self.dry_run_annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesPreviewResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "created_annotation_count",
            "createdAnnotationCount",
            "dry_run_annotations",
            "dryRunAnnotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedAnnotationCount,
            DryRunAnnotations,
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
                            "createdAnnotationCount" | "created_annotation_count" => Ok(GeneratedField::CreatedAnnotationCount),
                            "dryRunAnnotations" | "dry_run_annotations" => Ok(GeneratedField::DryRunAnnotations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesPreviewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesPreviewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesPreviewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut created_annotation_count__ = None;
                let mut dry_run_annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreatedAnnotationCount => {
                            if created_annotation_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAnnotationCount"));
                            }
                            created_annotation_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DryRunAnnotations => {
                            if dry_run_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRunAnnotations"));
                            }
                            dry_run_annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvaluateRulesPreviewResponse {
                    created_annotation_count: created_annotation_count__.unwrap_or_default(),
                    dry_run_annotations: dry_run_annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesPreviewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation_options.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.report_name.is_some() {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.mode.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesRequest", len)?;
        if let Some(v) = self.annotation_options.as_ref() {
            struct_ser.serialize_field("annotationOptions", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.report_name.as_ref() {
            struct_ser.serialize_field("reportName", v)?;
        }
        if let Some(v) = self.time.as_ref() {
            match v {
                evaluate_rules_request::Time::Run(v) => {
                    struct_ser.serialize_field("run", v)?;
                }
                evaluate_rules_request::Time::Assets(v) => {
                    struct_ser.serialize_field("assets", v)?;
                }
                evaluate_rules_request::Time::RunTimeRange(v) => {
                    struct_ser.serialize_field("runTimeRange", v)?;
                }
            }
        }
        if let Some(v) = self.mode.as_ref() {
            match v {
                evaluate_rules_request::Mode::Rules(v) => {
                    struct_ser.serialize_field("rules", v)?;
                }
                evaluate_rules_request::Mode::RuleVersions(v) => {
                    struct_ser.serialize_field("ruleVersions", v)?;
                }
                evaluate_rules_request::Mode::ReportTemplate(v) => {
                    struct_ser.serialize_field("reportTemplate", v)?;
                }
                evaluate_rules_request::Mode::AllApplicableRules(v) => {
                    struct_ser.serialize_field("allApplicableRules", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_options",
            "annotationOptions",
            "organization_id",
            "organizationId",
            "report_name",
            "reportName",
            "run",
            "assets",
            "run_time_range",
            "runTimeRange",
            "rules",
            "rule_versions",
            "ruleVersions",
            "report_template",
            "reportTemplate",
            "all_applicable_rules",
            "allApplicableRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationOptions,
            OrganizationId,
            ReportName,
            Run,
            Assets,
            RunTimeRange,
            Rules,
            RuleVersions,
            ReportTemplate,
            AllApplicableRules,
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
                            "annotationOptions" | "annotation_options" => Ok(GeneratedField::AnnotationOptions),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "reportName" | "report_name" => Ok(GeneratedField::ReportName),
                            "run" => Ok(GeneratedField::Run),
                            "assets" => Ok(GeneratedField::Assets),
                            "runTimeRange" | "run_time_range" => Ok(GeneratedField::RunTimeRange),
                            "rules" => Ok(GeneratedField::Rules),
                            "ruleVersions" | "rule_versions" => Ok(GeneratedField::RuleVersions),
                            "reportTemplate" | "report_template" => Ok(GeneratedField::ReportTemplate),
                            "allApplicableRules" | "all_applicable_rules" => Ok(GeneratedField::AllApplicableRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluateRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_options__ = None;
                let mut organization_id__ = None;
                let mut report_name__ = None;
                let mut time__ = None;
                let mut mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationOptions => {
                            if annotation_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationOptions"));
                            }
                            annotation_options__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportName => {
                            if report_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportName"));
                            }
                            report_name__ = map_.next_value()?;
                        }
                        GeneratedField::Run => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            time__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Time::Run)
;
                        }
                        GeneratedField::Assets => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            time__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Time::Assets)
;
                        }
                        GeneratedField::RunTimeRange => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runTimeRange"));
                            }
                            time__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Time::RunTimeRange)
;
                        }
                        GeneratedField::Rules => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Mode::Rules)
;
                        }
                        GeneratedField::RuleVersions => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersions"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Mode::RuleVersions)
;
                        }
                        GeneratedField::ReportTemplate => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplate"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Mode::ReportTemplate)
;
                        }
                        GeneratedField::AllApplicableRules => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allApplicableRules"));
                            }
                            mode__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Mode::AllApplicableRules);
                        }
                    }
                }
                Ok(EvaluateRulesRequest {
                    annotation_options: annotation_options__,
                    organization_id: organization_id__.unwrap_or_default(),
                    report_name: report_name__,
                    time: time__,
                    mode: mode__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluateRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.created_annotation_count != 0 {
            len += 1;
        }
        if self.report_id.is_some() {
            len += 1;
        }
        if self.job_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.EvaluateRulesResponse", len)?;
        if self.created_annotation_count != 0 {
            struct_ser.serialize_field("createdAnnotationCount", &self.created_annotation_count)?;
        }
        if let Some(v) = self.report_id.as_ref() {
            struct_ser.serialize_field("reportId", v)?;
        }
        if let Some(v) = self.job_id.as_ref() {
            struct_ser.serialize_field("jobId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluateRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "created_annotation_count",
            "createdAnnotationCount",
            "report_id",
            "reportId",
            "job_id",
            "jobId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedAnnotationCount,
            ReportId,
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
                            "createdAnnotationCount" | "created_annotation_count" => Ok(GeneratedField::CreatedAnnotationCount),
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
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
            type Value = EvaluateRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.EvaluateRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut created_annotation_count__ = None;
                let mut report_id__ = None;
                let mut job_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreatedAnnotationCount => {
                            if created_annotation_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAnnotationCount"));
                            }
                            created_annotation_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = map_.next_value()?;
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EvaluateRulesResponse {
                    created_annotation_count: created_annotation_count__.unwrap_or_default(),
                    report_id: report_id__,
                    job_id: job_id__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.EvaluateRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunTimeRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rule_evaluation.v1.RunTimeRange", len)?;
        if let Some(v) = self.run.as_ref() {
            struct_ser.serialize_field("run", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunTimeRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Run,
            StartTime,
            EndTime,
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
                            "run" => Ok(GeneratedField::Run),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunTimeRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rule_evaluation.v1.RunTimeRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RunTimeRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Run => {
                            if run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            run__ = map_.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RunTimeRange {
                    run: run__,
                    start_time: start_time__,
                    end_time: end_time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rule_evaluation.v1.RunTimeRange", FIELDS, GeneratedVisitor)
    }
}
