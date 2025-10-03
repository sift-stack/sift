// @generated
impl serde::Serialize for CreateReportTemplateRequest {
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
        if self.client_key.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.tag_names.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.rule_identifiers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.CreateReportTemplateRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.client_key.as_ref() {
            struct_ser.serialize_field("clientKey", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tag_names.is_empty() {
            struct_ser.serialize_field("tagNames", &self.tag_names)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if let Some(v) = self.rule_identifiers.as_ref() {
            match v {
                create_report_template_request::RuleIdentifiers::RuleIds(v) => {
                    struct_ser.serialize_field("ruleIds", v)?;
                }
                create_report_template_request::RuleIdentifiers::RuleClientKeys(v) => {
                    struct_ser.serialize_field("ruleClientKeys", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportTemplateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "client_key",
            "clientKey",
            "description",
            "tag_names",
            "tagNames",
            "organization_id",
            "organizationId",
            "metadata",
            "rule_ids",
            "ruleIds",
            "rule_client_keys",
            "ruleClientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ClientKey,
            Description,
            TagNames,
            OrganizationId,
            Metadata,
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
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "description" => Ok(GeneratedField::Description),
                            "tagNames" | "tag_names" => Ok(GeneratedField::TagNames),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "metadata" => Ok(GeneratedField::Metadata),
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
            type Value = CreateReportTemplateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.CreateReportTemplateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportTemplateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut client_key__ = None;
                let mut description__ = None;
                let mut tag_names__ = None;
                let mut organization_id__ = None;
                let mut metadata__ = None;
                let mut rule_identifiers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = map_.next_value()?;
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleIds => {
                            if rule_identifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_identifiers__ = map_.next_value::<::std::option::Option<_>>()?.map(create_report_template_request::RuleIdentifiers::RuleIds)
;
                        }
                        GeneratedField::RuleClientKeys => {
                            if rule_identifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleClientKeys"));
                            }
                            rule_identifiers__ = map_.next_value::<::std::option::Option<_>>()?.map(create_report_template_request::RuleIdentifiers::RuleClientKeys)
;
                        }
                    }
                }
                Ok(CreateReportTemplateRequest {
                    name: name__.unwrap_or_default(),
                    client_key: client_key__,
                    description: description__,
                    tag_names: tag_names__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    rule_identifiers: rule_identifiers__,
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.CreateReportTemplateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportTemplateRequestClientKeys {
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
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.CreateReportTemplateRequestClientKeys", len)?;
        if !self.rule_client_keys.is_empty() {
            struct_ser.serialize_field("ruleClientKeys", &self.rule_client_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportTemplateRequestClientKeys {
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
            type Value = CreateReportTemplateRequestClientKeys;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.CreateReportTemplateRequestClientKeys")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportTemplateRequestClientKeys, V::Error>
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
                Ok(CreateReportTemplateRequestClientKeys {
                    rule_client_keys: rule_client_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.CreateReportTemplateRequestClientKeys", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportTemplateRequestRuleIds {
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
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.CreateReportTemplateRequestRuleIds", len)?;
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportTemplateRequestRuleIds {
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
            type Value = CreateReportTemplateRequestRuleIds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.CreateReportTemplateRequestRuleIds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportTemplateRequestRuleIds, V::Error>
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
                Ok(CreateReportTemplateRequestRuleIds {
                    rule_ids: rule_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.CreateReportTemplateRequestRuleIds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateReportTemplateResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.CreateReportTemplateResponse", len)?;
        if let Some(v) = self.report_template.as_ref() {
            struct_ser.serialize_field("reportTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateReportTemplateResponse {
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
            type Value = CreateReportTemplateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.CreateReportTemplateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateReportTemplateResponse, V::Error>
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
                Ok(CreateReportTemplateResponse {
                    report_template: report_template__,
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.CreateReportTemplateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetReportTemplateRequest {
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
        if !self.client_key.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.GetReportTemplateRequest", len)?;
        if !self.report_template_id.is_empty() {
            struct_ser.serialize_field("reportTemplateId", &self.report_template_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetReportTemplateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_template_id",
            "reportTemplateId",
            "client_key",
            "clientKey",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportTemplateId,
            ClientKey,
            OrganizationId,
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
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetReportTemplateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.GetReportTemplateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetReportTemplateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_template_id__ = None;
                let mut client_key__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportTemplateId => {
                            if report_template_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplateId"));
                            }
                            report_template_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetReportTemplateRequest {
                    report_template_id: report_template_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.GetReportTemplateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetReportTemplateResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.GetReportTemplateResponse", len)?;
        if let Some(v) = self.report_template.as_ref() {
            struct_ser.serialize_field("reportTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetReportTemplateResponse {
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
            type Value = GetReportTemplateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.GetReportTemplateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetReportTemplateResponse, V::Error>
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
                Ok(GetReportTemplateResponse {
                    report_template: report_template__,
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.GetReportTemplateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListReportTemplatesRequest {
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
        if self.include_archived {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.ListReportTemplatesRequest", len)?;
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
        if self.include_archived {
            struct_ser.serialize_field("includeArchived", &self.include_archived)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListReportTemplatesRequest {
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
            "include_archived",
            "includeArchived",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrganizationId,
            IncludeArchived,
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
                            "includeArchived" | "include_archived" => Ok(GeneratedField::IncludeArchived),
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
            type Value = ListReportTemplatesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.ListReportTemplatesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListReportTemplatesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut organization_id__ = None;
                let mut include_archived__ = None;
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
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListReportTemplatesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.ListReportTemplatesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListReportTemplatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_templates.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.ListReportTemplatesResponse", len)?;
        if !self.report_templates.is_empty() {
            struct_ser.serialize_field("reportTemplates", &self.report_templates)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListReportTemplatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_templates",
            "reportTemplates",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportTemplates,
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
                            "reportTemplates" | "report_templates" => Ok(GeneratedField::ReportTemplates),
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
            type Value = ListReportTemplatesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.ListReportTemplatesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListReportTemplatesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_templates__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportTemplates => {
                            if report_templates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplates"));
                            }
                            report_templates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListReportTemplatesResponse {
                    report_templates: report_templates__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.ListReportTemplatesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportTemplate {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.client_key.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.archived_date.is_some() {
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
        if !self.rules.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.ReportTemplate", len)?;
        if !self.report_template_id.is_empty() {
            struct_ser.serialize_field("reportTemplateId", &self.report_template_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.client_key.as_ref() {
            struct_ser.serialize_field("clientKey", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
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
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
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
impl<'de> serde::Deserialize<'de> for ReportTemplate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_template_id",
            "reportTemplateId",
            "organization_id",
            "organizationId",
            "client_key",
            "clientKey",
            "name",
            "description",
            "archived_date",
            "archivedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "rules",
            "tags",
            "metadata",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportTemplateId,
            OrganizationId,
            ClientKey,
            Name,
            Description,
            ArchivedDate,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
            Rules,
            Tags,
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
                            "reportTemplateId" | "report_template_id" => Ok(GeneratedField::ReportTemplateId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "rules" => Ok(GeneratedField::Rules),
                            "tags" => Ok(GeneratedField::Tags),
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
            type Value = ReportTemplate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.ReportTemplate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportTemplate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_template_id__ = None;
                let mut organization_id__ = None;
                let mut client_key__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut archived_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut rules__ = None;
                let mut tags__ = None;
                let mut metadata__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportTemplateId => {
                            if report_template_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplateId"));
                            }
                            report_template_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = map_.next_value()?;
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
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
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
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
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
                Ok(ReportTemplate {
                    report_template_id: report_template_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    client_key: client_key__,
                    name: name__.unwrap_or_default(),
                    description: description__,
                    archived_date: archived_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    rules: rules__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.ReportTemplate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportTemplateRule {
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
        if !self.rule_version_id.is_empty() {
            len += 1;
        }
        if self.rule_version_number != 0 {
            len += 1;
        }
        if !self.client_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.ReportTemplateRule", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.rule_version_id.is_empty() {
            struct_ser.serialize_field("ruleVersionId", &self.rule_version_id)?;
        }
        if self.rule_version_number != 0 {
            struct_ser.serialize_field("ruleVersionNumber", &self.rule_version_number)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportTemplateRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "rule_version_id",
            "ruleVersionId",
            "rule_version_number",
            "ruleVersionNumber",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            RuleVersionId,
            RuleVersionNumber,
            ClientKey,
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
                            "ruleVersionId" | "rule_version_id" => Ok(GeneratedField::RuleVersionId),
                            "ruleVersionNumber" | "rule_version_number" => Ok(GeneratedField::RuleVersionNumber),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReportTemplateRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.ReportTemplateRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportTemplateRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut rule_version_id__ = None;
                let mut rule_version_number__ = None;
                let mut client_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
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
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReportTemplateRule {
                    rule_id: rule_id__.unwrap_or_default(),
                    rule_version_id: rule_version_id__.unwrap_or_default(),
                    rule_version_number: rule_version_number__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.ReportTemplateRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReportTemplateTag {
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
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.ReportTemplateTag", len)?;
        if !self.tag_name.is_empty() {
            struct_ser.serialize_field("tagName", &self.tag_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReportTemplateTag {
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
            type Value = ReportTemplateTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.ReportTemplateTag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReportTemplateTag, V::Error>
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
                Ok(ReportTemplateTag {
                    tag_name: tag_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.ReportTemplateTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateReportTemplateRequest {
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
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.UpdateReportTemplateRequest", len)?;
        if let Some(v) = self.report_template.as_ref() {
            struct_ser.serialize_field("reportTemplate", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateReportTemplateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_template",
            "reportTemplate",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportTemplate,
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
                            "reportTemplate" | "report_template" => Ok(GeneratedField::ReportTemplate),
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
            type Value = UpdateReportTemplateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.UpdateReportTemplateRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateReportTemplateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_template__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportTemplate => {
                            if report_template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplate"));
                            }
                            report_template__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateReportTemplateRequest {
                    report_template: report_template__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.UpdateReportTemplateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateReportTemplateResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.report_templates.v1.UpdateReportTemplateResponse", len)?;
        if let Some(v) = self.report_template.as_ref() {
            struct_ser.serialize_field("reportTemplate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateReportTemplateResponse {
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
            type Value = UpdateReportTemplateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.report_templates.v1.UpdateReportTemplateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateReportTemplateResponse, V::Error>
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
                Ok(UpdateReportTemplateResponse {
                    report_template: report_template__,
                })
            }
        }
        deserializer.deserialize_struct("sift.report_templates.v1.UpdateReportTemplateResponse", FIELDS, GeneratedVisitor)
    }
}
