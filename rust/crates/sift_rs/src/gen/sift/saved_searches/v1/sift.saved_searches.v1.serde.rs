// @generated
impl serde::Serialize for BatchDeleteSavedSearchesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.saved_search_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.BatchDeleteSavedSearchesRequest", len)?;
        if !self.saved_search_ids.is_empty() {
            struct_ser.serialize_field("savedSearchIds", &self.saved_search_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteSavedSearchesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search_ids",
            "savedSearchIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearchIds,
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
                            "savedSearchIds" | "saved_search_ids" => Ok(GeneratedField::SavedSearchIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchDeleteSavedSearchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.BatchDeleteSavedSearchesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteSavedSearchesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearchIds => {
                            if saved_search_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearchIds"));
                            }
                            saved_search_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchDeleteSavedSearchesRequest {
                    saved_search_ids: saved_search_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.BatchDeleteSavedSearchesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchDeleteSavedSearchesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.saved_searches.v1.BatchDeleteSavedSearchesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteSavedSearchesResponse {
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
            type Value = BatchDeleteSavedSearchesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.BatchDeleteSavedSearchesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteSavedSearchesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchDeleteSavedSearchesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.BatchDeleteSavedSearchesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSavedSearchRequest {
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
        if self.properties.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.CreateSavedSearchRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSavedSearchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "properties",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Properties,
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
                            "name" => Ok(GeneratedField::Name),
                            "properties" => Ok(GeneratedField::Properties),
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
            type Value = CreateSavedSearchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.CreateSavedSearchRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSavedSearchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut properties__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateSavedSearchRequest {
                    name: name__.unwrap_or_default(),
                    properties: properties__,
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.CreateSavedSearchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSavedSearchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.saved_search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.CreateSavedSearchResponse", len)?;
        if let Some(v) = self.saved_search.as_ref() {
            struct_ser.serialize_field("savedSearch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSavedSearchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search",
            "savedSearch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearch,
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
                            "savedSearch" | "saved_search" => Ok(GeneratedField::SavedSearch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateSavedSearchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.CreateSavedSearchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSavedSearchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearch => {
                            if saved_search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearch"));
                            }
                            saved_search__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateSavedSearchResponse {
                    saved_search: saved_search__,
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.CreateSavedSearchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSavedSearchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.saved_search_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.DeleteSavedSearchRequest", len)?;
        if !self.saved_search_id.is_empty() {
            struct_ser.serialize_field("savedSearchId", &self.saved_search_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSavedSearchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search_id",
            "savedSearchId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearchId,
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
                            "savedSearchId" | "saved_search_id" => Ok(GeneratedField::SavedSearchId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteSavedSearchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.DeleteSavedSearchRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSavedSearchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearchId => {
                            if saved_search_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearchId"));
                            }
                            saved_search_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteSavedSearchRequest {
                    saved_search_id: saved_search_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.DeleteSavedSearchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSavedSearchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.saved_searches.v1.DeleteSavedSearchResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSavedSearchResponse {
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
            type Value = DeleteSavedSearchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.DeleteSavedSearchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSavedSearchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteSavedSearchResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.DeleteSavedSearchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSavedSearchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.saved_search_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.GetSavedSearchRequest", len)?;
        if !self.saved_search_id.is_empty() {
            struct_ser.serialize_field("savedSearchId", &self.saved_search_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSavedSearchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search_id",
            "savedSearchId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearchId,
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
                            "savedSearchId" | "saved_search_id" => Ok(GeneratedField::SavedSearchId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSavedSearchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.GetSavedSearchRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSavedSearchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearchId => {
                            if saved_search_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearchId"));
                            }
                            saved_search_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetSavedSearchRequest {
                    saved_search_id: saved_search_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.GetSavedSearchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSavedSearchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.saved_search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.GetSavedSearchResponse", len)?;
        if let Some(v) = self.saved_search.as_ref() {
            struct_ser.serialize_field("savedSearch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSavedSearchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search",
            "savedSearch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearch,
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
                            "savedSearch" | "saved_search" => Ok(GeneratedField::SavedSearch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetSavedSearchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.GetSavedSearchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSavedSearchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearch => {
                            if saved_search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearch"));
                            }
                            saved_search__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetSavedSearchResponse {
                    saved_search: saved_search__,
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.GetSavedSearchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSavedSearchesRequest {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.ListSavedSearchesRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSavedSearchesRequest {
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
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
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
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
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
            type Value = ListSavedSearchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.ListSavedSearchesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSavedSearchesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut organization_id__ = None;
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListSavedSearchesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.ListSavedSearchesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSavedSearchesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.saved_searches.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.ListSavedSearchesResponse", len)?;
        if !self.saved_searches.is_empty() {
            struct_ser.serialize_field("savedSearches", &self.saved_searches)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSavedSearchesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_searches",
            "savedSearches",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearches,
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
                            "savedSearches" | "saved_searches" => Ok(GeneratedField::SavedSearches),
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
            type Value = ListSavedSearchesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.ListSavedSearchesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSavedSearchesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_searches__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearches => {
                            if saved_searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearches"));
                            }
                            saved_searches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListSavedSearchesResponse {
                    saved_searches: saved_searches__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.ListSavedSearchesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SavedSearch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.saved_search_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.SavedSearch", len)?;
        if !self.saved_search_id.is_empty() {
            struct_ser.serialize_field("savedSearchId", &self.saved_search_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SavedSearch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search_id",
            "savedSearchId",
            "organization_id",
            "organizationId",
            "name",
            "properties",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearchId,
            OrganizationId,
            Name,
            Properties,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
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
                            "savedSearchId" | "saved_search_id" => Ok(GeneratedField::SavedSearchId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "name" => Ok(GeneratedField::Name),
                            "properties" => Ok(GeneratedField::Properties),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SavedSearch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.SavedSearch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SavedSearch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search_id__ = None;
                let mut organization_id__ = None;
                let mut name__ = None;
                let mut properties__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearchId => {
                            if saved_search_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearchId"));
                            }
                            saved_search_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map_.next_value()?;
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
                    }
                }
                Ok(SavedSearch {
                    saved_search_id: saved_search_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    properties: properties__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.SavedSearch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SavedSearchFilterItem {
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
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.SavedSearchFilterItem", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SavedSearchFilterItem {
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
            type Value = SavedSearchFilterItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.SavedSearchFilterItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SavedSearchFilterItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SavedSearchFilterItem {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.SavedSearchFilterItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SavedSearchMetadataItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.SavedSearchMetadataItem", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                saved_search_metadata_item::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                saved_search_metadata_item::Value::NumberValue(v) => {
                    struct_ser.serialize_field("numberValue", v)?;
                }
                saved_search_metadata_item::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SavedSearchMetadataItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
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
            type Value = SavedSearchMetadataItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.SavedSearchMetadataItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SavedSearchMetadataItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(saved_search_metadata_item::Value::StringValue);
                        }
                        GeneratedField::NumberValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| saved_search_metadata_item::Value::NumberValue(x.0));
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(saved_search_metadata_item::Value::BooleanValue);
                        }
                    }
                }
                Ok(SavedSearchMetadataItem {
                    key: key__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.SavedSearchMetadataItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SavedSearchProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.overview_mode.is_empty() {
            len += 1;
        }
        if self.search_term.is_some() {
            len += 1;
        }
        if self.from_date_time.is_some() {
            len += 1;
        }
        if self.to_date_time.is_some() {
            len += 1;
        }
        if !self.asset_items.is_empty() {
            len += 1;
        }
        if !self.user_items.is_empty() {
            len += 1;
        }
        if !self.tag_items.is_empty() {
            len += 1;
        }
        if !self.annotation_items.is_empty() {
            len += 1;
        }
        if !self.run_items.is_empty() {
            len += 1;
        }
        if !self.report_template_items.is_empty() {
            len += 1;
        }
        if self.show_advanced_filters.is_some() {
            len += 1;
        }
        if self.include_archived.is_some() {
            len += 1;
        }
        if self.order_by.is_some() {
            len += 1;
        }
        if !self.metadata_items.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.SavedSearchProperties", len)?;
        if !self.overview_mode.is_empty() {
            struct_ser.serialize_field("overviewMode", &self.overview_mode)?;
        }
        if let Some(v) = self.search_term.as_ref() {
            struct_ser.serialize_field("searchTerm", v)?;
        }
        if let Some(v) = self.from_date_time.as_ref() {
            struct_ser.serialize_field("fromDateTime", v)?;
        }
        if let Some(v) = self.to_date_time.as_ref() {
            struct_ser.serialize_field("toDateTime", v)?;
        }
        if !self.asset_items.is_empty() {
            struct_ser.serialize_field("assetItems", &self.asset_items)?;
        }
        if !self.user_items.is_empty() {
            struct_ser.serialize_field("userItems", &self.user_items)?;
        }
        if !self.tag_items.is_empty() {
            struct_ser.serialize_field("tagItems", &self.tag_items)?;
        }
        if !self.annotation_items.is_empty() {
            struct_ser.serialize_field("annotationItems", &self.annotation_items)?;
        }
        if !self.run_items.is_empty() {
            struct_ser.serialize_field("runItems", &self.run_items)?;
        }
        if !self.report_template_items.is_empty() {
            struct_ser.serialize_field("reportTemplateItems", &self.report_template_items)?;
        }
        if let Some(v) = self.show_advanced_filters.as_ref() {
            struct_ser.serialize_field("showAdvancedFilters", v)?;
        }
        if let Some(v) = self.include_archived.as_ref() {
            struct_ser.serialize_field("includeArchived", v)?;
        }
        if let Some(v) = self.order_by.as_ref() {
            struct_ser.serialize_field("orderBy", v)?;
        }
        if !self.metadata_items.is_empty() {
            struct_ser.serialize_field("metadataItems", &self.metadata_items)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SavedSearchProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overview_mode",
            "overviewMode",
            "search_term",
            "searchTerm",
            "from_date_time",
            "fromDateTime",
            "to_date_time",
            "toDateTime",
            "asset_items",
            "assetItems",
            "user_items",
            "userItems",
            "tag_items",
            "tagItems",
            "annotation_items",
            "annotationItems",
            "run_items",
            "runItems",
            "report_template_items",
            "reportTemplateItems",
            "show_advanced_filters",
            "showAdvancedFilters",
            "include_archived",
            "includeArchived",
            "order_by",
            "orderBy",
            "metadata_items",
            "metadataItems",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OverviewMode,
            SearchTerm,
            FromDateTime,
            ToDateTime,
            AssetItems,
            UserItems,
            TagItems,
            AnnotationItems,
            RunItems,
            ReportTemplateItems,
            ShowAdvancedFilters,
            IncludeArchived,
            OrderBy,
            MetadataItems,
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
                            "overviewMode" | "overview_mode" => Ok(GeneratedField::OverviewMode),
                            "searchTerm" | "search_term" => Ok(GeneratedField::SearchTerm),
                            "fromDateTime" | "from_date_time" => Ok(GeneratedField::FromDateTime),
                            "toDateTime" | "to_date_time" => Ok(GeneratedField::ToDateTime),
                            "assetItems" | "asset_items" => Ok(GeneratedField::AssetItems),
                            "userItems" | "user_items" => Ok(GeneratedField::UserItems),
                            "tagItems" | "tag_items" => Ok(GeneratedField::TagItems),
                            "annotationItems" | "annotation_items" => Ok(GeneratedField::AnnotationItems),
                            "runItems" | "run_items" => Ok(GeneratedField::RunItems),
                            "reportTemplateItems" | "report_template_items" => Ok(GeneratedField::ReportTemplateItems),
                            "showAdvancedFilters" | "show_advanced_filters" => Ok(GeneratedField::ShowAdvancedFilters),
                            "includeArchived" | "include_archived" => Ok(GeneratedField::IncludeArchived),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "metadataItems" | "metadata_items" => Ok(GeneratedField::MetadataItems),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SavedSearchProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.SavedSearchProperties")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SavedSearchProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overview_mode__ = None;
                let mut search_term__ = None;
                let mut from_date_time__ = None;
                let mut to_date_time__ = None;
                let mut asset_items__ = None;
                let mut user_items__ = None;
                let mut tag_items__ = None;
                let mut annotation_items__ = None;
                let mut run_items__ = None;
                let mut report_template_items__ = None;
                let mut show_advanced_filters__ = None;
                let mut include_archived__ = None;
                let mut order_by__ = None;
                let mut metadata_items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OverviewMode => {
                            if overview_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overviewMode"));
                            }
                            overview_mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SearchTerm => {
                            if search_term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchTerm"));
                            }
                            search_term__ = map_.next_value()?;
                        }
                        GeneratedField::FromDateTime => {
                            if from_date_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromDateTime"));
                            }
                            from_date_time__ = map_.next_value()?;
                        }
                        GeneratedField::ToDateTime => {
                            if to_date_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("toDateTime"));
                            }
                            to_date_time__ = map_.next_value()?;
                        }
                        GeneratedField::AssetItems => {
                            if asset_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetItems"));
                            }
                            asset_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserItems => {
                            if user_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userItems"));
                            }
                            user_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TagItems => {
                            if tag_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagItems"));
                            }
                            tag_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AnnotationItems => {
                            if annotation_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationItems"));
                            }
                            annotation_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunItems => {
                            if run_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runItems"));
                            }
                            run_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReportTemplateItems => {
                            if report_template_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportTemplateItems"));
                            }
                            report_template_items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ShowAdvancedFilters => {
                            if show_advanced_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("showAdvancedFilters"));
                            }
                            show_advanced_filters__ = map_.next_value()?;
                        }
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = map_.next_value()?;
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataItems => {
                            if metadata_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataItems"));
                            }
                            metadata_items__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SavedSearchProperties {
                    overview_mode: overview_mode__.unwrap_or_default(),
                    search_term: search_term__,
                    from_date_time: from_date_time__,
                    to_date_time: to_date_time__,
                    asset_items: asset_items__.unwrap_or_default(),
                    user_items: user_items__.unwrap_or_default(),
                    tag_items: tag_items__.unwrap_or_default(),
                    annotation_items: annotation_items__.unwrap_or_default(),
                    run_items: run_items__.unwrap_or_default(),
                    report_template_items: report_template_items__.unwrap_or_default(),
                    show_advanced_filters: show_advanced_filters__,
                    include_archived: include_archived__,
                    order_by: order_by__,
                    metadata_items: metadata_items__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.SavedSearchProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSavedSearchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.saved_search.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.UpdateSavedSearchRequest", len)?;
        if let Some(v) = self.saved_search.as_ref() {
            struct_ser.serialize_field("savedSearch", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSavedSearchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search",
            "savedSearch",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearch,
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
                            "savedSearch" | "saved_search" => Ok(GeneratedField::SavedSearch),
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
            type Value = UpdateSavedSearchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.UpdateSavedSearchRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateSavedSearchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearch => {
                            if saved_search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearch"));
                            }
                            saved_search__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateSavedSearchRequest {
                    saved_search: saved_search__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.UpdateSavedSearchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateSavedSearchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.saved_search.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.saved_searches.v1.UpdateSavedSearchResponse", len)?;
        if let Some(v) = self.saved_search.as_ref() {
            struct_ser.serialize_field("savedSearch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateSavedSearchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "saved_search",
            "savedSearch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SavedSearch,
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
                            "savedSearch" | "saved_search" => Ok(GeneratedField::SavedSearch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateSavedSearchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.saved_searches.v1.UpdateSavedSearchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateSavedSearchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut saved_search__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SavedSearch => {
                            if saved_search__.is_some() {
                                return Err(serde::de::Error::duplicate_field("savedSearch"));
                            }
                            saved_search__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateSavedSearchResponse {
                    saved_search: saved_search__,
                })
            }
        }
        deserializer.deserialize_struct("sift.saved_searches.v1.UpdateSavedSearchResponse", FIELDS, GeneratedVisitor)
    }
}
