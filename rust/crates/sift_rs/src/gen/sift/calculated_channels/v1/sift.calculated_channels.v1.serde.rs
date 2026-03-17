// @generated
impl serde::Serialize for ErrorValidatingExpressionResult {
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
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ErrorValidatingExpressionResult", len)?;
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorValidatingExpressionResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorMessage,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorValidatingExpressionResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ErrorValidatingExpressionResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorValidatingExpressionResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ErrorValidatingExpressionResult {
                    error_message: error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ErrorValidatingExpressionResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionChannelReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_reference.is_empty() {
            len += 1;
        }
        if !self.channel_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ExpressionChannelReference", len)?;
        if !self.channel_reference.is_empty() {
            struct_ser.serialize_field("channelReference", &self.channel_reference)?;
        }
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionChannelReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_reference",
            "channelReference",
            "channel_id",
            "channelId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelReference,
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
                            "channelReference" | "channel_reference" => Ok(GeneratedField::ChannelReference),
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
            type Value = ExpressionChannelReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ExpressionChannelReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpressionChannelReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_reference__ = None;
                let mut channel_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelReference => {
                            if channel_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelReference"));
                            }
                            channel_reference__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExpressionChannelReference {
                    channel_reference: channel_reference__.unwrap_or_default(),
                    channel_id: channel_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ExpressionChannelReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionIdentifier {
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
        if self.r#type != 0 {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.library != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ExpressionIdentifier", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.r#type != 0 {
            let v = ExpressionIdentifierType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.library != 0 {
            let v = ExpressionIdentifierLibrary::try_from(self.library)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.library)))?;
            struct_ser.serialize_field("library", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "type",
            "display_name",
            "displayName",
            "library",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Type,
            DisplayName,
            Library,
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
                            "type" => Ok(GeneratedField::Type),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "library" => Ok(GeneratedField::Library),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpressionIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ExpressionIdentifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpressionIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut r#type__ = None;
                let mut display_name__ = None;
                let mut library__ = None;
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<ExpressionIdentifierType>()? as i32);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Library => {
                            if library__.is_some() {
                                return Err(serde::de::Error::duplicate_field("library"));
                            }
                            library__ = Some(map_.next_value::<ExpressionIdentifierLibrary>()? as i32);
                        }
                    }
                }
                Ok(ExpressionIdentifier {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    library: library__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ExpressionIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionIdentifierLibrary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXPRESSION_IDENTIFIER_LIBRARY_UNSPECIFIED",
            Self::Math => "EXPRESSION_IDENTIFIER_LIBRARY_MATH",
            Self::String => "EXPRESSION_IDENTIFIER_LIBRARY_STRING",
            Self::List => "EXPRESSION_IDENTIFIER_LIBRARY_LIST",
            Self::Iter => "EXPRESSION_IDENTIFIER_LIBRARY_ITER",
            Self::Stateful => "EXPRESSION_IDENTIFIER_LIBRARY_STATEFUL",
            Self::Summary => "EXPRESSION_IDENTIFIER_LIBRARY_SUMMARY",
            Self::UserDefinedFunctions => "EXPRESSION_IDENTIFIER_LIBRARY_USER_DEFINED_FUNCTIONS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionIdentifierLibrary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXPRESSION_IDENTIFIER_LIBRARY_UNSPECIFIED",
            "EXPRESSION_IDENTIFIER_LIBRARY_MATH",
            "EXPRESSION_IDENTIFIER_LIBRARY_STRING",
            "EXPRESSION_IDENTIFIER_LIBRARY_LIST",
            "EXPRESSION_IDENTIFIER_LIBRARY_ITER",
            "EXPRESSION_IDENTIFIER_LIBRARY_STATEFUL",
            "EXPRESSION_IDENTIFIER_LIBRARY_SUMMARY",
            "EXPRESSION_IDENTIFIER_LIBRARY_USER_DEFINED_FUNCTIONS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpressionIdentifierLibrary;

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
                    "EXPRESSION_IDENTIFIER_LIBRARY_UNSPECIFIED" => Ok(ExpressionIdentifierLibrary::Unspecified),
                    "EXPRESSION_IDENTIFIER_LIBRARY_MATH" => Ok(ExpressionIdentifierLibrary::Math),
                    "EXPRESSION_IDENTIFIER_LIBRARY_STRING" => Ok(ExpressionIdentifierLibrary::String),
                    "EXPRESSION_IDENTIFIER_LIBRARY_LIST" => Ok(ExpressionIdentifierLibrary::List),
                    "EXPRESSION_IDENTIFIER_LIBRARY_ITER" => Ok(ExpressionIdentifierLibrary::Iter),
                    "EXPRESSION_IDENTIFIER_LIBRARY_STATEFUL" => Ok(ExpressionIdentifierLibrary::Stateful),
                    "EXPRESSION_IDENTIFIER_LIBRARY_SUMMARY" => Ok(ExpressionIdentifierLibrary::Summary),
                    "EXPRESSION_IDENTIFIER_LIBRARY_USER_DEFINED_FUNCTIONS" => Ok(ExpressionIdentifierLibrary::UserDefinedFunctions),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionIdentifierType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED",
            Self::Function => "EXPRESSION_IDENTIFIER_TYPE_FUNCTION",
            Self::Channel => "EXPRESSION_IDENTIFIER_TYPE_CHANNEL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionIdentifierType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED",
            "EXPRESSION_IDENTIFIER_TYPE_FUNCTION",
            "EXPRESSION_IDENTIFIER_TYPE_CHANNEL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpressionIdentifierType;

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
                    "EXPRESSION_IDENTIFIER_TYPE_UNSPECIFIED" => Ok(ExpressionIdentifierType::Unspecified),
                    "EXPRESSION_IDENTIFIER_TYPE_FUNCTION" => Ok(ExpressionIdentifierType::Function),
                    "EXPRESSION_IDENTIFIER_TYPE_CHANNEL" => Ok(ExpressionIdentifierType::Channel),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "EXPRESSION_MODE_UNSPECIFIED",
            Self::Rules => "EXPRESSION_MODE_RULES",
            Self::CalculatedChannels => "EXPRESSION_MODE_CALCULATED_CHANNELS",
            Self::Ruler => "EXPRESSION_MODE_RULER",
            Self::StructuredData => "EXPRESSION_MODE_STRUCTURED_DATA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EXPRESSION_MODE_UNSPECIFIED",
            "EXPRESSION_MODE_RULES",
            "EXPRESSION_MODE_CALCULATED_CHANNELS",
            "EXPRESSION_MODE_RULER",
            "EXPRESSION_MODE_STRUCTURED_DATA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpressionMode;

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
                    "EXPRESSION_MODE_UNSPECIFIED" => Ok(ExpressionMode::Unspecified),
                    "EXPRESSION_MODE_RULES" => Ok(ExpressionMode::Rules),
                    "EXPRESSION_MODE_CALCULATED_CHANNELS" => Ok(ExpressionMode::CalculatedChannels),
                    "EXPRESSION_MODE_RULER" => Ok(ExpressionMode::Ruler),
                    "EXPRESSION_MODE_STRUCTURED_DATA" => Ok(ExpressionMode::StructuredData),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExpressionRequest {
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
        if !self.expression_channel_references.is_empty() {
            len += 1;
        }
        if !self.function_dependencies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ExpressionRequest", len)?;
        if !self.channel_references.is_empty() {
            struct_ser.serialize_field("channelReferences", &self.channel_references)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.expression_channel_references.is_empty() {
            struct_ser.serialize_field("expressionChannelReferences", &self.expression_channel_references)?;
        }
        if !self.function_dependencies.is_empty() {
            struct_ser.serialize_field("functionDependencies", &self.function_dependencies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpressionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_references",
            "channelReferences",
            "expression",
            "expression_channel_references",
            "expressionChannelReferences",
            "function_dependencies",
            "functionDependencies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelReferences,
            Expression,
            ExpressionChannelReferences,
            FunctionDependencies,
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
                            "expressionChannelReferences" | "expression_channel_references" => Ok(GeneratedField::ExpressionChannelReferences),
                            "functionDependencies" | "function_dependencies" => Ok(GeneratedField::FunctionDependencies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpressionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ExpressionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpressionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_references__ = None;
                let mut expression__ = None;
                let mut expression_channel_references__ = None;
                let mut function_dependencies__ = None;
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
                        GeneratedField::ExpressionChannelReferences => {
                            if expression_channel_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expressionChannelReferences"));
                            }
                            expression_channel_references__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionDependencies => {
                            if function_dependencies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionDependencies"));
                            }
                            function_dependencies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExpressionRequest {
                    channel_references: channel_references__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                    expression_channel_references: expression_channel_references__.unwrap_or_default(),
                    function_dependencies: function_dependencies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ExpressionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExpressionIdentifiersRequest {
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
        if self.mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ListExpressionIdentifiersRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.mode != 0 {
            let v = ExpressionMode::try_from(self.mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mode)))?;
            struct_ser.serialize_field("mode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExpressionIdentifiersRequest {
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
            "mode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Mode,
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
                            "mode" => Ok(GeneratedField::Mode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExpressionIdentifiersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ListExpressionIdentifiersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExpressionIdentifiersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut mode__ = None;
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
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map_.next_value::<ExpressionMode>()? as i32);
                        }
                    }
                }
                Ok(ListExpressionIdentifiersRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    mode: mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ListExpressionIdentifiersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExpressionIdentifiersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.identifiers.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ListExpressionIdentifiersResponse", len)?;
        if !self.identifiers.is_empty() {
            struct_ser.serialize_field("identifiers", &self.identifiers)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExpressionIdentifiersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifiers",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifiers,
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
                            "identifiers" => Ok(GeneratedField::Identifiers),
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
            type Value = ListExpressionIdentifiersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ListExpressionIdentifiersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExpressionIdentifiersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifiers__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Identifiers => {
                            if identifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifiers"));
                            }
                            identifiers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListExpressionIdentifiersResponse {
                    identifiers: identifiers__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ListExpressionIdentifiersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SuccessValidatingExpressionResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.SuccessValidatingExpressionResult", len)?;
        if self.data_type != 0 {
            let v = super::super::common::r#type::v1::ChannelDataType::try_from(self.data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SuccessValidatingExpressionResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_type",
            "dataType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataType,
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
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SuccessValidatingExpressionResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.SuccessValidatingExpressionResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SuccessValidatingExpressionResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value::<super::super::common::r#type::v1::ChannelDataType>()? as i32);
                        }
                    }
                }
                Ok(SuccessValidatingExpressionResult {
                    data_type: data_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.SuccessValidatingExpressionResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateExpressionRequest {
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
        if self.mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ValidateExpressionRequest", len)?;
        if let Some(v) = self.expression.as_ref() {
            struct_ser.serialize_field("expression", v)?;
        }
        if self.mode != 0 {
            let v = ExpressionMode::try_from(self.mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.mode)))?;
            struct_ser.serialize_field("mode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateExpressionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expression",
            "mode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expression,
            Mode,
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
                            "expression" => Ok(GeneratedField::Expression),
                            "mode" => Ok(GeneratedField::Mode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateExpressionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ValidateExpressionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateExpressionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expression__ = None;
                let mut mode__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = map_.next_value()?;
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map_.next_value::<ExpressionMode>()? as i32);
                        }
                    }
                }
                Ok(ValidateExpressionRequest {
                    expression: expression__,
                    mode: mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ValidateExpressionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateExpressionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v1.ValidateExpressionResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            match v {
                validate_expression_response::Result::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                validate_expression_response::Result::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateExpressionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            Success,
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
                            "error" => Ok(GeneratedField::Error),
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateExpressionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v1.ValidateExpressionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateExpressionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_expression_response::Result::Error)
;
                        }
                        GeneratedField::Success => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_expression_response::Result::Success)
;
                        }
                    }
                }
                Ok(ValidateExpressionResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v1.ValidateExpressionResponse", FIELDS, GeneratedVisitor)
    }
}
