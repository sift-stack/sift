// @generated
impl serde::Serialize for AnnotationCommentBodyElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if !self.text.is_empty() {
            len += 1;
        }
        if self.user_mention.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.AnnotationCommentBodyElement", len)?;
        if self.r#type != 0 {
            let v = AnnotationCommentBodyElementType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.user_mention.as_ref() {
            struct_ser.serialize_field("userMention", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationCommentBodyElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "text",
            "user_mention",
            "userMention",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Text,
            UserMention,
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
                            "type" => Ok(GeneratedField::Type),
                            "text" => Ok(GeneratedField::Text),
                            "userMention" | "user_mention" => Ok(GeneratedField::UserMention),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationCommentBodyElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.AnnotationCommentBodyElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationCommentBodyElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut text__ = None;
                let mut user_mention__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<AnnotationCommentBodyElementType>()? as i32);
                        }
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserMention => {
                            if user_mention__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userMention"));
                            }
                            user_mention__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AnnotationCommentBodyElement {
                    r#type: r#type__.unwrap_or_default(),
                    text: text__.unwrap_or_default(),
                    user_mention: user_mention__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.AnnotationCommentBodyElement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationCommentBodyElementType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED",
            Self::Text => "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT",
            Self::UserMention => "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationCommentBodyElementType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED",
            "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT",
            "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationCommentBodyElementType;

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
                    "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_UNSPECIFIED" => Ok(AnnotationCommentBodyElementType::Unspecified),
                    "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_TEXT" => Ok(AnnotationCommentBodyElementType::Text),
                    "ANNOTATION_COMMENT_BODY_ELEMENT_TYPE_USER_MENTION" => Ok(AnnotationCommentBodyElementType::UserMention),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationCommentUserMention {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.AnnotationCommentUserMention", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_email.is_empty() {
            struct_ser.serialize_field("userEmail", &self.user_email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationCommentUserMention {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_email",
            "userEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserEmail,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userEmail" | "user_email" => Ok(GeneratedField::UserEmail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationCommentUserMention;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.AnnotationCommentUserMention")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationCommentUserMention, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserEmail => {
                            if user_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userEmail"));
                            }
                            user_email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnnotationCommentUserMention {
                    user_id: user_id__.unwrap_or_default(),
                    user_email: user_email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.AnnotationCommentUserMention", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLogAssignedProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assigned_to_user_id.is_empty() {
            len += 1;
        }
        if !self.assigned_to_user_email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.AnnotationLogAssignedProperties", len)?;
        if !self.assigned_to_user_id.is_empty() {
            struct_ser.serialize_field("assignedToUserId", &self.assigned_to_user_id)?;
        }
        if !self.assigned_to_user_email.is_empty() {
            struct_ser.serialize_field("assignedToUserEmail", &self.assigned_to_user_email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLogAssignedProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assigned_to_user_id",
            "assignedToUserId",
            "assigned_to_user_email",
            "assignedToUserEmail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssignedToUserId,
            AssignedToUserEmail,
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
                            "assignedToUserId" | "assigned_to_user_id" => Ok(GeneratedField::AssignedToUserId),
                            "assignedToUserEmail" | "assigned_to_user_email" => Ok(GeneratedField::AssignedToUserEmail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLogAssignedProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.AnnotationLogAssignedProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationLogAssignedProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assigned_to_user_id__ = None;
                let mut assigned_to_user_email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssignedToUserId => {
                            if assigned_to_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignedToUserId"));
                            }
                            assigned_to_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssignedToUserEmail => {
                            if assigned_to_user_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignedToUserEmail"));
                            }
                            assigned_to_user_email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnnotationLogAssignedProperties {
                    assigned_to_user_id: assigned_to_user_id__.unwrap_or_default(),
                    assigned_to_user_email: assigned_to_user_email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.AnnotationLogAssignedProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLogCommentProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.body.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.AnnotationLogCommentProperties", len)?;
        if !self.body.is_empty() {
            struct_ser.serialize_field("body", &self.body)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLogCommentProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "body",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Body,
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
                            "body" => Ok(GeneratedField::Body),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLogCommentProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.AnnotationLogCommentProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationLogCommentProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnnotationLogCommentProperties {
                    body: body__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.AnnotationLogCommentProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLogKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ANNOTATION_LOG_KIND_UNSPECIFIED",
            Self::Comment => "ANNOTATION_LOG_KIND_COMMENT",
            Self::StateUpdate => "ANNOTATION_LOG_KIND_STATE_UPDATE",
            Self::Assigned => "ANNOTATION_LOG_KIND_ASSIGNED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLogKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANNOTATION_LOG_KIND_UNSPECIFIED",
            "ANNOTATION_LOG_KIND_COMMENT",
            "ANNOTATION_LOG_KIND_STATE_UPDATE",
            "ANNOTATION_LOG_KIND_ASSIGNED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLogKind;

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
                    "ANNOTATION_LOG_KIND_UNSPECIFIED" => Ok(AnnotationLogKind::Unspecified),
                    "ANNOTATION_LOG_KIND_COMMENT" => Ok(AnnotationLogKind::Comment),
                    "ANNOTATION_LOG_KIND_STATE_UPDATE" => Ok(AnnotationLogKind::StateUpdate),
                    "ANNOTATION_LOG_KIND_ASSIGNED" => Ok(AnnotationLogKind::Assigned),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLogSearchResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_log_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if !self.annotation_id.is_empty() {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.created_by_user_name.is_empty() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.AnnotationLogSearchResult", len)?;
        if !self.annotation_log_id.is_empty() {
            struct_ser.serialize_field("annotationLogId", &self.annotation_log_id)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        if self.kind != 0 {
            let v = AnnotationLogKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.created_by_user_name.is_empty() {
            struct_ser.serialize_field("createdByUserName", &self.created_by_user_name)?;
        }
        if let Some(v) = self.properties.as_ref() {
            match v {
                annotation_log_search_result::Properties::Assigned(v) => {
                    struct_ser.serialize_field("assigned", v)?;
                }
                annotation_log_search_result::Properties::StateUpdate(v) => {
                    struct_ser.serialize_field("stateUpdate", v)?;
                }
                annotation_log_search_result::Properties::Comment(v) => {
                    struct_ser.serialize_field("comment", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLogSearchResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_log_id",
            "annotationLogId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "annotation_id",
            "annotationId",
            "kind",
            "created_by_user_id",
            "createdByUserId",
            "created_by_user_name",
            "createdByUserName",
            "assigned",
            "state_update",
            "stateUpdate",
            "comment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationLogId,
            CreatedDate,
            ModifiedDate,
            AnnotationId,
            Kind,
            CreatedByUserId,
            CreatedByUserName,
            Assigned,
            StateUpdate,
            Comment,
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
                            "annotationLogId" | "annotation_log_id" => Ok(GeneratedField::AnnotationLogId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "annotationId" | "annotation_id" => Ok(GeneratedField::AnnotationId),
                            "kind" => Ok(GeneratedField::Kind),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "createdByUserName" | "created_by_user_name" => Ok(GeneratedField::CreatedByUserName),
                            "assigned" => Ok(GeneratedField::Assigned),
                            "stateUpdate" | "state_update" => Ok(GeneratedField::StateUpdate),
                            "comment" => Ok(GeneratedField::Comment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLogSearchResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.AnnotationLogSearchResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationLogSearchResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_log_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut annotation_id__ = None;
                let mut kind__ = None;
                let mut created_by_user_id__ = None;
                let mut created_by_user_name__ = None;
                let mut properties__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationLogId => {
                            if annotation_log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLogId"));
                            }
                            annotation_log_id__ = Some(map_.next_value()?);
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
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<AnnotationLogKind>()? as i32);
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedByUserName => {
                            if created_by_user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserName"));
                            }
                            created_by_user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Assigned => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assigned"));
                            }
                            properties__ = map_.next_value::<::std::option::Option<_>>()?.map(annotation_log_search_result::Properties::Assigned)
;
                        }
                        GeneratedField::StateUpdate => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateUpdate"));
                            }
                            properties__ = map_.next_value::<::std::option::Option<_>>()?.map(annotation_log_search_result::Properties::StateUpdate)
;
                        }
                        GeneratedField::Comment => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            properties__ = map_.next_value::<::std::option::Option<_>>()?.map(annotation_log_search_result::Properties::Comment)
;
                        }
                    }
                }
                Ok(AnnotationLogSearchResult {
                    annotation_log_id: annotation_log_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    annotation_id: annotation_id__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    created_by_user_name: created_by_user_name__.unwrap_or_default(),
                    properties: properties__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.AnnotationLogSearchResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLogState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ANNOTATION_LOG_STATE_UNSPECIFIED",
            Self::Created => "ANNOTATION_LOG_STATE_CREATED",
            Self::Open => "ANNOTATION_LOG_STATE_OPEN",
            Self::Flagged => "ANNOTATION_LOG_STATE_FLAGGED",
            Self::Resolved => "ANNOTATION_LOG_STATE_RESOLVED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLogState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANNOTATION_LOG_STATE_UNSPECIFIED",
            "ANNOTATION_LOG_STATE_CREATED",
            "ANNOTATION_LOG_STATE_OPEN",
            "ANNOTATION_LOG_STATE_FLAGGED",
            "ANNOTATION_LOG_STATE_RESOLVED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLogState;

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
                    "ANNOTATION_LOG_STATE_UNSPECIFIED" => Ok(AnnotationLogState::Unspecified),
                    "ANNOTATION_LOG_STATE_CREATED" => Ok(AnnotationLogState::Created),
                    "ANNOTATION_LOG_STATE_OPEN" => Ok(AnnotationLogState::Open),
                    "ANNOTATION_LOG_STATE_FLAGGED" => Ok(AnnotationLogState::Flagged),
                    "ANNOTATION_LOG_STATE_RESOLVED" => Ok(AnnotationLogState::Resolved),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationLogStateUpdateProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.AnnotationLogStateUpdateProperties", len)?;
        if self.state != 0 {
            let v = AnnotationLogState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationLogStateUpdateProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
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
                            "state" => Ok(GeneratedField::State),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationLogStateUpdateProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.AnnotationLogStateUpdateProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationLogStateUpdateProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<AnnotationLogState>()? as i32);
                        }
                    }
                }
                Ok(AnnotationLogStateUpdateProperties {
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.AnnotationLogStateUpdateProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAnnotationLogRequest {
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
        if self.kind != 0 {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.CreateAnnotationLogRequest", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        if self.kind != 0 {
            let v = AnnotationLogKind::try_from(self.kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if let Some(v) = self.properties.as_ref() {
            match v {
                create_annotation_log_request::Properties::Assigned(v) => {
                    struct_ser.serialize_field("assigned", v)?;
                }
                create_annotation_log_request::Properties::StateUpdate(v) => {
                    struct_ser.serialize_field("stateUpdate", v)?;
                }
                create_annotation_log_request::Properties::Comment(v) => {
                    struct_ser.serialize_field("comment", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAnnotationLogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
            "kind",
            "assigned",
            "state_update",
            "stateUpdate",
            "comment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
            Kind,
            Assigned,
            StateUpdate,
            Comment,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "assigned" => Ok(GeneratedField::Assigned),
                            "stateUpdate" | "state_update" => Ok(GeneratedField::StateUpdate),
                            "comment" => Ok(GeneratedField::Comment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAnnotationLogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.CreateAnnotationLogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAnnotationLogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                let mut kind__ = None;
                let mut properties__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value::<AnnotationLogKind>()? as i32);
                        }
                        GeneratedField::Assigned => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assigned"));
                            }
                            properties__ = map_.next_value::<::std::option::Option<_>>()?.map(create_annotation_log_request::Properties::Assigned)
;
                        }
                        GeneratedField::StateUpdate => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateUpdate"));
                            }
                            properties__ = map_.next_value::<::std::option::Option<_>>()?.map(create_annotation_log_request::Properties::StateUpdate)
;
                        }
                        GeneratedField::Comment => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comment"));
                            }
                            properties__ = map_.next_value::<::std::option::Option<_>>()?.map(create_annotation_log_request::Properties::Comment)
;
                        }
                    }
                }
                Ok(CreateAnnotationLogRequest {
                    annotation_id: annotation_id__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    properties: properties__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.CreateAnnotationLogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateAnnotationLogResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.annotation_log.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.CreateAnnotationLogResponse", len)?;
        if let Some(v) = self.annotation_log.as_ref() {
            struct_ser.serialize_field("annotationLog", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateAnnotationLogResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_log",
            "annotationLog",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationLog,
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
                            "annotationLog" | "annotation_log" => Ok(GeneratedField::AnnotationLog),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateAnnotationLogResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.CreateAnnotationLogResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateAnnotationLogResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_log__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationLog => {
                            if annotation_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLog"));
                            }
                            annotation_log__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateAnnotationLogResponse {
                    annotation_log: annotation_log__,
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.CreateAnnotationLogResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAnnotationLogRequest {
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
        if !self.annotation_log_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.DeleteAnnotationLogRequest", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        if !self.annotation_log_id.is_empty() {
            struct_ser.serialize_field("annotationLogId", &self.annotation_log_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAnnotationLogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
            "annotation_log_id",
            "annotationLogId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
            AnnotationLogId,
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
                            "annotationLogId" | "annotation_log_id" => Ok(GeneratedField::AnnotationLogId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAnnotationLogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.DeleteAnnotationLogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAnnotationLogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                let mut annotation_log_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationLogId => {
                            if annotation_log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLogId"));
                            }
                            annotation_log_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteAnnotationLogRequest {
                    annotation_id: annotation_id__.unwrap_or_default(),
                    annotation_log_id: annotation_log_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.DeleteAnnotationLogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAnnotationLogResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.DeleteAnnotationLogResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAnnotationLogResponse {
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
            type Value = DeleteAnnotationLogResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.DeleteAnnotationLogResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAnnotationLogResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAnnotationLogResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.DeleteAnnotationLogResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAnnotationLogsRequest {
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
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.ListAnnotationLogsRequest", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAnnotationLogsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
            PageSize,
            PageToken,
            Filter,
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
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAnnotationLogsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.ListAnnotationLogsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAnnotationLogsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(ListAnnotationLogsRequest {
                    annotation_id: annotation_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.ListAnnotationLogsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAnnotationLogsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_logs.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.annotation_logs.v1.ListAnnotationLogsResponse", len)?;
        if !self.annotation_logs.is_empty() {
            struct_ser.serialize_field("annotationLogs", &self.annotation_logs)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAnnotationLogsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_logs",
            "annotationLogs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationLogs,
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
                            "annotationLogs" | "annotation_logs" => Ok(GeneratedField::AnnotationLogs),
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
            type Value = ListAnnotationLogsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.annotation_logs.v1.ListAnnotationLogsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAnnotationLogsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_logs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationLogs => {
                            if annotation_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationLogs"));
                            }
                            annotation_logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAnnotationLogsResponse {
                    annotation_logs: annotation_logs__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.annotation_logs.v1.ListAnnotationLogsResponse", FIELDS, GeneratedVisitor)
    }
}
