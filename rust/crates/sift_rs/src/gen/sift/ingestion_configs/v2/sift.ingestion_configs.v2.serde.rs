// @generated
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
        if !self.unit.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.ChannelConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
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
            "unit",
            "description",
            "data_type",
            "dataType",
            "enum_types",
            "enumTypes",
            "bit_field_elements",
            "bitFieldElements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Unit,
            Description,
            DataType,
            EnumTypes,
            BitFieldElements,
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
                            "unit" => Ok(GeneratedField::Unit),
                            "description" => Ok(GeneratedField::Description),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "enumTypes" | "enum_types" => Ok(GeneratedField::EnumTypes),
                            "bitFieldElements" | "bit_field_elements" => Ok(GeneratedField::BitFieldElements),
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
                formatter.write_str("struct sift.ingestion_configs.v2.ChannelConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut unit__ = None;
                let mut description__ = None;
                let mut data_type__ = None;
                let mut enum_types__ = None;
                let mut bit_field_elements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(ChannelConfig {
                    name: name__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    enum_types: enum_types__.unwrap_or_default(),
                    bit_field_elements: bit_field_elements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.ChannelConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateIngestionConfigFlowsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ingestion_config_id.is_empty() {
            len += 1;
        }
        if !self.flows.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigFlowsRequest", len)?;
        if !self.ingestion_config_id.is_empty() {
            struct_ser.serialize_field("ingestionConfigId", &self.ingestion_config_id)?;
        }
        if !self.flows.is_empty() {
            struct_ser.serialize_field("flows", &self.flows)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateIngestionConfigFlowsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_config_id",
            "ingestionConfigId",
            "flows",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfigId,
            Flows,
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
                            "ingestionConfigId" | "ingestion_config_id" => Ok(GeneratedField::IngestionConfigId),
                            "flows" => Ok(GeneratedField::Flows),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateIngestionConfigFlowsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.CreateIngestionConfigFlowsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateIngestionConfigFlowsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_config_id__ = None;
                let mut flows__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfigId => {
                            if ingestion_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfigId"));
                            }
                            ingestion_config_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Flows => {
                            if flows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flows"));
                            }
                            flows__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateIngestionConfigFlowsRequest {
                    ingestion_config_id: ingestion_config_id__.unwrap_or_default(),
                    flows: flows__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigFlowsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateIngestionConfigFlowsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigFlowsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateIngestionConfigFlowsResponse {
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
            type Value = CreateIngestionConfigFlowsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.CreateIngestionConfigFlowsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateIngestionConfigFlowsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CreateIngestionConfigFlowsResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigFlowsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateIngestionConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.flows.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.client_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigRequest", len)?;
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.flows.is_empty() {
            struct_ser.serialize_field("flows", &self.flows)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateIngestionConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_name",
            "assetName",
            "flows",
            "organization_id",
            "organizationId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetName,
            Flows,
            OrganizationId,
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
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "flows" => Ok(GeneratedField::Flows),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
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
            type Value = CreateIngestionConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.CreateIngestionConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateIngestionConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_name__ = None;
                let mut flows__ = None;
                let mut organization_id__ = None;
                let mut client_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Flows => {
                            if flows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flows"));
                            }
                            flows__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateIngestionConfigRequest {
                    asset_name: asset_name__.unwrap_or_default(),
                    flows: flows__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateIngestionConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ingestion_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigResponse", len)?;
        if let Some(v) = self.ingestion_config.as_ref() {
            struct_ser.serialize_field("ingestionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateIngestionConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_config",
            "ingestionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfig,
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
                            "ingestionConfig" | "ingestion_config" => Ok(GeneratedField::IngestionConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateIngestionConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.CreateIngestionConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateIngestionConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfig => {
                            if ingestion_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfig"));
                            }
                            ingestion_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateIngestionConfigResponse {
                    ingestion_config: ingestion_config__,
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.CreateIngestionConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FlowConfig {
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
        if !self.channels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.FlowConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlowConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "channels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
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
            type Value = FlowConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.FlowConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FlowConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FlowConfig {
                    name: name__.unwrap_or_default(),
                    channels: channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.FlowConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetIngestionConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ingestion_config_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.GetIngestionConfigRequest", len)?;
        if !self.ingestion_config_id.is_empty() {
            struct_ser.serialize_field("ingestionConfigId", &self.ingestion_config_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetIngestionConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_config_id",
            "ingestionConfigId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfigId,
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
                            "ingestionConfigId" | "ingestion_config_id" => Ok(GeneratedField::IngestionConfigId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetIngestionConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.GetIngestionConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetIngestionConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_config_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfigId => {
                            if ingestion_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfigId"));
                            }
                            ingestion_config_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetIngestionConfigRequest {
                    ingestion_config_id: ingestion_config_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.GetIngestionConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetIngestionConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ingestion_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.GetIngestionConfigResponse", len)?;
        if let Some(v) = self.ingestion_config.as_ref() {
            struct_ser.serialize_field("ingestionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetIngestionConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_config",
            "ingestionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfig,
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
                            "ingestionConfig" | "ingestion_config" => Ok(GeneratedField::IngestionConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetIngestionConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.GetIngestionConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetIngestionConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfig => {
                            if ingestion_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfig"));
                            }
                            ingestion_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetIngestionConfigResponse {
                    ingestion_config: ingestion_config__,
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.GetIngestionConfigResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngestionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ingestion_config_id.is_empty() {
            len += 1;
        }
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.client_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.IngestionConfig", len)?;
        if !self.ingestion_config_id.is_empty() {
            struct_ser.serialize_field("ingestionConfigId", &self.ingestion_config_id)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngestionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_config_id",
            "ingestionConfigId",
            "asset_id",
            "assetId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfigId,
            AssetId,
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
                            "ingestionConfigId" | "ingestion_config_id" => Ok(GeneratedField::IngestionConfigId),
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
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
            type Value = IngestionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.IngestionConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IngestionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_config_id__ = None;
                let mut asset_id__ = None;
                let mut client_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfigId => {
                            if ingestion_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfigId"));
                            }
                            ingestion_config_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IngestionConfig {
                    ingestion_config_id: ingestion_config_id__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.IngestionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListIngestionConfigFlowsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ingestion_config_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.ListIngestionConfigFlowsRequest", len)?;
        if !self.ingestion_config_id.is_empty() {
            struct_ser.serialize_field("ingestionConfigId", &self.ingestion_config_id)?;
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
impl<'de> serde::Deserialize<'de> for ListIngestionConfigFlowsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_config_id",
            "ingestionConfigId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfigId,
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
                            "ingestionConfigId" | "ingestion_config_id" => Ok(GeneratedField::IngestionConfigId),
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
            type Value = ListIngestionConfigFlowsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.ListIngestionConfigFlowsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListIngestionConfigFlowsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_config_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfigId => {
                            if ingestion_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfigId"));
                            }
                            ingestion_config_id__ = Some(map_.next_value()?);
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
                Ok(ListIngestionConfigFlowsRequest {
                    ingestion_config_id: ingestion_config_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.ListIngestionConfigFlowsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListIngestionConfigFlowsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.flows.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.ListIngestionConfigFlowsResponse", len)?;
        if !self.flows.is_empty() {
            struct_ser.serialize_field("flows", &self.flows)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListIngestionConfigFlowsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "flows",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Flows,
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
                            "flows" => Ok(GeneratedField::Flows),
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
            type Value = ListIngestionConfigFlowsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.ListIngestionConfigFlowsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListIngestionConfigFlowsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut flows__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Flows => {
                            if flows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flows"));
                            }
                            flows__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListIngestionConfigFlowsResponse {
                    flows: flows__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.ListIngestionConfigFlowsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListIngestionConfigsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.ListIngestionConfigsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListIngestionConfigsRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListIngestionConfigsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.ListIngestionConfigsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListIngestionConfigsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
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
                    }
                }
                Ok(ListIngestionConfigsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.ListIngestionConfigsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListIngestionConfigsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ingestion_configs.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingestion_configs.v2.ListIngestionConfigsResponse", len)?;
        if !self.ingestion_configs.is_empty() {
            struct_ser.serialize_field("ingestionConfigs", &self.ingestion_configs)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListIngestionConfigsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_configs",
            "ingestionConfigs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfigs,
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
                            "ingestionConfigs" | "ingestion_configs" => Ok(GeneratedField::IngestionConfigs),
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
            type Value = ListIngestionConfigsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingestion_configs.v2.ListIngestionConfigsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListIngestionConfigsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_configs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfigs => {
                            if ingestion_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfigs"));
                            }
                            ingestion_configs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListIngestionConfigsResponse {
                    ingestion_configs: ingestion_configs__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingestion_configs.v2.ListIngestionConfigsResponse", FIELDS, GeneratedVisitor)
    }
}
