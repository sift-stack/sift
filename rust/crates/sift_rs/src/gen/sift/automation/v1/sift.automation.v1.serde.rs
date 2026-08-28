// @generated
impl serde::Serialize for AutomationTrigger {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.automation_trigger_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.canvas_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.match_filter.is_some() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.action_kind != 0 {
            len += 1;
        }
        if self.action_params.is_some() {
            len += 1;
        }
        if self.is_enabled {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.email_notifications_enabled {
            len += 1;
        }
        if !self.notification_recipient_emails.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.AutomationTrigger", len)?;
        if !self.automation_trigger_id.is_empty() {
            struct_ser.serialize_field("automationTriggerId", &self.automation_trigger_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.canvas_id.is_empty() {
            struct_ser.serialize_field("canvasId", &self.canvas_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.match_filter.as_ref() {
            struct_ser.serialize_field("matchFilter", v)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if self.action_kind != 0 {
            let v = AutomationTriggerActionKind::try_from(self.action_kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action_kind)))?;
            struct_ser.serialize_field("actionKind", &v)?;
        }
        if let Some(v) = self.action_params.as_ref() {
            struct_ser.serialize_field("actionParams", v)?;
        }
        if self.is_enabled {
            struct_ser.serialize_field("isEnabled", &self.is_enabled)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if self.email_notifications_enabled {
            struct_ser.serialize_field("emailNotificationsEnabled", &self.email_notifications_enabled)?;
        }
        if !self.notification_recipient_emails.is_empty() {
            struct_ser.serialize_field("notificationRecipientEmails", &self.notification_recipient_emails)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTrigger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_trigger_id",
            "automationTriggerId",
            "organization_id",
            "organizationId",
            "canvas_id",
            "canvasId",
            "name",
            "match_filter",
            "matchFilter",
            "user_id",
            "userId",
            "action_kind",
            "actionKind",
            "action_params",
            "actionParams",
            "is_enabled",
            "isEnabled",
            "archived_date",
            "archivedDate",
            "is_archived",
            "isArchived",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "email_notifications_enabled",
            "emailNotificationsEnabled",
            "notification_recipient_emails",
            "notificationRecipientEmails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTriggerId,
            OrganizationId,
            CanvasId,
            Name,
            MatchFilter,
            UserId,
            ActionKind,
            ActionParams,
            IsEnabled,
            ArchivedDate,
            IsArchived,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
            EmailNotificationsEnabled,
            NotificationRecipientEmails,
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
                            "automationTriggerId" | "automation_trigger_id" => Ok(GeneratedField::AutomationTriggerId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "canvasId" | "canvas_id" => Ok(GeneratedField::CanvasId),
                            "name" => Ok(GeneratedField::Name),
                            "matchFilter" | "match_filter" => Ok(GeneratedField::MatchFilter),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "actionKind" | "action_kind" => Ok(GeneratedField::ActionKind),
                            "actionParams" | "action_params" => Ok(GeneratedField::ActionParams),
                            "isEnabled" | "is_enabled" => Ok(GeneratedField::IsEnabled),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "isArchived" | "is_archived" => Ok(GeneratedField::IsArchived),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "emailNotificationsEnabled" | "email_notifications_enabled" => Ok(GeneratedField::EmailNotificationsEnabled),
                            "notificationRecipientEmails" | "notification_recipient_emails" => Ok(GeneratedField::NotificationRecipientEmails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTrigger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.AutomationTrigger")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutomationTrigger, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_trigger_id__ = None;
                let mut organization_id__ = None;
                let mut canvas_id__ = None;
                let mut name__ = None;
                let mut match_filter__ = None;
                let mut user_id__ = None;
                let mut action_kind__ = None;
                let mut action_params__ = None;
                let mut is_enabled__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut email_notifications_enabled__ = None;
                let mut notification_recipient_emails__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTriggerId => {
                            if automation_trigger_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTriggerId"));
                            }
                            automation_trigger_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CanvasId => {
                            if canvas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canvasId"));
                            }
                            canvas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MatchFilter => {
                            if match_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchFilter"));
                            }
                            match_filter__ = map_.next_value()?;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionKind => {
                            if action_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionKind"));
                            }
                            action_kind__ = Some(map_.next_value::<AutomationTriggerActionKind>()? as i32);
                        }
                        GeneratedField::ActionParams => {
                            if action_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionParams"));
                            }
                            action_params__ = map_.next_value()?;
                        }
                        GeneratedField::IsEnabled => {
                            if is_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEnabled"));
                            }
                            is_enabled__ = Some(map_.next_value()?);
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
                        GeneratedField::EmailNotificationsEnabled => {
                            if email_notifications_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailNotificationsEnabled"));
                            }
                            email_notifications_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotificationRecipientEmails => {
                            if notification_recipient_emails__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationRecipientEmails"));
                            }
                            notification_recipient_emails__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AutomationTrigger {
                    automation_trigger_id: automation_trigger_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    canvas_id: canvas_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    match_filter: match_filter__,
                    user_id: user_id__.unwrap_or_default(),
                    action_kind: action_kind__.unwrap_or_default(),
                    action_params: action_params__,
                    is_enabled: is_enabled__.unwrap_or_default(),
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    email_notifications_enabled: email_notifications_enabled__.unwrap_or_default(),
                    notification_recipient_emails: notification_recipient_emails__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.AutomationTrigger", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerActionKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTOMATION_TRIGGER_ACTION_KIND_UNSPECIFIED",
            Self::CanvasEvaluation => "AUTOMATION_TRIGGER_ACTION_KIND_CANVAS_EVALUATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerActionKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTOMATION_TRIGGER_ACTION_KIND_UNSPECIFIED",
            "AUTOMATION_TRIGGER_ACTION_KIND_CANVAS_EVALUATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerActionKind;

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
                    "AUTOMATION_TRIGGER_ACTION_KIND_UNSPECIFIED" => Ok(AutomationTriggerActionKind::Unspecified),
                    "AUTOMATION_TRIGGER_ACTION_KIND_CANVAS_EVALUATION" => Ok(AutomationTriggerActionKind::CanvasEvaluation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerActionParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.AutomationTriggerActionParams", len)?;
        if let Some(v) = self.kind.as_ref() {
            match v {
                automation_trigger_action_params::Kind::CanvasEvaluation(v) => {
                    struct_ser.serialize_field("canvasEvaluation", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerActionParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canvas_evaluation",
            "canvasEvaluation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanvasEvaluation,
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
                            "canvasEvaluation" | "canvas_evaluation" => Ok(GeneratedField::CanvasEvaluation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerActionParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.AutomationTriggerActionParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutomationTriggerActionParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanvasEvaluation => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canvasEvaluation"));
                            }
                            kind__ = map_.next_value::<::std::option::Option<_>>()?.map(automation_trigger_action_params::Kind::CanvasEvaluation)
;
                        }
                    }
                }
                Ok(AutomationTriggerActionParams {
                    kind: kind__,
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.AutomationTriggerActionParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerMatchFilter {
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
        if self.name_match.is_some() {
            len += 1;
        }
        if !self.metadata_matches.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.AutomationTriggerMatchFilter", len)?;
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if let Some(v) = self.name_match.as_ref() {
            struct_ser.serialize_field("nameMatch", v)?;
        }
        if !self.metadata_matches.is_empty() {
            struct_ser.serialize_field("metadataMatches", &self.metadata_matches)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerMatchFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_ids",
            "assetIds",
            "name_match",
            "nameMatch",
            "metadata_matches",
            "metadataMatches",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetIds,
            NameMatch,
            MetadataMatches,
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
                            "nameMatch" | "name_match" => Ok(GeneratedField::NameMatch),
                            "metadataMatches" | "metadata_matches" => Ok(GeneratedField::MetadataMatches),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerMatchFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.AutomationTriggerMatchFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutomationTriggerMatchFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_ids__ = None;
                let mut name_match__ = None;
                let mut metadata_matches__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NameMatch => {
                            if name_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameMatch"));
                            }
                            name_match__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataMatches => {
                            if metadata_matches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatches"));
                            }
                            metadata_matches__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AutomationTriggerMatchFilter {
                    asset_ids: asset_ids__.unwrap_or_default(),
                    name_match: name_match__,
                    metadata_matches: metadata_matches__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.AutomationTriggerMatchFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerMetadataMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.AutomationTriggerMetadataMatch", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerMetadataMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerMetadataMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.AutomationTriggerMetadataMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutomationTriggerMetadataMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AutomationTriggerMetadataMatch {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.AutomationTriggerMetadataMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerParamSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTOMATION_TRIGGER_PARAM_SOURCE_UNSPECIFIED",
            Self::TriggeringRun => "AUTOMATION_TRIGGER_PARAM_SOURCE_TRIGGERING_RUN",
            Self::TriggeringRunAssets => "AUTOMATION_TRIGGER_PARAM_SOURCE_TRIGGERING_RUN_ASSETS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerParamSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTOMATION_TRIGGER_PARAM_SOURCE_UNSPECIFIED",
            "AUTOMATION_TRIGGER_PARAM_SOURCE_TRIGGERING_RUN",
            "AUTOMATION_TRIGGER_PARAM_SOURCE_TRIGGERING_RUN_ASSETS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerParamSource;

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
                    "AUTOMATION_TRIGGER_PARAM_SOURCE_UNSPECIFIED" => Ok(AutomationTriggerParamSource::Unspecified),
                    "AUTOMATION_TRIGGER_PARAM_SOURCE_TRIGGERING_RUN" => Ok(AutomationTriggerParamSource::TriggeringRun),
                    "AUTOMATION_TRIGGER_PARAM_SOURCE_TRIGGERING_RUN_ASSETS" => Ok(AutomationTriggerParamSource::TriggeringRunAssets),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerParamValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.AutomationTriggerParamValue", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                automation_trigger_param_value::Value::Literal(v) => {
                    struct_ser.serialize_field("literal", v)?;
                }
                automation_trigger_param_value::Value::Source(v) => {
                    let v = AutomationTriggerParamSource::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("source", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerParamValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "literal",
            "source",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Literal,
            Source,
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
                            "literal" => Ok(GeneratedField::Literal),
                            "source" => Ok(GeneratedField::Source),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerParamValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.AutomationTriggerParamValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutomationTriggerParamValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Literal => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("literal"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(automation_trigger_param_value::Value::Literal)
;
                        }
                        GeneratedField::Source => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            value__ = map_.next_value::<::std::option::Option<AutomationTriggerParamSource>>()?.map(|x| automation_trigger_param_value::Value::Source(x as i32));
                        }
                    }
                }
                Ok(AutomationTriggerParamValue {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.AutomationTriggerParamValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerRunNameMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.AutomationTriggerRunNameMatch", len)?;
        if self.kind != 0 {
            let v = AutomationTriggerRunNameMatchKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerRunNameMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Value,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerRunNameMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.AutomationTriggerRunNameMatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutomationTriggerRunNameMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<AutomationTriggerRunNameMatchKind>()? as i32);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AutomationTriggerRunNameMatch {
                    kind: kind__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.AutomationTriggerRunNameMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggerRunNameMatchKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_UNSPECIFIED",
            Self::StartsWith => "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_STARTS_WITH",
            Self::EndsWith => "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_ENDS_WITH",
            Self::Contains => "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_CONTAINS",
            Self::Regex => "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_REGEX",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggerRunNameMatchKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_UNSPECIFIED",
            "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_STARTS_WITH",
            "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_ENDS_WITH",
            "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_CONTAINS",
            "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_REGEX",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggerRunNameMatchKind;

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
                    "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_UNSPECIFIED" => Ok(AutomationTriggerRunNameMatchKind::Unspecified),
                    "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_STARTS_WITH" => Ok(AutomationTriggerRunNameMatchKind::StartsWith),
                    "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_ENDS_WITH" => Ok(AutomationTriggerRunNameMatchKind::EndsWith),
                    "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_CONTAINS" => Ok(AutomationTriggerRunNameMatchKind::Contains),
                    "AUTOMATION_TRIGGER_RUN_NAME_MATCH_KIND_REGEX" => Ok(AutomationTriggerRunNameMatchKind::Regex),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggeredEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.automation_triggered_event_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.automation_trigger_id.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.dispatch_after.is_some() {
            len += 1;
        }
        if self.canvas_execution_id.is_some() {
            len += 1;
        }
        if self.attempt_count != 0 {
            len += 1;
        }
        if self.last_error.is_some() {
            len += 1;
        }
        if self.terminal_date.is_some() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.AutomationTriggeredEvent", len)?;
        if !self.automation_triggered_event_id.is_empty() {
            struct_ser.serialize_field("automationTriggeredEventId", &self.automation_triggered_event_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.automation_trigger_id.is_empty() {
            struct_ser.serialize_field("automationTriggerId", &self.automation_trigger_id)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if self.state != 0 {
            let v = AutomationTriggeredEventState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.dispatch_after.as_ref() {
            struct_ser.serialize_field("dispatchAfter", v)?;
        }
        if let Some(v) = self.canvas_execution_id.as_ref() {
            struct_ser.serialize_field("canvasExecutionId", v)?;
        }
        if self.attempt_count != 0 {
            struct_ser.serialize_field("attemptCount", &self.attempt_count)?;
        }
        if let Some(v) = self.last_error.as_ref() {
            struct_ser.serialize_field("lastError", v)?;
        }
        if let Some(v) = self.terminal_date.as_ref() {
            struct_ser.serialize_field("terminalDate", v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggeredEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_triggered_event_id",
            "automationTriggeredEventId",
            "organization_id",
            "organizationId",
            "automation_trigger_id",
            "automationTriggerId",
            "run_id",
            "runId",
            "state",
            "dispatch_after",
            "dispatchAfter",
            "canvas_execution_id",
            "canvasExecutionId",
            "attempt_count",
            "attemptCount",
            "last_error",
            "lastError",
            "terminal_date",
            "terminalDate",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTriggeredEventId,
            OrganizationId,
            AutomationTriggerId,
            RunId,
            State,
            DispatchAfter,
            CanvasExecutionId,
            AttemptCount,
            LastError,
            TerminalDate,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "automationTriggeredEventId" | "automation_triggered_event_id" => Ok(GeneratedField::AutomationTriggeredEventId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "automationTriggerId" | "automation_trigger_id" => Ok(GeneratedField::AutomationTriggerId),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "state" => Ok(GeneratedField::State),
                            "dispatchAfter" | "dispatch_after" => Ok(GeneratedField::DispatchAfter),
                            "canvasExecutionId" | "canvas_execution_id" => Ok(GeneratedField::CanvasExecutionId),
                            "attemptCount" | "attempt_count" => Ok(GeneratedField::AttemptCount),
                            "lastError" | "last_error" => Ok(GeneratedField::LastError),
                            "terminalDate" | "terminal_date" => Ok(GeneratedField::TerminalDate),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggeredEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.AutomationTriggeredEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AutomationTriggeredEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_triggered_event_id__ = None;
                let mut organization_id__ = None;
                let mut automation_trigger_id__ = None;
                let mut run_id__ = None;
                let mut state__ = None;
                let mut dispatch_after__ = None;
                let mut canvas_execution_id__ = None;
                let mut attempt_count__ = None;
                let mut last_error__ = None;
                let mut terminal_date__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTriggeredEventId => {
                            if automation_triggered_event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTriggeredEventId"));
                            }
                            automation_triggered_event_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AutomationTriggerId => {
                            if automation_trigger_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTriggerId"));
                            }
                            automation_trigger_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<AutomationTriggeredEventState>()? as i32);
                        }
                        GeneratedField::DispatchAfter => {
                            if dispatch_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dispatchAfter"));
                            }
                            dispatch_after__ = map_.next_value()?;
                        }
                        GeneratedField::CanvasExecutionId => {
                            if canvas_execution_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canvasExecutionId"));
                            }
                            canvas_execution_id__ = map_.next_value()?;
                        }
                        GeneratedField::AttemptCount => {
                            if attempt_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attemptCount"));
                            }
                            attempt_count__ =
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastError => {
                            if last_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastError"));
                            }
                            last_error__ = map_.next_value()?;
                        }
                        GeneratedField::TerminalDate => {
                            if terminal_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminalDate"));
                            }
                            terminal_date__ = map_.next_value()?;
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
                    }
                }
                Ok(AutomationTriggeredEvent {
                    automation_triggered_event_id: automation_triggered_event_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    automation_trigger_id: automation_trigger_id__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    dispatch_after: dispatch_after__,
                    canvas_execution_id: canvas_execution_id__,
                    attempt_count: attempt_count__.unwrap_or_default(),
                    last_error: last_error__,
                    terminal_date: terminal_date__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.AutomationTriggeredEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AutomationTriggeredEventState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "AUTOMATION_TRIGGERED_EVENT_STATE_UNSPECIFIED",
            Self::Waiting => "AUTOMATION_TRIGGERED_EVENT_STATE_WAITING",
            Self::Dispatching => "AUTOMATION_TRIGGERED_EVENT_STATE_DISPATCHING",
            Self::Done => "AUTOMATION_TRIGGERED_EVENT_STATE_DONE",
            Self::Failed => "AUTOMATION_TRIGGERED_EVENT_STATE_FAILED",
            Self::Cancelled => "AUTOMATION_TRIGGERED_EVENT_STATE_CANCELLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AutomationTriggeredEventState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTOMATION_TRIGGERED_EVENT_STATE_UNSPECIFIED",
            "AUTOMATION_TRIGGERED_EVENT_STATE_WAITING",
            "AUTOMATION_TRIGGERED_EVENT_STATE_DISPATCHING",
            "AUTOMATION_TRIGGERED_EVENT_STATE_DONE",
            "AUTOMATION_TRIGGERED_EVENT_STATE_FAILED",
            "AUTOMATION_TRIGGERED_EVENT_STATE_CANCELLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AutomationTriggeredEventState;

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
                    "AUTOMATION_TRIGGERED_EVENT_STATE_UNSPECIFIED" => Ok(AutomationTriggeredEventState::Unspecified),
                    "AUTOMATION_TRIGGERED_EVENT_STATE_WAITING" => Ok(AutomationTriggeredEventState::Waiting),
                    "AUTOMATION_TRIGGERED_EVENT_STATE_DISPATCHING" => Ok(AutomationTriggeredEventState::Dispatching),
                    "AUTOMATION_TRIGGERED_EVENT_STATE_DONE" => Ok(AutomationTriggeredEventState::Done),
                    "AUTOMATION_TRIGGERED_EVENT_STATE_FAILED" => Ok(AutomationTriggeredEventState::Failed),
                    "AUTOMATION_TRIGGERED_EVENT_STATE_CANCELLED" => Ok(AutomationTriggeredEventState::Cancelled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CanvasEvaluationActionParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.canvas_version_id.is_empty() {
            len += 1;
        }
        if !self.param_json.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.CanvasEvaluationActionParams", len)?;
        if !self.canvas_version_id.is_empty() {
            struct_ser.serialize_field("canvasVersionId", &self.canvas_version_id)?;
        }
        if !self.param_json.is_empty() {
            struct_ser.serialize_field("paramJson", &self.param_json)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CanvasEvaluationActionParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canvas_version_id",
            "canvasVersionId",
            "param_json",
            "paramJson",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanvasVersionId,
            ParamJson,
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
                            "canvasVersionId" | "canvas_version_id" => Ok(GeneratedField::CanvasVersionId),
                            "paramJson" | "param_json" => Ok(GeneratedField::ParamJson),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CanvasEvaluationActionParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.CanvasEvaluationActionParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CanvasEvaluationActionParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canvas_version_id__ = None;
                let mut param_json__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanvasVersionId => {
                            if canvas_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canvasVersionId"));
                            }
                            canvas_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParamJson => {
                            if param_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paramJson"));
                            }
                            param_json__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(CanvasEvaluationActionParams {
                    canvas_version_id: canvas_version_id__.unwrap_or_default(),
                    param_json: param_json__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.CanvasEvaluationActionParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAutomationTriggerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.canvas_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.match_filter.is_some() {
            len += 1;
        }
        if self.action_kind != 0 {
            len += 1;
        }
        if self.action_params.is_some() {
            len += 1;
        }
        if self.is_enabled.is_some() {
            len += 1;
        }
        if self.email_notifications_enabled.is_some() {
            len += 1;
        }
        if !self.notification_recipient_emails.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.CreateAutomationTriggerRequest", len)?;
        if !self.canvas_id.is_empty() {
            struct_ser.serialize_field("canvasId", &self.canvas_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.match_filter.as_ref() {
            struct_ser.serialize_field("matchFilter", v)?;
        }
        if self.action_kind != 0 {
            let v = AutomationTriggerActionKind::try_from(self.action_kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action_kind)))?;
            struct_ser.serialize_field("actionKind", &v)?;
        }
        if let Some(v) = self.action_params.as_ref() {
            struct_ser.serialize_field("actionParams", v)?;
        }
        if let Some(v) = self.is_enabled.as_ref() {
            struct_ser.serialize_field("isEnabled", v)?;
        }
        if let Some(v) = self.email_notifications_enabled.as_ref() {
            struct_ser.serialize_field("emailNotificationsEnabled", v)?;
        }
        if !self.notification_recipient_emails.is_empty() {
            struct_ser.serialize_field("notificationRecipientEmails", &self.notification_recipient_emails)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAutomationTriggerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canvas_id",
            "canvasId",
            "name",
            "match_filter",
            "matchFilter",
            "action_kind",
            "actionKind",
            "action_params",
            "actionParams",
            "is_enabled",
            "isEnabled",
            "email_notifications_enabled",
            "emailNotificationsEnabled",
            "notification_recipient_emails",
            "notificationRecipientEmails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanvasId,
            Name,
            MatchFilter,
            ActionKind,
            ActionParams,
            IsEnabled,
            EmailNotificationsEnabled,
            NotificationRecipientEmails,
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
                            "canvasId" | "canvas_id" => Ok(GeneratedField::CanvasId),
                            "name" => Ok(GeneratedField::Name),
                            "matchFilter" | "match_filter" => Ok(GeneratedField::MatchFilter),
                            "actionKind" | "action_kind" => Ok(GeneratedField::ActionKind),
                            "actionParams" | "action_params" => Ok(GeneratedField::ActionParams),
                            "isEnabled" | "is_enabled" => Ok(GeneratedField::IsEnabled),
                            "emailNotificationsEnabled" | "email_notifications_enabled" => Ok(GeneratedField::EmailNotificationsEnabled),
                            "notificationRecipientEmails" | "notification_recipient_emails" => Ok(GeneratedField::NotificationRecipientEmails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAutomationTriggerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.CreateAutomationTriggerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAutomationTriggerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canvas_id__ = None;
                let mut name__ = None;
                let mut match_filter__ = None;
                let mut action_kind__ = None;
                let mut action_params__ = None;
                let mut is_enabled__ = None;
                let mut email_notifications_enabled__ = None;
                let mut notification_recipient_emails__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanvasId => {
                            if canvas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canvasId"));
                            }
                            canvas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MatchFilter => {
                            if match_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchFilter"));
                            }
                            match_filter__ = map_.next_value()?;
                        }
                        GeneratedField::ActionKind => {
                            if action_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionKind"));
                            }
                            action_kind__ = Some(map_.next_value::<AutomationTriggerActionKind>()? as i32);
                        }
                        GeneratedField::ActionParams => {
                            if action_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionParams"));
                            }
                            action_params__ = map_.next_value()?;
                        }
                        GeneratedField::IsEnabled => {
                            if is_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEnabled"));
                            }
                            is_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::EmailNotificationsEnabled => {
                            if email_notifications_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailNotificationsEnabled"));
                            }
                            email_notifications_enabled__ = map_.next_value()?;
                        }
                        GeneratedField::NotificationRecipientEmails => {
                            if notification_recipient_emails__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notificationRecipientEmails"));
                            }
                            notification_recipient_emails__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateAutomationTriggerRequest {
                    canvas_id: canvas_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    match_filter: match_filter__,
                    action_kind: action_kind__.unwrap_or_default(),
                    action_params: action_params__,
                    is_enabled: is_enabled__,
                    email_notifications_enabled: email_notifications_enabled__,
                    notification_recipient_emails: notification_recipient_emails__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.CreateAutomationTriggerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAutomationTriggerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.automation_trigger.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.CreateAutomationTriggerResponse", len)?;
        if let Some(v) = self.automation_trigger.as_ref() {
            struct_ser.serialize_field("automationTrigger", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAutomationTriggerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_trigger",
            "automationTrigger",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTrigger,
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
                            "automationTrigger" | "automation_trigger" => Ok(GeneratedField::AutomationTrigger),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAutomationTriggerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.CreateAutomationTriggerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAutomationTriggerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_trigger__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTrigger => {
                            if automation_trigger__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTrigger"));
                            }
                            automation_trigger__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateAutomationTriggerResponse {
                    automation_trigger: automation_trigger__,
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.CreateAutomationTriggerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAutomationTriggerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.automation_trigger_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.DeleteAutomationTriggerRequest", len)?;
        if !self.automation_trigger_id.is_empty() {
            struct_ser.serialize_field("automationTriggerId", &self.automation_trigger_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAutomationTriggerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_trigger_id",
            "automationTriggerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTriggerId,
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
                            "automationTriggerId" | "automation_trigger_id" => Ok(GeneratedField::AutomationTriggerId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAutomationTriggerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.DeleteAutomationTriggerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAutomationTriggerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_trigger_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTriggerId => {
                            if automation_trigger_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTriggerId"));
                            }
                            automation_trigger_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteAutomationTriggerRequest {
                    automation_trigger_id: automation_trigger_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.DeleteAutomationTriggerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAutomationTriggerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.automation.v1.DeleteAutomationTriggerResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAutomationTriggerResponse {
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
            type Value = DeleteAutomationTriggerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.DeleteAutomationTriggerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAutomationTriggerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAutomationTriggerResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.DeleteAutomationTriggerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAutomationTriggerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.automation_trigger_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.GetAutomationTriggerRequest", len)?;
        if !self.automation_trigger_id.is_empty() {
            struct_ser.serialize_field("automationTriggerId", &self.automation_trigger_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAutomationTriggerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_trigger_id",
            "automationTriggerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTriggerId,
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
                            "automationTriggerId" | "automation_trigger_id" => Ok(GeneratedField::AutomationTriggerId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAutomationTriggerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.GetAutomationTriggerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAutomationTriggerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_trigger_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTriggerId => {
                            if automation_trigger_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTriggerId"));
                            }
                            automation_trigger_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAutomationTriggerRequest {
                    automation_trigger_id: automation_trigger_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.GetAutomationTriggerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAutomationTriggerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.automation_trigger.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.GetAutomationTriggerResponse", len)?;
        if let Some(v) = self.automation_trigger.as_ref() {
            struct_ser.serialize_field("automationTrigger", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAutomationTriggerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_trigger",
            "automationTrigger",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTrigger,
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
                            "automationTrigger" | "automation_trigger" => Ok(GeneratedField::AutomationTrigger),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAutomationTriggerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.GetAutomationTriggerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAutomationTriggerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_trigger__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTrigger => {
                            if automation_trigger__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTrigger"));
                            }
                            automation_trigger__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAutomationTriggerResponse {
                    automation_trigger: automation_trigger__,
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.GetAutomationTriggerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAutomationTriggeredEventsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.ListAutomationTriggeredEventsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListAutomationTriggeredEventsRequest {
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
            type Value = ListAutomationTriggeredEventsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.ListAutomationTriggeredEventsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAutomationTriggeredEventsRequest, V::Error>
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
                Ok(ListAutomationTriggeredEventsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.ListAutomationTriggeredEventsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAutomationTriggeredEventsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.automation_triggered_events.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.ListAutomationTriggeredEventsResponse", len)?;
        if !self.automation_triggered_events.is_empty() {
            struct_ser.serialize_field("automationTriggeredEvents", &self.automation_triggered_events)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAutomationTriggeredEventsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_triggered_events",
            "automationTriggeredEvents",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTriggeredEvents,
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
                            "automationTriggeredEvents" | "automation_triggered_events" => Ok(GeneratedField::AutomationTriggeredEvents),
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
            type Value = ListAutomationTriggeredEventsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.ListAutomationTriggeredEventsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAutomationTriggeredEventsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_triggered_events__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTriggeredEvents => {
                            if automation_triggered_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTriggeredEvents"));
                            }
                            automation_triggered_events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAutomationTriggeredEventsResponse {
                    automation_triggered_events: automation_triggered_events__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.ListAutomationTriggeredEventsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAutomationTriggersRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.ListAutomationTriggersRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListAutomationTriggersRequest {
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
            type Value = ListAutomationTriggersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.ListAutomationTriggersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAutomationTriggersRequest, V::Error>
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
                Ok(ListAutomationTriggersRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.ListAutomationTriggersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAutomationTriggersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.automation_triggers.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.ListAutomationTriggersResponse", len)?;
        if !self.automation_triggers.is_empty() {
            struct_ser.serialize_field("automationTriggers", &self.automation_triggers)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAutomationTriggersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_triggers",
            "automationTriggers",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTriggers,
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
                            "automationTriggers" | "automation_triggers" => Ok(GeneratedField::AutomationTriggers),
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
            type Value = ListAutomationTriggersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.ListAutomationTriggersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAutomationTriggersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_triggers__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTriggers => {
                            if automation_triggers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTriggers"));
                            }
                            automation_triggers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAutomationTriggersResponse {
                    automation_triggers: automation_triggers__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.ListAutomationTriggersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAutomationTriggerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.automation_trigger.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.UpdateAutomationTriggerRequest", len)?;
        if let Some(v) = self.automation_trigger.as_ref() {
            struct_ser.serialize_field("automationTrigger", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAutomationTriggerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_trigger",
            "automationTrigger",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTrigger,
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
                            "automationTrigger" | "automation_trigger" => Ok(GeneratedField::AutomationTrigger),
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
            type Value = UpdateAutomationTriggerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.UpdateAutomationTriggerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAutomationTriggerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_trigger__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTrigger => {
                            if automation_trigger__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTrigger"));
                            }
                            automation_trigger__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAutomationTriggerRequest {
                    automation_trigger: automation_trigger__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.UpdateAutomationTriggerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAutomationTriggerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.automation_trigger.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.automation.v1.UpdateAutomationTriggerResponse", len)?;
        if let Some(v) = self.automation_trigger.as_ref() {
            struct_ser.serialize_field("automationTrigger", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAutomationTriggerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "automation_trigger",
            "automationTrigger",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutomationTrigger,
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
                            "automationTrigger" | "automation_trigger" => Ok(GeneratedField::AutomationTrigger),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAutomationTriggerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.automation.v1.UpdateAutomationTriggerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAutomationTriggerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut automation_trigger__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AutomationTrigger => {
                            if automation_trigger__.is_some() {
                                return Err(serde::de::Error::duplicate_field("automationTrigger"));
                            }
                            automation_trigger__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAutomationTriggerResponse {
                    automation_trigger: automation_trigger__,
                })
            }
        }
        deserializer.deserialize_struct("sift.automation.v1.UpdateAutomationTriggerResponse", FIELDS, GeneratedVisitor)
    }
}
