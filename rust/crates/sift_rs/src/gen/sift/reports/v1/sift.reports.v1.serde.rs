// @generated
impl serde::Serialize for CancelReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.CancelReportRequest", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CancelReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelReportRequest {
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CancelReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.reports.v1.CancelReportResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelReportResponse {
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
            type Value = CancelReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CancelReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CancelReportResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CancelReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportFromReportTemplateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_template_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.CreateReportFromReportTemplateRequest", len)?;
        if !self.report_template_id.is_empty() {
            struct_ser.serialize_field("reportTemplateId", &self.report_template_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportFromReportTemplateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_template_id",
            "reportTemplateId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportTemplateId,
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
                            "reportTemplateId" | "report_template_id" => Ok(GeneratedField::ReportTemplateId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportFromReportTemplateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CreateReportFromReportTemplateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportFromReportTemplateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_template_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportTemplateId => {
                            if report_template_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplateId"));
                            }
                            report_template_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateReportFromReportTemplateRequest {
                    report_template_id: report_template_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CreateReportFromReportTemplateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportFromRulesRequest {
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
        if self.description.is_some() {
            len += 1;
        }
        if !self.tag_names.is_empty() {
            len += 1;
        }
        if self.rule_identifiers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.CreateReportFromRulesRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tag_names.is_empty() {
            struct_ser.serialize_field("tagNames", &self.tag_names)?;
        }
        if let Some(v) = self.rule_identifiers.as_ref() {
            match v {
                create_report_from_rules_request::RuleIdentifiers::RuleIds(v) => {
                    struct_ser.serialize_field("ruleIds", v)?;
                }
                create_report_from_rules_request::RuleIdentifiers::RuleClientKeys(v) => {
                    struct_ser.serialize_field("ruleClientKeys", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportFromRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tag_names",
            "tagNames",
            "rule_ids",
            "ruleIds",
            "rule_client_keys",
            "ruleClientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            TagNames,
            RuleIds,
            RuleClientKeys,
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
                            "description" => Ok(GeneratedField::Description),
                            "tagNames" | "tag_names" => Ok(GeneratedField::TagNames),
                            "ruleIds" | "rule_ids" => Ok(GeneratedField::RuleIds),
                            "ruleClientKeys" | "rule_client_keys" => Ok(GeneratedField::RuleClientKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportFromRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CreateReportFromRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportFromRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tag_names__ = None;
                let mut rule_identifiers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::TagNames => {
                            if tag_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagNames"));
                            }
                            tag_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleIds => {
                            if rule_identifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_identifiers__ = map_.next_value::<::std::option::Option<_>>()?.map(create_report_from_rules_request::RuleIdentifiers::RuleIds)
;
                        }
                        GeneratedField::RuleClientKeys => {
                            if rule_identifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleClientKeys"));
                            }
                            rule_identifiers__ = map_.next_value::<::std::option::Option<_>>()?.map(create_report_from_rules_request::RuleIdentifiers::RuleClientKeys)
;
                        }
                    }
                }
                Ok(CreateReportFromRulesRequest {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tag_names: tag_names__.unwrap_or_default(),
                    rule_identifiers: rule_identifiers__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CreateReportFromRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportRequest {
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
        if !self.run_id.is_empty() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.CreateReportRequest", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                create_report_request::Request::ReportFromReportTemplateRequest(v) => {
                    struct_ser.serialize_field("reportFromReportTemplateRequest", v)?;
                }
                create_report_request::Request::ReportFromRulesRequest(v) => {
                    struct_ser.serialize_field("reportFromRulesRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
            "run_id",
            "runId",
            "name",
            "metadata",
            "report_from_report_template_request",
            "reportFromReportTemplateRequest",
            "report_from_rules_request",
            "reportFromRulesRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
            RunId,
            Name,
            Metadata,
            ReportFromReportTemplateRequest,
            ReportFromRulesRequest,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "name" => Ok(GeneratedField::Name),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "reportFromReportTemplateRequest" | "report_from_report_template_request" => Ok(GeneratedField::ReportFromReportTemplateRequest),
                            "reportFromRulesRequest" | "report_from_rules_request" => Ok(GeneratedField::ReportFromRulesRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CreateReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                let mut run_id__ = None;
                let mut name__ = None;
                let mut metadata__ = None;
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportFromReportTemplateRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportFromReportTemplateRequest"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(create_report_request::Request::ReportFromReportTemplateRequest)
;
                        }
                        GeneratedField::ReportFromRulesRequest => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportFromRulesRequest"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(create_report_request::Request::ReportFromRulesRequest)
;
                        }
                    }
                }
                Ok(CreateReportRequest {
                    organization_id: organization_id__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    name: name__,
                    metadata: metadata__.unwrap_or_default(),
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CreateReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportRequestClientKeys {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_client_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.CreateReportRequestClientKeys", len)?;
        if !self.rule_client_keys.is_empty() {
            struct_ser.serialize_field("ruleClientKeys", &self.rule_client_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportRequestClientKeys {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_client_keys",
            "ruleClientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleClientKeys,
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
                            "ruleClientKeys" | "rule_client_keys" => Ok(GeneratedField::RuleClientKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportRequestClientKeys;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CreateReportRequestClientKeys")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportRequestClientKeys, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_client_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleClientKeys => {
                            if rule_client_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleClientKeys"));
                            }
                            rule_client_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateReportRequestClientKeys {
                    rule_client_keys: rule_client_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CreateReportRequestClientKeys", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportRequestRuleIds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.CreateReportRequestRuleIds", len)?;
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportRequestRuleIds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_ids",
            "ruleIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleIds,
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
                            "ruleIds" | "rule_ids" => Ok(GeneratedField::RuleIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportRequestRuleIds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CreateReportRequestRuleIds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportRequestRuleIds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleIds => {
                            if rule_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateReportRequestRuleIds {
                    rule_ids: rule_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CreateReportRequestRuleIds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.CreateReportResponse", len)?;
        if let Some(v) = self.report.as_ref() {
            struct_ser.serialize_field("report", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Report,
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
                            "report" => Ok(GeneratedField::Report),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.CreateReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Report => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            report__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateReportResponse {
                    report: report__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.CreateReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.GetReportRequest", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.GetReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetReportRequest {
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.GetReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.GetReportResponse", len)?;
        if let Some(v) = self.report.as_ref() {
            struct_ser.serialize_field("report", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Report,
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
                            "report" => Ok(GeneratedField::Report),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.GetReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Report => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            report__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetReportResponse {
                    report: report__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.GetReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListReportsRequest {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ListReportsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListReportsRequest {
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
            "organization_id",
            "organizationId",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrganizationId,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
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
            type Value = ListReportsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ListReportsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListReportsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut organization_id__ = None;
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListReportsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ListReportsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListReportsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reports.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ListReportsResponse", len)?;
        if !self.reports.is_empty() {
            struct_ser.serialize_field("reports", &self.reports)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListReportsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reports",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reports,
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
                            "reports" => Ok(GeneratedField::Reports),
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
            type Value = ListReportsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ListReportsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListReportsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reports__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reports => {
                            if reports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reports"));
                            }
                            reports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListReportsResponse {
                    reports: reports__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ListReportsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Report {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_id.is_empty() {
            len += 1;
        }
        if !self.report_template_id.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if !self.summaries.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.rerun_from_report_id.is_some() {
            len += 1;
        }
        if self.job_id.is_some() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.Report", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        if !self.report_template_id.is_empty() {
            struct_ser.serialize_field("reportTemplateId", &self.report_template_id)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if !self.summaries.is_empty() {
            struct_ser.serialize_field("summaries", &self.summaries)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.rerun_from_report_id.as_ref() {
            struct_ser.serialize_field("rerunFromReportId", v)?;
        }
        if let Some(v) = self.job_id.as_ref() {
            struct_ser.serialize_field("jobId", v)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Report {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
            "report_template_id",
            "reportTemplateId",
            "run_id",
            "runId",
            "organization_id",
            "organizationId",
            "name",
            "description",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "summaries",
            "tags",
            "rerun_from_report_id",
            "rerunFromReportId",
            "job_id",
            "jobId",
            "archived_date",
            "archivedDate",
            "metadata",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
            ReportTemplateId,
            RunId,
            OrganizationId,
            Name,
            Description,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
            Summaries,
            Tags,
            RerunFromReportId,
            JobId,
            ArchivedDate,
            Metadata,
            IsArchived,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            "reportTemplateId" | "report_template_id" => Ok(GeneratedField::ReportTemplateId),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "summaries" => Ok(GeneratedField::Summaries),
                            "tags" => Ok(GeneratedField::Tags),
                            "rerunFromReportId" | "rerun_from_report_id" => Ok(GeneratedField::RerunFromReportId),
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "isArchived" | "is_archived" => Ok(GeneratedField::IsArchived),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Report;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.Report")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Report, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                let mut report_template_id__ = None;
                let mut run_id__ = None;
                let mut organization_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut summaries__ = None;
                let mut tags__ = None;
                let mut rerun_from_report_id__ = None;
                let mut job_id__ = None;
                let mut archived_date__ = None;
                let mut metadata__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportTemplateId => {
                            if report_template_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplateId"));
                            }
                            report_template_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModifiedByUserId => {
                            if modified_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedByUserId"));
                            }
                            modified_by_user_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Summaries => {
                            if summaries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summaries"));
                            }
                            summaries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RerunFromReportId => {
                            if rerun_from_report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rerunFromReportId"));
                            }
                            rerun_from_report_id__ = map_.next_value()?;
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = map_.next_value()?;
                        }
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Report {
                    report_id: report_id__.unwrap_or_default(),
                    report_template_id: report_template_id__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    summaries: summaries__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    rerun_from_report_id: rerun_from_report_id__,
                    job_id: job_id__,
                    archived_date: archived_date__,
                    metadata: metadata__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.Report", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REPORT_RULE_STATUS_UNSPECIFIED",
            Self::Created => "REPORT_RULE_STATUS_CREATED",
            Self::Live => "REPORT_RULE_STATUS_LIVE",
            Self::Finished => "REPORT_RULE_STATUS_FINISHED",
            Self::Failed => "REPORT_RULE_STATUS_FAILED",
            Self::Canceled => "REPORT_RULE_STATUS_CANCELED",
            Self::Error => "REPORT_RULE_STATUS_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REPORT_RULE_STATUS_UNSPECIFIED",
            "REPORT_RULE_STATUS_CREATED",
            "REPORT_RULE_STATUS_LIVE",
            "REPORT_RULE_STATUS_FINISHED",
            "REPORT_RULE_STATUS_FAILED",
            "REPORT_RULE_STATUS_CANCELED",
            "REPORT_RULE_STATUS_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportRuleStatus;

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
                    "REPORT_RULE_STATUS_UNSPECIFIED" => Ok(ReportRuleStatus::Unspecified),
                    "REPORT_RULE_STATUS_CREATED" => Ok(ReportRuleStatus::Created),
                    "REPORT_RULE_STATUS_LIVE" => Ok(ReportRuleStatus::Live),
                    "REPORT_RULE_STATUS_FINISHED" => Ok(ReportRuleStatus::Finished),
                    "REPORT_RULE_STATUS_FAILED" => Ok(ReportRuleStatus::Failed),
                    "REPORT_RULE_STATUS_CANCELED" => Ok(ReportRuleStatus::Canceled),
                    "REPORT_RULE_STATUS_ERROR" => Ok(ReportRuleStatus::Error),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatusDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleStatusDetails", len)?;
        if let Some(v) = self.details.as_ref() {
            match v {
                report_rule_status_details::Details::Created(v) => {
                    struct_ser.serialize_field("created", v)?;
                }
                report_rule_status_details::Details::Live(v) => {
                    struct_ser.serialize_field("live", v)?;
                }
                report_rule_status_details::Details::Finished(v) => {
                    struct_ser.serialize_field("finished", v)?;
                }
                report_rule_status_details::Details::Failed(v) => {
                    struct_ser.serialize_field("failed", v)?;
                }
                report_rule_status_details::Details::Canceled(v) => {
                    struct_ser.serialize_field("canceled", v)?;
                }
                report_rule_status_details::Details::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatusDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "created",
            "live",
            "finished",
            "failed",
            "canceled",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Created,
            Live,
            Finished,
            Failed,
            Canceled,
            Error,
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
                            "created" => Ok(GeneratedField::Created),
                            "live" => Ok(GeneratedField::Live),
                            "finished" => Ok(GeneratedField::Finished),
                            "failed" => Ok(GeneratedField::Failed),
                            "canceled" => Ok(GeneratedField::Canceled),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportRuleStatusDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleStatusDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleStatusDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Created => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(report_rule_status_details::Details::Created)
;
                        }
                        GeneratedField::Live => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("live"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(report_rule_status_details::Details::Live)
;
                        }
                        GeneratedField::Finished => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finished"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(report_rule_status_details::Details::Finished)
;
                        }
                        GeneratedField::Failed => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(report_rule_status_details::Details::Failed)
;
                        }
                        GeneratedField::Canceled => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canceled"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(report_rule_status_details::Details::Canceled)
;
                        }
                        GeneratedField::Error => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(report_rule_status_details::Details::Error)
;
                        }
                    }
                }
                Ok(ReportRuleStatusDetails {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleStatusDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatusDetailsCanceled {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleStatusDetailsCanceled", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatusDetailsCanceled {
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
            type Value = ReportRuleStatusDetailsCanceled;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleStatusDetailsCanceled")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleStatusDetailsCanceled, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReportRuleStatusDetailsCanceled {
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleStatusDetailsCanceled", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatusDetailsCreated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleStatusDetailsCreated", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatusDetailsCreated {
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
            type Value = ReportRuleStatusDetailsCreated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleStatusDetailsCreated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleStatusDetailsCreated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReportRuleStatusDetailsCreated {
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleStatusDetailsCreated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatusDetailsError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.error_message.is_empty() {
            len += 1;
        }
        if self.exit_code.is_some() {
            len += 1;
        }
        if self.stdout.is_some() {
            len += 1;
        }
        if self.stderr.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleStatusDetailsError", len)?;
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        if let Some(v) = self.exit_code.as_ref() {
            struct_ser.serialize_field("exitCode", v)?;
        }
        if let Some(v) = self.stdout.as_ref() {
            struct_ser.serialize_field("stdout", v)?;
        }
        if let Some(v) = self.stderr.as_ref() {
            struct_ser.serialize_field("stderr", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatusDetailsError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error_message",
            "errorMessage",
            "exit_code",
            "exitCode",
            "stdout",
            "stderr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorMessage,
            ExitCode,
            Stdout,
            Stderr,
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
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "exitCode" | "exit_code" => Ok(GeneratedField::ExitCode),
                            "stdout" => Ok(GeneratedField::Stdout),
                            "stderr" => Ok(GeneratedField::Stderr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportRuleStatusDetailsError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleStatusDetailsError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleStatusDetailsError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error_message__ = None;
                let mut exit_code__ = None;
                let mut stdout__ = None;
                let mut stderr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exitCode"));
                            }
                            exit_code__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Stdout => {
                            if stdout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout__ = map_.next_value()?;
                        }
                        GeneratedField::Stderr => {
                            if stderr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReportRuleStatusDetailsError {
                    error_message: error_message__.unwrap_or_default(),
                    exit_code: exit_code__,
                    stdout: stdout__,
                    stderr: stderr__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleStatusDetailsError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatusDetailsFailed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.error_message.is_empty() {
            len += 1;
        }
        if self.exit_code.is_some() {
            len += 1;
        }
        if self.stdout.is_some() {
            len += 1;
        }
        if self.stderr.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleStatusDetailsFailed", len)?;
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        if let Some(v) = self.exit_code.as_ref() {
            struct_ser.serialize_field("exitCode", v)?;
        }
        if let Some(v) = self.stdout.as_ref() {
            struct_ser.serialize_field("stdout", v)?;
        }
        if let Some(v) = self.stderr.as_ref() {
            struct_ser.serialize_field("stderr", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatusDetailsFailed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error_message",
            "errorMessage",
            "exit_code",
            "exitCode",
            "stdout",
            "stderr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorMessage,
            ExitCode,
            Stdout,
            Stderr,
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
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "exitCode" | "exit_code" => Ok(GeneratedField::ExitCode),
                            "stdout" => Ok(GeneratedField::Stdout),
                            "stderr" => Ok(GeneratedField::Stderr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportRuleStatusDetailsFailed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleStatusDetailsFailed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleStatusDetailsFailed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error_message__ = None;
                let mut exit_code__ = None;
                let mut stdout__ = None;
                let mut stderr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exitCode"));
                            }
                            exit_code__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Stdout => {
                            if stdout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout__ = map_.next_value()?;
                        }
                        GeneratedField::Stderr => {
                            if stderr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReportRuleStatusDetailsFailed {
                    error_message: error_message__.unwrap_or_default(),
                    exit_code: exit_code__,
                    stdout: stdout__,
                    stderr: stderr__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleStatusDetailsFailed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatusDetailsFinished {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stdout.is_some() {
            len += 1;
        }
        if self.stderr.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleStatusDetailsFinished", len)?;
        if let Some(v) = self.stdout.as_ref() {
            struct_ser.serialize_field("stdout", v)?;
        }
        if let Some(v) = self.stderr.as_ref() {
            struct_ser.serialize_field("stderr", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatusDetailsFinished {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stdout",
            "stderr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stdout,
            Stderr,
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
                            "stdout" => Ok(GeneratedField::Stdout),
                            "stderr" => Ok(GeneratedField::Stderr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportRuleStatusDetailsFinished;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleStatusDetailsFinished")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleStatusDetailsFinished, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stdout__ = None;
                let mut stderr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stdout => {
                            if stdout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout__ = map_.next_value()?;
                        }
                        GeneratedField::Stderr => {
                            if stderr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReportRuleStatusDetailsFinished {
                    stdout: stdout__,
                    stderr: stderr__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleStatusDetailsFinished", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleStatusDetailsLive {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleStatusDetailsLive", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleStatusDetailsLive {
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
            type Value = ReportRuleStatusDetailsLive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleStatusDetailsLive")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleStatusDetailsLive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReportRuleStatusDetailsLive {
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleStatusDetailsLive", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportRuleSummary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_id.is_empty() {
            len += 1;
        }
        if !self.rule_client_key.is_empty() {
            len += 1;
        }
        if !self.rule_version_id.is_empty() {
            len += 1;
        }
        if self.rule_version_number != 0 {
            len += 1;
        }
        if !self.report_rule_version_id.is_empty() {
            len += 1;
        }
        if self.num_open != 0 {
            len += 1;
        }
        if self.num_failed != 0 {
            len += 1;
        }
        if self.num_passed != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.status_details.is_some() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if self.deleted_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ReportRuleSummary", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.rule_client_key.is_empty() {
            struct_ser.serialize_field("ruleClientKey", &self.rule_client_key)?;
        }
        if !self.rule_version_id.is_empty() {
            struct_ser.serialize_field("ruleVersionId", &self.rule_version_id)?;
        }
        if self.rule_version_number != 0 {
            struct_ser.serialize_field("ruleVersionNumber", &self.rule_version_number)?;
        }
        if !self.report_rule_version_id.is_empty() {
            struct_ser.serialize_field("reportRuleVersionId", &self.report_rule_version_id)?;
        }
        if self.num_open != 0 {
            struct_ser.serialize_field("numOpen", &self.num_open)?;
        }
        if self.num_failed != 0 {
            struct_ser.serialize_field("numFailed", &self.num_failed)?;
        }
        if self.num_passed != 0 {
            struct_ser.serialize_field("numPassed", &self.num_passed)?;
        }
        if self.status != 0 {
            let v = ReportRuleStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.status_details.as_ref() {
            struct_ser.serialize_field("statusDetails", v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if let Some(v) = self.deleted_date.as_ref() {
            struct_ser.serialize_field("deletedDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportRuleSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "rule_client_key",
            "ruleClientKey",
            "rule_version_id",
            "ruleVersionId",
            "rule_version_number",
            "ruleVersionNumber",
            "report_rule_version_id",
            "reportRuleVersionId",
            "num_open",
            "numOpen",
            "num_failed",
            "numFailed",
            "num_passed",
            "numPassed",
            "status",
            "status_details",
            "statusDetails",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "asset_id",
            "assetId",
            "deleted_date",
            "deletedDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            RuleClientKey,
            RuleVersionId,
            RuleVersionNumber,
            ReportRuleVersionId,
            NumOpen,
            NumFailed,
            NumPassed,
            Status,
            StatusDetails,
            CreatedDate,
            ModifiedDate,
            AssetId,
            DeletedDate,
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
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "ruleClientKey" | "rule_client_key" => Ok(GeneratedField::RuleClientKey),
                            "ruleVersionId" | "rule_version_id" => Ok(GeneratedField::RuleVersionId),
                            "ruleVersionNumber" | "rule_version_number" => Ok(GeneratedField::RuleVersionNumber),
                            "reportRuleVersionId" | "report_rule_version_id" => Ok(GeneratedField::ReportRuleVersionId),
                            "numOpen" | "num_open" => Ok(GeneratedField::NumOpen),
                            "numFailed" | "num_failed" => Ok(GeneratedField::NumFailed),
                            "numPassed" | "num_passed" => Ok(GeneratedField::NumPassed),
                            "status" => Ok(GeneratedField::Status),
                            "statusDetails" | "status_details" => Ok(GeneratedField::StatusDetails),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "deletedDate" | "deleted_date" => Ok(GeneratedField::DeletedDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportRuleSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportRuleSummary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportRuleSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut rule_client_key__ = None;
                let mut rule_version_id__ = None;
                let mut rule_version_number__ = None;
                let mut report_rule_version_id__ = None;
                let mut num_open__ = None;
                let mut num_failed__ = None;
                let mut num_passed__ = None;
                let mut status__ = None;
                let mut status_details__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut asset_id__ = None;
                let mut deleted_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleClientKey => {
                            if rule_client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleClientKey"));
                            }
                            rule_client_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleVersionId => {
                            if rule_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersionId"));
                            }
                            rule_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleVersionNumber => {
                            if rule_version_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersionNumber"));
                            }
                            rule_version_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReportRuleVersionId => {
                            if report_rule_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportRuleVersionId"));
                            }
                            report_rule_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumOpen => {
                            if num_open__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOpen"));
                            }
                            num_open__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumFailed => {
                            if num_failed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFailed"));
                            }
                            num_failed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumPassed => {
                            if num_passed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numPassed"));
                            }
                            num_passed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<ReportRuleStatus>()? as i32);
                        }
                        GeneratedField::StatusDetails => {
                            if status_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusDetails"));
                            }
                            status_details__ = map_.next_value()?;
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
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeletedDate => {
                            if deleted_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedDate"));
                            }
                            deleted_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReportRuleSummary {
                    rule_id: rule_id__.unwrap_or_default(),
                    rule_client_key: rule_client_key__.unwrap_or_default(),
                    rule_version_id: rule_version_id__.unwrap_or_default(),
                    rule_version_number: rule_version_number__.unwrap_or_default(),
                    report_rule_version_id: report_rule_version_id__.unwrap_or_default(),
                    num_open: num_open__.unwrap_or_default(),
                    num_failed: num_failed__.unwrap_or_default(),
                    num_passed: num_passed__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    status_details: status_details__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    asset_id: asset_id__.unwrap_or_default(),
                    deleted_date: deleted_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportRuleSummary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.ReportTag", len)?;
        if !self.tag_name.is_empty() {
            struct_ser.serialize_field("tagName", &self.tag_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag_name",
            "tagName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagName,
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
                            "tagName" | "tag_name" => Ok(GeneratedField::TagName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.ReportTag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TagName => {
                            if tag_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagName"));
                            }
                            tag_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportTag {
                    tag_name: tag_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.ReportTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RerunReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.RerunReportRequest", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RerunReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RerunReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.RerunReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RerunReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RerunReportRequest {
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.RerunReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RerunReportResponse {
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
        if !self.report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.RerunReportResponse", len)?;
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RerunReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_id",
            "jobId",
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobId,
            ReportId,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RerunReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.RerunReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RerunReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_id__ = None;
                let mut report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RerunReportResponse {
                    job_id: job_id__.unwrap_or_default(),
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.RerunReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.report.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.reports.v1.UpdateReportRequest", len)?;
        if let Some(v) = self.report.as_ref() {
            struct_ser.serialize_field("report", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Report,
            UpdateMask,
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
                            "report" => Ok(GeneratedField::Report),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.UpdateReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Report => {
                            if report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            report__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateReportRequest {
                    report: report__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.UpdateReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.reports.v1.UpdateReportResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateReportResponse {
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
            type Value = UpdateReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.reports.v1.UpdateReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateReportResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.reports.v1.UpdateReportResponse", FIELDS, GeneratedVisitor)
    }
}
