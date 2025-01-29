// @generated
impl serde::Serialize for ActionKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACTION_KIND_UNSPECIFIED",
            Self::Notification => "NOTIFICATION",
            Self::Annotation => "ANNOTATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ActionKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTION_KIND_UNSPECIFIED",
            "NOTIFICATION",
            "ANNOTATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionKind;

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
                    "ACTION_KIND_UNSPECIFIED" => Ok(ActionKind::Unspecified),
                    "NOTIFICATION" => Ok(ActionKind::Notification),
                    "ANNOTATION" => Ok(ActionKind::Annotation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationActionConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_ids.is_empty() {
            len += 1;
        }
        if self.annotation_type != 0 {
            len += 1;
        }
        if self.assigned_to_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.AnnotationActionConfiguration", len)?;
        if !self.tag_ids.is_empty() {
            struct_ser.serialize_field("tagIds", &self.tag_ids)?;
        }
        if self.annotation_type != 0 {
            let v = super::super::annotations::v1::AnnotationType::try_from(self.annotation_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.annotation_type)))?;
            struct_ser.serialize_field("annotationType", &v)?;
        }
        if let Some(v) = self.assigned_to_user_id.as_ref() {
            struct_ser.serialize_field("assignedToUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationActionConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag_ids",
            "tagIds",
            "annotation_type",
            "annotationType",
            "assigned_to_user_id",
            "assignedToUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagIds,
            AnnotationType,
            AssignedToUserId,
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
                            "tagIds" | "tag_ids" => Ok(GeneratedField::TagIds),
                            "annotationType" | "annotation_type" => Ok(GeneratedField::AnnotationType),
                            "assignedToUserId" | "assigned_to_user_id" => Ok(GeneratedField::AssignedToUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationActionConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.AnnotationActionConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationActionConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_ids__ = None;
                let mut annotation_type__ = None;
                let mut assigned_to_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TagIds => {
                            if tag_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagIds"));
                            }
                            tag_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationType => {
                            if annotation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationType"));
                            }
                            annotation_type__ = Some(map_.next_value::<super::super::annotations::v1::AnnotationType>()? as i32);
                        }
                        GeneratedField::AssignedToUserId => {
                            if assigned_to_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assignedToUserId"));
                            }
                            assigned_to_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AnnotationActionConfiguration {
                    tag_ids: tag_ids__.unwrap_or_default(),
                    annotation_type: annotation_type__.unwrap_or_default(),
                    assigned_to_user_id: assigned_to_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.AnnotationActionConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssetExpressionValidationResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.asset_tag_id.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.AssetExpressionValidationResult", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.asset_tag_id.is_empty() {
            struct_ser.serialize_field("assetTagId", &self.asset_tag_id)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetExpressionValidationResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "asset_name",
            "assetName",
            "asset_tag_id",
            "assetTagId",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            AssetName,
            AssetTagId,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "assetTagId" | "asset_tag_id" => Ok(GeneratedField::AssetTagId),
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
            type Value = AssetExpressionValidationResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.AssetExpressionValidationResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetExpressionValidationResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut asset_name__ = None;
                let mut asset_tag_id__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetTagId => {
                            if asset_tag_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetTagId"));
                            }
                            asset_tag_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AssetExpressionValidationResult {
                    asset_id: asset_id__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                    asset_tag_id: asset_tag_id__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.AssetExpressionValidationResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchDeleteRulesRequest {
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
        if !self.client_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchDeleteRulesRequest", len)?;
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
        }
        if !self.client_keys.is_empty() {
            struct_ser.serialize_field("clientKeys", &self.client_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_ids",
            "ruleIds",
            "client_keys",
            "clientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleIds,
            ClientKeys,
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
                            "clientKeys" | "client_keys" => Ok(GeneratedField::ClientKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchDeleteRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchDeleteRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_ids__ = None;
                let mut client_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleIds => {
                            if rule_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKeys => {
                            if client_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKeys"));
                            }
                            client_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchDeleteRulesRequest {
                    rule_ids: rule_ids__.unwrap_or_default(),
                    client_keys: client_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchDeleteRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchDeleteRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.rules.v1.BatchDeleteRulesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteRulesResponse {
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
            type Value = BatchDeleteRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchDeleteRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchDeleteRulesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchDeleteRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchGetRuleVersionsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchGetRuleVersionsRequest", len)?;
        if !self.rule_version_ids.is_empty() {
            struct_ser.serialize_field("ruleVersionIds", &self.rule_version_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchGetRuleVersionsRequest {
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
            type Value = BatchGetRuleVersionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchGetRuleVersionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchGetRuleVersionsRequest, V::Error>
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
                Ok(BatchGetRuleVersionsRequest {
                    rule_version_ids: rule_version_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchGetRuleVersionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchGetRuleVersionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchGetRuleVersionsResponse", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchGetRuleVersionsResponse {
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
            type Value = BatchGetRuleVersionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchGetRuleVersionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchGetRuleVersionsResponse, V::Error>
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
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchGetRuleVersionsResponse {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchGetRuleVersionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchGetRulesRequest {
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
        if !self.client_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchGetRulesRequest", len)?;
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
        }
        if !self.client_keys.is_empty() {
            struct_ser.serialize_field("clientKeys", &self.client_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchGetRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_ids",
            "ruleIds",
            "client_keys",
            "clientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleIds,
            ClientKeys,
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
                            "clientKeys" | "client_keys" => Ok(GeneratedField::ClientKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchGetRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchGetRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchGetRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_ids__ = None;
                let mut client_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleIds => {
                            if rule_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKeys => {
                            if client_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKeys"));
                            }
                            client_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchGetRulesRequest {
                    rule_ids: rule_ids__.unwrap_or_default(),
                    client_keys: client_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchGetRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchGetRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchGetRulesResponse", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchGetRulesResponse {
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
            type Value = BatchGetRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchGetRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchGetRulesResponse, V::Error>
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
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchGetRulesResponse {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchGetRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUndeleteRulesRequest {
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
        if !self.client_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchUndeleteRulesRequest", len)?;
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
        }
        if !self.client_keys.is_empty() {
            struct_ser.serialize_field("clientKeys", &self.client_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUndeleteRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_ids",
            "ruleIds",
            "client_keys",
            "clientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleIds,
            ClientKeys,
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
                            "clientKeys" | "client_keys" => Ok(GeneratedField::ClientKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUndeleteRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchUndeleteRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUndeleteRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_ids__ = None;
                let mut client_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleIds => {
                            if rule_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKeys => {
                            if client_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKeys"));
                            }
                            client_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUndeleteRulesRequest {
                    rule_ids: rule_ids__.unwrap_or_default(),
                    client_keys: client_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchUndeleteRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUndeleteRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.rules.v1.BatchUndeleteRulesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUndeleteRulesResponse {
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
            type Value = BatchUndeleteRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchUndeleteRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUndeleteRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchUndeleteRulesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchUndeleteRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUpdateRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        if self.validate_only {
            len += 1;
        }
        if self.override_expression_validation {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchUpdateRulesRequest", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if self.validate_only {
            struct_ser.serialize_field("validateOnly", &self.validate_only)?;
        }
        if self.override_expression_validation {
            struct_ser.serialize_field("overrideExpressionValidation", &self.override_expression_validation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUpdateRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
            "validate_only",
            "validateOnly",
            "override_expression_validation",
            "overrideExpressionValidation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
            ValidateOnly,
            OverrideExpressionValidation,
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
                            "validateOnly" | "validate_only" => Ok(GeneratedField::ValidateOnly),
                            "overrideExpressionValidation" | "override_expression_validation" => Ok(GeneratedField::OverrideExpressionValidation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUpdateRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchUpdateRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUpdateRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                let mut validate_only__ = None;
                let mut override_expression_validation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidateOnly => {
                            if validate_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateOnly"));
                            }
                            validate_only__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverrideExpressionValidation => {
                            if override_expression_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideExpressionValidation"));
                            }
                            override_expression_validation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUpdateRulesRequest {
                    rules: rules__.unwrap_or_default(),
                    validate_only: validate_only__.unwrap_or_default(),
                    override_expression_validation: override_expression_validation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchUpdateRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUpdateRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if self.rules_created_count != 0 {
            len += 1;
        }
        if self.rules_updated_count != 0 {
            len += 1;
        }
        if self.validate_only {
            len += 1;
        }
        if !self.validation_results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchUpdateRulesResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if self.rules_created_count != 0 {
            struct_ser.serialize_field("rulesCreatedCount", &self.rules_created_count)?;
        }
        if self.rules_updated_count != 0 {
            struct_ser.serialize_field("rulesUpdatedCount", &self.rules_updated_count)?;
        }
        if self.validate_only {
            struct_ser.serialize_field("validateOnly", &self.validate_only)?;
        }
        if !self.validation_results.is_empty() {
            struct_ser.serialize_field("validationResults", &self.validation_results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUpdateRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "rules_created_count",
            "rulesCreatedCount",
            "rules_updated_count",
            "rulesUpdatedCount",
            "validate_only",
            "validateOnly",
            "validation_results",
            "validationResults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            RulesCreatedCount,
            RulesUpdatedCount,
            ValidateOnly,
            ValidationResults,
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
                            "success" => Ok(GeneratedField::Success),
                            "rulesCreatedCount" | "rules_created_count" => Ok(GeneratedField::RulesCreatedCount),
                            "rulesUpdatedCount" | "rules_updated_count" => Ok(GeneratedField::RulesUpdatedCount),
                            "validateOnly" | "validate_only" => Ok(GeneratedField::ValidateOnly),
                            "validationResults" | "validation_results" => Ok(GeneratedField::ValidationResults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUpdateRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchUpdateRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUpdateRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut rules_created_count__ = None;
                let mut rules_updated_count__ = None;
                let mut validate_only__ = None;
                let mut validation_results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RulesCreatedCount => {
                            if rules_created_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesCreatedCount"));
                            }
                            rules_created_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RulesUpdatedCount => {
                            if rules_updated_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesUpdatedCount"));
                            }
                            rules_updated_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ValidateOnly => {
                            if validate_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateOnly"));
                            }
                            validate_only__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidationResults => {
                            if validation_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationResults"));
                            }
                            validation_results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUpdateRulesResponse {
                    success: success__.unwrap_or_default(),
                    rules_created_count: rules_created_count__.unwrap_or_default(),
                    rules_updated_count: rules_updated_count__.unwrap_or_default(),
                    validate_only: validate_only__.unwrap_or_default(),
                    validation_results: validation_results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.BatchUpdateRulesResponse", FIELDS, GeneratedVisitor)
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
        if !self.channel_references.is_empty() {
            len += 1;
        }
        if !self.expression.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.CalculatedChannelConfig", len)?;
        if !self.channel_references.is_empty() {
            struct_ser.serialize_field("channelReferences", &self.channel_references)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
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
            "channel_references",
            "channelReferences",
            "expression",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelReferences,
            Expression,
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
                            "channelReferences" | "channel_references" => Ok(GeneratedField::ChannelReferences),
                            "expression" => Ok(GeneratedField::Expression),
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
                formatter.write_str("struct sift.rules.v1.CalculatedChannelConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_references__ = None;
                let mut expression__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelReferences => {
                            if channel_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelReferences"));
                            }
                            channel_references__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CalculatedChannelConfig {
                    channel_references: channel_references__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.CalculatedChannelConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelReference {
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
        if !self.component.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ChannelReference", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.component.is_empty() {
            struct_ser.serialize_field("component", &self.component)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "component",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Component,
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
                            "component" => Ok(GeneratedField::Component),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ChannelReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut component__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Component => {
                            if component__.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            component__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ChannelReference {
                    name: name__.unwrap_or_default(),
                    component: component__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ChannelReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConditionComparator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CONDITION_COMPARATOR_UNSPECIFIED",
            Self::LessThan => "LESS_THAN",
            Self::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
            Self::GreaterThan => "GREATER_THAN",
            Self::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
            Self::Equal => "EQUAL",
            Self::NotEqual => "NOT_EQUAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConditionComparator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONDITION_COMPARATOR_UNSPECIFIED",
            "LESS_THAN",
            "LESS_THAN_OR_EQUAL",
            "GREATER_THAN",
            "GREATER_THAN_OR_EQUAL",
            "EQUAL",
            "NOT_EQUAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConditionComparator;

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
                    "CONDITION_COMPARATOR_UNSPECIFIED" => Ok(ConditionComparator::Unspecified),
                    "LESS_THAN" => Ok(ConditionComparator::LessThan),
                    "LESS_THAN_OR_EQUAL" => Ok(ConditionComparator::LessThanOrEqual),
                    "GREATER_THAN" => Ok(ConditionComparator::GreaterThan),
                    "GREATER_THAN_OR_EQUAL" => Ok(ConditionComparator::GreaterThanOrEqual),
                    "EQUAL" => Ok(ConditionComparator::Equal),
                    "NOT_EQUAL" => Ok(ConditionComparator::NotEqual),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ContextualChannels {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ContextualChannels", len)?;
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContextualChannels {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channels,
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
                            "channels" => Ok(GeneratedField::Channels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContextualChannels;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ContextualChannels")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContextualChannels, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ContextualChannels {
                    channels: channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ContextualChannels", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRuleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.CreateRuleRequest", len)?;
        if let Some(v) = self.update.as_ref() {
            struct_ser.serialize_field("update", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "update",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Update,
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
                            "update" => Ok(GeneratedField::Update),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.CreateRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut update__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Update => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            update__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateRuleRequest {
                    update: update__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.CreateRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRuleResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.CreateRuleResponse", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.CreateRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateRuleResponse {
                    rule_id: rule_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.CreateRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRuleRequest {
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
        if !self.client_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.DeleteRuleRequest", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
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
            type Value = DeleteRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.DeleteRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut client_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteRuleRequest {
                    rule_id: rule_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.DeleteRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRuleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.rules.v1.DeleteRuleResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRuleResponse {
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
            type Value = DeleteRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.DeleteRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteRuleResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.DeleteRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DryRunAnnotation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.condition_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if !self.condition_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.DryRunAnnotation", len)?;
        if !self.condition_id.is_empty() {
            struct_ser.serialize_field("conditionId", &self.condition_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if !self.condition_version_id.is_empty() {
            struct_ser.serialize_field("conditionVersionId", &self.condition_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DryRunAnnotation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "condition_id",
            "conditionId",
            "name",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "condition_version_id",
            "conditionVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConditionId,
            Name,
            StartTime,
            EndTime,
            ConditionVersionId,
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
                            "conditionId" | "condition_id" => Ok(GeneratedField::ConditionId),
                            "name" => Ok(GeneratedField::Name),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "conditionVersionId" | "condition_version_id" => Ok(GeneratedField::ConditionVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DryRunAnnotation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.DryRunAnnotation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DryRunAnnotation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut condition_id__ = None;
                let mut name__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut condition_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConditionId => {
                            if condition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionId"));
                            }
                            condition_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
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
                        GeneratedField::ConditionVersionId => {
                            if condition_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditionVersionId"));
                            }
                            condition_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DryRunAnnotation {
                    condition_id: condition_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    condition_version_id: condition_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.DryRunAnnotation", FIELDS, GeneratedVisitor)
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
        if !self.rule_ids.is_empty() {
            len += 1;
        }
        if self.annotation_options.is_some() {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.EvaluateRulesRequest", len)?;
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
        }
        if let Some(v) = self.annotation_options.as_ref() {
            struct_ser.serialize_field("annotationOptions", v)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        if let Some(v) = self.time.as_ref() {
            match v {
                evaluate_rules_request::Time::RunId(v) => {
                    struct_ser.serialize_field("runId", v)?;
                }
                evaluate_rules_request::Time::TimeRange(v) => {
                    struct_ser.serialize_field("timeRange", v)?;
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
            "rule_ids",
            "ruleIds",
            "annotation_options",
            "annotationOptions",
            "dry_run",
            "dryRun",
            "run_id",
            "runId",
            "time_range",
            "timeRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleIds,
            AnnotationOptions,
            DryRun,
            RunId,
            TimeRange,
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
                            "annotationOptions" | "annotation_options" => Ok(GeneratedField::AnnotationOptions),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "timeRange" | "time_range" => Ok(GeneratedField::TimeRange),
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
                formatter.write_str("struct sift.rules.v1.EvaluateRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_ids__ = None;
                let mut annotation_options__ = None;
                let mut dry_run__ = None;
                let mut time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleIds => {
                            if rule_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationOptions => {
                            if annotation_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationOptions"));
                            }
                            annotation_options__ = map_.next_value()?;
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            time__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Time::RunId);
                        }
                        GeneratedField::TimeRange => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeRange"));
                            }
                            time__ = map_.next_value::<::std::option::Option<_>>()?.map(evaluate_rules_request::Time::TimeRange)
;
                        }
                    }
                }
                Ok(EvaluateRulesRequest {
                    rule_ids: rule_ids__.unwrap_or_default(),
                    annotation_options: annotation_options__,
                    dry_run: dry_run__.unwrap_or_default(),
                    time: time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.EvaluateRulesRequest", FIELDS, GeneratedVisitor)
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
        if !self.dry_run_annotations.is_empty() {
            len += 1;
        }
        if self.job_id.is_some() {
            len += 1;
        }
        if self.report_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.EvaluateRulesResponse", len)?;
        if self.created_annotation_count != 0 {
            struct_ser.serialize_field("createdAnnotationCount", &self.created_annotation_count)?;
        }
        if !self.dry_run_annotations.is_empty() {
            struct_ser.serialize_field("dryRunAnnotations", &self.dry_run_annotations)?;
        }
        if let Some(v) = self.job_id.as_ref() {
            struct_ser.serialize_field("jobId", v)?;
        }
        if let Some(v) = self.report_id.as_ref() {
            struct_ser.serialize_field("reportId", v)?;
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
            "dry_run_annotations",
            "dryRunAnnotations",
            "job_id",
            "jobId",
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedAnnotationCount,
            DryRunAnnotations,
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
                            "createdAnnotationCount" | "created_annotation_count" => Ok(GeneratedField::CreatedAnnotationCount),
                            "dryRunAnnotations" | "dry_run_annotations" => Ok(GeneratedField::DryRunAnnotations),
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
            type Value = EvaluateRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.EvaluateRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluateRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut created_annotation_count__ = None;
                let mut dry_run_annotations__ = None;
                let mut job_id__ = None;
                let mut report_id__ = None;
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
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = map_.next_value()?;
                        }
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EvaluateRulesResponse {
                    created_annotation_count: created_annotation_count__.unwrap_or_default(),
                    dry_run_annotations: dry_run_annotations__.unwrap_or_default(),
                    job_id: job_id__,
                    report_id: report_id__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.EvaluateRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EvaluatedAnnotationOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.EvaluatedAnnotationOptions", len)?;
        if !self.tag_ids.is_empty() {
            struct_ser.serialize_field("tagIds", &self.tag_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EvaluatedAnnotationOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag_ids",
            "tagIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagIds,
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
                            "tagIds" | "tag_ids" => Ok(GeneratedField::TagIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EvaluatedAnnotationOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.EvaluatedAnnotationOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EvaluatedAnnotationOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TagIds => {
                            if tag_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagIds"));
                            }
                            tag_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EvaluatedAnnotationOptions {
                    tag_ids: tag_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.EvaluatedAnnotationOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRuleRequest {
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
        if !self.client_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.GetRuleRequest", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
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
            type Value = GetRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.GetRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut client_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetRuleRequest {
                    rule_id: rule_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.GetRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRuleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.GetRuleResponse", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.GetRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetRuleResponse {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.GetRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRuleVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.GetRuleVersionRequest", len)?;
        if !self.rule_version_id.is_empty() {
            struct_ser.serialize_field("ruleVersionId", &self.rule_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRuleVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_version_id",
            "ruleVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleVersionId,
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
                            "ruleVersionId" | "rule_version_id" => Ok(GeneratedField::RuleVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRuleVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.GetRuleVersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRuleVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleVersionId => {
                            if rule_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersionId"));
                            }
                            rule_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetRuleVersionRequest {
                    rule_version_id: rule_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.GetRuleVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRuleVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.GetRuleVersionResponse", len)?;
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRuleVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rule,
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
                            "rule" => Ok(GeneratedField::Rule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRuleVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.GetRuleVersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRuleVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetRuleVersionResponse {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.GetRuleVersionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JsonRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.rules_json.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.JsonRulesRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.rules_json.is_empty() {
            struct_ser.serialize_field("rulesJson", &self.rules_json)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JsonRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "rules_json",
            "rulesJson",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            RulesJson,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "rulesJson" | "rules_json" => Ok(GeneratedField::RulesJson),
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
            type Value = JsonRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.JsonRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JsonRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut rules_json__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RulesJson => {
                            if rules_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesJson"));
                            }
                            rules_json__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(JsonRulesRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                    rules_json: rules_json__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.JsonRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JsonRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if self.total_rules_count != 0 {
            len += 1;
        }
        if self.rules_created_count != 0 {
            len += 1;
        }
        if self.rules_updated_count != 0 {
            len += 1;
        }
        if self.rules_deleted_count != 0 {
            len += 1;
        }
        if self.error_messages.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.JsonRulesResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if self.total_rules_count != 0 {
            struct_ser.serialize_field("totalRulesCount", &self.total_rules_count)?;
        }
        if self.rules_created_count != 0 {
            struct_ser.serialize_field("rulesCreatedCount", &self.rules_created_count)?;
        }
        if self.rules_updated_count != 0 {
            struct_ser.serialize_field("rulesUpdatedCount", &self.rules_updated_count)?;
        }
        if self.rules_deleted_count != 0 {
            struct_ser.serialize_field("rulesDeletedCount", &self.rules_deleted_count)?;
        }
        if let Some(v) = self.error_messages.as_ref() {
            struct_ser.serialize_field("errorMessages", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JsonRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "total_rules_count",
            "totalRulesCount",
            "rules_created_count",
            "rulesCreatedCount",
            "rules_updated_count",
            "rulesUpdatedCount",
            "rules_deleted_count",
            "rulesDeletedCount",
            "error_messages",
            "errorMessages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            TotalRulesCount,
            RulesCreatedCount,
            RulesUpdatedCount,
            RulesDeletedCount,
            ErrorMessages,
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
                            "success" => Ok(GeneratedField::Success),
                            "totalRulesCount" | "total_rules_count" => Ok(GeneratedField::TotalRulesCount),
                            "rulesCreatedCount" | "rules_created_count" => Ok(GeneratedField::RulesCreatedCount),
                            "rulesUpdatedCount" | "rules_updated_count" => Ok(GeneratedField::RulesUpdatedCount),
                            "rulesDeletedCount" | "rules_deleted_count" => Ok(GeneratedField::RulesDeletedCount),
                            "errorMessages" | "error_messages" => Ok(GeneratedField::ErrorMessages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JsonRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.JsonRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JsonRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut total_rules_count__ = None;
                let mut rules_created_count__ = None;
                let mut rules_updated_count__ = None;
                let mut rules_deleted_count__ = None;
                let mut error_messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalRulesCount => {
                            if total_rules_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRulesCount"));
                            }
                            total_rules_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RulesCreatedCount => {
                            if rules_created_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesCreatedCount"));
                            }
                            rules_created_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RulesUpdatedCount => {
                            if rules_updated_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesUpdatedCount"));
                            }
                            rules_updated_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RulesDeletedCount => {
                            if rules_deleted_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesDeletedCount"));
                            }
                            rules_deleted_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ErrorMessages => {
                            if error_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessages"));
                            }
                            error_messages__ = map_.next_value()?;
                        }
                    }
                }
                Ok(JsonRulesResponse {
                    success: success__.unwrap_or_default(),
                    total_rules_count: total_rules_count__.unwrap_or_default(),
                    rules_created_count: rules_created_count__.unwrap_or_default(),
                    rules_updated_count: rules_updated_count__.unwrap_or_default(),
                    rules_deleted_count: rules_deleted_count__.unwrap_or_default(),
                    error_messages: error_messages__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.JsonRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LastValueThreshold {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.rules.v1.LastValueThreshold", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LastValueThreshold {
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
            type Value = LastValueThreshold;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.LastValueThreshold")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LastValueThreshold, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(LastValueThreshold {
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.LastValueThreshold", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRuleVersionsRequest {
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
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ListRuleVersionsRequest", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
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
impl<'de> serde::Deserialize<'de> for ListRuleVersionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
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
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
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
            type Value = ListRuleVersionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ListRuleVersionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRuleVersionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
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
                Ok(ListRuleVersionsRequest {
                    rule_id: rule_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ListRuleVersionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRuleVersionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_versions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ListRuleVersionsResponse", len)?;
        if !self.rule_versions.is_empty() {
            struct_ser.serialize_field("ruleVersions", &self.rule_versions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRuleVersionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_versions",
            "ruleVersions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleVersions,
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
                            "ruleVersions" | "rule_versions" => Ok(GeneratedField::RuleVersions),
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
            type Value = ListRuleVersionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ListRuleVersionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRuleVersionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_versions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleVersions => {
                            if rule_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersions"));
                            }
                            rule_versions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListRuleVersionsResponse {
                    rule_versions: rule_versions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ListRuleVersionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotificationActionConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.recipient_user_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.NotificationActionConfiguration", len)?;
        if !self.recipient_user_ids.is_empty() {
            struct_ser.serialize_field("recipientUserIds", &self.recipient_user_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotificationActionConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recipient_user_ids",
            "recipientUserIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RecipientUserIds,
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
                            "recipientUserIds" | "recipient_user_ids" => Ok(GeneratedField::RecipientUserIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotificationActionConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.NotificationActionConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NotificationActionConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recipient_user_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RecipientUserIds => {
                            if recipient_user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipientUserIds"));
                            }
                            recipient_user_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NotificationActionConfiguration {
                    recipient_user_ids: recipient_user_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.NotificationActionConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rule {
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
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.is_enabled {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.conditions.is_empty() {
            len += 1;
        }
        if self.rule_version.is_some() {
            len += 1;
        }
        if !self.client_key.is_empty() {
            len += 1;
        }
        if self.asset_configuration.is_some() {
            len += 1;
        }
        if self.contextual_channels.is_some() {
            len += 1;
        }
        if self.deleted_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.Rule", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.is_enabled {
            struct_ser.serialize_field("isEnabled", &self.is_enabled)?;
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
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        if let Some(v) = self.rule_version.as_ref() {
            struct_ser.serialize_field("ruleVersion", v)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        if let Some(v) = self.asset_configuration.as_ref() {
            struct_ser.serialize_field("assetConfiguration", v)?;
        }
        if let Some(v) = self.contextual_channels.as_ref() {
            struct_ser.serialize_field("contextualChannels", v)?;
        }
        if let Some(v) = self.deleted_date.as_ref() {
            struct_ser.serialize_field("deletedDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "asset_id",
            "assetId",
            "name",
            "description",
            "is_enabled",
            "isEnabled",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "organization_id",
            "organizationId",
            "conditions",
            "rule_version",
            "ruleVersion",
            "client_key",
            "clientKey",
            "asset_configuration",
            "assetConfiguration",
            "contextual_channels",
            "contextualChannels",
            "deleted_date",
            "deletedDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            AssetId,
            Name,
            Description,
            IsEnabled,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
            OrganizationId,
            Conditions,
            RuleVersion,
            ClientKey,
            AssetConfiguration,
            ContextualChannels,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "isEnabled" | "is_enabled" => Ok(GeneratedField::IsEnabled),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "conditions" => Ok(GeneratedField::Conditions),
                            "ruleVersion" | "rule_version" => Ok(GeneratedField::RuleVersion),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "assetConfiguration" | "asset_configuration" => Ok(GeneratedField::AssetConfiguration),
                            "contextualChannels" | "contextual_channels" => Ok(GeneratedField::ContextualChannels),
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
            type Value = Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.Rule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Rule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut asset_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut is_enabled__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut organization_id__ = None;
                let mut conditions__ = None;
                let mut rule_version__ = None;
                let mut client_key__ = None;
                let mut asset_configuration__ = None;
                let mut contextual_channels__ = None;
                let mut deleted_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
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
                        GeneratedField::IsEnabled => {
                            if is_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEnabled"));
                            }
                            is_enabled__ = Some(map_.next_value()?);
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleVersion => {
                            if rule_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleVersion"));
                            }
                            rule_version__ = map_.next_value()?;
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetConfiguration => {
                            if asset_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetConfiguration"));
                            }
                            asset_configuration__ = map_.next_value()?;
                        }
                        GeneratedField::ContextualChannels => {
                            if contextual_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextualChannels"));
                            }
                            contextual_channels__ = map_.next_value()?;
                        }
                        GeneratedField::DeletedDate => {
                            if deleted_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedDate"));
                            }
                            deleted_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Rule {
                    rule_id: rule_id__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    is_enabled: is_enabled__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    conditions: conditions__.unwrap_or_default(),
                    rule_version: rule_version__,
                    client_key: client_key__.unwrap_or_default(),
                    asset_configuration: asset_configuration__,
                    contextual_channels: contextual_channels__,
                    deleted_date: deleted_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.Rule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_action_id.is_empty() {
            len += 1;
        }
        if !self.rule_condition_id.is_empty() {
            len += 1;
        }
        if self.action_type != 0 {
            len += 1;
        }
        if self.configuration.is_some() {
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
        if !self.rule_action_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.RuleAction", len)?;
        if !self.rule_action_id.is_empty() {
            struct_ser.serialize_field("ruleActionId", &self.rule_action_id)?;
        }
        if !self.rule_condition_id.is_empty() {
            struct_ser.serialize_field("ruleConditionId", &self.rule_condition_id)?;
        }
        if self.action_type != 0 {
            let v = ActionKind::try_from(self.action_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action_type)))?;
            struct_ser.serialize_field("actionType", &v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
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
        if !self.rule_action_version_id.is_empty() {
            struct_ser.serialize_field("ruleActionVersionId", &self.rule_action_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_action_id",
            "ruleActionId",
            "rule_condition_id",
            "ruleConditionId",
            "action_type",
            "actionType",
            "configuration",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "rule_action_version_id",
            "ruleActionVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleActionId,
            RuleConditionId,
            ActionType,
            Configuration,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
            RuleActionVersionId,
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
                            "ruleActionId" | "rule_action_id" => Ok(GeneratedField::RuleActionId),
                            "ruleConditionId" | "rule_condition_id" => Ok(GeneratedField::RuleConditionId),
                            "actionType" | "action_type" => Ok(GeneratedField::ActionType),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "ruleActionVersionId" | "rule_action_version_id" => Ok(GeneratedField::RuleActionVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.RuleAction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_action_id__ = None;
                let mut rule_condition_id__ = None;
                let mut action_type__ = None;
                let mut configuration__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut rule_action_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleActionId => {
                            if rule_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleActionId"));
                            }
                            rule_action_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleConditionId => {
                            if rule_condition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleConditionId"));
                            }
                            rule_condition_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ActionType => {
                            if action_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionType"));
                            }
                            action_type__ = Some(map_.next_value::<ActionKind>()? as i32);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
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
                        GeneratedField::RuleActionVersionId => {
                            if rule_action_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleActionVersionId"));
                            }
                            rule_action_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RuleAction {
                    rule_action_id: rule_action_id__.unwrap_or_default(),
                    rule_condition_id: rule_condition_id__.unwrap_or_default(),
                    action_type: action_type__.unwrap_or_default(),
                    configuration: configuration__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    rule_action_version_id: rule_action_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.RuleAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleActionConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.RuleActionConfiguration", len)?;
        if let Some(v) = self.configuration.as_ref() {
            match v {
                rule_action_configuration::Configuration::Notification(v) => {
                    struct_ser.serialize_field("notification", v)?;
                }
                rule_action_configuration::Configuration::Annotation(v) => {
                    struct_ser.serialize_field("annotation", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleActionConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "notification",
            "annotation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Notification,
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
                            "notification" => Ok(GeneratedField::Notification),
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
            type Value = RuleActionConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.RuleActionConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleActionConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Notification => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notification"));
                            }
                            configuration__ = map_.next_value::<::std::option::Option<_>>()?.map(rule_action_configuration::Configuration::Notification)
;
                        }
                        GeneratedField::Annotation => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            configuration__ = map_.next_value::<::std::option::Option<_>>()?.map(rule_action_configuration::Configuration::Annotation)
;
                        }
                    }
                }
                Ok(RuleActionConfiguration {
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.RuleActionConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleAssetConfiguration {
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
        if !self.tag_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.RuleAssetConfiguration", len)?;
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if !self.tag_ids.is_empty() {
            struct_ser.serialize_field("tagIds", &self.tag_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleAssetConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_ids",
            "assetIds",
            "tag_ids",
            "tagIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetIds,
            TagIds,
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
                            "tagIds" | "tag_ids" => Ok(GeneratedField::TagIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleAssetConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.RuleAssetConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleAssetConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_ids__ = None;
                let mut tag_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TagIds => {
                            if tag_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagIds"));
                            }
                            tag_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RuleAssetConfiguration {
                    asset_ids: asset_ids__.unwrap_or_default(),
                    tag_ids: tag_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.RuleAssetConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleCondition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_condition_id.is_empty() {
            len += 1;
        }
        if !self.rule_id.is_empty() {
            len += 1;
        }
        if self.expression.is_some() {
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
        if !self.actions.is_empty() {
            len += 1;
        }
        if !self.rule_condition_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.RuleCondition", len)?;
        if !self.rule_condition_id.is_empty() {
            struct_ser.serialize_field("ruleConditionId", &self.rule_condition_id)?;
        }
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if let Some(v) = self.expression.as_ref() {
            struct_ser.serialize_field("expression", v)?;
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
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if !self.rule_condition_version_id.is_empty() {
            struct_ser.serialize_field("ruleConditionVersionId", &self.rule_condition_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleCondition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_condition_id",
            "ruleConditionId",
            "rule_id",
            "ruleId",
            "expression",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "actions",
            "rule_condition_version_id",
            "ruleConditionVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleConditionId,
            RuleId,
            Expression,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
            Actions,
            RuleConditionVersionId,
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
                            "ruleConditionId" | "rule_condition_id" => Ok(GeneratedField::RuleConditionId),
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "expression" => Ok(GeneratedField::Expression),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "actions" => Ok(GeneratedField::Actions),
                            "ruleConditionVersionId" | "rule_condition_version_id" => Ok(GeneratedField::RuleConditionVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleCondition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.RuleCondition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleCondition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_condition_id__ = None;
                let mut rule_id__ = None;
                let mut expression__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut actions__ = None;
                let mut rule_condition_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleConditionId => {
                            if rule_condition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleConditionId"));
                            }
                            rule_condition_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = map_.next_value()?;
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
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RuleConditionVersionId => {
                            if rule_condition_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleConditionVersionId"));
                            }
                            rule_condition_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RuleCondition {
                    rule_condition_id: rule_condition_id__.unwrap_or_default(),
                    rule_id: rule_id__.unwrap_or_default(),
                    expression: expression__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    rule_condition_version_id: rule_condition_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.RuleCondition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleConditionExpression {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expression.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.RuleConditionExpression", len)?;
        if let Some(v) = self.expression.as_ref() {
            match v {
                rule_condition_expression::Expression::SingleChannelComparison(v) => {
                    struct_ser.serialize_field("singleChannelComparison", v)?;
                }
                rule_condition_expression::Expression::CalculatedChannel(v) => {
                    struct_ser.serialize_field("calculatedChannel", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleConditionExpression {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "single_channel_comparison",
            "singleChannelComparison",
            "calculated_channel",
            "calculatedChannel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SingleChannelComparison,
            CalculatedChannel,
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
                            "singleChannelComparison" | "single_channel_comparison" => Ok(GeneratedField::SingleChannelComparison),
                            "calculatedChannel" | "calculated_channel" => Ok(GeneratedField::CalculatedChannel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleConditionExpression;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.RuleConditionExpression")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleConditionExpression, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expression__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SingleChannelComparison => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("singleChannelComparison"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(rule_condition_expression::Expression::SingleChannelComparison)
;
                        }
                        GeneratedField::CalculatedChannel => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannel"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(rule_condition_expression::Expression::CalculatedChannel)
;
                        }
                    }
                }
                Ok(RuleConditionExpression {
                    expression: expression__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.RuleConditionExpression", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleVersion {
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
        if !self.version.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.version_notes.is_empty() {
            len += 1;
        }
        if !self.generated_change_message.is_empty() {
            len += 1;
        }
        if self.deleted_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.RuleVersion", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.rule_version_id.is_empty() {
            struct_ser.serialize_field("ruleVersionId", &self.rule_version_id)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.version_notes.is_empty() {
            struct_ser.serialize_field("versionNotes", &self.version_notes)?;
        }
        if !self.generated_change_message.is_empty() {
            struct_ser.serialize_field("generatedChangeMessage", &self.generated_change_message)?;
        }
        if let Some(v) = self.deleted_date.as_ref() {
            struct_ser.serialize_field("deletedDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleVersion {
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
            "version",
            "created_date",
            "createdDate",
            "created_by_user_id",
            "createdByUserId",
            "version_notes",
            "versionNotes",
            "generated_change_message",
            "generatedChangeMessage",
            "deleted_date",
            "deletedDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            RuleVersionId,
            Version,
            CreatedDate,
            CreatedByUserId,
            VersionNotes,
            GeneratedChangeMessage,
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
                            "ruleVersionId" | "rule_version_id" => Ok(GeneratedField::RuleVersionId),
                            "version" => Ok(GeneratedField::Version),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "versionNotes" | "version_notes" => Ok(GeneratedField::VersionNotes),
                            "generatedChangeMessage" | "generated_change_message" => Ok(GeneratedField::GeneratedChangeMessage),
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
            type Value = RuleVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.RuleVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut rule_version_id__ = None;
                let mut version__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                let mut version_notes__ = None;
                let mut generated_change_message__ = None;
                let mut deleted_date__ = None;
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
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedDate => {
                            if created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdDate"));
                            }
                            created_date__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VersionNotes => {
                            if version_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNotes"));
                            }
                            version_notes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GeneratedChangeMessage => {
                            if generated_change_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generatedChangeMessage"));
                            }
                            generated_change_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeletedDate => {
                            if deleted_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedDate"));
                            }
                            deleted_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RuleVersion {
                    rule_id: rule_id__.unwrap_or_default(),
                    rule_version_id: rule_version_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    version_notes: version_notes__.unwrap_or_default(),
                    generated_change_message: generated_change_message__.unwrap_or_default(),
                    deleted_date: deleted_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.RuleVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchOrder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SEARCH_ORDER_UNSPECIFIED",
            Self::Asc => "SEARCH_ORDER_ASC",
            Self::Desc => "SEARCH_ORDER_DESC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SearchOrder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SEARCH_ORDER_UNSPECIFIED",
            "SEARCH_ORDER_ASC",
            "SEARCH_ORDER_DESC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchOrder;

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
                    "SEARCH_ORDER_UNSPECIFIED" => Ok(SearchOrder::Unspecified),
                    "SEARCH_ORDER_ASC" => Ok(SearchOrder::Asc),
                    "SEARCH_ORDER_DESC" => Ok(SearchOrder::Desc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SearchRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit.is_some() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if self.order.is_some() {
            len += 1;
        }
        if !self.name_matches.is_empty() {
            len += 1;
        }
        if self.case_sensitive {
            len += 1;
        }
        if self.regexp {
            len += 1;
        }
        if self.order_by.is_some() {
            len += 1;
        }
        if !self.rule_ids.is_empty() {
            len += 1;
        }
        if !self.asset_ids.is_empty() {
            len += 1;
        }
        if self.include_deleted {
            len += 1;
        }
        if self.asset_tags.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.SearchRulesRequest", len)?;
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        if let Some(v) = self.order.as_ref() {
            let v = SearchOrder::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("order", &v)?;
        }
        if !self.name_matches.is_empty() {
            struct_ser.serialize_field("nameMatches", &self.name_matches)?;
        }
        if self.case_sensitive {
            struct_ser.serialize_field("caseSensitive", &self.case_sensitive)?;
        }
        if self.regexp {
            struct_ser.serialize_field("regexp", &self.regexp)?;
        }
        if let Some(v) = self.order_by.as_ref() {
            struct_ser.serialize_field("orderBy", v)?;
        }
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
        }
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if self.include_deleted {
            struct_ser.serialize_field("includeDeleted", &self.include_deleted)?;
        }
        if let Some(v) = self.asset_tags.as_ref() {
            struct_ser.serialize_field("assetTags", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit",
            "offset",
            "order",
            "name_matches",
            "nameMatches",
            "case_sensitive",
            "caseSensitive",
            "regexp",
            "order_by",
            "orderBy",
            "rule_ids",
            "ruleIds",
            "asset_ids",
            "assetIds",
            "include_deleted",
            "includeDeleted",
            "asset_tags",
            "assetTags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Limit,
            Offset,
            Order,
            NameMatches,
            CaseSensitive,
            Regexp,
            OrderBy,
            RuleIds,
            AssetIds,
            IncludeDeleted,
            AssetTags,
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
                            "limit" => Ok(GeneratedField::Limit),
                            "offset" => Ok(GeneratedField::Offset),
                            "order" => Ok(GeneratedField::Order),
                            "nameMatches" | "name_matches" => Ok(GeneratedField::NameMatches),
                            "caseSensitive" | "case_sensitive" => Ok(GeneratedField::CaseSensitive),
                            "regexp" => Ok(GeneratedField::Regexp),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "ruleIds" | "rule_ids" => Ok(GeneratedField::RuleIds),
                            "assetIds" | "asset_ids" => Ok(GeneratedField::AssetIds),
                            "includeDeleted" | "include_deleted" => Ok(GeneratedField::IncludeDeleted),
                            "assetTags" | "asset_tags" => Ok(GeneratedField::AssetTags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.SearchRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit__ = None;
                let mut offset__ = None;
                let mut order__ = None;
                let mut name_matches__ = None;
                let mut case_sensitive__ = None;
                let mut regexp__ = None;
                let mut order_by__ = None;
                let mut rule_ids__ = None;
                let mut asset_ids__ = None;
                let mut include_deleted__ = None;
                let mut asset_tags__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Order => {
                            if order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("order"));
                            }
                            order__ = map_.next_value::<::std::option::Option<SearchOrder>>()?.map(|x| x as i32);
                        }
                        GeneratedField::NameMatches => {
                            if name_matches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameMatches"));
                            }
                            name_matches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CaseSensitive => {
                            if case_sensitive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caseSensitive"));
                            }
                            case_sensitive__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Regexp => {
                            if regexp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexp"));
                            }
                            regexp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = map_.next_value()?;
                        }
                        GeneratedField::RuleIds => {
                            if rule_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleIds"));
                            }
                            rule_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncludeDeleted => {
                            if include_deleted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeDeleted"));
                            }
                            include_deleted__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetTags => {
                            if asset_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetTags"));
                            }
                            asset_tags__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SearchRulesRequest {
                    limit: limit__,
                    offset: offset__.unwrap_or_default(),
                    order: order__,
                    name_matches: name_matches__.unwrap_or_default(),
                    case_sensitive: case_sensitive__.unwrap_or_default(),
                    regexp: regexp__.unwrap_or_default(),
                    order_by: order_by__,
                    rule_ids: rule_ids__.unwrap_or_default(),
                    asset_ids: asset_ids__.unwrap_or_default(),
                    include_deleted: include_deleted__.unwrap_or_default(),
                    asset_tags: asset_tags__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.SearchRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.SearchRulesResponse", len)?;
        if self.count != 0 {
            struct_ser.serialize_field("count", &self.count)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
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
                            "count" => Ok(GeneratedField::Count),
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
            type Value = SearchRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.SearchRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SearchRulesResponse {
                    count: count__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.SearchRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SingleChannelComparisonExpression {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_component.is_empty() {
            len += 1;
        }
        if !self.channel_name.is_empty() {
            len += 1;
        }
        if self.comparator != 0 {
            len += 1;
        }
        if self.threshold.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.SingleChannelComparisonExpression", len)?;
        if !self.channel_component.is_empty() {
            struct_ser.serialize_field("channelComponent", &self.channel_component)?;
        }
        if !self.channel_name.is_empty() {
            struct_ser.serialize_field("channelName", &self.channel_name)?;
        }
        if self.comparator != 0 {
            let v = ConditionComparator::try_from(self.comparator)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.comparator)))?;
            struct_ser.serialize_field("comparator", &v)?;
        }
        if let Some(v) = self.threshold.as_ref() {
            match v {
                single_channel_comparison_expression::Threshold::Double(v) => {
                    struct_ser.serialize_field("double", v)?;
                }
                single_channel_comparison_expression::Threshold::String(v) => {
                    struct_ser.serialize_field("string", v)?;
                }
                single_channel_comparison_expression::Threshold::LastValue(v) => {
                    struct_ser.serialize_field("lastValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SingleChannelComparisonExpression {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_component",
            "channelComponent",
            "channel_name",
            "channelName",
            "comparator",
            "double",
            "string",
            "last_value",
            "lastValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelComponent,
            ChannelName,
            Comparator,
            Double,
            String,
            LastValue,
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
                            "channelComponent" | "channel_component" => Ok(GeneratedField::ChannelComponent),
                            "channelName" | "channel_name" => Ok(GeneratedField::ChannelName),
                            "comparator" => Ok(GeneratedField::Comparator),
                            "double" => Ok(GeneratedField::Double),
                            "string" => Ok(GeneratedField::String),
                            "lastValue" | "last_value" => Ok(GeneratedField::LastValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SingleChannelComparisonExpression;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.SingleChannelComparisonExpression")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SingleChannelComparisonExpression, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_component__ = None;
                let mut channel_name__ = None;
                let mut comparator__ = None;
                let mut threshold__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelComponent => {
                            if channel_component__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelComponent"));
                            }
                            channel_component__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelName => {
                            if channel_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelName"));
                            }
                            channel_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Comparator => {
                            if comparator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparator"));
                            }
                            comparator__ = Some(map_.next_value::<ConditionComparator>()? as i32);
                        }
                        GeneratedField::Double => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("double"));
                            }
                            threshold__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| single_channel_comparison_expression::Threshold::Double(x.0));
                        }
                        GeneratedField::String => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("string"));
                            }
                            threshold__ = map_.next_value::<::std::option::Option<_>>()?.map(single_channel_comparison_expression::Threshold::String);
                        }
                        GeneratedField::LastValue => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastValue"));
                            }
                            threshold__ = map_.next_value::<::std::option::Option<_>>()?.map(single_channel_comparison_expression::Threshold::LastValue)
;
                        }
                    }
                }
                Ok(SingleChannelComparisonExpression {
                    channel_component: channel_component__.unwrap_or_default(),
                    channel_name: channel_name__.unwrap_or_default(),
                    comparator: comparator__.unwrap_or_default(),
                    threshold: threshold__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.SingleChannelComparisonExpression", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeRangeQuery {
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
        if self.end_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.TimeRangeQuery", len)?;
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeRangeQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start_time",
            "startTime",
            "end_time",
            "endTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = TimeRangeQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.TimeRangeQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimeRangeQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start_time__ = None;
                let mut end_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                Ok(TimeRangeQuery {
                    start_time: start_time__,
                    end_time: end_time__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.TimeRangeQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UndeleteRuleRequest {
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
        if !self.client_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UndeleteRuleRequest", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UndeleteRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
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
            type Value = UndeleteRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UndeleteRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UndeleteRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut client_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UndeleteRuleRequest {
                    rule_id: rule_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UndeleteRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UndeleteRuleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.rules.v1.UndeleteRuleResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UndeleteRuleResponse {
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
            type Value = UndeleteRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UndeleteRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UndeleteRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UndeleteRuleResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UndeleteRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateActionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule_action_id.is_some() {
            len += 1;
        }
        if self.action_type != 0 {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateActionRequest", len)?;
        if let Some(v) = self.rule_action_id.as_ref() {
            struct_ser.serialize_field("ruleActionId", v)?;
        }
        if self.action_type != 0 {
            let v = ActionKind::try_from(self.action_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action_type)))?;
            struct_ser.serialize_field("actionType", &v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateActionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_action_id",
            "ruleActionId",
            "action_type",
            "actionType",
            "configuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleActionId,
            ActionType,
            Configuration,
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
                            "ruleActionId" | "rule_action_id" => Ok(GeneratedField::RuleActionId),
                            "actionType" | "action_type" => Ok(GeneratedField::ActionType),
                            "configuration" => Ok(GeneratedField::Configuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateActionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateActionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateActionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_action_id__ = None;
                let mut action_type__ = None;
                let mut configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleActionId => {
                            if rule_action_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleActionId"));
                            }
                            rule_action_id__ = map_.next_value()?;
                        }
                        GeneratedField::ActionType => {
                            if action_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionType"));
                            }
                            action_type__ = Some(map_.next_value::<ActionKind>()? as i32);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateActionRequest {
                    rule_action_id: rule_action_id__,
                    action_type: action_type__.unwrap_or_default(),
                    configuration: configuration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateActionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateConditionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule_condition_id.is_some() {
            len += 1;
        }
        if self.expression.is_some() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateConditionRequest", len)?;
        if let Some(v) = self.rule_condition_id.as_ref() {
            struct_ser.serialize_field("ruleConditionId", v)?;
        }
        if let Some(v) = self.expression.as_ref() {
            struct_ser.serialize_field("expression", v)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateConditionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_condition_id",
            "ruleConditionId",
            "expression",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleConditionId,
            Expression,
            Actions,
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
                            "ruleConditionId" | "rule_condition_id" => Ok(GeneratedField::RuleConditionId),
                            "expression" => Ok(GeneratedField::Expression),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateConditionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateConditionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateConditionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_condition_id__ = None;
                let mut expression__ = None;
                let mut actions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleConditionId => {
                            if rule_condition_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleConditionId"));
                            }
                            rule_condition_id__ = map_.next_value()?;
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = map_.next_value()?;
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateConditionRequest {
                    rule_condition_id: rule_condition_id__,
                    expression: expression__,
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateConditionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateHumanFriendlyRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.rules_json.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateHumanFriendlyRulesRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.rules_json.is_empty() {
            struct_ser.serialize_field("rulesJson", &self.rules_json)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateHumanFriendlyRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "rules_json",
            "rulesJson",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            RulesJson,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "rulesJson" | "rules_json" => Ok(GeneratedField::RulesJson),
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
            type Value = UpdateHumanFriendlyRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateHumanFriendlyRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateHumanFriendlyRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut rules_json__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RulesJson => {
                            if rules_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesJson"));
                            }
                            rules_json__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateHumanFriendlyRulesRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                    rules_json: rules_json__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateHumanFriendlyRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateHumanFriendlyRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if self.rules_count != 0 {
            len += 1;
        }
        if !self.messages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateHumanFriendlyRulesResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if self.rules_count != 0 {
            struct_ser.serialize_field("rulesCount", &self.rules_count)?;
        }
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateHumanFriendlyRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "rules_count",
            "rulesCount",
            "messages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            RulesCount,
            Messages,
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
                            "success" => Ok(GeneratedField::Success),
                            "rulesCount" | "rules_count" => Ok(GeneratedField::RulesCount),
                            "messages" => Ok(GeneratedField::Messages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateHumanFriendlyRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateHumanFriendlyRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateHumanFriendlyRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut rules_count__ = None;
                let mut messages__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RulesCount => {
                            if rules_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesCount"));
                            }
                            rules_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateHumanFriendlyRulesResponse {
                    success: success__.unwrap_or_default(),
                    rules_count: rules_count__.unwrap_or_default(),
                    messages: messages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateHumanFriendlyRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateJsonRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateJsonRulesRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateJsonRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
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
                            "request" => Ok(GeneratedField::Request),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateJsonRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateJsonRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateJsonRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateJsonRulesRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateJsonRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateJsonRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateJsonRulesResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateJsonRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Response,
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
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateJsonRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateJsonRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateJsonRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateJsonRulesResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateJsonRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRuleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule_id.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if self.is_enabled {
            len += 1;
        }
        if !self.conditions.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.version_notes.is_empty() {
            len += 1;
        }
        if self.client_key.is_some() {
            len += 1;
        }
        if self.asset_configuration.is_some() {
            len += 1;
        }
        if self.contextual_channels.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateRuleRequest", len)?;
        if let Some(v) = self.rule_id.as_ref() {
            struct_ser.serialize_field("ruleId", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if self.is_enabled {
            struct_ser.serialize_field("isEnabled", &self.is_enabled)?;
        }
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.version_notes.is_empty() {
            struct_ser.serialize_field("versionNotes", &self.version_notes)?;
        }
        if let Some(v) = self.client_key.as_ref() {
            struct_ser.serialize_field("clientKey", v)?;
        }
        if let Some(v) = self.asset_configuration.as_ref() {
            struct_ser.serialize_field("assetConfiguration", v)?;
        }
        if let Some(v) = self.contextual_channels.as_ref() {
            struct_ser.serialize_field("contextualChannels", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRuleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "name",
            "description",
            "asset_id",
            "assetId",
            "is_enabled",
            "isEnabled",
            "conditions",
            "organization_id",
            "organizationId",
            "version_notes",
            "versionNotes",
            "client_key",
            "clientKey",
            "asset_configuration",
            "assetConfiguration",
            "contextual_channels",
            "contextualChannels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            Name,
            Description,
            AssetId,
            IsEnabled,
            Conditions,
            OrganizationId,
            VersionNotes,
            ClientKey,
            AssetConfiguration,
            ContextualChannels,
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
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "isEnabled" | "is_enabled" => Ok(GeneratedField::IsEnabled),
                            "conditions" => Ok(GeneratedField::Conditions),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "versionNotes" | "version_notes" => Ok(GeneratedField::VersionNotes),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "assetConfiguration" | "asset_configuration" => Ok(GeneratedField::AssetConfiguration),
                            "contextualChannels" | "contextual_channels" => Ok(GeneratedField::ContextualChannels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRuleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut asset_id__ = None;
                let mut is_enabled__ = None;
                let mut conditions__ = None;
                let mut organization_id__ = None;
                let mut version_notes__ = None;
                let mut client_key__ = None;
                let mut asset_configuration__ = None;
                let mut contextual_channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = map_.next_value()?;
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
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsEnabled => {
                            if is_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEnabled"));
                            }
                            is_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VersionNotes => {
                            if version_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNotes"));
                            }
                            version_notes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = map_.next_value()?;
                        }
                        GeneratedField::AssetConfiguration => {
                            if asset_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetConfiguration"));
                            }
                            asset_configuration__ = map_.next_value()?;
                        }
                        GeneratedField::ContextualChannels => {
                            if contextual_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextualChannels"));
                            }
                            contextual_channels__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateRuleRequest {
                    rule_id: rule_id__,
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                    is_enabled: is_enabled__.unwrap_or_default(),
                    conditions: conditions__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    version_notes: version_notes__.unwrap_or_default(),
                    client_key: client_key__,
                    asset_configuration: asset_configuration__,
                    contextual_channels: contextual_channels__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateRuleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRuleResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.UpdateRuleResponse", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRuleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRuleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.UpdateRuleResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRuleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateRuleResponse {
                    rule_id: rule_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.UpdateRuleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateJsonRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ValidateJsonRulesRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateJsonRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
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
                            "request" => Ok(GeneratedField::Request),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateJsonRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ValidateJsonRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateJsonRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ValidateJsonRulesRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ValidateJsonRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateJsonRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ValidateJsonRulesResponse", len)?;
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateJsonRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Response,
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
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateJsonRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ValidateJsonRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateJsonRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ValidateJsonRulesResponse {
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ValidateJsonRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidationResult {
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
        if !self.client_key.is_empty() {
            len += 1;
        }
        if !self.asset_expression_validation_results.is_empty() {
            len += 1;
        }
        if self.error.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ValidationResult", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        if !self.asset_expression_validation_results.is_empty() {
            struct_ser.serialize_field("assetExpressionValidationResults", &self.asset_expression_validation_results)?;
        }
        if let Some(v) = self.error.as_ref() {
            struct_ser.serialize_field("error", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidationResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "client_key",
            "clientKey",
            "asset_expression_validation_results",
            "assetExpressionValidationResults",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            ClientKey,
            AssetExpressionValidationResults,
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
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "assetExpressionValidationResults" | "asset_expression_validation_results" => Ok(GeneratedField::AssetExpressionValidationResults),
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
            type Value = ValidationResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ValidationResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidationResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut client_key__ = None;
                let mut asset_expression_validation_results__ = None;
                let mut error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetExpressionValidationResults => {
                            if asset_expression_validation_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetExpressionValidationResults"));
                            }
                            asset_expression_validation_results__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ValidationResult {
                    rule_id: rule_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                    asset_expression_validation_results: asset_expression_validation_results__.unwrap_or_default(),
                    error: error__,
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ValidationResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ViewHumanFriendlyRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ViewHumanFriendlyRulesRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ViewHumanFriendlyRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ViewHumanFriendlyRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ViewHumanFriendlyRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ViewHumanFriendlyRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ViewHumanFriendlyRulesRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ViewHumanFriendlyRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ViewHumanFriendlyRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules_json.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ViewHumanFriendlyRulesResponse", len)?;
        if !self.rules_json.is_empty() {
            struct_ser.serialize_field("rulesJson", &self.rules_json)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ViewHumanFriendlyRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules_json",
            "rulesJson",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RulesJson,
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
                            "rulesJson" | "rules_json" => Ok(GeneratedField::RulesJson),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ViewHumanFriendlyRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ViewHumanFriendlyRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ViewHumanFriendlyRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules_json__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RulesJson => {
                            if rules_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesJson"));
                            }
                            rules_json__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ViewHumanFriendlyRulesResponse {
                    rules_json: rules_json__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ViewHumanFriendlyRulesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ViewJsonRulesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ViewJsonRulesRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ViewJsonRulesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ViewJsonRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ViewJsonRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ViewJsonRulesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ViewJsonRulesRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ViewJsonRulesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ViewJsonRulesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules_json.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.ViewJsonRulesResponse", len)?;
        if !self.rules_json.is_empty() {
            struct_ser.serialize_field("rulesJson", &self.rules_json)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ViewJsonRulesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules_json",
            "rulesJson",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RulesJson,
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
                            "rulesJson" | "rules_json" => Ok(GeneratedField::RulesJson),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ViewJsonRulesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.ViewJsonRulesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ViewJsonRulesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules_json__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RulesJson => {
                            if rules_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rulesJson"));
                            }
                            rules_json__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ViewJsonRulesResponse {
                    rules_json: rules_json__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.rules.v1.ViewJsonRulesResponse", FIELDS, GeneratedVisitor)
    }
}
