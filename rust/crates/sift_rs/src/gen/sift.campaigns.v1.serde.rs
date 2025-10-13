// @generated
impl serde::Serialize for Campaign {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.campaign_id.is_empty() {
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
        if self.archived_date.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.reports.is_empty() {
            len += 1;
        }
        if self.created_from_campaign_id.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.Campaign", len)?;
        if !self.campaign_id.is_empty() {
            struct_ser.serialize_field("campaignId", &self.campaign_id)?;
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
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.reports.is_empty() {
            struct_ser.serialize_field("reports", &self.reports)?;
        }
        if let Some(v) = self.created_from_campaign_id.as_ref() {
            struct_ser.serialize_field("createdFromCampaignId", v)?;
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
impl<'de> serde::Deserialize<'de> for Campaign {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaign_id",
            "campaignId",
            "organization_id",
            "organizationId",
            "client_key",
            "clientKey",
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
            "archived_date",
            "archivedDate",
            "tags",
            "reports",
            "created_from_campaign_id",
            "createdFromCampaignId",
            "metadata",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CampaignId,
            OrganizationId,
            ClientKey,
            Name,
            Description,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
            ArchivedDate,
            Tags,
            Reports,
            CreatedFromCampaignId,
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
                            "campaignId" | "campaign_id" => Ok(GeneratedField::CampaignId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "tags" => Ok(GeneratedField::Tags),
                            "reports" => Ok(GeneratedField::Reports),
                            "createdFromCampaignId" | "created_from_campaign_id" => Ok(GeneratedField::CreatedFromCampaignId),
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
            type Value = Campaign;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.Campaign")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Campaign, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaign_id__ = None;
                let mut organization_id__ = None;
                let mut client_key__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut archived_date__ = None;
                let mut tags__ = None;
                let mut reports__ = None;
                let mut created_from_campaign_id__ = None;
                let mut metadata__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CampaignId => {
                            if campaign_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaignId"));
                            }
                            campaign_id__ = Some(map_.next_value()?);
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
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reports => {
                            if reports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reports"));
                            }
                            reports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedFromCampaignId => {
                            if created_from_campaign_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdFromCampaignId"));
                            }
                            created_from_campaign_id__ = map_.next_value()?;
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
                Ok(Campaign {
                    campaign_id: campaign_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    client_key: client_key__,
                    name: name__.unwrap_or_default(),
                    description: description__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    archived_date: archived_date__,
                    tags: tags__.unwrap_or_default(),
                    reports: reports__.unwrap_or_default(),
                    created_from_campaign_id: created_from_campaign_id__,
                    metadata: metadata__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.Campaign", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CampaignReport {
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
        if !self.report_name.is_empty() {
            len += 1;
        }
        if self.num_annotations != 0 {
            len += 1;
        }
        if self.num_passed_rules != 0 {
            len += 1;
        }
        if self.num_accepted_rules != 0 {
            len += 1;
        }
        if self.num_failed_rules != 0 {
            len += 1;
        }
        if self.num_open_rules != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.CampaignReport", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        if !self.report_name.is_empty() {
            struct_ser.serialize_field("reportName", &self.report_name)?;
        }
        if self.num_annotations != 0 {
            struct_ser.serialize_field("numAnnotations", &self.num_annotations)?;
        }
        if self.num_passed_rules != 0 {
            struct_ser.serialize_field("numPassedRules", &self.num_passed_rules)?;
        }
        if self.num_accepted_rules != 0 {
            struct_ser.serialize_field("numAcceptedRules", &self.num_accepted_rules)?;
        }
        if self.num_failed_rules != 0 {
            struct_ser.serialize_field("numFailedRules", &self.num_failed_rules)?;
        }
        if self.num_open_rules != 0 {
            struct_ser.serialize_field("numOpenRules", &self.num_open_rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CampaignReport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
            "report_name",
            "reportName",
            "num_annotations",
            "numAnnotations",
            "num_passed_rules",
            "numPassedRules",
            "num_accepted_rules",
            "numAcceptedRules",
            "num_failed_rules",
            "numFailedRules",
            "num_open_rules",
            "numOpenRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
            ReportName,
            NumAnnotations,
            NumPassedRules,
            NumAcceptedRules,
            NumFailedRules,
            NumOpenRules,
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
                            "reportName" | "report_name" => Ok(GeneratedField::ReportName),
                            "numAnnotations" | "num_annotations" => Ok(GeneratedField::NumAnnotations),
                            "numPassedRules" | "num_passed_rules" => Ok(GeneratedField::NumPassedRules),
                            "numAcceptedRules" | "num_accepted_rules" => Ok(GeneratedField::NumAcceptedRules),
                            "numFailedRules" | "num_failed_rules" => Ok(GeneratedField::NumFailedRules),
                            "numOpenRules" | "num_open_rules" => Ok(GeneratedField::NumOpenRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CampaignReport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.CampaignReport")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CampaignReport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                let mut report_name__ = None;
                let mut num_annotations__ = None;
                let mut num_passed_rules__ = None;
                let mut num_accepted_rules__ = None;
                let mut num_failed_rules__ = None;
                let mut num_open_rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportName => {
                            if report_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportName"));
                            }
                            report_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumAnnotations => {
                            if num_annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numAnnotations"));
                            }
                            num_annotations__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumPassedRules => {
                            if num_passed_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numPassedRules"));
                            }
                            num_passed_rules__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumAcceptedRules => {
                            if num_accepted_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numAcceptedRules"));
                            }
                            num_accepted_rules__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumFailedRules => {
                            if num_failed_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numFailedRules"));
                            }
                            num_failed_rules__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumOpenRules => {
                            if num_open_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numOpenRules"));
                            }
                            num_open_rules__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CampaignReport {
                    report_id: report_id__.unwrap_or_default(),
                    report_name: report_name__.unwrap_or_default(),
                    num_annotations: num_annotations__.unwrap_or_default(),
                    num_passed_rules: num_passed_rules__.unwrap_or_default(),
                    num_accepted_rules: num_accepted_rules__.unwrap_or_default(),
                    num_failed_rules: num_failed_rules__.unwrap_or_default(),
                    num_open_rules: num_open_rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.CampaignReport", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCampaignFrom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.initializer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.CreateCampaignFrom", len)?;
        if let Some(v) = self.initializer.as_ref() {
            match v {
                create_campaign_from::Initializer::Reports(v) => {
                    struct_ser.serialize_field("reports", v)?;
                }
                create_campaign_from::Initializer::Runs(v) => {
                    struct_ser.serialize_field("runs", v)?;
                }
                create_campaign_from::Initializer::OtherCampaign(v) => {
                    struct_ser.serialize_field("otherCampaign", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCampaignFrom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reports",
            "runs",
            "other_campaign",
            "otherCampaign",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reports,
            Runs,
            OtherCampaign,
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
                            "runs" => Ok(GeneratedField::Runs),
                            "otherCampaign" | "other_campaign" => Ok(GeneratedField::OtherCampaign),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCampaignFrom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.CreateCampaignFrom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCampaignFrom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut initializer__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reports => {
                            if initializer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reports"));
                            }
                            initializer__ = map_.next_value::<::std::option::Option<_>>()?.map(create_campaign_from::Initializer::Reports)
;
                        }
                        GeneratedField::Runs => {
                            if initializer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runs"));
                            }
                            initializer__ = map_.next_value::<::std::option::Option<_>>()?.map(create_campaign_from::Initializer::Runs)
;
                        }
                        GeneratedField::OtherCampaign => {
                            if initializer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otherCampaign"));
                            }
                            initializer__ = map_.next_value::<::std::option::Option<_>>()?.map(create_campaign_from::Initializer::OtherCampaign)
;
                        }
                    }
                }
                Ok(CreateCampaignFrom {
                    initializer: initializer__,
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.CreateCampaignFrom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCampaignRequest {
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
        if self.tags.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.client_key.is_some() {
            len += 1;
        }
        if self.create_from.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.CreateCampaignRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.tags.as_ref() {
            struct_ser.serialize_field("tags", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.client_key.as_ref() {
            struct_ser.serialize_field("clientKey", v)?;
        }
        if let Some(v) = self.create_from.as_ref() {
            struct_ser.serialize_field("createFrom", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCampaignRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tags",
            "organization_id",
            "organizationId",
            "client_key",
            "clientKey",
            "create_from",
            "createFrom",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Tags,
            OrganizationId,
            ClientKey,
            CreateFrom,
            Metadata,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "createFrom" | "create_from" => Ok(GeneratedField::CreateFrom),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCampaignRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.CreateCampaignRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCampaignRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tags__ = None;
                let mut organization_id__ = None;
                let mut client_key__ = None;
                let mut create_from__ = None;
                let mut metadata__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = map_.next_value()?;
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
                        GeneratedField::CreateFrom => {
                            if create_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createFrom"));
                            }
                            create_from__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateCampaignRequest {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tags: tags__,
                    organization_id: organization_id__.unwrap_or_default(),
                    client_key: client_key__,
                    create_from: create_from__,
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.CreateCampaignRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCampaignResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.campaign.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.CreateCampaignResponse", len)?;
        if let Some(v) = self.campaign.as_ref() {
            struct_ser.serialize_field("campaign", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCampaignResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaign",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Campaign,
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
                            "campaign" => Ok(GeneratedField::Campaign),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCampaignResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.CreateCampaignResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCampaignResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaign__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Campaign => {
                            if campaign__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaign"));
                            }
                            campaign__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateCampaignResponse {
                    campaign: campaign__,
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.CreateCampaignResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCampaignRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.campaign_id.is_empty() {
            len += 1;
        }
        if !self.client_key.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.GetCampaignRequest", len)?;
        if !self.campaign_id.is_empty() {
            struct_ser.serialize_field("campaignId", &self.campaign_id)?;
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
impl<'de> serde::Deserialize<'de> for GetCampaignRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaign_id",
            "campaignId",
            "client_key",
            "clientKey",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CampaignId,
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
                            "campaignId" | "campaign_id" => Ok(GeneratedField::CampaignId),
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
            type Value = GetCampaignRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.GetCampaignRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCampaignRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaign_id__ = None;
                let mut client_key__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CampaignId => {
                            if campaign_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaignId"));
                            }
                            campaign_id__ = Some(map_.next_value()?);
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
                Ok(GetCampaignRequest {
                    campaign_id: campaign_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.GetCampaignRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCampaignResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.campaign.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.GetCampaignResponse", len)?;
        if let Some(v) = self.campaign.as_ref() {
            struct_ser.serialize_field("campaign", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCampaignResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaign",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Campaign,
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
                            "campaign" => Ok(GeneratedField::Campaign),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCampaignResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.GetCampaignResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCampaignResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaign__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Campaign => {
                            if campaign__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaign"));
                            }
                            campaign__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCampaignResponse {
                    campaign: campaign__,
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.GetCampaignResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCampaignAnnotationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.campaign_id.is_empty() {
            len += 1;
        }
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
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.ListCampaignAnnotationsRequest", len)?;
        if !self.campaign_id.is_empty() {
            struct_ser.serialize_field("campaignId", &self.campaign_id)?;
        }
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
impl<'de> serde::Deserialize<'de> for ListCampaignAnnotationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaign_id",
            "campaignId",
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
            CampaignId,
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
                            "campaignId" | "campaign_id" => Ok(GeneratedField::CampaignId),
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
            type Value = ListCampaignAnnotationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.ListCampaignAnnotationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCampaignAnnotationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaign_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CampaignId => {
                            if campaign_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaignId"));
                            }
                            campaign_id__ = Some(map_.next_value()?);
                        }
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
                Ok(ListCampaignAnnotationsRequest {
                    campaign_id: campaign_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.ListCampaignAnnotationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCampaignAnnotationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.ListCampaignAnnotationsResponse", len)?;
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCampaignAnnotationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotations",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotations,
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
                            "annotations" => Ok(GeneratedField::Annotations),
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
            type Value = ListCampaignAnnotationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.ListCampaignAnnotationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCampaignAnnotationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotations__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListCampaignAnnotationsResponse {
                    annotations: annotations__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.ListCampaignAnnotationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCampaignsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.ListCampaignsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListCampaignsRequest {
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
            type Value = ListCampaignsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.ListCampaignsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCampaignsRequest, V::Error>
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
                Ok(ListCampaignsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.ListCampaignsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCampaignsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.campaigns.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.ListCampaignsResponse", len)?;
        if !self.campaigns.is_empty() {
            struct_ser.serialize_field("campaigns", &self.campaigns)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCampaignsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaigns",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Campaigns,
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
                            "campaigns" => Ok(GeneratedField::Campaigns),
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
            type Value = ListCampaignsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.ListCampaignsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCampaignsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaigns__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Campaigns => {
                            if campaigns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaigns"));
                            }
                            campaigns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListCampaignsResponse {
                    campaigns: campaigns__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.ListCampaignsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCampaignRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.campaign.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.UpdateCampaignRequest", len)?;
        if let Some(v) = self.campaign.as_ref() {
            struct_ser.serialize_field("campaign", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCampaignRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaign",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Campaign,
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
                            "campaign" => Ok(GeneratedField::Campaign),
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
            type Value = UpdateCampaignRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.UpdateCampaignRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCampaignRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaign__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Campaign => {
                            if campaign__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaign"));
                            }
                            campaign__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCampaignRequest {
                    campaign: campaign__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.UpdateCampaignRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCampaignResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.campaign.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.campaigns.v1.UpdateCampaignResponse", len)?;
        if let Some(v) = self.campaign.as_ref() {
            struct_ser.serialize_field("campaign", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCampaignResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "campaign",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Campaign,
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
                            "campaign" => Ok(GeneratedField::Campaign),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCampaignResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.campaigns.v1.UpdateCampaignResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCampaignResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut campaign__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Campaign => {
                            if campaign__.is_some() {
                                return Err(serde::de::Error::duplicate_field("campaign"));
                            }
                            campaign__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCampaignResponse {
                    campaign: campaign__,
                })
            }
        }
        deserializer.deserialize_struct("sift.campaigns.v1.UpdateCampaignResponse", FIELDS, GeneratedVisitor)
    }
}
