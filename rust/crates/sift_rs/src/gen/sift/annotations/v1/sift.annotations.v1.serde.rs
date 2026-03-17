// @generated
impl serde::Serialize for Annotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
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
        if self.run_id.is_some() {
            len += 1;
        }
        if self.state.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.assigned_to_user_id.is_empty() {
            len += 1;
        }
        if self.annotation_type != 0 {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.legend_config.is_some() {
            len += 1;
        }
        if self.created_by_condition_id.is_some() {
            len += 1;
        }
        if self.created_by_rule_condition_version_id.is_some() {
            len += 1;
        }
        if self.report_rule_version_id.is_some() {
            len += 1;
        }
        if self.pending {
            len += 1;
        }
        if self.assigned_to_user.is_some() {
            len += 1;
        }
        if self.deleted_date.is_some() {
            len += 1;
        }
        if !self.linked_channels.is_empty() {
            len += 1;
        }
        if !self.asset_ids.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.Annotation", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
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
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.state.as_ref() {
            let v = AnnotationState::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.assigned_to_user_id.is_empty() {
            struct_ser.serialize_field("assignedToUserId", &self.assigned_to_user_id)?;
        }
        if self.annotation_type != 0 {
            let v = AnnotationType::try_from(self.annotation_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.annotation_type)))?;
            struct_ser.serialize_field("annotationType", &v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.legend_config.as_ref() {
            struct_ser.serialize_field("legendConfig", v)?;
        }
        if let Some(v) = self.created_by_condition_id.as_ref() {
            struct_ser.serialize_field("createdByConditionId", v)?;
        }
        if let Some(v) = self.created_by_rule_condition_version_id.as_ref() {
            struct_ser.serialize_field("createdByRuleConditionVersionId", v)?;
        }
        if let Some(v) = self.report_rule_version_id.as_ref() {
            struct_ser.serialize_field("reportRuleVersionId", v)?;
        }
        if self.pending {
            struct_ser.serialize_field("pending", &self.pending)?;
        }
        if let Some(v) = self.assigned_to_user.as_ref() {
            struct_ser.serialize_field("assignedToUser", v)?;
        }
        if let Some(v) = self.deleted_date.as_ref() {
            struct_ser.serialize_field("deletedDate", v)?;
        }
        if !self.linked_channels.is_empty() {
            struct_ser.serialize_field("linkedChannels", &self.linked_channels)?;
        }
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Annotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
            "name",
            "description",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "run_id",
            "runId",
            "state",
            "organization_id",
            "organizationId",
            "assigned_to_user_id",
            "assignedToUserId",
            "annotation_type",
            "annotationType",
            "tags",
            "legend_config",
            "legendConfig",
            "created_by_condition_id",
            "createdByConditionId",
            "created_by_rule_condition_version_id",
            "createdByRuleConditionVersionId",
            "report_rule_version_id",
            "reportRuleVersionId",
            "pending",
            "assigned_to_user",
            "assignedToUser",
            "deleted_date",
            "deletedDate",
            "linked_channels",
            "linkedChannels",
            "asset_ids",
            "assetIds",
            "metadata",
            "archived_date",
            "archivedDate",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
            Name,
            Description,
            StartTime,
            EndTime,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
            RunId,
            State,
            OrganizationId,
            AssignedToUserId,
            AnnotationType,
            Tags,
            LegendConfig,
            CreatedByConditionId,
            CreatedByRuleConditionVersionId,
            ReportRuleVersionId,
            Pending,
            AssignedToUser,
            DeletedDate,
            LinkedChannels,
            AssetIds,
            Metadata,
            ArchivedDate,
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
                            "annotationId" | "annotation_id" => Ok(GeneratedField::AnnotationId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "state" => Ok(GeneratedField::State),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "assignedToUserId" | "assigned_to_user_id" => Ok(GeneratedField::AssignedToUserId),
                            "annotationType" | "annotation_type" => Ok(GeneratedField::AnnotationType),
                            "tags" => Ok(GeneratedField::Tags),
                            "legendConfig" | "legend_config" => Ok(GeneratedField::LegendConfig),
                            "createdByConditionId" | "created_by_condition_id" => Ok(GeneratedField::CreatedByConditionId),
                            "createdByRuleConditionVersionId" | "created_by_rule_condition_version_id" => Ok(GeneratedField::CreatedByRuleConditionVersionId),
                            "reportRuleVersionId" | "report_rule_version_id" => Ok(GeneratedField::ReportRuleVersionId),
                            "pending" => Ok(GeneratedField::Pending),
                            "assignedToUser" | "assigned_to_user" => Ok(GeneratedField::AssignedToUser),
                            "deletedDate" | "deleted_date" => Ok(GeneratedField::DeletedDate),
                            "linkedChannels" | "linked_channels" => Ok(GeneratedField::LinkedChannels),
                            "assetIds" | "asset_ids" => Ok(GeneratedField::AssetIds),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
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
            type Value = Annotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.Annotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Annotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut run_id__ = None;
                let mut state__ = None;
                let mut organization_id__ = None;
                let mut assigned_to_user_id__ = None;
                let mut annotation_type__ = None;
                let mut tags__ = None;
                let mut legend_config__ = None;
                let mut created_by_condition_id__ = None;
                let mut created_by_rule_condition_version_id__ = None;
                let mut report_rule_version_id__ = None;
                let mut pending__ = None;
                let mut assigned_to_user__ = None;
                let mut deleted_date__ = None;
                let mut linked_channels__ = None;
                let mut asset_ids__ = None;
                let mut metadata__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
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
                            description__ = Some(map_.next_value()?);
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
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map_.next_value()?;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = map_.next_value::<::std::option::Option<AnnotationState>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssignedToUserId => {
                            if assigned_to_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignedToUserId"));
                            }
                            assigned_to_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationType => {
                            if annotation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationType"));
                            }
                            annotation_type__ = Some(map_.next_value::<AnnotationType>()? as i32);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LegendConfig => {
                            if legend_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("legendConfig"));
                            }
                            legend_config__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByConditionId => {
                            if created_by_condition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByConditionId"));
                            }
                            created_by_condition_id__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByRuleConditionVersionId => {
                            if created_by_rule_condition_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByRuleConditionVersionId"));
                            }
                            created_by_rule_condition_version_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReportRuleVersionId => {
                            if report_rule_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportRuleVersionId"));
                            }
                            report_rule_version_id__ = map_.next_value()?;
                        }
                        GeneratedField::Pending => {
                            if pending__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pending"));
                            }
                            pending__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssignedToUser => {
                            if assigned_to_user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignedToUser"));
                            }
                            assigned_to_user__ = map_.next_value()?;
                        }
                        GeneratedField::DeletedDate => {
                            if deleted_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedDate"));
                            }
                            deleted_date__ = map_.next_value()?;
                        }
                        GeneratedField::LinkedChannels => {
                            if linked_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkedChannels"));
                            }
                            linked_channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Annotation {
                    annotation_id: annotation_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    run_id: run_id__,
                    state: state__,
                    organization_id: organization_id__.unwrap_or_default(),
                    assigned_to_user_id: assigned_to_user_id__.unwrap_or_default(),
                    annotation_type: annotation_type__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    legend_config: legend_config__,
                    created_by_condition_id: created_by_condition_id__,
                    created_by_rule_condition_version_id: created_by_rule_condition_version_id__,
                    report_rule_version_id: report_rule_version_id__,
                    pending: pending__.unwrap_or_default(),
                    assigned_to_user: assigned_to_user__,
                    deleted_date: deleted_date__,
                    linked_channels: linked_channels__.unwrap_or_default(),
                    asset_ids: asset_ids__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.Annotation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLinkedChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.AnnotationLinkedChannel", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                annotation_linked_channel::Type::Channel(v) => {
                    struct_ser.serialize_field("channel", v)?;
                }
                annotation_linked_channel::Type::BitFieldElement(v) => {
                    struct_ser.serialize_field("bitFieldElement", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLinkedChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
            "bit_field_element",
            "bitFieldElement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
            BitFieldElement,
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
                            "channel" => Ok(GeneratedField::Channel),
                            "bitFieldElement" | "bit_field_element" => Ok(GeneratedField::BitFieldElement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLinkedChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.AnnotationLinkedChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationLinkedChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(annotation_linked_channel::Type::Channel)
;
                        }
                        GeneratedField::BitFieldElement => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldElement"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(annotation_linked_channel::Type::BitFieldElement)
;
                        }
                    }
                }
                Ok(AnnotationLinkedChannel {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.AnnotationLinkedChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLinkedChannelsBitFieldElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_id.is_empty() {
            len += 1;
        }
        if !self.bit_field_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.AnnotationLinkedChannelsBitFieldElement", len)?;
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        if !self.bit_field_name.is_empty() {
            struct_ser.serialize_field("bitFieldName", &self.bit_field_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLinkedChannelsBitFieldElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_id",
            "channelId",
            "bit_field_name",
            "bitFieldName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelId,
            BitFieldName,
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
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            "bitFieldName" | "bit_field_name" => Ok(GeneratedField::BitFieldName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLinkedChannelsBitFieldElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.AnnotationLinkedChannelsBitFieldElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationLinkedChannelsBitFieldElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_id__ = None;
                let mut bit_field_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BitFieldName => {
                            if bit_field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldName"));
                            }
                            bit_field_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnnotationLinkedChannelsBitFieldElement {
                    channel_id: channel_id__.unwrap_or_default(),
                    bit_field_name: bit_field_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.AnnotationLinkedChannelsBitFieldElement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLinkedChannelsChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.AnnotationLinkedChannelsChannel", len)?;
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLinkedChannelsChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_id",
            "channelId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelId,
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
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLinkedChannelsChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.AnnotationLinkedChannelsChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationLinkedChannelsChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnnotationLinkedChannelsChannel {
                    channel_id: channel_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.AnnotationLinkedChannelsChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ANNOTATION_STATE_UNSPECIFIED",
            Self::Open => "ANNOTATION_STATE_OPEN",
            Self::Flagged => "ANNOTATION_STATE_FLAGGED",
            Self::Resolved => "ANNOTATION_STATE_RESOLVED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANNOTATION_STATE_UNSPECIFIED",
            "ANNOTATION_STATE_OPEN",
            "ANNOTATION_STATE_FLAGGED",
            "ANNOTATION_STATE_RESOLVED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationState;

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
                    "ANNOTATION_STATE_UNSPECIFIED" => Ok(AnnotationState::Unspecified),
                    "ANNOTATION_STATE_OPEN" => Ok(AnnotationState::Open),
                    "ANNOTATION_STATE_FLAGGED" => Ok(AnnotationState::Flagged),
                    "ANNOTATION_STATE_RESOLVED" => Ok(AnnotationState::Resolved),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ANNOTATION_TYPE_UNSPECIFIED",
            Self::DataReview => "ANNOTATION_TYPE_DATA_REVIEW",
            Self::Phase => "ANNOTATION_TYPE_PHASE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANNOTATION_TYPE_UNSPECIFIED",
            "ANNOTATION_TYPE_DATA_REVIEW",
            "ANNOTATION_TYPE_PHASE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationType;

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
                    "ANNOTATION_TYPE_UNSPECIFIED" => Ok(AnnotationType::Unspecified),
                    "ANNOTATION_TYPE_DATA_REVIEW" => Ok(AnnotationType::DataReview),
                    "ANNOTATION_TYPE_PHASE" => Ok(AnnotationType::Phase),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveAnnotationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.ArchiveAnnotationRequest", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveAnnotationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
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
                            "annotationId" | "annotation_id" => Ok(GeneratedField::AnnotationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveAnnotationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.ArchiveAnnotationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveAnnotationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveAnnotationRequest {
                    annotation_id: annotation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.ArchiveAnnotationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveAnnotationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.ArchiveAnnotationResponse", len)?;
        if let Some(v) = self.annotation.as_ref() {
            struct_ser.serialize_field("annotation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveAnnotationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotation,
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
                            "annotation" => Ok(GeneratedField::Annotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveAnnotationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.ArchiveAnnotationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveAnnotationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotation => {
                            if annotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            annotation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ArchiveAnnotationResponse {
                    annotation: annotation__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.ArchiveAnnotationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveAnnotationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.BatchArchiveAnnotationsRequest", len)?;
        if !self.annotation_ids.is_empty() {
            struct_ser.serialize_field("annotationIds", &self.annotation_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveAnnotationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_ids",
            "annotationIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationIds,
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
                            "annotationIds" | "annotation_ids" => Ok(GeneratedField::AnnotationIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchArchiveAnnotationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.BatchArchiveAnnotationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveAnnotationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationIds => {
                            if annotation_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationIds"));
                            }
                            annotation_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchArchiveAnnotationsRequest {
                    annotation_ids: annotation_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.BatchArchiveAnnotationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveAnnotationsResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.BatchArchiveAnnotationsResponse", len)?;
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveAnnotationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotations,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchArchiveAnnotationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.BatchArchiveAnnotationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveAnnotationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchArchiveAnnotationsResponse {
                    annotations: annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.BatchArchiveAnnotationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchDeleteAnnotationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.BatchDeleteAnnotationsRequest", len)?;
        if !self.annotation_ids.is_empty() {
            struct_ser.serialize_field("annotationIds", &self.annotation_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteAnnotationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_ids",
            "annotationIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationIds,
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
                            "annotationIds" | "annotation_ids" => Ok(GeneratedField::AnnotationIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchDeleteAnnotationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.BatchDeleteAnnotationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteAnnotationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationIds => {
                            if annotation_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationIds"));
                            }
                            annotation_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchDeleteAnnotationsRequest {
                    annotation_ids: annotation_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.BatchDeleteAnnotationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchDeleteAnnotationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.annotations.v1.BatchDeleteAnnotationsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteAnnotationsResponse {
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
            type Value = BatchDeleteAnnotationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.BatchDeleteAnnotationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteAnnotationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchDeleteAnnotationsResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.BatchDeleteAnnotationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveAnnotationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.BatchUnarchiveAnnotationsRequest", len)?;
        if !self.annotation_ids.is_empty() {
            struct_ser.serialize_field("annotationIds", &self.annotation_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveAnnotationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_ids",
            "annotationIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationIds,
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
                            "annotationIds" | "annotation_ids" => Ok(GeneratedField::AnnotationIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUnarchiveAnnotationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.BatchUnarchiveAnnotationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveAnnotationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationIds => {
                            if annotation_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationIds"));
                            }
                            annotation_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUnarchiveAnnotationsRequest {
                    annotation_ids: annotation_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.BatchUnarchiveAnnotationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveAnnotationsResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.BatchUnarchiveAnnotationsResponse", len)?;
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveAnnotationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotations,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUnarchiveAnnotationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.BatchUnarchiveAnnotationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveAnnotationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUnarchiveAnnotationsResponse {
                    annotations: annotations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.BatchUnarchiveAnnotationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAnnotationRequest {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if !self.assets.is_empty() {
            len += 1;
        }
        if !self.linked_channels.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.run_id.is_some() {
            len += 1;
        }
        if self.assign_to_user_id.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.state.is_some() {
            len += 1;
        }
        if self.annotation_type != 0 {
            len += 1;
        }
        if self.created_by_condition_id.is_some() {
            len += 1;
        }
        if self.legend_config.is_some() {
            len += 1;
        }
        if self.created_by_rule_condition_version_id.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.CreateAnnotationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if !self.assets.is_empty() {
            struct_ser.serialize_field("assets", &self.assets)?;
        }
        if !self.linked_channels.is_empty() {
            struct_ser.serialize_field("linkedChannels", &self.linked_channels)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.assign_to_user_id.as_ref() {
            struct_ser.serialize_field("assignToUserId", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.state.as_ref() {
            let v = AnnotationState::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if self.annotation_type != 0 {
            let v = AnnotationType::try_from(self.annotation_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.annotation_type)))?;
            struct_ser.serialize_field("annotationType", &v)?;
        }
        if let Some(v) = self.created_by_condition_id.as_ref() {
            struct_ser.serialize_field("createdByConditionId", v)?;
        }
        if let Some(v) = self.legend_config.as_ref() {
            struct_ser.serialize_field("legendConfig", v)?;
        }
        if let Some(v) = self.created_by_rule_condition_version_id.as_ref() {
            struct_ser.serialize_field("createdByRuleConditionVersionId", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAnnotationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "assets",
            "linked_channels",
            "linkedChannels",
            "tags",
            "run_id",
            "runId",
            "assign_to_user_id",
            "assignToUserId",
            "organization_id",
            "organizationId",
            "state",
            "annotation_type",
            "annotationType",
            "created_by_condition_id",
            "createdByConditionId",
            "legend_config",
            "legendConfig",
            "created_by_rule_condition_version_id",
            "createdByRuleConditionVersionId",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            StartTime,
            EndTime,
            Assets,
            LinkedChannels,
            Tags,
            RunId,
            AssignToUserId,
            OrganizationId,
            State,
            AnnotationType,
            CreatedByConditionId,
            LegendConfig,
            CreatedByRuleConditionVersionId,
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
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "assets" => Ok(GeneratedField::Assets),
                            "linkedChannels" | "linked_channels" => Ok(GeneratedField::LinkedChannels),
                            "tags" => Ok(GeneratedField::Tags),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "assignToUserId" | "assign_to_user_id" => Ok(GeneratedField::AssignToUserId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "state" => Ok(GeneratedField::State),
                            "annotationType" | "annotation_type" => Ok(GeneratedField::AnnotationType),
                            "createdByConditionId" | "created_by_condition_id" => Ok(GeneratedField::CreatedByConditionId),
                            "legendConfig" | "legend_config" => Ok(GeneratedField::LegendConfig),
                            "createdByRuleConditionVersionId" | "created_by_rule_condition_version_id" => Ok(GeneratedField::CreatedByRuleConditionVersionId),
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
            type Value = CreateAnnotationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.CreateAnnotationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAnnotationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut assets__ = None;
                let mut linked_channels__ = None;
                let mut tags__ = None;
                let mut run_id__ = None;
                let mut assign_to_user_id__ = None;
                let mut organization_id__ = None;
                let mut state__ = None;
                let mut annotation_type__ = None;
                let mut created_by_condition_id__ = None;
                let mut legend_config__ = None;
                let mut created_by_rule_condition_version_id__ = None;
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
                            description__ = Some(map_.next_value()?);
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
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkedChannels => {
                            if linked_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkedChannels"));
                            }
                            linked_channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map_.next_value()?;
                        }
                        GeneratedField::AssignToUserId => {
                            if assign_to_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignToUserId"));
                            }
                            assign_to_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = map_.next_value::<::std::option::Option<AnnotationState>>()?.map(|x| x as i32);
                        }
                        GeneratedField::AnnotationType => {
                            if annotation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationType"));
                            }
                            annotation_type__ = Some(map_.next_value::<AnnotationType>()? as i32);
                        }
                        GeneratedField::CreatedByConditionId => {
                            if created_by_condition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByConditionId"));
                            }
                            created_by_condition_id__ = map_.next_value()?;
                        }
                        GeneratedField::LegendConfig => {
                            if legend_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("legendConfig"));
                            }
                            legend_config__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByRuleConditionVersionId => {
                            if created_by_rule_condition_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByRuleConditionVersionId"));
                            }
                            created_by_rule_condition_version_id__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateAnnotationRequest {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    assets: assets__.unwrap_or_default(),
                    linked_channels: linked_channels__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    run_id: run_id__,
                    assign_to_user_id: assign_to_user_id__,
                    organization_id: organization_id__.unwrap_or_default(),
                    state: state__,
                    annotation_type: annotation_type__.unwrap_or_default(),
                    created_by_condition_id: created_by_condition_id__,
                    legend_config: legend_config__,
                    created_by_rule_condition_version_id: created_by_rule_condition_version_id__,
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.CreateAnnotationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAnnotationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.CreateAnnotationResponse", len)?;
        if let Some(v) = self.annotation.as_ref() {
            struct_ser.serialize_field("annotation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAnnotationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotation,
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
                            "annotation" => Ok(GeneratedField::Annotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAnnotationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.CreateAnnotationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAnnotationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotation => {
                            if annotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            annotation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateAnnotationResponse {
                    annotation: annotation__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.CreateAnnotationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAnnotationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.DeleteAnnotationRequest", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAnnotationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
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
                            "annotationId" | "annotation_id" => Ok(GeneratedField::AnnotationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAnnotationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.DeleteAnnotationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAnnotationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteAnnotationRequest {
                    annotation_id: annotation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.DeleteAnnotationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAnnotationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.annotations.v1.DeleteAnnotationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAnnotationResponse {
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
            type Value = DeleteAnnotationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.DeleteAnnotationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAnnotationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAnnotationResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.DeleteAnnotationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAnnotationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.GetAnnotationRequest", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAnnotationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
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
                            "annotationId" | "annotation_id" => Ok(GeneratedField::AnnotationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAnnotationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.GetAnnotationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAnnotationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAnnotationRequest {
                    annotation_id: annotation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.GetAnnotationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAnnotationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.GetAnnotationResponse", len)?;
        if let Some(v) = self.annotation.as_ref() {
            struct_ser.serialize_field("annotation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAnnotationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotation,
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
                            "annotation" => Ok(GeneratedField::Annotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAnnotationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.GetAnnotationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAnnotationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotation => {
                            if annotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            annotation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAnnotationResponse {
                    annotation: annotation__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.GetAnnotationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAnnotationsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.ListAnnotationsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListAnnotationsRequest {
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
            type Value = ListAnnotationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.ListAnnotationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAnnotationsRequest, V::Error>
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
                Ok(ListAnnotationsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.ListAnnotationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAnnotationsResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.ListAnnotationsResponse", len)?;
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAnnotationsResponse {
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
            type Value = ListAnnotationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.ListAnnotationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAnnotationsResponse, V::Error>
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
                Ok(ListAnnotationsResponse {
                    annotations: annotations__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.ListAnnotationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveAnnotationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.UnarchiveAnnotationRequest", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveAnnotationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
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
                            "annotationId" | "annotation_id" => Ok(GeneratedField::AnnotationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveAnnotationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.UnarchiveAnnotationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveAnnotationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchiveAnnotationRequest {
                    annotation_id: annotation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.UnarchiveAnnotationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveAnnotationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.UnarchiveAnnotationResponse", len)?;
        if let Some(v) = self.annotation.as_ref() {
            struct_ser.serialize_field("annotation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveAnnotationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotation,
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
                            "annotation" => Ok(GeneratedField::Annotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveAnnotationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.UnarchiveAnnotationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveAnnotationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotation => {
                            if annotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            annotation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UnarchiveAnnotationResponse {
                    annotation: annotation__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.UnarchiveAnnotationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAnnotationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.UpdateAnnotationRequest", len)?;
        if let Some(v) = self.annotation.as_ref() {
            struct_ser.serialize_field("annotation", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAnnotationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotation,
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
                            "annotation" => Ok(GeneratedField::Annotation),
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
            type Value = UpdateAnnotationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.UpdateAnnotationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAnnotationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotation => {
                            if annotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            annotation__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAnnotationRequest {
                    annotation: annotation__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.UpdateAnnotationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAnnotationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotations.v1.UpdateAnnotationResponse", len)?;
        if let Some(v) = self.annotation.as_ref() {
            struct_ser.serialize_field("annotation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAnnotationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Annotation,
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
                            "annotation" => Ok(GeneratedField::Annotation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAnnotationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotations.v1.UpdateAnnotationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAnnotationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Annotation => {
                            if annotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            annotation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAnnotationResponse {
                    annotation: annotation__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotations.v1.UpdateAnnotationResponse", FIELDS, GeneratedVisitor)
    }
}
