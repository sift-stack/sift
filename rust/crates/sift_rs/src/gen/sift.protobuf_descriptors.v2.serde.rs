// @generated
impl serde::Serialize for AddProtobufDescriptorRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protobuf_descriptor.is_some() {
            len += 1;
        }
        if self.force_duplicate_registration {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.AddProtobufDescriptorRequest", len)?;
        if let Some(v) = self.protobuf_descriptor.as_ref() {
            struct_ser.serialize_field("protobufDescriptor", v)?;
        }
        if self.force_duplicate_registration {
            struct_ser.serialize_field("forceDuplicateRegistration", &self.force_duplicate_registration)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddProtobufDescriptorRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protobuf_descriptor",
            "protobufDescriptor",
            "force_duplicate_registration",
            "forceDuplicateRegistration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtobufDescriptor,
            ForceDuplicateRegistration,
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
                            "protobufDescriptor" | "protobuf_descriptor" => Ok(GeneratedField::ProtobufDescriptor),
                            "forceDuplicateRegistration" | "force_duplicate_registration" => Ok(GeneratedField::ForceDuplicateRegistration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddProtobufDescriptorRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.AddProtobufDescriptorRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddProtobufDescriptorRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protobuf_descriptor__ = None;
                let mut force_duplicate_registration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtobufDescriptor => {
                            if protobuf_descriptor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufDescriptor"));
                            }
                            protobuf_descriptor__ = map_.next_value()?;
                        }
                        GeneratedField::ForceDuplicateRegistration => {
                            if force_duplicate_registration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forceDuplicateRegistration"));
                            }
                            force_duplicate_registration__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddProtobufDescriptorRequest {
                    protobuf_descriptor: protobuf_descriptor__,
                    force_duplicate_registration: force_duplicate_registration__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.AddProtobufDescriptorRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddProtobufDescriptorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protobuf_descriptor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.AddProtobufDescriptorResponse", len)?;
        if let Some(v) = self.protobuf_descriptor.as_ref() {
            struct_ser.serialize_field("protobufDescriptor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddProtobufDescriptorResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protobuf_descriptor",
            "protobufDescriptor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtobufDescriptor,
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
                            "protobufDescriptor" | "protobuf_descriptor" => Ok(GeneratedField::ProtobufDescriptor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddProtobufDescriptorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.AddProtobufDescriptorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddProtobufDescriptorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protobuf_descriptor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtobufDescriptor => {
                            if protobuf_descriptor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufDescriptor"));
                            }
                            protobuf_descriptor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddProtobufDescriptorResponse {
                    protobuf_descriptor: protobuf_descriptor__,
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.AddProtobufDescriptorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArrayIndexOverrideType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ArrayIndexOverrideUnspecified => "ARRAY_INDEX_OVERRIDE_UNSPECIFIED",
            Self::ArrayIndexOverrideTarget => "ARRAY_INDEX_OVERRIDE_TARGET",
            Self::ArrayIndexOverrideSource => "ARRAY_INDEX_OVERRIDE_SOURCE",
            Self::ArrayIndexOverrideRemoveIndex => "ARRAY_INDEX_OVERRIDE_REMOVE_INDEX",
            Self::ArrayIndexOverrideEnum => "ARRAY_INDEX_OVERRIDE_ENUM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ArrayIndexOverrideType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ARRAY_INDEX_OVERRIDE_UNSPECIFIED",
            "ARRAY_INDEX_OVERRIDE_TARGET",
            "ARRAY_INDEX_OVERRIDE_SOURCE",
            "ARRAY_INDEX_OVERRIDE_REMOVE_INDEX",
            "ARRAY_INDEX_OVERRIDE_ENUM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArrayIndexOverrideType;

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
                    "ARRAY_INDEX_OVERRIDE_UNSPECIFIED" => Ok(ArrayIndexOverrideType::ArrayIndexOverrideUnspecified),
                    "ARRAY_INDEX_OVERRIDE_TARGET" => Ok(ArrayIndexOverrideType::ArrayIndexOverrideTarget),
                    "ARRAY_INDEX_OVERRIDE_SOURCE" => Ok(ArrayIndexOverrideType::ArrayIndexOverrideSource),
                    "ARRAY_INDEX_OVERRIDE_REMOVE_INDEX" => Ok(ArrayIndexOverrideType::ArrayIndexOverrideRemoveIndex),
                    "ARRAY_INDEX_OVERRIDE_ENUM" => Ok(ArrayIndexOverrideType::ArrayIndexOverrideEnum),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for BytesDecodingType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "BYTES_DECODING_TYPE_UNSPECIFIED",
            Self::Utf8 => "BYTES_DECODING_TYPE_UTF8",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for BytesDecodingType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BYTES_DECODING_TYPE_UNSPECIFIED",
            "BYTES_DECODING_TYPE_UTF8",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BytesDecodingType;

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
                    "BYTES_DECODING_TYPE_UNSPECIFIED" => Ok(BytesDecodingType::Unspecified),
                    "BYTES_DECODING_TYPE_UTF8" => Ok(BytesDecodingType::Utf8),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CheckProtobufDescriptorCompatibilityRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protobuf_descriptor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.CheckProtobufDescriptorCompatibilityRequest", len)?;
        if let Some(v) = self.protobuf_descriptor.as_ref() {
            struct_ser.serialize_field("protobufDescriptor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckProtobufDescriptorCompatibilityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protobuf_descriptor",
            "protobufDescriptor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtobufDescriptor,
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
                            "protobufDescriptor" | "protobuf_descriptor" => Ok(GeneratedField::ProtobufDescriptor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckProtobufDescriptorCompatibilityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.CheckProtobufDescriptorCompatibilityRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckProtobufDescriptorCompatibilityRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protobuf_descriptor__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtobufDescriptor => {
                            if protobuf_descriptor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufDescriptor"));
                            }
                            protobuf_descriptor__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckProtobufDescriptorCompatibilityRequest {
                    protobuf_descriptor: protobuf_descriptor__,
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.CheckProtobufDescriptorCompatibilityRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckProtobufDescriptorCompatibilityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_valid {
            len += 1;
        }
        if !self.incompatible_protobuf_descriptor_fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.CheckProtobufDescriptorCompatibilityResponse", len)?;
        if self.is_valid {
            struct_ser.serialize_field("isValid", &self.is_valid)?;
        }
        if !self.incompatible_protobuf_descriptor_fields.is_empty() {
            struct_ser.serialize_field("incompatibleProtobufDescriptorFields", &self.incompatible_protobuf_descriptor_fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckProtobufDescriptorCompatibilityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_valid",
            "isValid",
            "incompatible_protobuf_descriptor_fields",
            "incompatibleProtobufDescriptorFields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsValid,
            IncompatibleProtobufDescriptorFields,
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
                            "isValid" | "is_valid" => Ok(GeneratedField::IsValid),
                            "incompatibleProtobufDescriptorFields" | "incompatible_protobuf_descriptor_fields" => Ok(GeneratedField::IncompatibleProtobufDescriptorFields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckProtobufDescriptorCompatibilityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.CheckProtobufDescriptorCompatibilityResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckProtobufDescriptorCompatibilityResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_valid__ = None;
                let mut incompatible_protobuf_descriptor_fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsValid => {
                            if is_valid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isValid"));
                            }
                            is_valid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IncompatibleProtobufDescriptorFields => {
                            if incompatible_protobuf_descriptor_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("incompatibleProtobufDescriptorFields"));
                            }
                            incompatible_protobuf_descriptor_fields__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckProtobufDescriptorCompatibilityResponse {
                    is_valid: is_valid__.unwrap_or_default(),
                    incompatible_protobuf_descriptor_fields: incompatible_protobuf_descriptor_fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.CheckProtobufDescriptorCompatibilityResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteProtobufDescriptorsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_type_full_name.is_empty() {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.protobuf_descriptor_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.DeleteProtobufDescriptorsRequest", len)?;
        if !self.message_type_full_name.is_empty() {
            struct_ser.serialize_field("messageTypeFullName", &self.message_type_full_name)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.protobuf_descriptor_id.is_empty() {
            struct_ser.serialize_field("protobufDescriptorId", &self.protobuf_descriptor_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteProtobufDescriptorsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_type_full_name",
            "messageTypeFullName",
            "namespace",
            "organization_id",
            "organizationId",
            "protobuf_descriptor_id",
            "protobufDescriptorId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageTypeFullName,
            Namespace,
            OrganizationId,
            ProtobufDescriptorId,
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
                            "messageTypeFullName" | "message_type_full_name" => Ok(GeneratedField::MessageTypeFullName),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "protobufDescriptorId" | "protobuf_descriptor_id" => Ok(GeneratedField::ProtobufDescriptorId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteProtobufDescriptorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.DeleteProtobufDescriptorsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteProtobufDescriptorsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_type_full_name__ = None;
                let mut namespace__ = None;
                let mut organization_id__ = None;
                let mut protobuf_descriptor_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessageTypeFullName => {
                            if message_type_full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageTypeFullName"));
                            }
                            message_type_full_name__ = Some(map_.next_value()?);
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
                        GeneratedField::ProtobufDescriptorId => {
                            if protobuf_descriptor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufDescriptorId"));
                            }
                            protobuf_descriptor_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteProtobufDescriptorsRequest {
                    message_type_full_name: message_type_full_name__.unwrap_or_default(),
                    namespace: namespace__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    protobuf_descriptor_id: protobuf_descriptor_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.DeleteProtobufDescriptorsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteProtobufDescriptorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.DeleteProtobufDescriptorsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteProtobufDescriptorsResponse {
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
            type Value = DeleteProtobufDescriptorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.DeleteProtobufDescriptorsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteProtobufDescriptorsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteProtobufDescriptorsResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.DeleteProtobufDescriptorsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IncompatibleProtobufField {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.protobuf_descriptor_id.is_empty() {
            len += 1;
        }
        if !self.message_full_name.is_empty() {
            len += 1;
        }
        if !self.desired_field_name.is_empty() {
            len += 1;
        }
        if !self.current_field_name.is_empty() {
            len += 1;
        }
        if !self.field_number.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        if !self.field_kind.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.IncompatibleProtobufField", len)?;
        if !self.protobuf_descriptor_id.is_empty() {
            struct_ser.serialize_field("protobufDescriptorId", &self.protobuf_descriptor_id)?;
        }
        if !self.message_full_name.is_empty() {
            struct_ser.serialize_field("messageFullName", &self.message_full_name)?;
        }
        if !self.desired_field_name.is_empty() {
            struct_ser.serialize_field("desiredFieldName", &self.desired_field_name)?;
        }
        if !self.current_field_name.is_empty() {
            struct_ser.serialize_field("currentFieldName", &self.current_field_name)?;
        }
        if !self.field_number.is_empty() {
            struct_ser.serialize_field("fieldNumber", &self.field_number)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        if !self.field_kind.is_empty() {
            struct_ser.serialize_field("fieldKind", &self.field_kind)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IncompatibleProtobufField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protobuf_descriptor_id",
            "protobufDescriptorId",
            "message_full_name",
            "messageFullName",
            "desired_field_name",
            "desiredFieldName",
            "current_field_name",
            "currentFieldName",
            "field_number",
            "fieldNumber",
            "reason",
            "details",
            "field_kind",
            "fieldKind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtobufDescriptorId,
            MessageFullName,
            DesiredFieldName,
            CurrentFieldName,
            FieldNumber,
            Reason,
            Details,
            FieldKind,
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
                            "protobufDescriptorId" | "protobuf_descriptor_id" => Ok(GeneratedField::ProtobufDescriptorId),
                            "messageFullName" | "message_full_name" => Ok(GeneratedField::MessageFullName),
                            "desiredFieldName" | "desired_field_name" => Ok(GeneratedField::DesiredFieldName),
                            "currentFieldName" | "current_field_name" => Ok(GeneratedField::CurrentFieldName),
                            "fieldNumber" | "field_number" => Ok(GeneratedField::FieldNumber),
                            "reason" => Ok(GeneratedField::Reason),
                            "details" => Ok(GeneratedField::Details),
                            "fieldKind" | "field_kind" => Ok(GeneratedField::FieldKind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IncompatibleProtobufField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.IncompatibleProtobufField")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IncompatibleProtobufField, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protobuf_descriptor_id__ = None;
                let mut message_full_name__ = None;
                let mut desired_field_name__ = None;
                let mut current_field_name__ = None;
                let mut field_number__ = None;
                let mut reason__ = None;
                let mut details__ = None;
                let mut field_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtobufDescriptorId => {
                            if protobuf_descriptor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufDescriptorId"));
                            }
                            protobuf_descriptor_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MessageFullName => {
                            if message_full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageFullName"));
                            }
                            message_full_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DesiredFieldName => {
                            if desired_field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("desiredFieldName"));
                            }
                            desired_field_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CurrentFieldName => {
                            if current_field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentFieldName"));
                            }
                            current_field_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldNumber => {
                            if field_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldNumber"));
                            }
                            field_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FieldKind => {
                            if field_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldKind"));
                            }
                            field_kind__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IncompatibleProtobufField {
                    protobuf_descriptor_id: protobuf_descriptor_id__.unwrap_or_default(),
                    message_full_name: message_full_name__.unwrap_or_default(),
                    desired_field_name: desired_field_name__.unwrap_or_default(),
                    current_field_name: current_field_name__.unwrap_or_default(),
                    field_number: field_number__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                    field_kind: field_kind__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.IncompatibleProtobufField", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProtobufDescriptorsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.ListProtobufDescriptorsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListProtobufDescriptorsRequest {
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
            type Value = ListProtobufDescriptorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.ListProtobufDescriptorsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProtobufDescriptorsRequest, V::Error>
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
                Ok(ListProtobufDescriptorsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.ListProtobufDescriptorsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListProtobufDescriptorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.protobuf_descriptors.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.ListProtobufDescriptorsResponse", len)?;
        if !self.protobuf_descriptors.is_empty() {
            struct_ser.serialize_field("protobufDescriptors", &self.protobuf_descriptors)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListProtobufDescriptorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protobuf_descriptors",
            "protobufDescriptors",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtobufDescriptors,
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
                            "protobufDescriptors" | "protobuf_descriptors" => Ok(GeneratedField::ProtobufDescriptors),
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
            type Value = ListProtobufDescriptorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.ListProtobufDescriptorsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListProtobufDescriptorsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protobuf_descriptors__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtobufDescriptors => {
                            if protobuf_descriptors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufDescriptors"));
                            }
                            protobuf_descriptors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListProtobufDescriptorsResponse {
                    protobuf_descriptors: protobuf_descriptors__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.ListProtobufDescriptorsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MapKeyOverrideType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MapKeyOverrideUnspecified => "MAP_KEY_OVERRIDE_UNSPECIFIED",
            Self::MapKeyOverrideTarget => "MAP_KEY_OVERRIDE_TARGET",
            Self::MapKeyOverrideSource => "MAP_KEY_OVERRIDE_SOURCE",
            Self::MapKeyOverrideRemoveKey => "MAP_KEY_OVERRIDE_REMOVE_KEY",
            Self::MapKeyOverrideEnum => "MAP_KEY_OVERRIDE_ENUM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MapKeyOverrideType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MAP_KEY_OVERRIDE_UNSPECIFIED",
            "MAP_KEY_OVERRIDE_TARGET",
            "MAP_KEY_OVERRIDE_SOURCE",
            "MAP_KEY_OVERRIDE_REMOVE_KEY",
            "MAP_KEY_OVERRIDE_ENUM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MapKeyOverrideType;

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
                    "MAP_KEY_OVERRIDE_UNSPECIFIED" => Ok(MapKeyOverrideType::MapKeyOverrideUnspecified),
                    "MAP_KEY_OVERRIDE_TARGET" => Ok(MapKeyOverrideType::MapKeyOverrideTarget),
                    "MAP_KEY_OVERRIDE_SOURCE" => Ok(MapKeyOverrideType::MapKeyOverrideSource),
                    "MAP_KEY_OVERRIDE_REMOVE_KEY" => Ok(MapKeyOverrideType::MapKeyOverrideRemoveKey),
                    "MAP_KEY_OVERRIDE_ENUM" => Ok(MapKeyOverrideType::MapKeyOverrideEnum),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ProtobufDescriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message_type_full_name.is_empty() {
            len += 1;
        }
        if !self.file_descriptor_set.is_empty() {
            len += 1;
        }
        if !self.proto_file_name.is_empty() {
            len += 1;
        }
        if !self.namespace.is_empty() {
            len += 1;
        }
        if !self.protobuf_descriptor_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.ProtobufDescriptor", len)?;
        if !self.message_type_full_name.is_empty() {
            struct_ser.serialize_field("messageTypeFullName", &self.message_type_full_name)?;
        }
        if !self.file_descriptor_set.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("fileDescriptorSet", pbjson::private::base64::encode(&self.file_descriptor_set).as_str())?;
        }
        if !self.proto_file_name.is_empty() {
            struct_ser.serialize_field("protoFileName", &self.proto_file_name)?;
        }
        if !self.namespace.is_empty() {
            struct_ser.serialize_field("namespace", &self.namespace)?;
        }
        if !self.protobuf_descriptor_id.is_empty() {
            struct_ser.serialize_field("protobufDescriptorId", &self.protobuf_descriptor_id)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProtobufDescriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message_type_full_name",
            "messageTypeFullName",
            "file_descriptor_set",
            "fileDescriptorSet",
            "proto_file_name",
            "protoFileName",
            "namespace",
            "protobuf_descriptor_id",
            "protobufDescriptorId",
            "created_date",
            "createdDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MessageTypeFullName,
            FileDescriptorSet,
            ProtoFileName,
            Namespace,
            ProtobufDescriptorId,
            CreatedDate,
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
                            "messageTypeFullName" | "message_type_full_name" => Ok(GeneratedField::MessageTypeFullName),
                            "fileDescriptorSet" | "file_descriptor_set" => Ok(GeneratedField::FileDescriptorSet),
                            "protoFileName" | "proto_file_name" => Ok(GeneratedField::ProtoFileName),
                            "namespace" => Ok(GeneratedField::Namespace),
                            "protobufDescriptorId" | "protobuf_descriptor_id" => Ok(GeneratedField::ProtobufDescriptorId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtobufDescriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.ProtobufDescriptor")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProtobufDescriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message_type_full_name__ = None;
                let mut file_descriptor_set__ = None;
                let mut proto_file_name__ = None;
                let mut namespace__ = None;
                let mut protobuf_descriptor_id__ = None;
                let mut created_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MessageTypeFullName => {
                            if message_type_full_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageTypeFullName"));
                            }
                            message_type_full_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileDescriptorSet => {
                            if file_descriptor_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileDescriptorSet"));
                            }
                            file_descriptor_set__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtoFileName => {
                            if proto_file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoFileName"));
                            }
                            proto_file_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Namespace => {
                            if namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namespace"));
                            }
                            namespace__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProtobufDescriptorId => {
                            if protobuf_descriptor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protobufDescriptorId"));
                            }
                            protobuf_descriptor_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedDate => {
                            if created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdDate"));
                            }
                            created_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ProtobufDescriptor {
                    message_type_full_name: message_type_full_name__.unwrap_or_default(),
                    file_descriptor_set: file_descriptor_set__.unwrap_or_default(),
                    proto_file_name: proto_file_name__.unwrap_or_default(),
                    namespace: namespace__.unwrap_or_default(),
                    protobuf_descriptor_id: protobuf_descriptor_id__.unwrap_or_default(),
                    created_date: created_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.ProtobufDescriptor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed_tag_target.is_some() {
            len += 1;
        }
        if self.tag_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.TagSource", len)?;
        if let Some(v) = self.allowed_tag_target.as_ref() {
            let v = TagTargetType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("allowedTagTarget", &v)?;
        }
        if let Some(v) = self.tag_name.as_ref() {
            struct_ser.serialize_field("tagName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_tag_target",
            "allowedTagTarget",
            "tag_name",
            "tagName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedTagTarget,
            TagName,
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
                            "allowedTagTarget" | "allowed_tag_target" => Ok(GeneratedField::AllowedTagTarget),
                            "tagName" | "tag_name" => Ok(GeneratedField::TagName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.TagSource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TagSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_tag_target__ = None;
                let mut tag_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedTagTarget => {
                            if allowed_tag_target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedTagTarget"));
                            }
                            allowed_tag_target__ = map_.next_value::<::std::option::Option<TagTargetType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::TagName => {
                            if tag_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagName"));
                            }
                            tag_name__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TagSource {
                    allowed_tag_target: allowed_tag_target__,
                    tag_name: tag_name__,
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.TagSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagSourceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DescendantSources => "DESCENDANT_SOURCES",
            Self::SiblingSources => "SIBLING_SOURCES",
            Self::DescendantAndSiblingSources => "DESCENDANT_AND_SIBLING_SOURCES",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TagSourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DESCENDANT_SOURCES",
            "SIBLING_SOURCES",
            "DESCENDANT_AND_SIBLING_SOURCES",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagSourceType;

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
                    "DESCENDANT_SOURCES" => Ok(TagSourceType::DescendantSources),
                    "SIBLING_SOURCES" => Ok(TagSourceType::SiblingSources),
                    "DESCENDANT_AND_SIBLING_SOURCES" => Ok(TagSourceType::DescendantAndSiblingSources),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TagTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed_tag_source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.protobuf_descriptors.v2.TagTarget", len)?;
        if let Some(v) = self.allowed_tag_source.as_ref() {
            let v = TagSourceType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("allowedTagSource", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_tag_source",
            "allowedTagSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedTagSource,
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
                            "allowedTagSource" | "allowed_tag_source" => Ok(GeneratedField::AllowedTagSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.protobuf_descriptors.v2.TagTarget")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TagTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_tag_source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedTagSource => {
                            if allowed_tag_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedTagSource"));
                            }
                            allowed_tag_source__ = map_.next_value::<::std::option::Option<TagSourceType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(TagTarget {
                    allowed_tag_source: allowed_tag_source__,
                })
            }
        }
        deserializer.deserialize_struct("sift.protobuf_descriptors.v2.TagTarget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagTargetType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AncestorTargets => "ANCESTOR_TARGETS",
            Self::SiblingTargets => "SIBLING_TARGETS",
            Self::AncestorAndSiblingTargets => "ANCESTOR_AND_SIBLING_TARGETS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TagTargetType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANCESTOR_TARGETS",
            "SIBLING_TARGETS",
            "ANCESTOR_AND_SIBLING_TARGETS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagTargetType;

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
                    "ANCESTOR_TARGETS" => Ok(TagTargetType::AncestorTargets),
                    "SIBLING_TARGETS" => Ok(TagTargetType::SiblingTargets),
                    "ANCESTOR_AND_SIBLING_TARGETS" => Ok(TagTargetType::AncestorAndSiblingTargets),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
