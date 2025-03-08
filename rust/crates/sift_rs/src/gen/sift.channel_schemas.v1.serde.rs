// @generated
impl serde::Serialize for BatchCreateChannelSchemasRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channel_schemas.v1.BatchCreateChannelSchemasRequest", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateChannelSchemasRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
            "asset_name",
            "assetName",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
            AssetName,
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
                            "requests" => Ok(GeneratedField::Requests),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
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
            type Value = BatchCreateChannelSchemasRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channel_schemas.v1.BatchCreateChannelSchemasRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateChannelSchemasRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                let mut asset_name__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateChannelSchemasRequest {
                    requests: requests__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channel_schemas.v1.BatchCreateChannelSchemasRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateChannelSchemasResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.channel_schemas.v1.BatchCreateChannelSchemasResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateChannelSchemasResponse {
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
            type Value = BatchCreateChannelSchemasResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channel_schemas.v1.BatchCreateChannelSchemasResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateChannelSchemasResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchCreateChannelSchemasResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.channel_schemas.v1.BatchCreateChannelSchemasResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateChannelSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.component.is_empty() {
            len += 1;
        }
        if !self.channel.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
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
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channel_schemas.v1.CreateChannelSchemaRequest", len)?;
        if !self.component.is_empty() {
            struct_ser.serialize_field("component", &self.component)?;
        }
        if !self.channel.is_empty() {
            struct_ser.serialize_field("channel", &self.channel)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if self.data_type != 0 {
            let v = super::super::common::r#type::v1::ChannelDataType::try_from(self.data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        if !self.enum_types.is_empty() {
            struct_ser.serialize_field("enumTypes", &self.enum_types)?;
        }
        if !self.bit_field_elements.is_empty() {
            struct_ser.serialize_field("bitFieldElements", &self.bit_field_elements)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateChannelSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "component",
            "channel",
            "unit",
            "data_type",
            "dataType",
            "enum_types",
            "enumTypes",
            "bit_field_elements",
            "bitFieldElements",
            "asset_name",
            "assetName",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Component,
            Channel,
            Unit,
            DataType,
            EnumTypes,
            BitFieldElements,
            AssetName,
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
                            "component" => Ok(GeneratedField::Component),
                            "channel" => Ok(GeneratedField::Channel),
                            "unit" => Ok(GeneratedField::Unit),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "enumTypes" | "enum_types" => Ok(GeneratedField::EnumTypes),
                            "bitFieldElements" | "bit_field_elements" => Ok(GeneratedField::BitFieldElements),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
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
            type Value = CreateChannelSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channel_schemas.v1.CreateChannelSchemaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateChannelSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut component__ = None;
                let mut channel__ = None;
                let mut unit__ = None;
                let mut data_type__ = None;
                let mut enum_types__ = None;
                let mut bit_field_elements__ = None;
                let mut asset_name__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Component => {
                            if component__.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            component__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value::<super::super::common::r#type::v1::ChannelDataType>()? as i32);
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
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateChannelSchemaRequest {
                    component: component__.unwrap_or_default(),
                    channel: channel__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    enum_types: enum_types__.unwrap_or_default(),
                    bit_field_elements: bit_field_elements__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channel_schemas.v1.CreateChannelSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateChannelSchemaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.channel_schemas.v1.CreateChannelSchemaResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateChannelSchemaResponse {
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
            type Value = CreateChannelSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channel_schemas.v1.CreateChannelSchemaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateChannelSchemaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CreateChannelSchemaResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.channel_schemas.v1.CreateChannelSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
