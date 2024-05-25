// @generated
impl serde::Serialize for ChannelBitFieldElement {
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
        if self.index != 0 {
            len += 1;
        }
        if self.bit_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ChannelBitFieldElement", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if self.bit_count != 0 {
            struct_ser.serialize_field("bitCount", &self.bit_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelBitFieldElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "index",
            "bit_count",
            "bitCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Index,
            BitCount,
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
                            "index" => Ok(GeneratedField::Index),
                            "bitCount" | "bit_count" => Ok(GeneratedField::BitCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelBitFieldElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.ChannelBitFieldElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelBitFieldElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut index__ = None;
                let mut bit_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BitCount => {
                            if bit_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitCount"));
                            }
                            bit_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ChannelBitFieldElement {
                    name: name__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    bit_count: bit_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ChannelBitFieldElement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelDataType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CHANNEL_DATA_TYPE_UNSPECIFIED",
            Self::Double => "CHANNEL_DATA_TYPE_DOUBLE",
            Self::String => "CHANNEL_DATA_TYPE_STRING",
            Self::Enum => "CHANNEL_DATA_TYPE_ENUM",
            Self::BitField => "CHANNEL_DATA_TYPE_BIT_FIELD",
            Self::Bool => "CHANNEL_DATA_TYPE_BOOL",
            Self::Float => "CHANNEL_DATA_TYPE_FLOAT",
            Self::Int32 => "CHANNEL_DATA_TYPE_INT_32",
            Self::Uint32 => "CHANNEL_DATA_TYPE_UINT_32",
            Self::Int64 => "CHANNEL_DATA_TYPE_INT_64",
            Self::Uint64 => "CHANNEL_DATA_TYPE_UINT_64",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ChannelDataType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CHANNEL_DATA_TYPE_UNSPECIFIED",
            "CHANNEL_DATA_TYPE_DOUBLE",
            "CHANNEL_DATA_TYPE_STRING",
            "CHANNEL_DATA_TYPE_ENUM",
            "CHANNEL_DATA_TYPE_BIT_FIELD",
            "CHANNEL_DATA_TYPE_BOOL",
            "CHANNEL_DATA_TYPE_FLOAT",
            "CHANNEL_DATA_TYPE_INT_32",
            "CHANNEL_DATA_TYPE_UINT_32",
            "CHANNEL_DATA_TYPE_INT_64",
            "CHANNEL_DATA_TYPE_UINT_64",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelDataType;

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
                    "CHANNEL_DATA_TYPE_UNSPECIFIED" => Ok(ChannelDataType::Unspecified),
                    "CHANNEL_DATA_TYPE_DOUBLE" => Ok(ChannelDataType::Double),
                    "CHANNEL_DATA_TYPE_STRING" => Ok(ChannelDataType::String),
                    "CHANNEL_DATA_TYPE_ENUM" => Ok(ChannelDataType::Enum),
                    "CHANNEL_DATA_TYPE_BIT_FIELD" => Ok(ChannelDataType::BitField),
                    "CHANNEL_DATA_TYPE_BOOL" => Ok(ChannelDataType::Bool),
                    "CHANNEL_DATA_TYPE_FLOAT" => Ok(ChannelDataType::Float),
                    "CHANNEL_DATA_TYPE_INT_32" => Ok(ChannelDataType::Int32),
                    "CHANNEL_DATA_TYPE_UINT_32" => Ok(ChannelDataType::Uint32),
                    "CHANNEL_DATA_TYPE_INT_64" => Ok(ChannelDataType::Int64),
                    "CHANNEL_DATA_TYPE_UINT_64" => Ok(ChannelDataType::Uint64),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ChannelEnumType {
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
        if self.key != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ChannelEnumType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.key != 0 {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelEnumType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelEnumType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.ChannelEnumType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelEnumType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ChannelEnumType {
                    name: name__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ChannelEnumType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Organization {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.organization_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.Organization", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.organization_name.is_empty() {
            struct_ser.serialize_field("organizationName", &self.organization_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Organization {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
            "organization_name",
            "organizationName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
            OrganizationName,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "organizationName" | "organization_name" => Ok(GeneratedField::OrganizationName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Organization;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.Organization")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Organization, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                let mut organization_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationName => {
                            if organization_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationName"));
                            }
                            organization_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Organization {
                    organization_id: organization_id__.unwrap_or_default(),
                    organization_name: organization_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.Organization", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
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
        if !self.user_name.is_empty() {
            len += 1;
        }
        if !self.organizations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.User", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if !self.organizations.is_empty() {
            struct_ser.serialize_field("organizations", &self.organizations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_name",
            "userName",
            "organizations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserName,
            Organizations,
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "organizations" => Ok(GeneratedField::Organizations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_name__ = None;
                let mut organizations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Organizations => {
                            if organizations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizations"));
                            }
                            organizations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(User {
                    user_id: user_id__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                    organizations: organizations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.User", FIELDS, GeneratedVisitor)
    }
}
