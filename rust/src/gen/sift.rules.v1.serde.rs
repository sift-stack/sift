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
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.BatchGetRulesRequest", len)?;
        if !self.rule_ids.is_empty() {
            struct_ser.serialize_field("ruleIds", &self.rule_ids)?;
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
            type Value = BatchGetRulesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.BatchGetRulesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchGetRulesRequest, V::Error>
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
                Ok(BatchGetRulesRequest {
                    rule_ids: rule_ids__.unwrap_or_default(),
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
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.DeleteRuleRequest", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
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
            type Value = DeleteRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.DeleteRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRuleRequest, V::Error>
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
                Ok(DeleteRuleRequest {
                    rule_id: rule_id__.unwrap_or_default(),
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
            "run_id",
            "runId",
            "time_range",
            "timeRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleIds,
            AnnotationOptions,
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
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.EvaluateRulesResponse", len)?;
        if self.created_annotation_count != 0 {
            struct_ser.serialize_field("createdAnnotationCount", &self.created_annotation_count)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreatedAnnotationCount,
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
                    }
                }
                Ok(EvaluateRulesResponse {
                    created_annotation_count: created_annotation_count__.unwrap_or_default(),
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
        let mut struct_ser = serializer.serialize_struct("sift.rules.v1.GetRuleRequest", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
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
            type Value = GetRuleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.rules.v1.GetRuleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRuleRequest, V::Error>
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
                Ok(GetRuleRequest {
                    rule_id: rule_id__.unwrap_or_default(),
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
        if !self.current_status.is_empty() {
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
        if !self.current_status.is_empty() {
            struct_ser.serialize_field("currentStatus", &self.current_status)?;
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
            "current_status",
            "currentStatus",
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            AssetId,
            Name,
            Description,
            CurrentStatus,
            IsEnabled,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
            OrganizationId,
            Conditions,
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
                            "currentStatus" | "current_status" => Ok(GeneratedField::CurrentStatus),
                            "isEnabled" | "is_enabled" => Ok(GeneratedField::IsEnabled),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "conditions" => Ok(GeneratedField::Conditions),
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
                let mut current_status__ = None;
                let mut is_enabled__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut organization_id__ = None;
                let mut conditions__ = None;
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
                        GeneratedField::CurrentStatus => {
                            if current_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentStatus"));
                            }
                            current_status__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(Rule {
                    rule_id: rule_id__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    current_status: current_status__.unwrap_or_default(),
                    is_enabled: is_enabled__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    conditions: conditions__.unwrap_or_default(),
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
        if !self.status.is_empty() {
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
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
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
            "status",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleConditionId,
            RuleId,
            Expression,
            Status,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "expression" => Ok(GeneratedField::Expression),
                            "status" => Ok(GeneratedField::Status),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
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
                let mut status__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut actions__ = None;
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
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(RuleCondition {
                    rule_condition_id: rule_condition_id__.unwrap_or_default(),
                    rule_id: rule_id__.unwrap_or_default(),
                    expression: expression__,
                    status: status__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
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
        if !self.status.is_empty() {
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
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
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
            "status",
            "expression",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleConditionId,
            Status,
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
                            "status" => Ok(GeneratedField::Status),
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
                let mut status__ = None;
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
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value()?);
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
                    status: status__.unwrap_or_default(),
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
