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
impl serde::Serialize for ChannelConfig {
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
        if !self.units.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.data_type != 0 {
            len += 1;
        }
        if !self.enum_types.is_empty() {
            len += 1;
        }
        if !self.bit_field_elements.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ChannelConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.units.is_empty() {
            struct_ser.serialize_field("units", &self.units)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.data_type != 0 {
            let v = ChannelDataType::try_from(self.data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        if !self.enum_types.is_empty() {
            struct_ser.serialize_field("enumTypes", &self.enum_types)?;
        }
        if !self.bit_field_elements.is_empty() {
            struct_ser.serialize_field("bitFieldElements", &self.bit_field_elements)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "units",
            "description",
            "data_type",
            "dataType",
            "enum_types",
            "enumTypes",
            "bit_field_elements",
            "bitFieldElements",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Units,
            Description,
            DataType,
            EnumTypes,
            BitFieldElements,
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
                            "units" => Ok(GeneratedField::Units),
                            "description" => Ok(GeneratedField::Description),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "enumTypes" | "enum_types" => Ok(GeneratedField::EnumTypes),
                            "bitFieldElements" | "bit_field_elements" => Ok(GeneratedField::BitFieldElements),
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
            type Value = ChannelConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.ChannelConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut units__ = None;
                let mut description__ = None;
                let mut data_type__ = None;
                let mut enum_types__ = None;
                let mut bit_field_elements__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Units => {
                            if units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("units"));
                            }
                            units__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value::<ChannelDataType>()? as i32);
                        }
                        GeneratedField::EnumTypes => {
                            if enum_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumTypes"));
                            }
                            enum_types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BitFieldElements => {
                            if bit_field_elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldElements"));
                            }
                            bit_field_elements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ChannelConfig {
                    name: name__.unwrap_or_default(),
                    units: units__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    enum_types: enum_types__.unwrap_or_default(),
                    bit_field_elements: bit_field_elements__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ChannelConfig", FIELDS, GeneratedVisitor)
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
            Self::Bytes => "CHANNEL_DATA_TYPE_BYTES",
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
            "CHANNEL_DATA_TYPE_BYTES",
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
                    "CHANNEL_DATA_TYPE_BYTES" => Ok(ChannelDataType::Bytes),
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
        if self.is_signed {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ChannelEnumType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.key != 0 {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.is_signed {
            struct_ser.serialize_field("isSigned", &self.is_signed)?;
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
            "is_signed",
            "isSigned",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Key,
            IsSigned,
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
                            "isSigned" | "is_signed" => Ok(GeneratedField::IsSigned),
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
                let mut is_signed__ = None;
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
                        GeneratedField::IsSigned => {
                            if is_signed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSigned"));
                            }
                            is_signed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ChannelEnumType {
                    name: name__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    is_signed: is_signed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ChannelEnumType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientKeys {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ClientKeys", len)?;
        if !self.client_keys.is_empty() {
            struct_ser.serialize_field("clientKeys", &self.client_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientKeys {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_keys",
            "clientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ClientKeys;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.ClientKeys")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientKeys, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientKeys => {
                            if client_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKeys"));
                            }
                            client_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClientKeys {
                    client_keys: client_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ClientKeys", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FindResourceBy {
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
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.FindResourceBy", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.identifier.as_ref() {
            match v {
                find_resource_by::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                find_resource_by::Identifier::ClientKey(v) => {
                    struct_ser.serialize_field("clientKey", v)?;
                }
                find_resource_by::Identifier::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FindResourceBy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
            "id",
            "client_key",
            "clientKey",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
            Id,
            ClientKey,
            Name,
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
                            "id" => Ok(GeneratedField::Id),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FindResourceBy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.FindResourceBy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FindResourceBy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(find_resource_by::Identifier::Id);
                        }
                        GeneratedField::ClientKey => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(find_resource_by::Identifier::ClientKey);
                        }
                        GeneratedField::Name => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(find_resource_by::Identifier::Name);
                        }
                    }
                }
                Ok(FindResourceBy {
                    organization_id: organization_id__.unwrap_or_default(),
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.FindResourceBy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionDataType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FUNCTION_DATA_TYPE_UNSPECIFIED",
            Self::Numeric => "FUNCTION_DATA_TYPE_NUMERIC",
            Self::String => "FUNCTION_DATA_TYPE_STRING",
            Self::Bool => "FUNCTION_DATA_TYPE_BOOL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FunctionDataType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FUNCTION_DATA_TYPE_UNSPECIFIED",
            "FUNCTION_DATA_TYPE_NUMERIC",
            "FUNCTION_DATA_TYPE_STRING",
            "FUNCTION_DATA_TYPE_BOOL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionDataType;

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
                    "FUNCTION_DATA_TYPE_UNSPECIFIED" => Ok(FunctionDataType::Unspecified),
                    "FUNCTION_DATA_TYPE_NUMERIC" => Ok(FunctionDataType::Numeric),
                    "FUNCTION_DATA_TYPE_STRING" => Ok(FunctionDataType::String),
                    "FUNCTION_DATA_TYPE_BOOL" => Ok(FunctionDataType::Bool),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionDependency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_function_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.FunctionDependency", len)?;
        if !self.user_defined_function_version_id.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionVersionId", &self.user_defined_function_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionDependency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function_version_id",
            "userDefinedFunctionVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctionVersionId,
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
                            "userDefinedFunctionVersionId" | "user_defined_function_version_id" => Ok(GeneratedField::UserDefinedFunctionVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionDependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.FunctionDependency")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionDependency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctionVersionId => {
                            if user_defined_function_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionVersionId"));
                            }
                            user_defined_function_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionDependency {
                    user_defined_function_version_id: user_defined_function_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.FunctionDependency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FunctionInput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.identifier.is_empty() {
            len += 1;
        }
        if self.data_type != 0 {
            len += 1;
        }
        if self.constant {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.FunctionInput", len)?;
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
        }
        if self.data_type != 0 {
            let v = FunctionDataType::try_from(self.data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        if self.constant {
            struct_ser.serialize_field("constant", &self.constant)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FunctionInput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifier",
            "data_type",
            "dataType",
            "constant",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
            DataType,
            Constant,
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
                            "identifier" => Ok(GeneratedField::Identifier),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "constant" => Ok(GeneratedField::Constant),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FunctionInput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.FunctionInput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FunctionInput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                let mut data_type__ = None;
                let mut constant__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value::<FunctionDataType>()? as i32);
                        }
                        GeneratedField::Constant => {
                            if constant__.is_some() {
                                return Err(serde::de::Error::duplicate_field("constant"));
                            }
                            constant__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FunctionInput {
                    identifier: identifier__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    constant: constant__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.FunctionInput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Ids {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.Ids", len)?;
        if !self.ids.is_empty() {
            struct_ser.serialize_field("ids", &self.ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ids {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ids",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
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
                            "ids" => Ok(GeneratedField::Ids),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ids;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.Ids")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Ids, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Ids {
                    ids: ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.Ids", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NamedResource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.NamedResource", len)?;
        if let Some(v) = self.resource.as_ref() {
            match v {
                named_resource::Resource::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                named_resource::Resource::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NamedResource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
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
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NamedResource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.NamedResource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NamedResource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(named_resource::Resource::Id);
                        }
                        GeneratedField::Name => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            resource__ = map_.next_value::<::std::option::Option<_>>()?.map(named_resource::Resource::Name);
                        }
                    }
                }
                Ok(NamedResource {
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.NamedResource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NamedResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resources.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.NamedResources", len)?;
        if let Some(v) = self.resources.as_ref() {
            match v {
                named_resources::Resources::Ids(v) => {
                    struct_ser.serialize_field("ids", v)?;
                }
                named_resources::Resources::Names(v) => {
                    struct_ser.serialize_field("names", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NamedResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ids",
            "names",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
            Names,
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
                            "ids" => Ok(GeneratedField::Ids),
                            "names" => Ok(GeneratedField::Names),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NamedResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.NamedResources")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NamedResources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            resources__ = map_.next_value::<::std::option::Option<_>>()?.map(named_resources::Resources::Ids)
;
                        }
                        GeneratedField::Names => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("names"));
                            }
                            resources__ = map_.next_value::<::std::option::Option<_>>()?.map(named_resources::Resources::Names)
;
                        }
                    }
                }
                Ok(NamedResources {
                    resources: resources__,
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.NamedResources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Names {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.Names", len)?;
        if !self.names.is_empty() {
            struct_ser.serialize_field("names", &self.names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Names {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "names",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Names,
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
                            "names" => Ok(GeneratedField::Names),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Names;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.Names")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Names, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut names__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Names => {
                            if names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("names"));
                            }
                            names__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Names {
                    names: names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.Names", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for ResourceIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ResourceIdentifier", len)?;
        if let Some(v) = self.identifier.as_ref() {
            match v {
                resource_identifier::Identifier::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                resource_identifier::Identifier::ClientKey(v) => {
                    struct_ser.serialize_field("clientKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
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
            type Value = ResourceIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.ResourceIdentifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_identifier::Identifier::Id);
                        }
                        GeneratedField::ClientKey => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_identifier::Identifier::ClientKey);
                        }
                    }
                }
                Ok(ResourceIdentifier {
                    identifier: identifier__,
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ResourceIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceIdentifiers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifiers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ResourceIdentifiers", len)?;
        if let Some(v) = self.identifiers.as_ref() {
            match v {
                resource_identifiers::Identifiers::Ids(v) => {
                    struct_ser.serialize_field("ids", v)?;
                }
                resource_identifiers::Identifiers::ClientKeys(v) => {
                    struct_ser.serialize_field("clientKeys", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceIdentifiers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ids",
            "client_keys",
            "clientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ids,
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
                            "ids" => Ok(GeneratedField::Ids),
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
            type Value = ResourceIdentifiers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.ResourceIdentifiers")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceIdentifiers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifiers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ids => {
                            if identifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ids"));
                            }
                            identifiers__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_identifiers::Identifiers::Ids)
;
                        }
                        GeneratedField::ClientKeys => {
                            if identifiers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKeys"));
                            }
                            identifiers__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_identifiers::Identifiers::ClientKeys)
;
                        }
                    }
                }
                Ok(ResourceIdentifiers {
                    identifiers: identifiers__,
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ResourceIdentifiers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.client_key.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.find_by.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.ResourceRef", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.client_key.as_ref() {
            struct_ser.serialize_field("clientKey", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.find_by.as_ref() {
            struct_ser.serialize_field("findBy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "client_key",
            "clientKey",
            "name",
            "find_by",
            "findBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ClientKey,
            Name,
            FindBy,
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
                            "id" => Ok(GeneratedField::Id),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "name" => Ok(GeneratedField::Name),
                            "findBy" | "find_by" => Ok(GeneratedField::FindBy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.ResourceRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut client_key__ = None;
                let mut name__ = None;
                let mut find_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = map_.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FindBy => {
                            if find_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("findBy"));
                            }
                            find_by__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResourceRef {
                    id: id__.unwrap_or_default(),
                    client_key: client_key__,
                    name: name__.unwrap_or_default(),
                    find_by: find_by__,
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.ResourceRef", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for UserDefinedFunction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_function_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if !self.user_defined_function_version_id.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.change_message.is_empty() {
            len += 1;
        }
        if !self.user_notes.is_empty() {
            len += 1;
        }
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.function_inputs.is_empty() {
            len += 1;
        }
        if self.function_output_type != 0 {
            len += 1;
        }
        if !self.function_dependencies.is_empty() {
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
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.common.r#type.v1.UserDefinedFunction", len)?;
        if !self.user_defined_function_id.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionId", &self.user_defined_function_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if !self.user_defined_function_version_id.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionVersionId", &self.user_defined_function_version_id)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.change_message.is_empty() {
            struct_ser.serialize_field("changeMessage", &self.change_message)?;
        }
        if !self.user_notes.is_empty() {
            struct_ser.serialize_field("userNotes", &self.user_notes)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.function_inputs.is_empty() {
            struct_ser.serialize_field("functionInputs", &self.function_inputs)?;
        }
        if self.function_output_type != 0 {
            let v = FunctionDataType::try_from(self.function_output_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.function_output_type)))?;
            struct_ser.serialize_field("functionOutputType", &v)?;
        }
        if !self.function_dependencies.is_empty() {
            struct_ser.serialize_field("functionDependencies", &self.function_dependencies)?;
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
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserDefinedFunction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function_id",
            "userDefinedFunctionId",
            "name",
            "archived_date",
            "archivedDate",
            "user_defined_function_version_id",
            "userDefinedFunctionVersionId",
            "version",
            "description",
            "change_message",
            "changeMessage",
            "user_notes",
            "userNotes",
            "expression",
            "function_inputs",
            "functionInputs",
            "function_output_type",
            "functionOutputType",
            "function_dependencies",
            "functionDependencies",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "metadata",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctionId,
            Name,
            ArchivedDate,
            UserDefinedFunctionVersionId,
            Version,
            Description,
            ChangeMessage,
            UserNotes,
            Expression,
            FunctionInputs,
            FunctionOutputType,
            FunctionDependencies,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
            Metadata,
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
                            "userDefinedFunctionId" | "user_defined_function_id" => Ok(GeneratedField::UserDefinedFunctionId),
                            "name" => Ok(GeneratedField::Name),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "userDefinedFunctionVersionId" | "user_defined_function_version_id" => Ok(GeneratedField::UserDefinedFunctionVersionId),
                            "version" => Ok(GeneratedField::Version),
                            "description" => Ok(GeneratedField::Description),
                            "changeMessage" | "change_message" => Ok(GeneratedField::ChangeMessage),
                            "userNotes" | "user_notes" => Ok(GeneratedField::UserNotes),
                            "expression" => Ok(GeneratedField::Expression),
                            "functionInputs" | "function_inputs" => Ok(GeneratedField::FunctionInputs),
                            "functionOutputType" | "function_output_type" => Ok(GeneratedField::FunctionOutputType),
                            "functionDependencies" | "function_dependencies" => Ok(GeneratedField::FunctionDependencies),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "metadata" => Ok(GeneratedField::Metadata),
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
            type Value = UserDefinedFunction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.common.r#type.v1.UserDefinedFunction")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserDefinedFunction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function_id__ = None;
                let mut name__ = None;
                let mut archived_date__ = None;
                let mut user_defined_function_version_id__ = None;
                let mut version__ = None;
                let mut description__ = None;
                let mut change_message__ = None;
                let mut user_notes__ = None;
                let mut expression__ = None;
                let mut function_inputs__ = None;
                let mut function_output_type__ = None;
                let mut function_dependencies__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut metadata__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctionId => {
                            if user_defined_function_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionId"));
                            }
                            user_defined_function_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                        GeneratedField::UserDefinedFunctionVersionId => {
                            if user_defined_function_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionVersionId"));
                            }
                            user_defined_function_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChangeMessage => {
                            if change_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeMessage"));
                            }
                            change_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserNotes => {
                            if user_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNotes"));
                            }
                            user_notes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionInputs => {
                            if function_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionInputs"));
                            }
                            function_inputs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionOutputType => {
                            if function_output_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionOutputType"));
                            }
                            function_output_type__ = Some(map_.next_value::<FunctionDataType>()? as i32);
                        }
                        GeneratedField::FunctionDependencies => {
                            if function_dependencies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionDependencies"));
                            }
                            function_dependencies__ = Some(map_.next_value()?);
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
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserDefinedFunction {
                    user_defined_function_id: user_defined_function_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    archived_date: archived_date__,
                    user_defined_function_version_id: user_defined_function_version_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    change_message: change_message__.unwrap_or_default(),
                    user_notes: user_notes__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                    function_inputs: function_inputs__.unwrap_or_default(),
                    function_output_type: function_output_type__.unwrap_or_default(),
                    function_dependencies: function_dependencies__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.common.r#type.v1.UserDefinedFunction", FIELDS, GeneratedVisitor)
    }
}
