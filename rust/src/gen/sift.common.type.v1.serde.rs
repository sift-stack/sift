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
