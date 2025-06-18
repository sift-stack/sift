// @generated
impl serde::Serialize for IngestArbitraryProtobufDataStreamRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_type_identifier.is_empty() {
            len += 1;
        }
        if self.message_type_display_name.is_some() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.end_stream_on_validation_error {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingest.v1.IngestArbitraryProtobufDataStreamRequest", len)?;
        if !self.message_type_identifier.is_empty() {
            struct_ser.serialize_field("messageTypeIdentifier", &self.message_type_identifier)?;
        }
        if let Some(v) = self.message_type_display_name.as_ref() {
            struct_ser.serialize_field("messageTypeDisplayName", v)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.value.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if self.end_stream_on_validation_error {
            struct_ser.serialize_field("endStreamOnValidationError", &self.end_stream_on_validation_error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngestArbitraryProtobufDataStreamRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_type_identifier",
            "messageTypeIdentifier",
            "message_type_display_name",
            "messageTypeDisplayName",
            "asset_name",
            "assetName",
            "timestamp",
            "value",
            "run_id",
            "runId",
            "namespace",
            "organization_id",
            "organizationId",
            "end_stream_on_validation_error",
            "endStreamOnValidationError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageTypeIdentifier,
            MessageTypeDisplayName,
            AssetName,
            Timestamp,
            Value,
            RunId,
            Namespace,
            OrganizationId,
            EndStreamOnValidationError,
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
                            "messageTypeIdentifier" | "message_type_identifier" => Ok(GeneratedField::MessageTypeIdentifier),
                            "messageTypeDisplayName" | "message_type_display_name" => Ok(GeneratedField::MessageTypeDisplayName),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "value" => Ok(GeneratedField::Value),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "endStreamOnValidationError" | "end_stream_on_validation_error" => Ok(GeneratedField::EndStreamOnValidationError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngestArbitraryProtobufDataStreamRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingest.v1.IngestArbitraryProtobufDataStreamRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IngestArbitraryProtobufDataStreamRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_type_identifier__ = None;
                let mut message_type_display_name__ = None;
                let mut asset_name__ = None;
                let mut timestamp__ = None;
                let mut value__ = None;
                let mut run_id__ = None;
                let mut namespace__ = None;
                let mut organization_id__ = None;
                let mut end_stream_on_validation_error__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessageTypeIdentifier => {
                            if message_type_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageTypeIdentifier"));
                            }
                            message_type_identifier__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessageTypeDisplayName => {
                            if message_type_display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageTypeDisplayName"));
                            }
                            message_type_display_name__ = map_.next_value()?;
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndStreamOnValidationError => {
                            if end_stream_on_validation_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endStreamOnValidationError"));
                            }
                            end_stream_on_validation_error__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IngestArbitraryProtobufDataStreamRequest {
                    message_type_identifier: message_type_identifier__.unwrap_or_default(),
                    message_type_display_name: message_type_display_name__,
                    asset_name: asset_name__.unwrap_or_default(),
                    timestamp: timestamp__,
                    value: value__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    namespace: namespace__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    end_stream_on_validation_error: end_stream_on_validation_error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingest.v1.IngestArbitraryProtobufDataStreamRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngestArbitraryProtobufDataStreamResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.ingest.v1.IngestArbitraryProtobufDataStreamResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngestArbitraryProtobufDataStreamResponse {
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
            type Value = IngestArbitraryProtobufDataStreamResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingest.v1.IngestArbitraryProtobufDataStreamResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IngestArbitraryProtobufDataStreamResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(IngestArbitraryProtobufDataStreamResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.ingest.v1.IngestArbitraryProtobufDataStreamResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngestWithConfigDataChannelValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingest.v1.IngestWithConfigDataChannelValue", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                ingest_with_config_data_channel_value::Type::String(v) => {
                    struct_ser.serialize_field("string", v)?;
                }
                ingest_with_config_data_channel_value::Type::Double(v) => {
                    struct_ser.serialize_field("double", v)?;
                }
                ingest_with_config_data_channel_value::Type::Float(v) => {
                    struct_ser.serialize_field("float", v)?;
                }
                ingest_with_config_data_channel_value::Type::Bool(v) => {
                    struct_ser.serialize_field("bool", v)?;
                }
                ingest_with_config_data_channel_value::Type::Int32(v) => {
                    struct_ser.serialize_field("int32", v)?;
                }
                ingest_with_config_data_channel_value::Type::Uint32(v) => {
                    struct_ser.serialize_field("uint32", v)?;
                }
                ingest_with_config_data_channel_value::Type::Int64(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("int64", ToString::to_string(&v).as_str())?;
                }
                ingest_with_config_data_channel_value::Type::Uint64(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("uint64", ToString::to_string(&v).as_str())?;
                }
                ingest_with_config_data_channel_value::Type::BitField(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("bitField", pbjson::private::base64::encode(&v).as_str())?;
                }
                ingest_with_config_data_channel_value::Type::Enum(v) => {
                    struct_ser.serialize_field("enum", v)?;
                }
                ingest_with_config_data_channel_value::Type::Empty(v) => {
                    struct_ser.serialize_field("empty", v)?;
                }
                ingest_with_config_data_channel_value::Type::Bytes(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("bytes", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngestWithConfigDataChannelValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "string",
            "double",
            "float",
            "bool",
            "int32",
            "uint32",
            "int64",
            "uint64",
            "bit_field",
            "bitField",
            "enum",
            "empty",
            "bytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            String,
            Double,
            Float,
            Bool,
            Int32,
            Uint32,
            Int64,
            Uint64,
            BitField,
            Enum,
            Empty,
            Bytes,
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
                            "string" => Ok(GeneratedField::String),
                            "double" => Ok(GeneratedField::Double),
                            "float" => Ok(GeneratedField::Float),
                            "bool" => Ok(GeneratedField::Bool),
                            "int32" => Ok(GeneratedField::Int32),
                            "uint32" => Ok(GeneratedField::Uint32),
                            "int64" => Ok(GeneratedField::Int64),
                            "uint64" => Ok(GeneratedField::Uint64),
                            "bitField" | "bit_field" => Ok(GeneratedField::BitField),
                            "enum" => Ok(GeneratedField::Enum),
                            "empty" => Ok(GeneratedField::Empty),
                            "bytes" => Ok(GeneratedField::Bytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IngestWithConfigDataChannelValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingest.v1.IngestWithConfigDataChannelValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IngestWithConfigDataChannelValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::String => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("string"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(ingest_with_config_data_channel_value::Type::String);
                        }
                        GeneratedField::Double => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("double"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Double(x.0));
                        }
                        GeneratedField::Float => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("float"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Float(x.0));
                        }
                        GeneratedField::Bool => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bool"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(ingest_with_config_data_channel_value::Type::Bool);
                        }
                        GeneratedField::Int32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Int32(x.0));
                        }
                        GeneratedField::Uint32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Uint32(x.0));
                        }
                        GeneratedField::Int64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Int64(x.0));
                        }
                        GeneratedField::Uint64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Uint64(x.0));
                        }
                        GeneratedField::BitField => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitField"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::BitField(x.0));
                        }
                        GeneratedField::Enum => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enum"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Enum(x.0));
                        }
                        GeneratedField::Empty => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("empty"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(ingest_with_config_data_channel_value::Type::Empty)
;
                        }
                        GeneratedField::Bytes => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| ingest_with_config_data_channel_value::Type::Bytes(x.0));
                        }
                    }
                }
                Ok(IngestWithConfigDataChannelValue {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("sift.ingest.v1.IngestWithConfigDataChannelValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngestWithConfigDataStreamRequest {
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
        if !self.flow.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.channel_values.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        if self.end_stream_on_validation_error {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ingest.v1.IngestWithConfigDataStreamRequest", len)?;
        if !self.ingestion_config_id.is_empty() {
            struct_ser.serialize_field("ingestionConfigId", &self.ingestion_config_id)?;
        }
        if !self.flow.is_empty() {
            struct_ser.serialize_field("flow", &self.flow)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.channel_values.is_empty() {
            struct_ser.serialize_field("channelValues", &self.channel_values)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if self.end_stream_on_validation_error {
            struct_ser.serialize_field("endStreamOnValidationError", &self.end_stream_on_validation_error)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngestWithConfigDataStreamRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ingestion_config_id",
            "ingestionConfigId",
            "flow",
            "timestamp",
            "channel_values",
            "channelValues",
            "run_id",
            "runId",
            "end_stream_on_validation_error",
            "endStreamOnValidationError",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IngestionConfigId,
            Flow,
            Timestamp,
            ChannelValues,
            RunId,
            EndStreamOnValidationError,
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
                            "ingestionConfigId" | "ingestion_config_id" => Ok(GeneratedField::IngestionConfigId),
                            "flow" => Ok(GeneratedField::Flow),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "channelValues" | "channel_values" => Ok(GeneratedField::ChannelValues),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "endStreamOnValidationError" | "end_stream_on_validation_error" => Ok(GeneratedField::EndStreamOnValidationError),
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
            type Value = IngestWithConfigDataStreamRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingest.v1.IngestWithConfigDataStreamRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IngestWithConfigDataStreamRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ingestion_config_id__ = None;
                let mut flow__ = None;
                let mut timestamp__ = None;
                let mut channel_values__ = None;
                let mut run_id__ = None;
                let mut end_stream_on_validation_error__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IngestionConfigId => {
                            if ingestion_config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingestionConfigId"));
                            }
                            ingestion_config_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Flow => {
                            if flow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flow"));
                            }
                            flow__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::ChannelValues => {
                            if channel_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelValues"));
                            }
                            channel_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndStreamOnValidationError => {
                            if end_stream_on_validation_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endStreamOnValidationError"));
                            }
                            end_stream_on_validation_error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IngestWithConfigDataStreamRequest {
                    ingestion_config_id: ingestion_config_id__.unwrap_or_default(),
                    flow: flow__.unwrap_or_default(),
                    timestamp: timestamp__,
                    channel_values: channel_values__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    end_stream_on_validation_error: end_stream_on_validation_error__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ingest.v1.IngestWithConfigDataStreamRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IngestWithConfigDataStreamResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.ingest.v1.IngestWithConfigDataStreamResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IngestWithConfigDataStreamResponse {
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
            type Value = IngestWithConfigDataStreamResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ingest.v1.IngestWithConfigDataStreamResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IngestWithConfigDataStreamResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(IngestWithConfigDataStreamResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.ingest.v1.IngestWithConfigDataStreamResponse", FIELDS, GeneratedVisitor)
    }
}
