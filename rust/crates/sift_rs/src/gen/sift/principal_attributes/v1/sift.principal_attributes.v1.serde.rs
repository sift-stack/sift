// @generated
impl serde::Serialize for ArchivePrincipalAttributeEnumValueRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeEnumValueRequest", len)?;
        if !self.archived_enum_value_id.is_empty() {
            struct_ser.serialize_field("archivedEnumValueId", &self.archived_enum_value_id)?;
        }
        if !self.replacement_enum_value_id.is_empty() {
            struct_ser.serialize_field("replacementEnumValueId", &self.replacement_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePrincipalAttributeEnumValueRequest {
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
            type Value = ArchivePrincipalAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ArchivePrincipalAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePrincipalAttributeEnumValueRequest, V::Error>
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
                Ok(ArchivePrincipalAttributeEnumValueRequest {
                    archived_enum_value_id: archived_enum_value_id__.unwrap_or_default(),
                    replacement_enum_value_id: replacement_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchivePrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_values_migrated != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeEnumValueResponse", len)?;
        if self.principal_attribute_values_migrated != 0 {
            struct_ser.serialize_field("principalAttributeValuesMigrated", &self.principal_attribute_values_migrated)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_values_migrated",
            "principalAttributeValuesMigrated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValuesMigrated,
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
                            "principalAttributeValuesMigrated" | "principal_attribute_values_migrated" => Ok(GeneratedField::PrincipalAttributeValuesMigrated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchivePrincipalAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ArchivePrincipalAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePrincipalAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_values_migrated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValuesMigrated => {
                            if principal_attribute_values_migrated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValuesMigrated"));
                            }
                            principal_attribute_values_migrated__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ArchivePrincipalAttributeEnumValueResponse {
                    principal_attribute_values_migrated: principal_attribute_values_migrated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchivePrincipalAttributeKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeKeysRequest", len)?;
        if !self.principal_attribute_key_ids.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyIds", &self.principal_attribute_key_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePrincipalAttributeKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_ids",
            "principalAttributeKeyIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyIds,
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
                            "principalAttributeKeyIds" | "principal_attribute_key_ids" => Ok(GeneratedField::PrincipalAttributeKeyIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchivePrincipalAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ArchivePrincipalAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePrincipalAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyIds => {
                            if principal_attribute_key_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyIds"));
                            }
                            principal_attribute_key_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchivePrincipalAttributeKeysRequest {
                    principal_attribute_key_ids: principal_attribute_key_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchivePrincipalAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeKeysResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePrincipalAttributeKeysResponse {
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
            type Value = ArchivePrincipalAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ArchivePrincipalAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePrincipalAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ArchivePrincipalAttributeKeysResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchivePrincipalAttributeValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_value_ids.is_empty() {
            len += 1;
        }
        if self.principal_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeValuesRequest", len)?;
        if !self.principal_attribute_value_ids.is_empty() {
            struct_ser.serialize_field("principalAttributeValueIds", &self.principal_attribute_value_ids)?;
        }
        if self.principal_type != 0 {
            let v = PrincipalAttributePrincipalType::try_from(self.principal_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.principal_type)))?;
            struct_ser.serialize_field("principalType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePrincipalAttributeValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_value_ids",
            "principalAttributeValueIds",
            "principal_type",
            "principalType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValueIds,
            PrincipalType,
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
                            "principalAttributeValueIds" | "principal_attribute_value_ids" => Ok(GeneratedField::PrincipalAttributeValueIds),
                            "principalType" | "principal_type" => Ok(GeneratedField::PrincipalType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchivePrincipalAttributeValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ArchivePrincipalAttributeValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePrincipalAttributeValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_value_ids__ = None;
                let mut principal_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValueIds => {
                            if principal_attribute_value_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValueIds"));
                            }
                            principal_attribute_value_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalType => {
                            if principal_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalType"));
                            }
                            principal_type__ = Some(map_.next_value::<PrincipalAttributePrincipalType>()? as i32);
                        }
                    }
                }
                Ok(ArchivePrincipalAttributeValuesRequest {
                    principal_attribute_value_ids: principal_attribute_value_ids__.unwrap_or_default(),
                    principal_type: principal_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchivePrincipalAttributeValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeValuesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePrincipalAttributeValuesResponse {
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
            type Value = ArchivePrincipalAttributeValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ArchivePrincipalAttributeValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePrincipalAttributeValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ArchivePrincipalAttributeValuesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ArchivePrincipalAttributeValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchivePrincipalAttributeEnumValuesRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesRequest", len)?;
        if !self.archival_requests.is_empty() {
            struct_ser.serialize_field("archivalRequests", &self.archival_requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchivePrincipalAttributeEnumValuesRequest {
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
            type Value = BatchArchivePrincipalAttributeEnumValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchivePrincipalAttributeEnumValuesRequest, V::Error>
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
                Ok(BatchArchivePrincipalAttributeEnumValuesRequest {
                    archival_requests: archival_requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for batch_archive_principal_attribute_enum_values_request::EnumValueArchival {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesRequest.EnumValueArchival", len)?;
        if !self.archived_enum_value_id.is_empty() {
            struct_ser.serialize_field("archivedEnumValueId", &self.archived_enum_value_id)?;
        }
        if !self.replacement_enum_value_id.is_empty() {
            struct_ser.serialize_field("replacementEnumValueId", &self.replacement_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for batch_archive_principal_attribute_enum_values_request::EnumValueArchival {
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
            type Value = batch_archive_principal_attribute_enum_values_request::EnumValueArchival;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesRequest.EnumValueArchival")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<batch_archive_principal_attribute_enum_values_request::EnumValueArchival, V::Error>
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
                Ok(batch_archive_principal_attribute_enum_values_request::EnumValueArchival {
                    archived_enum_value_id: archived_enum_value_id__.unwrap_or_default(),
                    replacement_enum_value_id: replacement_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesRequest.EnumValueArchival", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchArchivePrincipalAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total_principal_attribute_values_migrated != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesResponse", len)?;
        if self.total_principal_attribute_values_migrated != 0 {
            struct_ser.serialize_field("totalPrincipalAttributeValuesMigrated", &self.total_principal_attribute_values_migrated)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchArchivePrincipalAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total_principal_attribute_values_migrated",
            "totalPrincipalAttributeValuesMigrated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TotalPrincipalAttributeValuesMigrated,
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
                            "totalPrincipalAttributeValuesMigrated" | "total_principal_attribute_values_migrated" => Ok(GeneratedField::TotalPrincipalAttributeValuesMigrated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchArchivePrincipalAttributeEnumValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchArchivePrincipalAttributeEnumValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total_principal_attribute_values_migrated__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TotalPrincipalAttributeValuesMigrated => {
                            if total_principal_attribute_values_migrated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPrincipalAttributeValuesMigrated"));
                            }
                            total_principal_attribute_values_migrated__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BatchArchivePrincipalAttributeEnumValuesResponse {
                    total_principal_attribute_values_migrated: total_principal_attribute_values_migrated__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.BatchArchivePrincipalAttributeEnumValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreatePrincipalAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.principal_ids.is_empty() {
            len += 1;
        }
        if self.principal_type != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.BatchCreatePrincipalAttributeValueRequest", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
        }
        if !self.principal_ids.is_empty() {
            struct_ser.serialize_field("principalIds", &self.principal_ids)?;
        }
        if self.principal_type != 0 {
            let v = PrincipalAttributePrincipalType::try_from(self.principal_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.principal_type)))?;
            struct_ser.serialize_field("principalType", &v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                batch_create_principal_attribute_value_request::Value::PrincipalAttributeEnumValueId(v) => {
                    struct_ser.serialize_field("principalAttributeEnumValueId", v)?;
                }
                batch_create_principal_attribute_value_request::Value::PrincipalAttributeEnumValueIds(v) => {
                    struct_ser.serialize_field("principalAttributeEnumValueIds", v)?;
                }
                batch_create_principal_attribute_value_request::Value::NumberValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("numberValue", ToString::to_string(&v).as_str())?;
                }
                batch_create_principal_attribute_value_request::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreatePrincipalAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
            "principal_ids",
            "principalIds",
            "principal_type",
            "principalType",
            "principal_attribute_enum_value_id",
            "principalAttributeEnumValueId",
            "principal_attribute_enum_value_ids",
            "principalAttributeEnumValueIds",
            "number_value",
            "numberValue",
            "boolean_value",
            "booleanValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyId,
            PrincipalIds,
            PrincipalType,
            PrincipalAttributeEnumValueId,
            PrincipalAttributeEnumValueIds,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
                            "principalIds" | "principal_ids" => Ok(GeneratedField::PrincipalIds),
                            "principalType" | "principal_type" => Ok(GeneratedField::PrincipalType),
                            "principalAttributeEnumValueId" | "principal_attribute_enum_value_id" => Ok(GeneratedField::PrincipalAttributeEnumValueId),
                            "principalAttributeEnumValueIds" | "principal_attribute_enum_value_ids" => Ok(GeneratedField::PrincipalAttributeEnumValueIds),
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
            type Value = BatchCreatePrincipalAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.BatchCreatePrincipalAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreatePrincipalAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                let mut principal_ids__ = None;
                let mut principal_type__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalIds => {
                            if principal_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalIds"));
                            }
                            principal_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalType => {
                            if principal_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalType"));
                            }
                            principal_type__ = Some(map_.next_value::<PrincipalAttributePrincipalType>()? as i32);
                        }
                        GeneratedField::PrincipalAttributeEnumValueId => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueId"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_create_principal_attribute_value_request::Value::PrincipalAttributeEnumValueId);
                        }
                        GeneratedField::PrincipalAttributeEnumValueIds => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueIds"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_create_principal_attribute_value_request::Value::PrincipalAttributeEnumValueIds)
;
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| batch_create_principal_attribute_value_request::Value::NumberValue(x.0));
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_create_principal_attribute_value_request::Value::BooleanValue);
                        }
                    }
                }
                Ok(BatchCreatePrincipalAttributeValueRequest {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    principal_ids: principal_ids__.unwrap_or_default(),
                    principal_type: principal_type__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.BatchCreatePrincipalAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreatePrincipalAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.BatchCreatePrincipalAttributeValueResponse", len)?;
        if !self.principal_attribute_values.is_empty() {
            struct_ser.serialize_field("principalAttributeValues", &self.principal_attribute_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreatePrincipalAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_values",
            "principalAttributeValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValues,
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
                            "principalAttributeValues" | "principal_attribute_values" => Ok(GeneratedField::PrincipalAttributeValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreatePrincipalAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.BatchCreatePrincipalAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreatePrincipalAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValues => {
                            if principal_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValues"));
                            }
                            principal_attribute_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreatePrincipalAttributeValueResponse {
                    principal_attribute_values: principal_attribute_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.BatchCreatePrincipalAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchivePrincipalAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_enum_value_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.BatchUnarchivePrincipalAttributeEnumValuesRequest", len)?;
        if !self.principal_attribute_enum_value_ids.is_empty() {
            struct_ser.serialize_field("principalAttributeEnumValueIds", &self.principal_attribute_enum_value_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchivePrincipalAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value_ids",
            "principalAttributeEnumValueIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValueIds,
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
                            "principalAttributeEnumValueIds" | "principal_attribute_enum_value_ids" => Ok(GeneratedField::PrincipalAttributeEnumValueIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchUnarchivePrincipalAttributeEnumValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.BatchUnarchivePrincipalAttributeEnumValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchivePrincipalAttributeEnumValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValueIds => {
                            if principal_attribute_enum_value_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueIds"));
                            }
                            principal_attribute_enum_value_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchUnarchivePrincipalAttributeEnumValuesRequest {
                    principal_attribute_enum_value_ids: principal_attribute_enum_value_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.BatchUnarchivePrincipalAttributeEnumValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchUnarchivePrincipalAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.BatchUnarchivePrincipalAttributeEnumValuesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchUnarchivePrincipalAttributeEnumValuesResponse {
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
            type Value = BatchUnarchivePrincipalAttributeEnumValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.BatchUnarchivePrincipalAttributeEnumValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchUnarchivePrincipalAttributeEnumValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchUnarchivePrincipalAttributeEnumValuesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.BatchUnarchivePrincipalAttributeEnumValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPrincipalAttributeKeyArchiveImpactRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.CheckPrincipalAttributeKeyArchiveImpactRequest", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPrincipalAttributeKeyArchiveImpactRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyId,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPrincipalAttributeKeyArchiveImpactRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.CheckPrincipalAttributeKeyArchiveImpactRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckPrincipalAttributeKeyArchiveImpactRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckPrincipalAttributeKeyArchiveImpactRequest {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.CheckPrincipalAttributeKeyArchiveImpactRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPrincipalAttributeKeyArchiveImpactResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.active_user_principal_attribute_value_count != 0 {
            len += 1;
        }
        if self.active_user_group_principal_attribute_value_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.CheckPrincipalAttributeKeyArchiveImpactResponse", len)?;
        if self.active_user_principal_attribute_value_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("activeUserPrincipalAttributeValueCount", ToString::to_string(&self.active_user_principal_attribute_value_count).as_str())?;
        }
        if self.active_user_group_principal_attribute_value_count != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("activeUserGroupPrincipalAttributeValueCount", ToString::to_string(&self.active_user_group_principal_attribute_value_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPrincipalAttributeKeyArchiveImpactResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "active_user_principal_attribute_value_count",
            "activeUserPrincipalAttributeValueCount",
            "active_user_group_principal_attribute_value_count",
            "activeUserGroupPrincipalAttributeValueCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActiveUserPrincipalAttributeValueCount,
            ActiveUserGroupPrincipalAttributeValueCount,
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
                            "activeUserPrincipalAttributeValueCount" | "active_user_principal_attribute_value_count" => Ok(GeneratedField::ActiveUserPrincipalAttributeValueCount),
                            "activeUserGroupPrincipalAttributeValueCount" | "active_user_group_principal_attribute_value_count" => Ok(GeneratedField::ActiveUserGroupPrincipalAttributeValueCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPrincipalAttributeKeyArchiveImpactResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.CheckPrincipalAttributeKeyArchiveImpactResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckPrincipalAttributeKeyArchiveImpactResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active_user_principal_attribute_value_count__ = None;
                let mut active_user_group_principal_attribute_value_count__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActiveUserPrincipalAttributeValueCount => {
                            if active_user_principal_attribute_value_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeUserPrincipalAttributeValueCount"));
                            }
                            active_user_principal_attribute_value_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ActiveUserGroupPrincipalAttributeValueCount => {
                            if active_user_group_principal_attribute_value_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeUserGroupPrincipalAttributeValueCount"));
                            }
                            active_user_group_principal_attribute_value_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CheckPrincipalAttributeKeyArchiveImpactResponse {
                    active_user_principal_attribute_value_count: active_user_principal_attribute_value_count__.unwrap_or_default(),
                    active_user_group_principal_attribute_value_count: active_user_group_principal_attribute_value_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.CheckPrincipalAttributeKeyArchiveImpactResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeEnumValueRequest", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
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
impl<'de> serde::Deserialize<'de> for CreatePrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
            "display_name",
            "displayName",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyId,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
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
            type Value = CreatePrincipalAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.CreatePrincipalAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePrincipalAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
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
                Ok(CreatePrincipalAttributeEnumValueRequest {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_enum_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeEnumValueResponse", len)?;
        if let Some(v) = self.principal_attribute_enum_value.as_ref() {
            struct_ser.serialize_field("principalAttributeEnumValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value",
            "principalAttributeEnumValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValue,
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
                            "principalAttributeEnumValue" | "principal_attribute_enum_value" => Ok(GeneratedField::PrincipalAttributeEnumValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePrincipalAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.CreatePrincipalAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePrincipalAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValue => {
                            if principal_attribute_enum_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValue"));
                            }
                            principal_attribute_enum_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePrincipalAttributeEnumValueResponse {
                    principal_attribute_enum_value: principal_attribute_enum_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePrincipalAttributeKeyRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeKeyRequest", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.r#type != 0 {
            let v = PrincipalAttributeValueType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.initial_enum_values.is_empty() {
            struct_ser.serialize_field("initialEnumValues", &self.initial_enum_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePrincipalAttributeKeyRequest {
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
            type Value = CreatePrincipalAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.CreatePrincipalAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePrincipalAttributeKeyRequest, V::Error>
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
                            r#type__ = Some(map_.next_value::<PrincipalAttributeValueType>()? as i32);
                        }
                        GeneratedField::InitialEnumValues => {
                            if initial_enum_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialEnumValues"));
                            }
                            initial_enum_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreatePrincipalAttributeKeyRequest {
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    initial_enum_values: initial_enum_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_principal_attribute_key_request::InitialEnumValue {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeKeyRequest.InitialEnumValue", len)?;
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_principal_attribute_key_request::InitialEnumValue {
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
            type Value = create_principal_attribute_key_request::InitialEnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.CreatePrincipalAttributeKeyRequest.InitialEnumValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<create_principal_attribute_key_request::InitialEnumValue, V::Error>
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
                Ok(create_principal_attribute_key_request::InitialEnumValue {
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeKeyRequest.InitialEnumValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePrincipalAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_key.is_some() {
            len += 1;
        }
        if !self.enum_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeKeyResponse", len)?;
        if let Some(v) = self.principal_attribute_key.as_ref() {
            struct_ser.serialize_field("principalAttributeKey", v)?;
        }
        if !self.enum_values.is_empty() {
            struct_ser.serialize_field("enumValues", &self.enum_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePrincipalAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key",
            "principalAttributeKey",
            "enum_values",
            "enumValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKey,
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
                            "principalAttributeKey" | "principal_attribute_key" => Ok(GeneratedField::PrincipalAttributeKey),
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
            type Value = CreatePrincipalAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.CreatePrincipalAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePrincipalAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key__ = None;
                let mut enum_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKey => {
                            if principal_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKey"));
                            }
                            principal_attribute_key__ = map_.next_value()?;
                        }
                        GeneratedField::EnumValues => {
                            if enum_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumValues"));
                            }
                            enum_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreatePrincipalAttributeKeyResponse {
                    principal_attribute_key: principal_attribute_key__,
                    enum_values: enum_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.CreatePrincipalAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_enum_value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeEnumValueRequest", len)?;
        if !self.principal_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("principalAttributeEnumValueId", &self.principal_attribute_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value_id",
            "principalAttributeEnumValueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValueId,
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
                            "principalAttributeEnumValueId" | "principal_attribute_enum_value_id" => Ok(GeneratedField::PrincipalAttributeEnumValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPrincipalAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.GetPrincipalAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPrincipalAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValueId => {
                            if principal_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueId"));
                            }
                            principal_attribute_enum_value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPrincipalAttributeEnumValueRequest {
                    principal_attribute_enum_value_id: principal_attribute_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_enum_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeEnumValueResponse", len)?;
        if let Some(v) = self.principal_attribute_enum_value.as_ref() {
            struct_ser.serialize_field("principalAttributeEnumValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value",
            "principalAttributeEnumValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValue,
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
                            "principalAttributeEnumValue" | "principal_attribute_enum_value" => Ok(GeneratedField::PrincipalAttributeEnumValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPrincipalAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.GetPrincipalAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPrincipalAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValue => {
                            if principal_attribute_enum_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValue"));
                            }
                            principal_attribute_enum_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPrincipalAttributeEnumValueResponse {
                    principal_attribute_enum_value: principal_attribute_enum_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPrincipalAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeKeyRequest", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPrincipalAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyId,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPrincipalAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.GetPrincipalAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPrincipalAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPrincipalAttributeKeyRequest {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPrincipalAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeKeyResponse", len)?;
        if let Some(v) = self.principal_attribute_key.as_ref() {
            struct_ser.serialize_field("principalAttributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPrincipalAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key",
            "principalAttributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKey,
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
                            "principalAttributeKey" | "principal_attribute_key" => Ok(GeneratedField::PrincipalAttributeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPrincipalAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.GetPrincipalAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPrincipalAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKey => {
                            if principal_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKey"));
                            }
                            principal_attribute_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPrincipalAttributeKeyResponse {
                    principal_attribute_key: principal_attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPrincipalAttributeValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_value_id.is_empty() {
            len += 1;
        }
        if self.principal_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeValueRequest", len)?;
        if !self.principal_attribute_value_id.is_empty() {
            struct_ser.serialize_field("principalAttributeValueId", &self.principal_attribute_value_id)?;
        }
        if self.principal_type != 0 {
            let v = PrincipalAttributePrincipalType::try_from(self.principal_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.principal_type)))?;
            struct_ser.serialize_field("principalType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPrincipalAttributeValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_value_id",
            "principalAttributeValueId",
            "principal_type",
            "principalType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValueId,
            PrincipalType,
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
                            "principalAttributeValueId" | "principal_attribute_value_id" => Ok(GeneratedField::PrincipalAttributeValueId),
                            "principalType" | "principal_type" => Ok(GeneratedField::PrincipalType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPrincipalAttributeValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.GetPrincipalAttributeValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPrincipalAttributeValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_value_id__ = None;
                let mut principal_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValueId => {
                            if principal_attribute_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValueId"));
                            }
                            principal_attribute_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalType => {
                            if principal_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalType"));
                            }
                            principal_type__ = Some(map_.next_value::<PrincipalAttributePrincipalType>()? as i32);
                        }
                    }
                }
                Ok(GetPrincipalAttributeValueRequest {
                    principal_attribute_value_id: principal_attribute_value_id__.unwrap_or_default(),
                    principal_type: principal_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPrincipalAttributeValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeValueResponse", len)?;
        if let Some(v) = self.principal_attribute_value.as_ref() {
            struct_ser.serialize_field("principalAttributeValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPrincipalAttributeValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_value",
            "principalAttributeValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValue,
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
                            "principalAttributeValue" | "principal_attribute_value" => Ok(GeneratedField::PrincipalAttributeValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPrincipalAttributeValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.GetPrincipalAttributeValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPrincipalAttributeValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValue => {
                            if principal_attribute_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValue"));
                            }
                            principal_attribute_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPrincipalAttributeValueResponse {
                    principal_attribute_value: principal_attribute_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.GetPrincipalAttributeValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeEnumValuesRequest", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
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
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeEnumValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
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
            PrincipalAttributeKeyId,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
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
            type Value = ListPrincipalAttributeEnumValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeEnumValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeEnumValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut include_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
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
                Ok(ListPrincipalAttributeEnumValuesRequest {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeEnumValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_enum_values.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeEnumValuesResponse", len)?;
        if !self.principal_attribute_enum_values.is_empty() {
            struct_ser.serialize_field("principalAttributeEnumValues", &self.principal_attribute_enum_values)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeEnumValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_values",
            "principalAttributeEnumValues",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValues,
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
                            "principalAttributeEnumValues" | "principal_attribute_enum_values" => Ok(GeneratedField::PrincipalAttributeEnumValues),
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
            type Value = ListPrincipalAttributeEnumValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeEnumValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeEnumValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_values__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValues => {
                            if principal_attribute_enum_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValues"));
                            }
                            principal_attribute_enum_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPrincipalAttributeEnumValuesResponse {
                    principal_attribute_enum_values: principal_attribute_enum_values__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeEnumValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeKeyValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
            len += 1;
        }
        if self.principal_type != 0 {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeyValuesRequest", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
        }
        if self.principal_type != 0 {
            let v = PrincipalAttributePrincipalType::try_from(self.principal_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.principal_type)))?;
            struct_ser.serialize_field("principalType", &v)?;
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
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeKeyValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
            "principal_type",
            "principalType",
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
            PrincipalAttributeKeyId,
            PrincipalType,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
                            "principalType" | "principal_type" => Ok(GeneratedField::PrincipalType),
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
            type Value = ListPrincipalAttributeKeyValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeKeyValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeKeyValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                let mut principal_type__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut include_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalType => {
                            if principal_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalType"));
                            }
                            principal_type__ = Some(map_.next_value::<PrincipalAttributePrincipalType>()? as i32);
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
                Ok(ListPrincipalAttributeKeyValuesRequest {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    principal_type: principal_type__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeyValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeKeyValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_values.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeyValuesResponse", len)?;
        if !self.principal_attribute_values.is_empty() {
            struct_ser.serialize_field("principalAttributeValues", &self.principal_attribute_values)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeKeyValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_values",
            "principalAttributeValues",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValues,
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
                            "principalAttributeValues" | "principal_attribute_values" => Ok(GeneratedField::PrincipalAttributeValues),
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
            type Value = ListPrincipalAttributeKeyValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeKeyValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeKeyValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_values__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValues => {
                            if principal_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValues"));
                            }
                            principal_attribute_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPrincipalAttributeKeyValuesResponse {
                    principal_attribute_values: principal_attribute_values__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeyValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeKeysRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeysRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeKeysRequest {
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
            type Value = ListPrincipalAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeKeysRequest, V::Error>
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
                Ok(ListPrincipalAttributeKeysRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_keys.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeysResponse", len)?;
        if !self.principal_attribute_keys.is_empty() {
            struct_ser.serialize_field("principalAttributeKeys", &self.principal_attribute_keys)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeKeysResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_keys",
            "principalAttributeKeys",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeys,
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
                            "principalAttributeKeys" | "principal_attribute_keys" => Ok(GeneratedField::PrincipalAttributeKeys),
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
            type Value = ListPrincipalAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_keys__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeys => {
                            if principal_attribute_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeys"));
                            }
                            principal_attribute_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPrincipalAttributeKeysResponse {
                    principal_attribute_keys: principal_attribute_keys__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_type != 0 {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeValuesRequest", len)?;
        if self.principal_type != 0 {
            let v = PrincipalAttributePrincipalType::try_from(self.principal_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.principal_type)))?;
            struct_ser.serialize_field("principalType", &v)?;
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
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_type",
            "principalType",
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
            PrincipalType,
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
                            "principalType" | "principal_type" => Ok(GeneratedField::PrincipalType),
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
            type Value = ListPrincipalAttributeValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_type__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut include_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalType => {
                            if principal_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalType"));
                            }
                            principal_type__ = Some(map_.next_value::<PrincipalAttributePrincipalType>()? as i32);
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
                Ok(ListPrincipalAttributeValuesRequest {
                    principal_type: principal_type__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPrincipalAttributeValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_values.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeValuesResponse", len)?;
        if !self.principal_attribute_values.is_empty() {
            struct_ser.serialize_field("principalAttributeValues", &self.principal_attribute_values)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPrincipalAttributeValuesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_values",
            "principalAttributeValues",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValues,
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
                            "principalAttributeValues" | "principal_attribute_values" => Ok(GeneratedField::PrincipalAttributeValues),
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
            type Value = ListPrincipalAttributeValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.ListPrincipalAttributeValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPrincipalAttributeValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_values__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValues => {
                            if principal_attribute_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValues"));
                            }
                            principal_attribute_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPrincipalAttributeValuesResponse {
                    principal_attribute_values: principal_attribute_values__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.ListPrincipalAttributeValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrincipalAttributeEnumValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_enum_value_id.is_empty() {
            len += 1;
        }
        if !self.principal_attribute_key_id.is_empty() {
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
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.PrincipalAttributeEnumValue", len)?;
        if !self.principal_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("principalAttributeEnumValueId", &self.principal_attribute_enum_value_id)?;
        }
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
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
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrincipalAttributeEnumValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value_id",
            "principalAttributeEnumValueId",
            "principal_attribute_key_id",
            "principalAttributeKeyId",
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
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValueId,
            PrincipalAttributeKeyId,
            DisplayName,
            Description,
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
                            "principalAttributeEnumValueId" | "principal_attribute_enum_value_id" => Ok(GeneratedField::PrincipalAttributeEnumValueId),
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "description" => Ok(GeneratedField::Description),
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
            type Value = PrincipalAttributeEnumValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.PrincipalAttributeEnumValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrincipalAttributeEnumValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value_id__ = None;
                let mut principal_attribute_key_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_date__ = None;
                let mut modified_by_user_id__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValueId => {
                            if principal_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueId"));
                            }
                            principal_attribute_enum_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
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
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PrincipalAttributeEnumValue {
                    principal_attribute_enum_value_id: principal_attribute_enum_value_id__.unwrap_or_default(),
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_date: modified_date__,
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.PrincipalAttributeEnumValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrincipalAttributeEnumValueIdList {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.PrincipalAttributeEnumValueIdList", len)?;
        if !self.ids.is_empty() {
            struct_ser.serialize_field("ids", &self.ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrincipalAttributeEnumValueIdList {
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
            type Value = PrincipalAttributeEnumValueIdList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.PrincipalAttributeEnumValueIdList")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrincipalAttributeEnumValueIdList, V::Error>
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
                Ok(PrincipalAttributeEnumValueIdList {
                    ids: ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.PrincipalAttributeEnumValueIdList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrincipalAttributeKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
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
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.PrincipalAttributeKey", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
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
            let v = PrincipalAttributeValueType::try_from(self.r#type)
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
impl<'de> serde::Deserialize<'de> for PrincipalAttributeKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
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
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyId,
            OrganizationId,
            DisplayName,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
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
            type Value = PrincipalAttributeKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.PrincipalAttributeKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrincipalAttributeKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                let mut organization_id__ = None;
                let mut display_name__ = None;
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
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
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
                            r#type__ = Some(map_.next_value::<PrincipalAttributeValueType>()? as i32);
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
                Ok(PrincipalAttributeKey {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
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
        deserializer.deserialize_struct("sift.principal_attributes.v1.PrincipalAttributeKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrincipalAttributePrincipalType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_UNSPECIFIED",
            Self::User => "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER",
            Self::UserGroup => "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER_GROUP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PrincipalAttributePrincipalType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_UNSPECIFIED",
            "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER",
            "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER_GROUP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrincipalAttributePrincipalType;

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
                    "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_UNSPECIFIED" => Ok(PrincipalAttributePrincipalType::Unspecified),
                    "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER" => Ok(PrincipalAttributePrincipalType::User),
                    "PRINCIPAL_ATTRIBUTE_PRINCIPAL_TYPE_USER_GROUP" => Ok(PrincipalAttributePrincipalType::UserGroup),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PrincipalAttributeValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_value_id.is_empty() {
            len += 1;
        }
        if !self.principal_attribute_key_id.is_empty() {
            len += 1;
        }
        if !self.principal_id.is_empty() {
            len += 1;
        }
        if self.principal_type != 0 {
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
        if self.enum_value_details.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.PrincipalAttributeValue", len)?;
        if !self.principal_attribute_value_id.is_empty() {
            struct_ser.serialize_field("principalAttributeValueId", &self.principal_attribute_value_id)?;
        }
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
        }
        if !self.principal_id.is_empty() {
            struct_ser.serialize_field("principalId", &self.principal_id)?;
        }
        if self.principal_type != 0 {
            let v = PrincipalAttributePrincipalType::try_from(self.principal_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.principal_type)))?;
            struct_ser.serialize_field("principalType", &v)?;
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
        if let Some(v) = self.enum_value_details.as_ref() {
            struct_ser.serialize_field("enumValueDetails", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                principal_attribute_value::Value::PrincipalAttributeEnumValueId(v) => {
                    struct_ser.serialize_field("principalAttributeEnumValueId", v)?;
                }
                principal_attribute_value::Value::NumberValue(v) => {
                    #[allow(clippy::needless_borrow)]
                    #[allow(clippy::needless_borrows_for_generic_args)]
                    struct_ser.serialize_field("numberValue", ToString::to_string(&v).as_str())?;
                }
                principal_attribute_value::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrincipalAttributeValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_value_id",
            "principalAttributeValueId",
            "principal_attribute_key_id",
            "principalAttributeKeyId",
            "principal_id",
            "principalId",
            "principal_type",
            "principalType",
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
            "enum_value_details",
            "enumValueDetails",
            "principal_attribute_enum_value_id",
            "principalAttributeEnumValueId",
            "number_value",
            "numberValue",
            "boolean_value",
            "booleanValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValueId,
            PrincipalAttributeKeyId,
            PrincipalId,
            PrincipalType,
            OrganizationId,
            CreatedByUserId,
            CreatedDate,
            ArchivedDate,
            IsArchived,
            Key,
            EnumValueDetails,
            PrincipalAttributeEnumValueId,
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
                            "principalAttributeValueId" | "principal_attribute_value_id" => Ok(GeneratedField::PrincipalAttributeValueId),
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
                            "principalId" | "principal_id" => Ok(GeneratedField::PrincipalId),
                            "principalType" | "principal_type" => Ok(GeneratedField::PrincipalType),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "isArchived" | "is_archived" => Ok(GeneratedField::IsArchived),
                            "key" => Ok(GeneratedField::Key),
                            "enumValueDetails" | "enum_value_details" => Ok(GeneratedField::EnumValueDetails),
                            "principalAttributeEnumValueId" | "principal_attribute_enum_value_id" => Ok(GeneratedField::PrincipalAttributeEnumValueId),
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
            type Value = PrincipalAttributeValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.PrincipalAttributeValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PrincipalAttributeValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_value_id__ = None;
                let mut principal_attribute_key_id__ = None;
                let mut principal_id__ = None;
                let mut principal_type__ = None;
                let mut organization_id__ = None;
                let mut created_by_user_id__ = None;
                let mut created_date__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                let mut key__ = None;
                let mut enum_value_details__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValueId => {
                            if principal_attribute_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValueId"));
                            }
                            principal_attribute_value_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalId => {
                            if principal_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalId"));
                            }
                            principal_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalType => {
                            if principal_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalType"));
                            }
                            principal_type__ = Some(map_.next_value::<PrincipalAttributePrincipalType>()? as i32);
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
                        GeneratedField::EnumValueDetails => {
                            if enum_value_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumValueDetails"));
                            }
                            enum_value_details__ = map_.next_value()?;
                        }
                        GeneratedField::PrincipalAttributeEnumValueId => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueId"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(principal_attribute_value::Value::PrincipalAttributeEnumValueId);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| principal_attribute_value::Value::NumberValue(x.0));
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(principal_attribute_value::Value::BooleanValue);
                        }
                    }
                }
                Ok(PrincipalAttributeValue {
                    principal_attribute_value_id: principal_attribute_value_id__.unwrap_or_default(),
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    principal_id: principal_id__.unwrap_or_default(),
                    principal_type: principal_type__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                    key: key__,
                    enum_value_details: enum_value_details__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.PrincipalAttributeValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrincipalAttributeValueType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            Self::Enum => "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_ENUM",
            Self::Boolean => "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_BOOLEAN",
            Self::Number => "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_NUMBER",
            Self::SetOfEnum => "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PrincipalAttributeValueType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_UNSPECIFIED",
            "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_ENUM",
            "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_BOOLEAN",
            "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_NUMBER",
            "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrincipalAttributeValueType;

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
                    "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_UNSPECIFIED" => Ok(PrincipalAttributeValueType::Unspecified),
                    "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_ENUM" => Ok(PrincipalAttributeValueType::Enum),
                    "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_BOOLEAN" => Ok(PrincipalAttributeValueType::Boolean),
                    "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_NUMBER" => Ok(PrincipalAttributeValueType::Number),
                    "PRINCIPAL_ATTRIBUTE_VALUE_TYPE_SET_OF_ENUM" => Ok(PrincipalAttributeValueType::SetOfEnum),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchivePrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_enum_value_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeEnumValueRequest", len)?;
        if !self.principal_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("principalAttributeEnumValueId", &self.principal_attribute_enum_value_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchivePrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value_id",
            "principalAttributeEnumValueId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValueId,
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
                            "principalAttributeEnumValueId" | "principal_attribute_enum_value_id" => Ok(GeneratedField::PrincipalAttributeEnumValueId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchivePrincipalAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UnarchivePrincipalAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchivePrincipalAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValueId => {
                            if principal_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueId"));
                            }
                            principal_attribute_enum_value_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchivePrincipalAttributeEnumValueRequest {
                    principal_attribute_enum_value_id: principal_attribute_enum_value_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchivePrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeEnumValueResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchivePrincipalAttributeEnumValueResponse {
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
            type Value = UnarchivePrincipalAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UnarchivePrincipalAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchivePrincipalAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchivePrincipalAttributeEnumValueResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchivePrincipalAttributeKeysRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeKeysRequest", len)?;
        if !self.principal_attribute_key_ids.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyIds", &self.principal_attribute_key_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchivePrincipalAttributeKeysRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_ids",
            "principalAttributeKeyIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyIds,
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
                            "principalAttributeKeyIds" | "principal_attribute_key_ids" => Ok(GeneratedField::PrincipalAttributeKeyIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchivePrincipalAttributeKeysRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UnarchivePrincipalAttributeKeysRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchivePrincipalAttributeKeysRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyIds => {
                            if principal_attribute_key_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyIds"));
                            }
                            principal_attribute_key_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchivePrincipalAttributeKeysRequest {
                    principal_attribute_key_ids: principal_attribute_key_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeKeysRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchivePrincipalAttributeKeysResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeKeysResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchivePrincipalAttributeKeysResponse {
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
            type Value = UnarchivePrincipalAttributeKeysResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UnarchivePrincipalAttributeKeysResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchivePrincipalAttributeKeysResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchivePrincipalAttributeKeysResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeKeysResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchivePrincipalAttributeValuesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_value_ids.is_empty() {
            len += 1;
        }
        if self.principal_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeValuesRequest", len)?;
        if !self.principal_attribute_value_ids.is_empty() {
            struct_ser.serialize_field("principalAttributeValueIds", &self.principal_attribute_value_ids)?;
        }
        if self.principal_type != 0 {
            let v = PrincipalAttributePrincipalType::try_from(self.principal_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.principal_type)))?;
            struct_ser.serialize_field("principalType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchivePrincipalAttributeValuesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_value_ids",
            "principalAttributeValueIds",
            "principal_type",
            "principalType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeValueIds,
            PrincipalType,
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
                            "principalAttributeValueIds" | "principal_attribute_value_ids" => Ok(GeneratedField::PrincipalAttributeValueIds),
                            "principalType" | "principal_type" => Ok(GeneratedField::PrincipalType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchivePrincipalAttributeValuesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UnarchivePrincipalAttributeValuesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchivePrincipalAttributeValuesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_value_ids__ = None;
                let mut principal_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeValueIds => {
                            if principal_attribute_value_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeValueIds"));
                            }
                            principal_attribute_value_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrincipalType => {
                            if principal_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalType"));
                            }
                            principal_type__ = Some(map_.next_value::<PrincipalAttributePrincipalType>()? as i32);
                        }
                    }
                }
                Ok(UnarchivePrincipalAttributeValuesRequest {
                    principal_attribute_value_ids: principal_attribute_value_ids__.unwrap_or_default(),
                    principal_type: principal_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeValuesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchivePrincipalAttributeValuesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeValuesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchivePrincipalAttributeValuesResponse {
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
            type Value = UnarchivePrincipalAttributeValuesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UnarchivePrincipalAttributeValuesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchivePrincipalAttributeValuesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchivePrincipalAttributeValuesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UnarchivePrincipalAttributeValuesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_enum_value_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeEnumValueRequest", len)?;
        if !self.principal_attribute_enum_value_id.is_empty() {
            struct_ser.serialize_field("principalAttributeEnumValueId", &self.principal_attribute_enum_value_id)?;
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
impl<'de> serde::Deserialize<'de> for UpdatePrincipalAttributeEnumValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value_id",
            "principalAttributeEnumValueId",
            "display_name",
            "displayName",
            "description",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValueId,
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
                            "principalAttributeEnumValueId" | "principal_attribute_enum_value_id" => Ok(GeneratedField::PrincipalAttributeEnumValueId),
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
            type Value = UpdatePrincipalAttributeEnumValueRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UpdatePrincipalAttributeEnumValueRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePrincipalAttributeEnumValueRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValueId => {
                            if principal_attribute_enum_value_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValueId"));
                            }
                            principal_attribute_enum_value_id__ = Some(map_.next_value()?);
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
                Ok(UpdatePrincipalAttributeEnumValueRequest {
                    principal_attribute_enum_value_id: principal_attribute_enum_value_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeEnumValueRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_enum_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeEnumValueResponse", len)?;
        if let Some(v) = self.principal_attribute_enum_value.as_ref() {
            struct_ser.serialize_field("principalAttributeEnumValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePrincipalAttributeEnumValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_enum_value",
            "principalAttributeEnumValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeEnumValue,
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
                            "principalAttributeEnumValue" | "principal_attribute_enum_value" => Ok(GeneratedField::PrincipalAttributeEnumValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePrincipalAttributeEnumValueResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UpdatePrincipalAttributeEnumValueResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePrincipalAttributeEnumValueResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_enum_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeEnumValue => {
                            if principal_attribute_enum_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeEnumValue"));
                            }
                            principal_attribute_enum_value__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePrincipalAttributeEnumValueResponse {
                    principal_attribute_enum_value: principal_attribute_enum_value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeEnumValueResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePrincipalAttributeKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.principal_attribute_key_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeKeyRequest", len)?;
        if !self.principal_attribute_key_id.is_empty() {
            struct_ser.serialize_field("principalAttributeKeyId", &self.principal_attribute_key_id)?;
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
impl<'de> serde::Deserialize<'de> for UpdatePrincipalAttributeKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key_id",
            "principalAttributeKeyId",
            "display_name",
            "displayName",
            "description",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKeyId,
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
                            "principalAttributeKeyId" | "principal_attribute_key_id" => Ok(GeneratedField::PrincipalAttributeKeyId),
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
            type Value = UpdatePrincipalAttributeKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UpdatePrincipalAttributeKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePrincipalAttributeKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key_id__ = None;
                let mut display_name__ = None;
                let mut description__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKeyId => {
                            if principal_attribute_key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKeyId"));
                            }
                            principal_attribute_key_id__ = Some(map_.next_value()?);
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
                Ok(UpdatePrincipalAttributeKeyRequest {
                    principal_attribute_key_id: principal_attribute_key_id__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePrincipalAttributeKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.principal_attribute_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeKeyResponse", len)?;
        if let Some(v) = self.principal_attribute_key.as_ref() {
            struct_ser.serialize_field("principalAttributeKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePrincipalAttributeKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "principal_attribute_key",
            "principalAttributeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrincipalAttributeKey,
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
                            "principalAttributeKey" | "principal_attribute_key" => Ok(GeneratedField::PrincipalAttributeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePrincipalAttributeKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.principal_attributes.v1.UpdatePrincipalAttributeKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePrincipalAttributeKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut principal_attribute_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrincipalAttributeKey => {
                            if principal_attribute_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("principalAttributeKey"));
                            }
                            principal_attribute_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePrincipalAttributeKeyResponse {
                    principal_attribute_key: principal_attribute_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.principal_attributes.v1.UpdatePrincipalAttributeKeyResponse", FIELDS, GeneratedVisitor)
    }
}
