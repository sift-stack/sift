// @generated
impl serde::Serialize for ActionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACTION_TYPE_UNSPECIFIED",
            Self::IngestData => "ACTION_TYPE_INGEST_DATA",
            Self::CreateRun => "ACTION_TYPE_CREATE_RUN",
            Self::ViewDetails => "ACTION_TYPE_VIEW_DETAILS",
            Self::ViewData => "ACTION_TYPE_VIEW_DATA",
            Self::Export => "ACTION_TYPE_EXPORT",
            Self::EditDetails => "ACTION_TYPE_EDIT_DETAILS",
            Self::EditTags => "ACTION_TYPE_EDIT_TAGS",
            Self::EditMetadata => "ACTION_TYPE_EDIT_METADATA",
            Self::Archive => "ACTION_TYPE_ARCHIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ActionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTION_TYPE_UNSPECIFIED",
            "ACTION_TYPE_INGEST_DATA",
            "ACTION_TYPE_CREATE_RUN",
            "ACTION_TYPE_VIEW_DETAILS",
            "ACTION_TYPE_VIEW_DATA",
            "ACTION_TYPE_EXPORT",
            "ACTION_TYPE_EDIT_DETAILS",
            "ACTION_TYPE_EDIT_TAGS",
            "ACTION_TYPE_EDIT_METADATA",
            "ACTION_TYPE_ARCHIVE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActionType;

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
                    "ACTION_TYPE_UNSPECIFIED" => Ok(ActionType::Unspecified),
                    "ACTION_TYPE_INGEST_DATA" => Ok(ActionType::IngestData),
                    "ACTION_TYPE_CREATE_RUN" => Ok(ActionType::CreateRun),
                    "ACTION_TYPE_VIEW_DETAILS" => Ok(ActionType::ViewDetails),
                    "ACTION_TYPE_VIEW_DATA" => Ok(ActionType::ViewData),
                    "ACTION_TYPE_EXPORT" => Ok(ActionType::Export),
                    "ACTION_TYPE_EDIT_DETAILS" => Ok(ActionType::EditDetails),
                    "ACTION_TYPE_EDIT_TAGS" => Ok(ActionType::EditTags),
                    "ACTION_TYPE_EDIT_METADATA" => Ok(ActionType::EditMetadata),
                    "ACTION_TYPE_ARCHIVE" => Ok(ActionType::Archive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FilterField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field_name.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.enum_values.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.v1.FilterField", len)?;
        if !self.field_name.is_empty() {
            struct_ser.serialize_field("fieldName", &self.field_name)?;
        }
        if self.r#type != 0 {
            let v = FilterFieldType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.enum_values.is_empty() {
            struct_ser.serialize_field("enumValues", &self.enum_values)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_name",
            "fieldName",
            "type",
            "enum_values",
            "enumValues",
            "description",
            "display_name",
            "displayName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldName,
            Type,
            EnumValues,
            Description,
            DisplayName,
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
                            "fieldName" | "field_name" => Ok(GeneratedField::FieldName),
                            "type" => Ok(GeneratedField::Type),
                            "enumValues" | "enum_values" => Ok(GeneratedField::EnumValues),
                            "description" => Ok(GeneratedField::Description),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.v1.FilterField")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterField, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_name__ = None;
                let mut r#type__ = None;
                let mut enum_values__ = None;
                let mut description__ = None;
                let mut display_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldName => {
                            if field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldName"));
                            }
                            field_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<FilterFieldType>()? as i32);
                        }
                        GeneratedField::EnumValues => {
                            if enum_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumValues"));
                            }
                            enum_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FilterField {
                    field_name: field_name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    enum_values: enum_values__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.v1.FilterField", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterFieldType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FILTER_FIELD_TYPE_UNSPECIFIED",
            Self::String => "FILTER_FIELD_TYPE_STRING",
            Self::Timestamp => "FILTER_FIELD_TYPE_TIMESTAMP",
            Self::Int => "FILTER_FIELD_TYPE_INT",
            Self::Bool => "FILTER_FIELD_TYPE_BOOL",
            Self::Duration => "FILTER_FIELD_TYPE_DURATION",
            Self::Double => "FILTER_FIELD_TYPE_DOUBLE",
            Self::Enum => "FILTER_FIELD_TYPE_ENUM",
            Self::Uuid => "FILTER_FIELD_TYPE_UUID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FilterFieldType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FILTER_FIELD_TYPE_UNSPECIFIED",
            "FILTER_FIELD_TYPE_STRING",
            "FILTER_FIELD_TYPE_TIMESTAMP",
            "FILTER_FIELD_TYPE_INT",
            "FILTER_FIELD_TYPE_BOOL",
            "FILTER_FIELD_TYPE_DURATION",
            "FILTER_FIELD_TYPE_DOUBLE",
            "FILTER_FIELD_TYPE_ENUM",
            "FILTER_FIELD_TYPE_UUID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterFieldType;

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
                    "FILTER_FIELD_TYPE_UNSPECIFIED" => Ok(FilterFieldType::Unspecified),
                    "FILTER_FIELD_TYPE_STRING" => Ok(FilterFieldType::String),
                    "FILTER_FIELD_TYPE_TIMESTAMP" => Ok(FilterFieldType::Timestamp),
                    "FILTER_FIELD_TYPE_INT" => Ok(FilterFieldType::Int),
                    "FILTER_FIELD_TYPE_BOOL" => Ok(FilterFieldType::Bool),
                    "FILTER_FIELD_TYPE_DURATION" => Ok(FilterFieldType::Duration),
                    "FILTER_FIELD_TYPE_DOUBLE" => Ok(FilterFieldType::Double),
                    "FILTER_FIELD_TYPE_ENUM" => Ok(FilterFieldType::Enum),
                    "FILTER_FIELD_TYPE_UUID" => Ok(FilterFieldType::Uuid),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RESOURCE_TYPE_UNSPECIFIED",
            Self::Asset => "RESOURCE_TYPE_ASSET",
            Self::Channel => "RESOURCE_TYPE_CHANNEL",
            Self::Run => "RESOURCE_TYPE_RUN",
            Self::Annotation => "RESOURCE_TYPE_ANNOTATION",
            Self::Campaign => "RESOURCE_TYPE_CAMPAIGN",
            Self::Report => "RESOURCE_TYPE_REPORT",
            Self::Rule => "RESOURCE_TYPE_RULE",
            Self::RuleVersion => "RESOURCE_TYPE_RULE_VERSION",
            Self::UserDefinedFunction => "RESOURCE_TYPE_USER_DEFINED_FUNCTION",
            Self::CalculatedChannel => "RESOURCE_TYPE_CALCULATED_CHANNEL",
            Self::ReportTemplate => "RESOURCE_TYPE_REPORT_TEMPLATE",
            Self::TestReport => "RESOURCE_TYPE_TEST_REPORT",
            Self::SiftApp => "RESOURCE_TYPE_SIFT_APP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ResourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RESOURCE_TYPE_UNSPECIFIED",
            "RESOURCE_TYPE_ASSET",
            "RESOURCE_TYPE_CHANNEL",
            "RESOURCE_TYPE_RUN",
            "RESOURCE_TYPE_ANNOTATION",
            "RESOURCE_TYPE_CAMPAIGN",
            "RESOURCE_TYPE_REPORT",
            "RESOURCE_TYPE_RULE",
            "RESOURCE_TYPE_RULE_VERSION",
            "RESOURCE_TYPE_USER_DEFINED_FUNCTION",
            "RESOURCE_TYPE_CALCULATED_CHANNEL",
            "RESOURCE_TYPE_REPORT_TEMPLATE",
            "RESOURCE_TYPE_TEST_REPORT",
            "RESOURCE_TYPE_SIFT_APP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceType;

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
                    "RESOURCE_TYPE_UNSPECIFIED" => Ok(ResourceType::Unspecified),
                    "RESOURCE_TYPE_ASSET" => Ok(ResourceType::Asset),
                    "RESOURCE_TYPE_CHANNEL" => Ok(ResourceType::Channel),
                    "RESOURCE_TYPE_RUN" => Ok(ResourceType::Run),
                    "RESOURCE_TYPE_ANNOTATION" => Ok(ResourceType::Annotation),
                    "RESOURCE_TYPE_CAMPAIGN" => Ok(ResourceType::Campaign),
                    "RESOURCE_TYPE_REPORT" => Ok(ResourceType::Report),
                    "RESOURCE_TYPE_RULE" => Ok(ResourceType::Rule),
                    "RESOURCE_TYPE_RULE_VERSION" => Ok(ResourceType::RuleVersion),
                    "RESOURCE_TYPE_USER_DEFINED_FUNCTION" => Ok(ResourceType::UserDefinedFunction),
                    "RESOURCE_TYPE_CALCULATED_CHANNEL" => Ok(ResourceType::CalculatedChannel),
                    "RESOURCE_TYPE_REPORT_TEMPLATE" => Ok(ResourceType::ReportTemplate),
                    "RESOURCE_TYPE_TEST_REPORT" => Ok(ResourceType::TestReport),
                    "RESOURCE_TYPE_SIFT_APP" => Ok(ResourceType::SiftApp),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
