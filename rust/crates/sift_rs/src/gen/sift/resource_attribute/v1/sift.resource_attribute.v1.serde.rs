// @generated
impl serde::Serialize for ArchiveResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.archived_enum_value_id.is_empty() {
            len += 1;
        }
        if !self.replacement_enum_value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeEnumValueRequest", len)?;
        if !self.archived_enum_value_id.is_empty() {
            struct_ser.serialize_field("archivedEnumValueId", &self.archived_enum_value_id)?;
        }
        if !self.replacement_enum_value_id.is_empty() {
            struct_ser.serialize_field("replacementEnumValueId", &self.replacement_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "archived_enum_value_id",
            "archivedEnumValueId",
            "replacement_enum_value_id",
            "replacementEnumValueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArchivedEnumValueId,
            ReplacementEnumValueId,
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
                            "archivedEnumValueId" | "archived_enum_value_id" => Ok(GeneratedField::ArchivedEnumValueId),
                            "replacementEnumValueId" | "replacement_enum_value_id" => Ok(GeneratedField::ReplacementEnumValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveResourceAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ArchiveResourceAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveResourceAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut archived_enum_value_id__ = None;
                let mut replacement_enum_value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArchivedEnumValueId => {
                            if archived_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedEnumValueId"));
                            }
                            archived_enum_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReplacementEnumValueId => {
                            if replacement_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replacementEnumValueId"));
                            }
                            replacement_enum_value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveResourceAttributeEnumValueRequest {
                    archived_enum_value_id: archived_enum_value_id__.unwrap_or_default(),
                    replacement_enum_value_id: replacement_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attributes_migrated != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeEnumValueResponse", len)?;
        if self.resource_attributes_migrated != 0 {
            struct_ser.serialize_field("resourceAttributesMigrated", &self.resource_attributes_migrated)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attributes_migrated",
            "resourceAttributesMigrated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributesMigrated,
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
                            "resourceAttributesMigrated" | "resource_attributes_migrated" => Ok(GeneratedField::ResourceAttributesMigrated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveResourceAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ArchiveResourceAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveResourceAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attributes_migrated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributesMigrated => {
                            if resource_attributes_migrated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributesMigrated"));
                            }
                            resource_attributes_migrated__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ArchiveResourceAttributeEnumValueResponse {
                    resource_attributes_migrated: resource_attributes_migrated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeKeyRequest", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyId,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveResourceAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ArchiveResourceAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveResourceAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveResourceAttributeKeyRequest {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeKeyResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveResourceAttributeKeyResponse {
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
            type Value = ArchiveResourceAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ArchiveResourceAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveResourceAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ArchiveResourceAttributeKeyResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveResourceAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeRequest", len)?;
        if !self.resource_attribute_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeId", &self.resource_attribute_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveResourceAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_id",
            "resourceAttributeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeId,
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
                            "resourceAttributeId" | "resource_attribute_id" => Ok(GeneratedField::ResourceAttributeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveResourceAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ArchiveResourceAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveResourceAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeId => {
                            if resource_attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeId"));
                            }
                            resource_attribute_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveResourceAttributeRequest {
                    resource_attribute_id: resource_attribute_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveResourceAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveResourceAttributeResponse {
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
            type Value = ArchiveResourceAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ArchiveResourceAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveResourceAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ArchiveResourceAttributeResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ArchiveResourceAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveResourceAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.archival_requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesRequest", len)?;
        if !self.archival_requests.is_empty() {
            struct_ser.serialize_field("archivalRequests", &self.archival_requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveResourceAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "archival_requests",
            "archivalRequests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArchivalRequests,
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
                            "archivalRequests" | "archival_requests" => Ok(GeneratedField::ArchivalRequests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchArchiveResourceAttributeEnumValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveResourceAttributeEnumValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut archival_requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArchivalRequests => {
                            if archival_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivalRequests"));
                            }
                            archival_requests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchArchiveResourceAttributeEnumValuesRequest {
                    archival_requests: archival_requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for batch_archive_resource_attribute_enum_values_request::EnumValueArchival {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.archived_enum_value_id.is_empty() {
            len += 1;
        }
        if !self.replacement_enum_value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesRequest.EnumValueArchival", len)?;
        if !self.archived_enum_value_id.is_empty() {
            struct_ser.serialize_field("archivedEnumValueId", &self.archived_enum_value_id)?;
        }
        if !self.replacement_enum_value_id.is_empty() {
            struct_ser.serialize_field("replacementEnumValueId", &self.replacement_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for batch_archive_resource_attribute_enum_values_request::EnumValueArchival {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "archived_enum_value_id",
            "archivedEnumValueId",
            "replacement_enum_value_id",
            "replacementEnumValueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArchivedEnumValueId,
            ReplacementEnumValueId,
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
                            "archivedEnumValueId" | "archived_enum_value_id" => Ok(GeneratedField::ArchivedEnumValueId),
                            "replacementEnumValueId" | "replacement_enum_value_id" => Ok(GeneratedField::ReplacementEnumValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = batch_archive_resource_attribute_enum_values_request::EnumValueArchival;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesRequest.EnumValueArchival")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<batch_archive_resource_attribute_enum_values_request::EnumValueArchival, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut archived_enum_value_id__ = None;
                let mut replacement_enum_value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArchivedEnumValueId => {
                            if archived_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedEnumValueId"));
                            }
                            archived_enum_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReplacementEnumValueId => {
                            if replacement_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replacementEnumValueId"));
                            }
                            replacement_enum_value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(batch_archive_resource_attribute_enum_values_request::EnumValueArchival {
                    archived_enum_value_id: archived_enum_value_id__.unwrap_or_default(),
                    replacement_enum_value_id: replacement_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesRequest.EnumValueArchival", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveResourceAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_resource_attributes_migrated != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesResponse", len)?;
        if self.total_resource_attributes_migrated != 0 {
            struct_ser.serialize_field("totalResourceAttributesMigrated", &self.total_resource_attributes_migrated)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveResourceAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_resource_attributes_migrated",
            "totalResourceAttributesMigrated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalResourceAttributesMigrated,
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
                            "totalResourceAttributesMigrated" | "total_resource_attributes_migrated" => Ok(GeneratedField::TotalResourceAttributesMigrated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchArchiveResourceAttributeEnumValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveResourceAttributeEnumValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_resource_attributes_migrated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalResourceAttributesMigrated => {
                            if total_resource_attributes_migrated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalResourceAttributesMigrated"));
                            }
                            total_resource_attributes_migrated__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BatchArchiveResourceAttributeEnumValuesResponse {
                    total_resource_attributes_migrated: total_resource_attributes_migrated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeEnumValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveResourceAttributeKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeKeysRequest", len)?;
        if !self.resource_attribute_key_ids.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyIds", &self.resource_attribute_key_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveResourceAttributeKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_ids",
            "resourceAttributeKeyIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyIds,
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
                            "resourceAttributeKeyIds" | "resource_attribute_key_ids" => Ok(GeneratedField::ResourceAttributeKeyIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchArchiveResourceAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchArchiveResourceAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveResourceAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyIds => {
                            if resource_attribute_key_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyIds"));
                            }
                            resource_attribute_key_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchArchiveResourceAttributeKeysRequest {
                    resource_attribute_key_ids: resource_attribute_key_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveResourceAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeKeysResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveResourceAttributeKeysResponse {
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
            type Value = BatchArchiveResourceAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchArchiveResourceAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveResourceAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchArchiveResourceAttributeKeysResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveResourceAttributesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributesRequest", len)?;
        if !self.resource_attribute_ids.is_empty() {
            struct_ser.serialize_field("resourceAttributeIds", &self.resource_attribute_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveResourceAttributesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_ids",
            "resourceAttributeIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeIds,
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
                            "resourceAttributeIds" | "resource_attribute_ids" => Ok(GeneratedField::ResourceAttributeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchArchiveResourceAttributesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchArchiveResourceAttributesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveResourceAttributesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeIds => {
                            if resource_attribute_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeIds"));
                            }
                            resource_attribute_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchArchiveResourceAttributesRequest {
                    resource_attribute_ids: resource_attribute_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchiveResourceAttributesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchiveResourceAttributesResponse {
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
            type Value = BatchArchiveResourceAttributesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchArchiveResourceAttributesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchiveResourceAttributesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchArchiveResourceAttributesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchArchiveResourceAttributesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateResourceAttributesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.entities.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchCreateResourceAttributesRequest", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        if !self.entities.is_empty() {
            struct_ser.serialize_field("entities", &self.entities)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                batch_create_resource_attributes_request::Value::ResourceAttributeEnumValueId(v) => {
                    struct_ser.serialize_field("resourceAttributeEnumValueId", v)?;
                }
                batch_create_resource_attributes_request::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
                batch_create_resource_attributes_request::Value::NumberValue(v) => {
                    struct_ser.serialize_field("numberValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateResourceAttributesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
            "entities",
            "resource_attribute_enum_value_id",
            "resourceAttributeEnumValueId",
            "boolean_value",
            "booleanValue",
            "number_value",
            "numberValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyId,
            Entities,
            ResourceAttributeEnumValueId,
            BooleanValue,
            NumberValue,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            "entities" => Ok(GeneratedField::Entities),
                            "resourceAttributeEnumValueId" | "resource_attribute_enum_value_id" => Ok(GeneratedField::ResourceAttributeEnumValueId),
                            "booleanValue" | "boolean_value" => Ok(GeneratedField::BooleanValue),
                            "numberValue" | "number_value" => Ok(GeneratedField::NumberValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateResourceAttributesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchCreateResourceAttributesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateResourceAttributesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                let mut entities__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entities => {
                            if entities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entities"));
                            }
                            entities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceAttributeEnumValueId => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueId"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_create_resource_attributes_request::Value::ResourceAttributeEnumValueId);
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_create_resource_attributes_request::Value::BooleanValue);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| batch_create_resource_attributes_request::Value::NumberValue(x.0));
                        }
                    }
                }
                Ok(BatchCreateResourceAttributesRequest {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    entities: entities__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchCreateResourceAttributesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateResourceAttributesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attributes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchCreateResourceAttributesResponse", len)?;
        if !self.resource_attributes.is_empty() {
            struct_ser.serialize_field("resourceAttributes", &self.resource_attributes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateResourceAttributesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attributes",
            "resourceAttributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributes,
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
                            "resourceAttributes" | "resource_attributes" => Ok(GeneratedField::ResourceAttributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateResourceAttributesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchCreateResourceAttributesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateResourceAttributesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attributes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributes => {
                            if resource_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributes"));
                            }
                            resource_attributes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateResourceAttributesResponse {
                    resource_attributes: resource_attributes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchCreateResourceAttributesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveResourceAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_enum_value_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeEnumValuesRequest", len)?;
        if !self.resource_attribute_enum_value_ids.is_empty() {
            struct_ser.serialize_field("resourceAttributeEnumValueIds", &self.resource_attribute_enum_value_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveResourceAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value_ids",
            "resourceAttributeEnumValueIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValueIds,
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
                            "resourceAttributeEnumValueIds" | "resource_attribute_enum_value_ids" => Ok(GeneratedField::ResourceAttributeEnumValueIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUnarchiveResourceAttributeEnumValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchUnarchiveResourceAttributeEnumValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveResourceAttributeEnumValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValueIds => {
                            if resource_attribute_enum_value_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueIds"));
                            }
                            resource_attribute_enum_value_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUnarchiveResourceAttributeEnumValuesRequest {
                    resource_attribute_enum_value_ids: resource_attribute_enum_value_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeEnumValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveResourceAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeEnumValuesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveResourceAttributeEnumValuesResponse {
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
            type Value = BatchUnarchiveResourceAttributeEnumValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchUnarchiveResourceAttributeEnumValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveResourceAttributeEnumValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchUnarchiveResourceAttributeEnumValuesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeEnumValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveResourceAttributeKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeKeysRequest", len)?;
        if !self.resource_attribute_key_ids.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyIds", &self.resource_attribute_key_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveResourceAttributeKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_ids",
            "resourceAttributeKeyIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyIds,
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
                            "resourceAttributeKeyIds" | "resource_attribute_key_ids" => Ok(GeneratedField::ResourceAttributeKeyIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUnarchiveResourceAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchUnarchiveResourceAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveResourceAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyIds => {
                            if resource_attribute_key_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyIds"));
                            }
                            resource_attribute_key_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUnarchiveResourceAttributeKeysRequest {
                    resource_attribute_key_ids: resource_attribute_key_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveResourceAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeKeysResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveResourceAttributeKeysResponse {
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
            type Value = BatchUnarchiveResourceAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchUnarchiveResourceAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveResourceAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchUnarchiveResourceAttributeKeysResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveResourceAttributesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributesRequest", len)?;
        if !self.resource_attribute_ids.is_empty() {
            struct_ser.serialize_field("resourceAttributeIds", &self.resource_attribute_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveResourceAttributesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_ids",
            "resourceAttributeIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeIds,
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
                            "resourceAttributeIds" | "resource_attribute_ids" => Ok(GeneratedField::ResourceAttributeIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUnarchiveResourceAttributesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchUnarchiveResourceAttributesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveResourceAttributesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeIds => {
                            if resource_attribute_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeIds"));
                            }
                            resource_attribute_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUnarchiveResourceAttributesRequest {
                    resource_attribute_ids: resource_attribute_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchiveResourceAttributesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchiveResourceAttributesResponse {
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
            type Value = BatchUnarchiveResourceAttributesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.BatchUnarchiveResourceAttributesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchiveResourceAttributesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchUnarchiveResourceAttributesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.BatchUnarchiveResourceAttributesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.CreateResourceAttributeEnumValueRequest", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
            "display_name",
            "displayName",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyId,
            DisplayName,
            Description,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.CreateResourceAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateResourceAttributeEnumValueRequest {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.CreateResourceAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute_enum_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.CreateResourceAttributeEnumValueResponse", len)?;
        if let Some(v) = self.resource_attribute_enum_value.as_ref() {
            struct_ser.serialize_field("resourceAttributeEnumValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value",
            "resourceAttributeEnumValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValue,
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
                            "resourceAttributeEnumValue" | "resource_attribute_enum_value" => Ok(GeneratedField::ResourceAttributeEnumValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.CreateResourceAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValue => {
                            if resource_attribute_enum_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValue"));
                            }
                            resource_attribute_enum_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateResourceAttributeEnumValueResponse {
                    resource_attribute_enum_value: resource_attribute_enum_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.CreateResourceAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if !self.initial_enum_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.CreateResourceAttributeKeyRequest", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.r#type != 0 {
            let v = ResourceAttributeKeyType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.initial_enum_values.is_empty() {
            struct_ser.serialize_field("initialEnumValues", &self.initial_enum_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "description",
            "type",
            "initial_enum_values",
            "initialEnumValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            Description,
            Type,
            InitialEnumValues,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "type" => Ok(GeneratedField::Type),
                            "initialEnumValues" | "initial_enum_values" => Ok(GeneratedField::InitialEnumValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.CreateResourceAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut description__ = None;
                let mut r#type__ = None;
                let mut initial_enum_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
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
                            r#type__ = Some(map_.next_value::<ResourceAttributeKeyType>()? as i32);
                        }
                        GeneratedField::InitialEnumValues => {
                            if initial_enum_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialEnumValues"));
                            }
                            initial_enum_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateResourceAttributeKeyRequest {
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    initial_enum_values: initial_enum_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.CreateResourceAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_resource_attribute_key_request::InitialEnumValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.CreateResourceAttributeKeyRequest.InitialEnumValue", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_resource_attribute_key_request::InitialEnumValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "display_name",
            "displayName",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DisplayName,
            Description,
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
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_resource_attribute_key_request::InitialEnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.CreateResourceAttributeKeyRequest.InitialEnumValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_resource_attribute_key_request::InitialEnumValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut display_name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(create_resource_attribute_key_request::InitialEnumValue {
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.CreateResourceAttributeKeyRequest.InitialEnumValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute_key.is_some() {
            len += 1;
        }
        if !self.enum_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.CreateResourceAttributeKeyResponse", len)?;
        if let Some(v) = self.resource_attribute_key.as_ref() {
            struct_ser.serialize_field("resourceAttributeKey", v)?;
        }
        if !self.enum_values.is_empty() {
            struct_ser.serialize_field("enumValues", &self.enum_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key",
            "resourceAttributeKey",
            "enum_values",
            "enumValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKey,
            EnumValues,
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
                            "resourceAttributeKey" | "resource_attribute_key" => Ok(GeneratedField::ResourceAttributeKey),
                            "enumValues" | "enum_values" => Ok(GeneratedField::EnumValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.CreateResourceAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key__ = None;
                let mut enum_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKey => {
                            if resource_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKey"));
                            }
                            resource_attribute_key__ = map_.next_value()?;
                        }
                        GeneratedField::EnumValues => {
                            if enum_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumValues"));
                            }
                            enum_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateResourceAttributeKeyResponse {
                    resource_attribute_key: resource_attribute_key__,
                    enum_values: enum_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.CreateResourceAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.CreateResourceAttributeRequest", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                create_resource_attribute_request::Value::ResourceAttributeEnumValueId(v) => {
                    struct_ser.serialize_field("resourceAttributeEnumValueId", v)?;
                }
                create_resource_attribute_request::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
                create_resource_attribute_request::Value::NumberValue(v) => {
                    struct_ser.serialize_field("numberValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
            "resource_attribute_enum_value_id",
            "resourceAttributeEnumValueId",
            "boolean_value",
            "booleanValue",
            "number_value",
            "numberValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
            ResourceAttributeKeyId,
            ResourceAttributeEnumValueId,
            BooleanValue,
            NumberValue,
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
                            "entity" => Ok(GeneratedField::Entity),
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            "resourceAttributeEnumValueId" | "resource_attribute_enum_value_id" => Ok(GeneratedField::ResourceAttributeEnumValueId),
                            "booleanValue" | "boolean_value" => Ok(GeneratedField::BooleanValue),
                            "numberValue" | "number_value" => Ok(GeneratedField::NumberValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.CreateResourceAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                let mut resource_attribute_key_id__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceAttributeEnumValueId => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueId"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(create_resource_attribute_request::Value::ResourceAttributeEnumValueId);
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(create_resource_attribute_request::Value::BooleanValue);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| create_resource_attribute_request::Value::NumberValue(x.0));
                        }
                    }
                }
                Ok(CreateResourceAttributeRequest {
                    entity: entity__,
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.CreateResourceAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateResourceAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.CreateResourceAttributeResponse", len)?;
        if let Some(v) = self.resource_attribute.as_ref() {
            struct_ser.serialize_field("resourceAttribute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateResourceAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute",
            "resourceAttribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttribute,
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
                            "resourceAttribute" | "resource_attribute" => Ok(GeneratedField::ResourceAttribute),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateResourceAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.CreateResourceAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateResourceAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttribute => {
                            if resource_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttribute"));
                            }
                            resource_attribute__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateResourceAttributeResponse {
                    resource_attribute: resource_attribute__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.CreateResourceAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_enum_value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.GetResourceAttributeEnumValueRequest", len)?;
        if !self.resource_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeEnumValueId", &self.resource_attribute_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value_id",
            "resourceAttributeEnumValueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValueId,
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
                            "resourceAttributeEnumValueId" | "resource_attribute_enum_value_id" => Ok(GeneratedField::ResourceAttributeEnumValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.GetResourceAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValueId => {
                            if resource_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueId"));
                            }
                            resource_attribute_enum_value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetResourceAttributeEnumValueRequest {
                    resource_attribute_enum_value_id: resource_attribute_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.GetResourceAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute_enum_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.GetResourceAttributeEnumValueResponse", len)?;
        if let Some(v) = self.resource_attribute_enum_value.as_ref() {
            struct_ser.serialize_field("resourceAttributeEnumValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value",
            "resourceAttributeEnumValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValue,
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
                            "resourceAttributeEnumValue" | "resource_attribute_enum_value" => Ok(GeneratedField::ResourceAttributeEnumValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.GetResourceAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValue => {
                            if resource_attribute_enum_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValue"));
                            }
                            resource_attribute_enum_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetResourceAttributeEnumValueResponse {
                    resource_attribute_enum_value: resource_attribute_enum_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.GetResourceAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.GetResourceAttributeKeyRequest", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyId,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.GetResourceAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetResourceAttributeKeyRequest {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.GetResourceAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.GetResourceAttributeKeyResponse", len)?;
        if let Some(v) = self.resource_attribute_key.as_ref() {
            struct_ser.serialize_field("resourceAttributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key",
            "resourceAttributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKey,
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
                            "resourceAttributeKey" | "resource_attribute_key" => Ok(GeneratedField::ResourceAttributeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.GetResourceAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKey => {
                            if resource_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKey"));
                            }
                            resource_attribute_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetResourceAttributeKeyResponse {
                    resource_attribute_key: resource_attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.GetResourceAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.GetResourceAttributeRequest", len)?;
        if !self.resource_attribute_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeId", &self.resource_attribute_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_id",
            "resourceAttributeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeId,
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
                            "resourceAttributeId" | "resource_attribute_id" => Ok(GeneratedField::ResourceAttributeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.GetResourceAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeId => {
                            if resource_attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeId"));
                            }
                            resource_attribute_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetResourceAttributeRequest {
                    resource_attribute_id: resource_attribute_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.GetResourceAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetResourceAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.GetResourceAttributeResponse", len)?;
        if let Some(v) = self.resource_attribute.as_ref() {
            struct_ser.serialize_field("resourceAttribute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetResourceAttributeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute",
            "resourceAttribute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttribute,
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
                            "resourceAttribute" | "resource_attribute" => Ok(GeneratedField::ResourceAttribute),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetResourceAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.GetResourceAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetResourceAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttribute => {
                            if resource_attribute__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttribute"));
                            }
                            resource_attribute__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetResourceAttributeResponse {
                    resource_attribute: resource_attribute__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.GetResourceAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributeEnumValuesRequest", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
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
impl<'de> serde::Deserialize<'de> for ListResourceAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
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
            ResourceAttributeKeyId,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
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
            type Value = ListResourceAttributeEnumValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributeEnumValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributeEnumValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut include_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
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
                Ok(ListResourceAttributeEnumValuesRequest {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributeEnumValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_enum_values.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributeEnumValuesResponse", len)?;
        if !self.resource_attribute_enum_values.is_empty() {
            struct_ser.serialize_field("resourceAttributeEnumValues", &self.resource_attribute_enum_values)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_values",
            "resourceAttributeEnumValues",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValues,
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
                            "resourceAttributeEnumValues" | "resource_attribute_enum_values" => Ok(GeneratedField::ResourceAttributeEnumValues),
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
            type Value = ListResourceAttributeEnumValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributeEnumValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributeEnumValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_values__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValues => {
                            if resource_attribute_enum_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValues"));
                            }
                            resource_attribute_enum_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListResourceAttributeEnumValuesResponse {
                    resource_attribute_enum_values: resource_attribute_enum_values__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributeEnumValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributeKeysRequest {
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
        if self.include_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributeKeysRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListResourceAttributeKeysRequest {
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
            "include_archived",
            "includeArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListResourceAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
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
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListResourceAttributeKeysRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_keys.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributeKeysResponse", len)?;
        if !self.resource_attribute_keys.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeys", &self.resource_attribute_keys)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceAttributeKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_keys",
            "resourceAttributeKeys",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeys,
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
                            "resourceAttributeKeys" | "resource_attribute_keys" => Ok(GeneratedField::ResourceAttributeKeys),
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
            type Value = ListResourceAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_keys__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeys => {
                            if resource_attribute_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeys"));
                            }
                            resource_attribute_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListResourceAttributeKeysResponse {
                    resource_attribute_keys: resource_attribute_keys__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributesByEntityRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity.is_some() {
            len += 1;
        }
        if self.include_archived {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributesByEntityRequest", len)?;
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        if self.include_archived {
            struct_ser.serialize_field("includeArchived", &self.include_archived)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceAttributesByEntityRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity",
            "include_archived",
            "includeArchived",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entity,
            IncludeArchived,
            PageSize,
            PageToken,
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
                            "entity" => Ok(GeneratedField::Entity),
                            "includeArchived" | "include_archived" => Ok(GeneratedField::IncludeArchived),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListResourceAttributesByEntityRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributesByEntityRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributesByEntityRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity__ = None;
                let mut include_archived__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(ListResourceAttributesByEntityRequest {
                    entity: entity__,
                    include_archived: include_archived__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributesByEntityRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributesByEntityResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attributes.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributesByEntityResponse", len)?;
        if !self.resource_attributes.is_empty() {
            struct_ser.serialize_field("resourceAttributes", &self.resource_attributes)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceAttributesByEntityResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attributes",
            "resourceAttributes",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributes,
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
                            "resourceAttributes" | "resource_attributes" => Ok(GeneratedField::ResourceAttributes),
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
            type Value = ListResourceAttributesByEntityResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributesByEntityResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributesByEntityResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attributes__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributes => {
                            if resource_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributes"));
                            }
                            resource_attributes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListResourceAttributesByEntityResponse {
                    resource_attributes: resource_attributes__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributesByEntityResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributesRequest {
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
        if self.include_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributesRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListResourceAttributesRequest {
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
            "include_archived",
            "includeArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ListResourceAttributesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
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
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListResourceAttributesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResourceAttributesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attributes.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ListResourceAttributesResponse", len)?;
        if !self.resource_attributes.is_empty() {
            struct_ser.serialize_field("resourceAttributes", &self.resource_attributes)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResourceAttributesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attributes",
            "resourceAttributes",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributes,
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
                            "resourceAttributes" | "resource_attributes" => Ok(GeneratedField::ResourceAttributes),
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
            type Value = ListResourceAttributesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ListResourceAttributesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResourceAttributesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attributes__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributes => {
                            if resource_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributes"));
                            }
                            resource_attributes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListResourceAttributesResponse {
                    resource_attributes: resource_attributes__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ListResourceAttributesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAttribute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.entity.is_some() {
            len += 1;
        }
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.enum_value_details.is_some() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ResourceAttribute", len)?;
        if !self.resource_attribute_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeId", &self.resource_attribute_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.entity.as_ref() {
            struct_ser.serialize_field("entity", v)?;
        }
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.enum_value_details.as_ref() {
            struct_ser.serialize_field("enumValueDetails", v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                resource_attribute::Value::ResourceAttributeEnumValueId(v) => {
                    struct_ser.serialize_field("resourceAttributeEnumValueId", v)?;
                }
                resource_attribute::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
                resource_attribute::Value::NumberValue(v) => {
                    struct_ser.serialize_field("numberValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAttribute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_id",
            "resourceAttributeId",
            "organization_id",
            "organizationId",
            "entity",
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
            "key",
            "enum_value_details",
            "enumValueDetails",
            "created_date",
            "createdDate",
            "created_by_user_id",
            "createdByUserId",
            "archived_date",
            "archivedDate",
            "resource_attribute_enum_value_id",
            "resourceAttributeEnumValueId",
            "boolean_value",
            "booleanValue",
            "number_value",
            "numberValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeId,
            OrganizationId,
            Entity,
            ResourceAttributeKeyId,
            Key,
            EnumValueDetails,
            CreatedDate,
            CreatedByUserId,
            ArchivedDate,
            ResourceAttributeEnumValueId,
            BooleanValue,
            NumberValue,
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
                            "resourceAttributeId" | "resource_attribute_id" => Ok(GeneratedField::ResourceAttributeId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "entity" => Ok(GeneratedField::Entity),
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            "key" => Ok(GeneratedField::Key),
                            "enumValueDetails" | "enum_value_details" => Ok(GeneratedField::EnumValueDetails),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "resourceAttributeEnumValueId" | "resource_attribute_enum_value_id" => Ok(GeneratedField::ResourceAttributeEnumValueId),
                            "booleanValue" | "boolean_value" => Ok(GeneratedField::BooleanValue),
                            "numberValue" | "number_value" => Ok(GeneratedField::NumberValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAttribute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ResourceAttribute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceAttribute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_id__ = None;
                let mut organization_id__ = None;
                let mut entity__ = None;
                let mut resource_attribute_key_id__ = None;
                let mut key__ = None;
                let mut enum_value_details__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                let mut archived_date__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeId => {
                            if resource_attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeId"));
                            }
                            resource_attribute_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Entity => {
                            if entity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            entity__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::EnumValueDetails => {
                            if enum_value_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumValueDetails"));
                            }
                            enum_value_details__ = map_.next_value()?;
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
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                        GeneratedField::ResourceAttributeEnumValueId => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueId"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_attribute::Value::ResourceAttributeEnumValueId);
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(resource_attribute::Value::BooleanValue);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| resource_attribute::Value::NumberValue(x.0));
                        }
                    }
                }
                Ok(ResourceAttribute {
                    resource_attribute_id: resource_attribute_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    entity: entity__,
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    key: key__,
                    enum_value_details: enum_value_details__,
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    archived_date: archived_date__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ResourceAttribute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAttributeEntityIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if self.entity_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ResourceAttributeEntityIdentifier", len)?;
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if self.entity_type != 0 {
            let v = ResourceAttributeEntityType::try_from(self.entity_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.entity_type)))?;
            struct_ser.serialize_field("entityType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAttributeEntityIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "entity_type",
            "entityType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            EntityType,
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
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "entityType" | "entity_type" => Ok(GeneratedField::EntityType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAttributeEntityIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ResourceAttributeEntityIdentifier")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceAttributeEntityIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut entity_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EntityType => {
                            if entity_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityType"));
                            }
                            entity_type__ = Some(map_.next_value::<ResourceAttributeEntityType>()? as i32);
                        }
                    }
                }
                Ok(ResourceAttributeEntityIdentifier {
                    entity_id: entity_id__.unwrap_or_default(),
                    entity_type: entity_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ResourceAttributeEntityIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAttributeEntityType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RESOURCE_ATTRIBUTE_ENTITY_TYPE_UNSPECIFIED",
            Self::Asset => "RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET",
            Self::Channel => "RESOURCE_ATTRIBUTE_ENTITY_TYPE_CHANNEL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAttributeEntityType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RESOURCE_ATTRIBUTE_ENTITY_TYPE_UNSPECIFIED",
            "RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET",
            "RESOURCE_ATTRIBUTE_ENTITY_TYPE_CHANNEL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAttributeEntityType;

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
                    "RESOURCE_ATTRIBUTE_ENTITY_TYPE_UNSPECIFIED" => Ok(ResourceAttributeEntityType::Unspecified),
                    "RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET" => Ok(ResourceAttributeEntityType::Asset),
                    "RESOURCE_ATTRIBUTE_ENTITY_TYPE_CHANNEL" => Ok(ResourceAttributeEntityType::Channel),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAttributeEnumValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_enum_value_id.is_empty() {
            len += 1;
        }
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ResourceAttributeEnumValue", len)?;
        if !self.resource_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeEnumValueId", &self.resource_attribute_enum_value_id)?;
        }
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAttributeEnumValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value_id",
            "resourceAttributeEnumValueId",
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
            "display_name",
            "displayName",
            "description",
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValueId,
            ResourceAttributeKeyId,
            DisplayName,
            Description,
            CreatedDate,
            CreatedByUserId,
            ModifiedDate,
            ModifiedByUserId,
            ArchivedDate,
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
                            "resourceAttributeEnumValueId" | "resource_attribute_enum_value_id" => Ok(GeneratedField::ResourceAttributeEnumValueId),
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAttributeEnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ResourceAttributeEnumValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceAttributeEnumValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value_id__ = None;
                let mut resource_attribute_key_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_date__ = None;
                let mut modified_by_user_id__ = None;
                let mut archived_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValueId => {
                            if resource_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueId"));
                            }
                            resource_attribute_enum_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(ResourceAttributeEnumValue {
                    resource_attribute_enum_value_id: resource_attribute_enum_value_id__.unwrap_or_default(),
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_date: modified_date__,
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    archived_date: archived_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ResourceAttributeEnumValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAttributeKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.ResourceAttributeKey", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.r#type != 0 {
            let v = ResourceAttributeKeyType::try_from(self.r#type)
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAttributeKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
            "organization_id",
            "organizationId",
            "display_name",
            "displayName",
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyId,
            OrganizationId,
            DisplayName,
            Description,
            Type,
            CreatedDate,
            CreatedByUserId,
            ModifiedDate,
            ModifiedByUserId,
            ArchivedDate,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
                            "type" => Ok(GeneratedField::Type),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAttributeKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.ResourceAttributeKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceAttributeKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                let mut organization_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut r#type__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_date__ = None;
                let mut modified_by_user_id__ = None;
                let mut archived_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
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
                            r#type__ = Some(map_.next_value::<ResourceAttributeKeyType>()? as i32);
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
                    }
                }
                Ok(ResourceAttributeKey {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_date: modified_date__,
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    archived_date: archived_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.ResourceAttributeKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAttributeKeyType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RESOURCE_ATTRIBUTE_KEY_TYPE_UNSPECIFIED",
            Self::Enum => "RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM",
            Self::Boolean => "RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN",
            Self::Number => "RESOURCE_ATTRIBUTE_KEY_TYPE_NUMBER",
            Self::SetOfEnum => "RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAttributeKeyType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RESOURCE_ATTRIBUTE_KEY_TYPE_UNSPECIFIED",
            "RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM",
            "RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN",
            "RESOURCE_ATTRIBUTE_KEY_TYPE_NUMBER",
            "RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAttributeKeyType;

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
                    "RESOURCE_ATTRIBUTE_KEY_TYPE_UNSPECIFIED" => Ok(ResourceAttributeKeyType::Unspecified),
                    "RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM" => Ok(ResourceAttributeKeyType::Enum),
                    "RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN" => Ok(ResourceAttributeKeyType::Boolean),
                    "RESOURCE_ATTRIBUTE_KEY_TYPE_NUMBER" => Ok(ResourceAttributeKeyType::Number),
                    "RESOURCE_ATTRIBUTE_KEY_TYPE_SET_OF_ENUM" => Ok(ResourceAttributeKeyType::SetOfEnum),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_enum_value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeEnumValueRequest", len)?;
        if !self.resource_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeEnumValueId", &self.resource_attribute_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value_id",
            "resourceAttributeEnumValueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValueId,
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
                            "resourceAttributeEnumValueId" | "resource_attribute_enum_value_id" => Ok(GeneratedField::ResourceAttributeEnumValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveResourceAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UnarchiveResourceAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveResourceAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValueId => {
                            if resource_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueId"));
                            }
                            resource_attribute_enum_value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchiveResourceAttributeEnumValueRequest {
                    resource_attribute_enum_value_id: resource_attribute_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeEnumValueResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveResourceAttributeEnumValueResponse {
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
            type Value = UnarchiveResourceAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UnarchiveResourceAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveResourceAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchiveResourceAttributeEnumValueResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeKeyRequest", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyId,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveResourceAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UnarchiveResourceAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveResourceAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchiveResourceAttributeKeyRequest {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeKeyResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveResourceAttributeKeyResponse {
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
            type Value = UnarchiveResourceAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UnarchiveResourceAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveResourceAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchiveResourceAttributeKeyResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveResourceAttributeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeRequest", len)?;
        if !self.resource_attribute_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeId", &self.resource_attribute_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveResourceAttributeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_id",
            "resourceAttributeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeId,
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
                            "resourceAttributeId" | "resource_attribute_id" => Ok(GeneratedField::ResourceAttributeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveResourceAttributeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UnarchiveResourceAttributeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveResourceAttributeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeId => {
                            if resource_attribute_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeId"));
                            }
                            resource_attribute_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchiveResourceAttributeRequest {
                    resource_attribute_id: resource_attribute_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveResourceAttributeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveResourceAttributeResponse {
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
            type Value = UnarchiveResourceAttributeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UnarchiveResourceAttributeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveResourceAttributeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchiveResourceAttributeResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UnarchiveResourceAttributeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_enum_value_id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeEnumValueRequest", len)?;
        if !self.resource_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeEnumValueId", &self.resource_attribute_enum_value_id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
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
impl<'de> serde::Deserialize<'de> for UpdateResourceAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value_id",
            "resourceAttributeEnumValueId",
            "display_name",
            "displayName",
            "description",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValueId,
            DisplayName,
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
                            "resourceAttributeEnumValueId" | "resource_attribute_enum_value_id" => Ok(GeneratedField::ResourceAttributeEnumValueId),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
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
            type Value = UpdateResourceAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UpdateResourceAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValueId => {
                            if resource_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValueId"));
                            }
                            resource_attribute_enum_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
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
                Ok(UpdateResourceAttributeEnumValueRequest {
                    resource_attribute_enum_value_id: resource_attribute_enum_value_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute_enum_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeEnumValueResponse", len)?;
        if let Some(v) = self.resource_attribute_enum_value.as_ref() {
            struct_ser.serialize_field("resourceAttributeEnumValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateResourceAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_enum_value",
            "resourceAttributeEnumValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeEnumValue,
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
                            "resourceAttributeEnumValue" | "resource_attribute_enum_value" => Ok(GeneratedField::ResourceAttributeEnumValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateResourceAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UpdateResourceAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_enum_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeEnumValue => {
                            if resource_attribute_enum_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeEnumValue"));
                            }
                            resource_attribute_enum_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateResourceAttributeEnumValueResponse {
                    resource_attribute_enum_value: resource_attribute_enum_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeKeyRequest", len)?;
        if !self.resource_attribute_key_id.is_empty() {
            struct_ser.serialize_field("resourceAttributeKeyId", &self.resource_attribute_key_id)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
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
impl<'de> serde::Deserialize<'de> for UpdateResourceAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key_id",
            "resourceAttributeKeyId",
            "display_name",
            "displayName",
            "description",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKeyId,
            DisplayName,
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
                            "resourceAttributeKeyId" | "resource_attribute_key_id" => Ok(GeneratedField::ResourceAttributeKeyId),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
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
            type Value = UpdateResourceAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UpdateResourceAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKeyId => {
                            if resource_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKeyId"));
                            }
                            resource_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map_.next_value()?);
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
                Ok(UpdateResourceAttributeKeyRequest {
                    resource_attribute_key_id: resource_attribute_key_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resource_attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeKeyResponse", len)?;
        if let Some(v) = self.resource_attribute_key.as_ref() {
            struct_ser.serialize_field("resourceAttributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateResourceAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_attribute_key",
            "resourceAttributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceAttributeKey,
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
                            "resourceAttributeKey" | "resource_attribute_key" => Ok(GeneratedField::ResourceAttributeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateResourceAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.resource_attribute.v1.UpdateResourceAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateResourceAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceAttributeKey => {
                            if resource_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributeKey"));
                            }
                            resource_attribute_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateResourceAttributeKeyResponse {
                    resource_attribute_key: resource_attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.resource_attribute.v1.UpdateResourceAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
