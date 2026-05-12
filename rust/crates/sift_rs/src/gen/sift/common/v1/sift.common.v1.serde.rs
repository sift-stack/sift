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
