// @generated
impl serde::Serialize for ArchiveUserAttributeKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ArchiveUserAttributeKeysRequest", len)?;
        if !self.user_attribute_key_ids.is_empty() {
            struct_ser.serialize_field("userAttributeKeyIds", &self.user_attribute_key_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveUserAttributeKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_ids",
            "userAttributeKeyIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyIds,
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
                            "userAttributeKeyIds" | "user_attribute_key_ids" => Ok(GeneratedField::UserAttributeKeyIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveUserAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ArchiveUserAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveUserAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyIds => {
                            if user_attribute_key_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyIds"));
                            }
                            user_attribute_key_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveUserAttributeKeysRequest {
                    user_attribute_key_ids: user_attribute_key_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ArchiveUserAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveUserAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ArchiveUserAttributeKeysResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveUserAttributeKeysResponse {
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
            type Value = ArchiveUserAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ArchiveUserAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveUserAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ArchiveUserAttributeKeysResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ArchiveUserAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveUserAttributeValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_value_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ArchiveUserAttributeValuesRequest", len)?;
        if !self.user_attribute_value_ids.is_empty() {
            struct_ser.serialize_field("userAttributeValueIds", &self.user_attribute_value_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveUserAttributeValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_value_ids",
            "userAttributeValueIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValueIds,
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
                            "userAttributeValueIds" | "user_attribute_value_ids" => Ok(GeneratedField::UserAttributeValueIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveUserAttributeValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ArchiveUserAttributeValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveUserAttributeValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_value_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValueIds => {
                            if user_attribute_value_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValueIds"));
                            }
                            user_attribute_value_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveUserAttributeValuesRequest {
                    user_attribute_value_ids: user_attribute_value_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ArchiveUserAttributeValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveUserAttributeValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ArchiveUserAttributeValuesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveUserAttributeValuesResponse {
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
            type Value = ArchiveUserAttributeValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ArchiveUserAttributeValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveUserAttributeValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ArchiveUserAttributeValuesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ArchiveUserAttributeValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateUserAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.user_ids.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.BatchCreateUserAttributeValueRequest", len)?;
        if !self.user_attribute_key_id.is_empty() {
            struct_ser.serialize_field("userAttributeKeyId", &self.user_attribute_key_id)?;
        }
        if !self.user_ids.is_empty() {
            struct_ser.serialize_field("userIds", &self.user_ids)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                batch_create_user_attribute_value_request::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                batch_create_user_attribute_value_request::Value::NumberValue(v) => {
                    struct_ser.serialize_field("numberValue", v)?;
                }
                batch_create_user_attribute_value_request::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateUserAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_id",
            "userAttributeKeyId",
            "user_ids",
            "userIds",
            "string_value",
            "stringValue",
            "number_value",
            "numberValue",
            "boolean_value",
            "booleanValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyId,
            UserIds,
            StringValue,
            NumberValue,
            BooleanValue,
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
                            "userAttributeKeyId" | "user_attribute_key_id" => Ok(GeneratedField::UserAttributeKeyId),
                            "userIds" | "user_ids" => Ok(GeneratedField::UserIds),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "numberValue" | "number_value" => Ok(GeneratedField::NumberValue),
                            "booleanValue" | "boolean_value" => Ok(GeneratedField::BooleanValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateUserAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.BatchCreateUserAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateUserAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_id__ = None;
                let mut user_ids__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyId => {
                            if user_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyId"));
                            }
                            user_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserIds => {
                            if user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIds"));
                            }
                            user_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_create_user_attribute_value_request::Value::StringValue);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| batch_create_user_attribute_value_request::Value::NumberValue(x.0));
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_create_user_attribute_value_request::Value::BooleanValue);
                        }
                    }
                }
                Ok(BatchCreateUserAttributeValueRequest {
                    user_attribute_key_id: user_attribute_key_id__.unwrap_or_default(),
                    user_ids: user_ids__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.BatchCreateUserAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateUserAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.BatchCreateUserAttributeValueResponse", len)?;
        if !self.user_attribute_values.is_empty() {
            struct_ser.serialize_field("userAttributeValues", &self.user_attribute_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateUserAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_values",
            "userAttributeValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValues,
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
                            "userAttributeValues" | "user_attribute_values" => Ok(GeneratedField::UserAttributeValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateUserAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.BatchCreateUserAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateUserAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValues => {
                            if user_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValues"));
                            }
                            user_attribute_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateUserAttributeValueResponse {
                    user_attribute_values: user_attribute_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.BatchCreateUserAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserAttributeKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.CreateUserAttributeKeyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.r#type != 0 {
            let v = UserAttributeValueType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Type,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.CreateUserAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut r#type__ = None;
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
                            r#type__ = Some(map_.next_value::<UserAttributeValueType>()? as i32);
                        }
                    }
                }
                Ok(CreateUserAttributeKeyRequest {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.CreateUserAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.CreateUserAttributeKeyResponse", len)?;
        if let Some(v) = self.user_attribute_key.as_ref() {
            struct_ser.serialize_field("userAttributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key",
            "userAttributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKey,
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
                            "userAttributeKey" | "user_attribute_key" => Ok(GeneratedField::UserAttributeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.CreateUserAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKey => {
                            if user_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKey"));
                            }
                            user_attribute_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateUserAttributeKeyResponse {
                    user_attribute_key: user_attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.CreateUserAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.CreateUserAttributeValueRequest", len)?;
        if !self.user_attribute_key_id.is_empty() {
            struct_ser.serialize_field("userAttributeKeyId", &self.user_attribute_key_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                create_user_attribute_value_request::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                create_user_attribute_value_request::Value::NumberValue(v) => {
                    struct_ser.serialize_field("numberValue", v)?;
                }
                create_user_attribute_value_request::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_id",
            "userAttributeKeyId",
            "user_id",
            "userId",
            "string_value",
            "stringValue",
            "number_value",
            "numberValue",
            "boolean_value",
            "booleanValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyId,
            UserId,
            StringValue,
            NumberValue,
            BooleanValue,
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
                            "userAttributeKeyId" | "user_attribute_key_id" => Ok(GeneratedField::UserAttributeKeyId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "numberValue" | "number_value" => Ok(GeneratedField::NumberValue),
                            "booleanValue" | "boolean_value" => Ok(GeneratedField::BooleanValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.CreateUserAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_id__ = None;
                let mut user_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyId => {
                            if user_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyId"));
                            }
                            user_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(create_user_attribute_value_request::Value::StringValue);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| create_user_attribute_value_request::Value::NumberValue(x.0));
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(create_user_attribute_value_request::Value::BooleanValue);
                        }
                    }
                }
                Ok(CreateUserAttributeValueRequest {
                    user_attribute_key_id: user_attribute_key_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.CreateUserAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_attribute_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.CreateUserAttributeValueResponse", len)?;
        if let Some(v) = self.user_attribute_value.as_ref() {
            struct_ser.serialize_field("userAttributeValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_value",
            "userAttributeValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValue,
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
                            "userAttributeValue" | "user_attribute_value" => Ok(GeneratedField::UserAttributeValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.CreateUserAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValue => {
                            if user_attribute_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValue"));
                            }
                            user_attribute_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateUserAttributeValueResponse {
                    user_attribute_value: user_attribute_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.CreateUserAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.GetUserAttributeKeyRequest", len)?;
        if !self.user_attribute_key_id.is_empty() {
            struct_ser.serialize_field("userAttributeKeyId", &self.user_attribute_key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_id",
            "userAttributeKeyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyId,
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
                            "userAttributeKeyId" | "user_attribute_key_id" => Ok(GeneratedField::UserAttributeKeyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.GetUserAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyId => {
                            if user_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyId"));
                            }
                            user_attribute_key_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserAttributeKeyRequest {
                    user_attribute_key_id: user_attribute_key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.GetUserAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.GetUserAttributeKeyResponse", len)?;
        if let Some(v) = self.user_attribute_key.as_ref() {
            struct_ser.serialize_field("userAttributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key",
            "userAttributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKey,
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
                            "userAttributeKey" | "user_attribute_key" => Ok(GeneratedField::UserAttributeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.GetUserAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKey => {
                            if user_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKey"));
                            }
                            user_attribute_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserAttributeKeyResponse {
                    user_attribute_key: user_attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.GetUserAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.GetUserAttributeValueRequest", len)?;
        if !self.user_attribute_value_id.is_empty() {
            struct_ser.serialize_field("userAttributeValueId", &self.user_attribute_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_value_id",
            "userAttributeValueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValueId,
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
                            "userAttributeValueId" | "user_attribute_value_id" => Ok(GeneratedField::UserAttributeValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.GetUserAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValueId => {
                            if user_attribute_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValueId"));
                            }
                            user_attribute_value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserAttributeValueRequest {
                    user_attribute_value_id: user_attribute_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.GetUserAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_attribute_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.GetUserAttributeValueResponse", len)?;
        if let Some(v) = self.user_attribute_value.as_ref() {
            struct_ser.serialize_field("userAttributeValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_value",
            "userAttributeValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValue,
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
                            "userAttributeValue" | "user_attribute_value" => Ok(GeneratedField::UserAttributeValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.GetUserAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValue => {
                            if user_attribute_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValue"));
                            }
                            user_attribute_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserAttributeValueResponse {
                    user_attribute_value: user_attribute_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.GetUserAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserAttributeKeyValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_id.is_empty() {
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
        if self.include_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ListUserAttributeKeyValuesRequest", len)?;
        if !self.user_attribute_key_id.is_empty() {
            struct_ser.serialize_field("userAttributeKeyId", &self.user_attribute_key_id)?;
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
        if self.include_archived {
            struct_ser.serialize_field("includeArchived", &self.include_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserAttributeKeyValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_id",
            "userAttributeKeyId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
            "order_by",
            "orderBy",
            "include_archived",
            "includeArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyId,
            PageSize,
            PageToken,
            Filter,
            OrderBy,
            IncludeArchived,
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
                            "userAttributeKeyId" | "user_attribute_key_id" => Ok(GeneratedField::UserAttributeKeyId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "includeArchived" | "include_archived" => Ok(GeneratedField::IncludeArchived),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUserAttributeKeyValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ListUserAttributeKeyValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserAttributeKeyValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut include_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyId => {
                            if user_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyId"));
                            }
                            user_attribute_key_id__ = Some(map_.next_value()?);
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
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUserAttributeKeyValuesRequest {
                    user_attribute_key_id: user_attribute_key_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ListUserAttributeKeyValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserAttributeKeyValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_values.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ListUserAttributeKeyValuesResponse", len)?;
        if !self.user_attribute_values.is_empty() {
            struct_ser.serialize_field("userAttributeValues", &self.user_attribute_values)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserAttributeKeyValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_values",
            "userAttributeValues",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValues,
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
                            "userAttributeValues" | "user_attribute_values" => Ok(GeneratedField::UserAttributeValues),
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
            type Value = ListUserAttributeKeyValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ListUserAttributeKeyValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserAttributeKeyValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_values__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValues => {
                            if user_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValues"));
                            }
                            user_attribute_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUserAttributeKeyValuesResponse {
                    user_attribute_values: user_attribute_values__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ListUserAttributeKeyValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserAttributeKeysRequest {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.include_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ListUserAttributeKeysRequest", len)?;
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
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if self.include_archived {
            struct_ser.serialize_field("includeArchived", &self.include_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserAttributeKeysRequest {
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
            "organization_id",
            "organizationId",
            "include_archived",
            "includeArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrderBy,
            OrganizationId,
            IncludeArchived,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "includeArchived" | "include_archived" => Ok(GeneratedField::IncludeArchived),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUserAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ListUserAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut organization_id__ = None;
                let mut include_archived__ = None;
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
                    }
                }
                Ok(ListUserAttributeKeysRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ListUserAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_keys.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ListUserAttributeKeysResponse", len)?;
        if !self.user_attribute_keys.is_empty() {
            struct_ser.serialize_field("userAttributeKeys", &self.user_attribute_keys)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserAttributeKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_keys",
            "userAttributeKeys",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeys,
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
                            "userAttributeKeys" | "user_attribute_keys" => Ok(GeneratedField::UserAttributeKeys),
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
            type Value = ListUserAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ListUserAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_keys__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeys => {
                            if user_attribute_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeys"));
                            }
                            user_attribute_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUserAttributeKeysResponse {
                    user_attribute_keys: user_attribute_keys__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ListUserAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserAttributeValuesRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ListUserAttributeValuesRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListUserAttributeValuesRequest {
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
            type Value = ListUserAttributeValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ListUserAttributeValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserAttributeValuesRequest, V::Error>
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
                Ok(ListUserAttributeValuesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ListUserAttributeValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserAttributeValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_values.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.ListUserAttributeValuesResponse", len)?;
        if !self.user_attribute_values.is_empty() {
            struct_ser.serialize_field("userAttributeValues", &self.user_attribute_values)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserAttributeValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_values",
            "userAttributeValues",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValues,
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
                            "userAttributeValues" | "user_attribute_values" => Ok(GeneratedField::UserAttributeValues),
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
            type Value = ListUserAttributeValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.ListUserAttributeValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserAttributeValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_values__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValues => {
                            if user_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValues"));
                            }
                            user_attribute_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUserAttributeValuesResponse {
                    user_attribute_values: user_attribute_values__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.ListUserAttributeValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveUserAttributeKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeKeysRequest", len)?;
        if !self.user_attribute_key_ids.is_empty() {
            struct_ser.serialize_field("userAttributeKeyIds", &self.user_attribute_key_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveUserAttributeKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_ids",
            "userAttributeKeyIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyIds,
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
                            "userAttributeKeyIds" | "user_attribute_key_ids" => Ok(GeneratedField::UserAttributeKeyIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveUserAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UnarchiveUserAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveUserAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyIds => {
                            if user_attribute_key_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyIds"));
                            }
                            user_attribute_key_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchiveUserAttributeKeysRequest {
                    user_attribute_key_ids: user_attribute_key_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveUserAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeKeysResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveUserAttributeKeysResponse {
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
            type Value = UnarchiveUserAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UnarchiveUserAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveUserAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchiveUserAttributeKeysResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveUserAttributeValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_value_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeValuesRequest", len)?;
        if !self.user_attribute_value_ids.is_empty() {
            struct_ser.serialize_field("userAttributeValueIds", &self.user_attribute_value_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveUserAttributeValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_value_ids",
            "userAttributeValueIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValueIds,
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
                            "userAttributeValueIds" | "user_attribute_value_ids" => Ok(GeneratedField::UserAttributeValueIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveUserAttributeValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UnarchiveUserAttributeValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveUserAttributeValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_value_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValueIds => {
                            if user_attribute_value_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValueIds"));
                            }
                            user_attribute_value_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchiveUserAttributeValuesRequest {
                    user_attribute_value_ids: user_attribute_value_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveUserAttributeValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeValuesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveUserAttributeValuesResponse {
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
            type Value = UnarchiveUserAttributeValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UnarchiveUserAttributeValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveUserAttributeValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchiveUserAttributeValuesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UnarchiveUserAttributeValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UpdateUserAttributeKeyRequest", len)?;
        if !self.user_attribute_key_id.is_empty() {
            struct_ser.serialize_field("userAttributeKeyId", &self.user_attribute_key_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_id",
            "userAttributeKeyId",
            "name",
            "description",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyId,
            Name,
            Description,
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
                            "userAttributeKeyId" | "user_attribute_key_id" => Ok(GeneratedField::UserAttributeKeyId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
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
            type Value = UpdateUserAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UpdateUserAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyId => {
                            if user_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyId"));
                            }
                            user_attribute_key_id__ = Some(map_.next_value()?);
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
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUserAttributeKeyRequest {
                    user_attribute_key_id: user_attribute_key_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UpdateUserAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UpdateUserAttributeKeyResponse", len)?;
        if let Some(v) = self.user_attribute_key.as_ref() {
            struct_ser.serialize_field("userAttributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key",
            "userAttributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKey,
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
                            "userAttributeKey" | "user_attribute_key" => Ok(GeneratedField::UserAttributeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateUserAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UpdateUserAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKey => {
                            if user_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKey"));
                            }
                            user_attribute_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUserAttributeKeyResponse {
                    user_attribute_key: user_attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UpdateUserAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserAttributeKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UserAttributeKey", len)?;
        if !self.user_attribute_key_id.is_empty() {
            struct_ser.serialize_field("userAttributeKeyId", &self.user_attribute_key_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.r#type != 0 {
            let v = UserAttributeValueType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
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
impl<'de> serde::Deserialize<'de> for UserAttributeKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_key_id",
            "userAttributeKeyId",
            "organization_id",
            "organizationId",
            "name",
            "description",
            "type",
            "created_date",
            "createdDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_date",
            "modifiedDate",
            "modified_by_user_id",
            "modifiedByUserId",
            "archived_date",
            "archivedDate",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeKeyId,
            OrganizationId,
            Name,
            Description,
            Type,
            CreatedDate,
            CreatedByUserId,
            ModifiedDate,
            ModifiedByUserId,
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
                            "userAttributeKeyId" | "user_attribute_key_id" => Ok(GeneratedField::UserAttributeKeyId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "type" => Ok(GeneratedField::Type),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
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
            type Value = UserAttributeKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UserAttributeKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserAttributeKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_key_id__ = None;
                let mut organization_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut r#type__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_date__ = None;
                let mut modified_by_user_id__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeKeyId => {
                            if user_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyId"));
                            }
                            user_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<UserAttributeValueType>()? as i32);
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
                        GeneratedField::ModifiedDate => {
                            if modified_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedDate"));
                            }
                            modified_date__ = map_.next_value()?;
                        }
                        GeneratedField::ModifiedByUserId => {
                            if modified_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedByUserId"));
                            }
                            modified_by_user_id__ = Some(map_.next_value()?);
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
                Ok(UserAttributeKey {
                    user_attribute_key_id: user_attribute_key_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_date: modified_date__,
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UserAttributeKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserAttributeValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_attribute_value_id.is_empty() {
            len += 1;
        }
        if !self.user_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_attributes.v1.UserAttributeValue", len)?;
        if !self.user_attribute_value_id.is_empty() {
            struct_ser.serialize_field("userAttributeValueId", &self.user_attribute_value_id)?;
        }
        if !self.user_attribute_key_id.is_empty() {
            struct_ser.serialize_field("userAttributeKeyId", &self.user_attribute_key_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                user_attribute_value::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                user_attribute_value::Value::NumberValue(v) => {
                    struct_ser.serialize_field("numberValue", v)?;
                }
                user_attribute_value::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserAttributeValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_attribute_value_id",
            "userAttributeValueId",
            "user_attribute_key_id",
            "userAttributeKeyId",
            "user_id",
            "userId",
            "organization_id",
            "organizationId",
            "created_by_user_id",
            "createdByUserId",
            "created_date",
            "createdDate",
            "archived_date",
            "archivedDate",
            "is_archived",
            "isArchived",
            "key",
            "string_value",
            "stringValue",
            "number_value",
            "numberValue",
            "boolean_value",
            "booleanValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserAttributeValueId,
            UserAttributeKeyId,
            UserId,
            OrganizationId,
            CreatedByUserId,
            CreatedDate,
            ArchivedDate,
            IsArchived,
            Key,
            StringValue,
            NumberValue,
            BooleanValue,
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
                            "userAttributeValueId" | "user_attribute_value_id" => Ok(GeneratedField::UserAttributeValueId),
                            "userAttributeKeyId" | "user_attribute_key_id" => Ok(GeneratedField::UserAttributeKeyId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "isArchived" | "is_archived" => Ok(GeneratedField::IsArchived),
                            "key" => Ok(GeneratedField::Key),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "numberValue" | "number_value" => Ok(GeneratedField::NumberValue),
                            "booleanValue" | "boolean_value" => Ok(GeneratedField::BooleanValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserAttributeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_attributes.v1.UserAttributeValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserAttributeValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_attribute_value_id__ = None;
                let mut user_attribute_key_id__ = None;
                let mut user_id__ = None;
                let mut organization_id__ = None;
                let mut created_by_user_id__ = None;
                let mut created_date__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserAttributeValueId => {
                            if user_attribute_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeValueId"));
                            }
                            user_attribute_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserAttributeKeyId => {
                            if user_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAttributeKeyId"));
                            }
                            user_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedByUserId => {
                            if created_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdByUserId"));
                            }
                            created_by_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedDate => {
                            if created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdDate"));
                            }
                            created_date__ = map_.next_value()?;
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
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(user_attribute_value::Value::StringValue);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| user_attribute_value::Value::NumberValue(x.0));
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(user_attribute_value::Value::BooleanValue);
                        }
                    }
                }
                Ok(UserAttributeValue {
                    user_attribute_value_id: user_attribute_value_id__.unwrap_or_default(),
                    user_attribute_key_id: user_attribute_key_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_attributes.v1.UserAttributeValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserAttributeValueType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "USER_ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            Self::String => "USER_ATTRIBUTE_VALUE_TYPE_STRING",
            Self::Boolean => "USER_ATTRIBUTE_VALUE_TYPE_BOOLEAN",
            Self::Number => "USER_ATTRIBUTE_VALUE_TYPE_NUMBER",
            Self::SetOfString => "USER_ATTRIBUTE_VALUE_TYPE_SET_OF_STRING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UserAttributeValueType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USER_ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            "USER_ATTRIBUTE_VALUE_TYPE_STRING",
            "USER_ATTRIBUTE_VALUE_TYPE_BOOLEAN",
            "USER_ATTRIBUTE_VALUE_TYPE_NUMBER",
            "USER_ATTRIBUTE_VALUE_TYPE_SET_OF_STRING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserAttributeValueType;

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
                    "USER_ATTRIBUTE_VALUE_TYPE_UNSPECIFIED" => Ok(UserAttributeValueType::Unspecified),
                    "USER_ATTRIBUTE_VALUE_TYPE_STRING" => Ok(UserAttributeValueType::String),
                    "USER_ATTRIBUTE_VALUE_TYPE_BOOLEAN" => Ok(UserAttributeValueType::Boolean),
                    "USER_ATTRIBUTE_VALUE_TYPE_NUMBER" => Ok(UserAttributeValueType::Number),
                    "USER_ATTRIBUTE_VALUE_TYPE_SET_OF_STRING" => Ok(UserAttributeValueType::SetOfString),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
