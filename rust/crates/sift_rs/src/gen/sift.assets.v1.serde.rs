// @generated
impl serde::Serialize for ArchiveAssetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if self.archive_runs {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.ArchiveAssetRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if self.archive_runs {
            struct_ser.serialize_field("archiveRuns", &self.archive_runs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveAssetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "archive_runs",
            "archiveRuns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            ArchiveRuns,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "archiveRuns" | "archive_runs" => Ok(GeneratedField::ArchiveRuns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveAssetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.ArchiveAssetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveAssetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut archive_runs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArchiveRuns => {
                            if archive_runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archiveRuns"));
                            }
                            archive_runs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveAssetRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                    archive_runs: archive_runs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.ArchiveAssetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveAssetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.archived_run_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.ArchiveAssetResponse", len)?;
        if !self.archived_run_ids.is_empty() {
            struct_ser.serialize_field("archivedRunIds", &self.archived_run_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveAssetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "archived_run_ids",
            "archivedRunIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArchivedRunIds,
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
                            "archivedRunIds" | "archived_run_ids" => Ok(GeneratedField::ArchivedRunIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveAssetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.ArchiveAssetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveAssetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut archived_run_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArchivedRunIds => {
                            if archived_run_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedRunIds"));
                            }
                            archived_run_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveAssetResponse {
                    archived_run_ids: archived_run_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.ArchiveAssetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Asset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
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
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.Asset", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
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
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
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
impl<'de> serde::Deserialize<'de> for Asset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "name",
            "organization_id",
            "organizationId",
            "created_date",
            "createdDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_date",
            "modifiedDate",
            "modified_by_user_id",
            "modifiedByUserId",
            "tags",
            "metadata",
            "archived_date",
            "archivedDate",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            Name,
            OrganizationId,
            CreatedDate,
            CreatedByUserId,
            ModifiedDate,
            ModifiedByUserId,
            Tags,
            Metadata,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "name" => Ok(GeneratedField::Name),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "tags" => Ok(GeneratedField::Tags),
                            "metadata" => Ok(GeneratedField::Metadata),
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
            type Value = Asset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.Asset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Asset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut name__ = None;
                let mut organization_id__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_date__ = None;
                let mut modified_by_user_id__ = None;
                let mut tags__ = None;
                let mut metadata__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
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
                Ok(Asset {
                    asset_id: asset_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_date: modified_date__,
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.Asset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAssetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if self.archive_runs {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.DeleteAssetRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if self.archive_runs {
            struct_ser.serialize_field("archiveRuns", &self.archive_runs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAssetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "archive_runs",
            "archiveRuns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            ArchiveRuns,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "archiveRuns" | "archive_runs" => Ok(GeneratedField::ArchiveRuns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteAssetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.DeleteAssetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAssetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut archive_runs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArchiveRuns => {
                            if archive_runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archiveRuns"));
                            }
                            archive_runs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteAssetRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                    archive_runs: archive_runs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.DeleteAssetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteAssetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.assets.v1.DeleteAssetResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteAssetResponse {
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
            type Value = DeleteAssetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.DeleteAssetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteAssetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteAssetResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.DeleteAssetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAssetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.GetAssetRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAssetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAssetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.GetAssetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAssetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetAssetRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.GetAssetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAssetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.asset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.GetAssetResponse", len)?;
        if let Some(v) = self.asset.as_ref() {
            struct_ser.serialize_field("asset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAssetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Asset,
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
                            "asset" => Ok(GeneratedField::Asset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAssetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.GetAssetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetAssetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Asset => {
                            if asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asset"));
                            }
                            asset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetAssetResponse {
                    asset: asset__,
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.GetAssetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAssetsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.ListAssetsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListAssetsRequest {
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
            type Value = ListAssetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.ListAssetsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAssetsRequest, V::Error>
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
                Ok(ListAssetsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.ListAssetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAssetsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assets.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.ListAssetsResponse", len)?;
        if !self.assets.is_empty() {
            struct_ser.serialize_field("assets", &self.assets)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAssetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assets",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assets,
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
                            "assets" => Ok(GeneratedField::Assets),
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
            type Value = ListAssetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.ListAssetsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAssetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assets__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAssetsResponse {
                    assets: assets__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.ListAssetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAssetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.asset.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.UpdateAssetRequest", len)?;
        if let Some(v) = self.asset.as_ref() {
            struct_ser.serialize_field("asset", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAssetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Asset,
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
                            "asset" => Ok(GeneratedField::Asset),
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
            type Value = UpdateAssetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.UpdateAssetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAssetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Asset => {
                            if asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asset"));
                            }
                            asset__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAssetRequest {
                    asset: asset__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.UpdateAssetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAssetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.asset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.assets.v1.UpdateAssetResponse", len)?;
        if let Some(v) = self.asset.as_ref() {
            struct_ser.serialize_field("asset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAssetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Asset,
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
                            "asset" => Ok(GeneratedField::Asset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateAssetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.assets.v1.UpdateAssetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAssetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Asset => {
                            if asset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asset"));
                            }
                            asset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateAssetResponse {
                    asset: asset__,
                })
            }
        }
        deserializer.deserialize_struct("sift.assets.v1.UpdateAssetResponse", FIELDS, GeneratedVisitor)
    }
}
