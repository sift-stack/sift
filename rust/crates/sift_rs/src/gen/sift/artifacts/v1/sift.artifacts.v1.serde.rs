// @generated
impl serde::Serialize for ArchiveArtifactRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.ArchiveArtifactRequest", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveArtifactRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchiveArtifactRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.ArchiveArtifactRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveArtifactRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchiveArtifactRequest {
                    artifact_id: artifact_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.ArchiveArtifactRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchiveArtifactResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.artifacts.v1.ArchiveArtifactResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchiveArtifactResponse {
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
            type Value = ArchiveArtifactResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.ArchiveArtifactResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchiveArtifactResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ArchiveArtifactResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.ArchiveArtifactResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Artifact {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if self.authoring_kind != 0 {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if !self.artifact_version_id.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if self.title.is_some() {
            len += 1;
        }
        if self.summary.is_some() {
            len += 1;
        }
        if self.authoring_message_id.is_some() {
            len += 1;
        }
        if !self.source_tool_use_ids.is_empty() {
            len += 1;
        }
        if self.remote_file_id.is_some() {
            len += 1;
        }
        if self.version_created_date.is_some() {
            len += 1;
        }
        if self.file_name.is_some() {
            len += 1;
        }
        if self.file_mime_type.is_some() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.Artifact", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if self.authoring_kind != 0 {
            let v = ArtifactAuthoringKind::try_from(self.authoring_kind)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.authoring_kind)))?;
            struct_ser.serialize_field("authoringKind", &v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if !self.artifact_version_id.is_empty() {
            struct_ser.serialize_field("artifactVersionId", &self.artifact_version_id)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        if let Some(v) = self.authoring_message_id.as_ref() {
            struct_ser.serialize_field("authoringMessageId", v)?;
        }
        if !self.source_tool_use_ids.is_empty() {
            struct_ser.serialize_field("sourceToolUseIds", &self.source_tool_use_ids)?;
        }
        if let Some(v) = self.remote_file_id.as_ref() {
            struct_ser.serialize_field("remoteFileId", v)?;
        }
        if let Some(v) = self.version_created_date.as_ref() {
            struct_ser.serialize_field("versionCreatedDate", v)?;
        }
        if let Some(v) = self.file_name.as_ref() {
            struct_ser.serialize_field("fileName", v)?;
        }
        if let Some(v) = self.file_mime_type.as_ref() {
            struct_ser.serialize_field("fileMimeType", v)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Artifact {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
            "organization_id",
            "organizationId",
            "created_by_user_id",
            "createdByUserId",
            "authoring_kind",
            "authoringKind",
            "created_date",
            "createdDate",
            "artifact_version_id",
            "artifactVersionId",
            "version",
            "title",
            "summary",
            "authoring_message_id",
            "authoringMessageId",
            "source_tool_use_ids",
            "sourceToolUseIds",
            "remote_file_id",
            "remoteFileId",
            "version_created_date",
            "versionCreatedDate",
            "file_name",
            "fileName",
            "file_mime_type",
            "fileMimeType",
            "archived_date",
            "archivedDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
            OrganizationId,
            CreatedByUserId,
            AuthoringKind,
            CreatedDate,
            ArtifactVersionId,
            Version,
            Title,
            Summary,
            AuthoringMessageId,
            SourceToolUseIds,
            RemoteFileId,
            VersionCreatedDate,
            FileName,
            FileMimeType,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "authoringKind" | "authoring_kind" => Ok(GeneratedField::AuthoringKind),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "artifactVersionId" | "artifact_version_id" => Ok(GeneratedField::ArtifactVersionId),
                            "version" => Ok(GeneratedField::Version),
                            "title" => Ok(GeneratedField::Title),
                            "summary" => Ok(GeneratedField::Summary),
                            "authoringMessageId" | "authoring_message_id" => Ok(GeneratedField::AuthoringMessageId),
                            "sourceToolUseIds" | "source_tool_use_ids" => Ok(GeneratedField::SourceToolUseIds),
                            "remoteFileId" | "remote_file_id" => Ok(GeneratedField::RemoteFileId),
                            "versionCreatedDate" | "version_created_date" => Ok(GeneratedField::VersionCreatedDate),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "fileMimeType" | "file_mime_type" => Ok(GeneratedField::FileMimeType),
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
            type Value = Artifact;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.Artifact")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Artifact, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                let mut organization_id__ = None;
                let mut created_by_user_id__ = None;
                let mut authoring_kind__ = None;
                let mut created_date__ = None;
                let mut artifact_version_id__ = None;
                let mut version__ = None;
                let mut title__ = None;
                let mut summary__ = None;
                let mut authoring_message_id__ = None;
                let mut source_tool_use_ids__ = None;
                let mut remote_file_id__ = None;
                let mut version_created_date__ = None;
                let mut file_name__ = None;
                let mut file_mime_type__ = None;
                let mut archived_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
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
                        GeneratedField::AuthoringKind => {
                            if authoring_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoringKind"));
                            }
                            authoring_kind__ = Some(map_.next_value::<ArtifactAuthoringKind>()? as i32);
                        }
                        GeneratedField::CreatedDate => {
                            if created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdDate"));
                            }
                            created_date__ = map_.next_value()?;
                        }
                        GeneratedField::ArtifactVersionId => {
                            if artifact_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactVersionId"));
                            }
                            artifact_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
                        }
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = map_.next_value()?;
                        }
                        GeneratedField::AuthoringMessageId => {
                            if authoring_message_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoringMessageId"));
                            }
                            authoring_message_id__ = map_.next_value()?;
                        }
                        GeneratedField::SourceToolUseIds => {
                            if source_tool_use_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceToolUseIds"));
                            }
                            source_tool_use_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteFileId => {
                            if remote_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileId"));
                            }
                            remote_file_id__ = map_.next_value()?;
                        }
                        GeneratedField::VersionCreatedDate => {
                            if version_created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionCreatedDate"));
                            }
                            version_created_date__ = map_.next_value()?;
                        }
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = map_.next_value()?;
                        }
                        GeneratedField::FileMimeType => {
                            if file_mime_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileMimeType"));
                            }
                            file_mime_type__ = map_.next_value()?;
                        }
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Artifact {
                    artifact_id: artifact_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    authoring_kind: authoring_kind__.unwrap_or_default(),
                    created_date: created_date__,
                    artifact_version_id: artifact_version_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    title: title__,
                    summary: summary__,
                    authoring_message_id: authoring_message_id__,
                    source_tool_use_ids: source_tool_use_ids__.unwrap_or_default(),
                    remote_file_id: remote_file_id__,
                    version_created_date: version_created_date__,
                    file_name: file_name__,
                    file_mime_type: file_mime_type__,
                    archived_date: archived_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.Artifact", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArtifactAuthoringKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ARTIFACT_AUTHORING_KIND_UNSPECIFIED",
            Self::Agent => "ARTIFACT_AUTHORING_KIND_AGENT",
            Self::User => "ARTIFACT_AUTHORING_KIND_USER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ArtifactAuthoringKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ARTIFACT_AUTHORING_KIND_UNSPECIFIED",
            "ARTIFACT_AUTHORING_KIND_AGENT",
            "ARTIFACT_AUTHORING_KIND_USER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArtifactAuthoringKind;

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
                    "ARTIFACT_AUTHORING_KIND_UNSPECIFIED" => Ok(ArtifactAuthoringKind::Unspecified),
                    "ARTIFACT_AUTHORING_KIND_AGENT" => Ok(ArtifactAuthoringKind::Agent),
                    "ARTIFACT_AUTHORING_KIND_USER" => Ok(ArtifactAuthoringKind::User),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ArtifactVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_version_id.is_empty() {
            len += 1;
        }
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if self.title.is_some() {
            len += 1;
        }
        if self.summary.is_some() {
            len += 1;
        }
        if self.authoring_message_id.is_some() {
            len += 1;
        }
        if !self.source_tool_use_ids.is_empty() {
            len += 1;
        }
        if self.remote_file_id.is_some() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.file_name.is_some() {
            len += 1;
        }
        if self.file_mime_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.ArtifactVersion", len)?;
        if !self.artifact_version_id.is_empty() {
            struct_ser.serialize_field("artifactVersionId", &self.artifact_version_id)?;
        }
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        if let Some(v) = self.authoring_message_id.as_ref() {
            struct_ser.serialize_field("authoringMessageId", v)?;
        }
        if !self.source_tool_use_ids.is_empty() {
            struct_ser.serialize_field("sourceToolUseIds", &self.source_tool_use_ids)?;
        }
        if let Some(v) = self.remote_file_id.as_ref() {
            struct_ser.serialize_field("remoteFileId", v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.file_name.as_ref() {
            struct_ser.serialize_field("fileName", v)?;
        }
        if let Some(v) = self.file_mime_type.as_ref() {
            struct_ser.serialize_field("fileMimeType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArtifactVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_version_id",
            "artifactVersionId",
            "artifact_id",
            "artifactId",
            "version",
            "title",
            "summary",
            "authoring_message_id",
            "authoringMessageId",
            "source_tool_use_ids",
            "sourceToolUseIds",
            "remote_file_id",
            "remoteFileId",
            "created_date",
            "createdDate",
            "file_name",
            "fileName",
            "file_mime_type",
            "fileMimeType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactVersionId,
            ArtifactId,
            Version,
            Title,
            Summary,
            AuthoringMessageId,
            SourceToolUseIds,
            RemoteFileId,
            CreatedDate,
            FileName,
            FileMimeType,
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
                            "artifactVersionId" | "artifact_version_id" => Ok(GeneratedField::ArtifactVersionId),
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            "version" => Ok(GeneratedField::Version),
                            "title" => Ok(GeneratedField::Title),
                            "summary" => Ok(GeneratedField::Summary),
                            "authoringMessageId" | "authoring_message_id" => Ok(GeneratedField::AuthoringMessageId),
                            "sourceToolUseIds" | "source_tool_use_ids" => Ok(GeneratedField::SourceToolUseIds),
                            "remoteFileId" | "remote_file_id" => Ok(GeneratedField::RemoteFileId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "fileMimeType" | "file_mime_type" => Ok(GeneratedField::FileMimeType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArtifactVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.ArtifactVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArtifactVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_version_id__ = None;
                let mut artifact_id__ = None;
                let mut version__ = None;
                let mut title__ = None;
                let mut summary__ = None;
                let mut authoring_message_id__ = None;
                let mut source_tool_use_ids__ = None;
                let mut remote_file_id__ = None;
                let mut created_date__ = None;
                let mut file_name__ = None;
                let mut file_mime_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactVersionId => {
                            if artifact_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactVersionId"));
                            }
                            artifact_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
                        }
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = map_.next_value()?;
                        }
                        GeneratedField::AuthoringMessageId => {
                            if authoring_message_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoringMessageId"));
                            }
                            authoring_message_id__ = map_.next_value()?;
                        }
                        GeneratedField::SourceToolUseIds => {
                            if source_tool_use_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceToolUseIds"));
                            }
                            source_tool_use_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RemoteFileId => {
                            if remote_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileId"));
                            }
                            remote_file_id__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedDate => {
                            if created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdDate"));
                            }
                            created_date__ = map_.next_value()?;
                        }
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = map_.next_value()?;
                        }
                        GeneratedField::FileMimeType => {
                            if file_mime_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileMimeType"));
                            }
                            file_mime_type__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ArtifactVersion {
                    artifact_version_id: artifact_version_id__.unwrap_or_default(),
                    artifact_id: artifact_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    title: title__,
                    summary: summary__,
                    authoring_message_id: authoring_message_id__,
                    source_tool_use_ids: source_tool_use_ids__.unwrap_or_default(),
                    remote_file_id: remote_file_id__,
                    created_date: created_date__,
                    file_name: file_name__,
                    file_mime_type: file_mime_type__,
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.ArtifactVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateArtifactRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.artifact_id.is_some() {
            len += 1;
        }
        if self.conversation_id.is_some() {
            len += 1;
        }
        if self.title.is_some() {
            len += 1;
        }
        if self.summary.is_some() {
            len += 1;
        }
        if self.authoring_kind.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.CreateArtifactRequest", len)?;
        if let Some(v) = self.artifact_id.as_ref() {
            struct_ser.serialize_field("artifactId", v)?;
        }
        if let Some(v) = self.conversation_id.as_ref() {
            struct_ser.serialize_field("conversationId", v)?;
        }
        if let Some(v) = self.title.as_ref() {
            struct_ser.serialize_field("title", v)?;
        }
        if let Some(v) = self.summary.as_ref() {
            struct_ser.serialize_field("summary", v)?;
        }
        if let Some(v) = self.authoring_kind.as_ref() {
            let v = ArtifactAuthoringKind::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("authoringKind", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateArtifactRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
            "conversation_id",
            "conversationId",
            "title",
            "summary",
            "authoring_kind",
            "authoringKind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
            ConversationId,
            Title,
            Summary,
            AuthoringKind,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            "conversationId" | "conversation_id" => Ok(GeneratedField::ConversationId),
                            "title" => Ok(GeneratedField::Title),
                            "summary" => Ok(GeneratedField::Summary),
                            "authoringKind" | "authoring_kind" => Ok(GeneratedField::AuthoringKind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateArtifactRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.CreateArtifactRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateArtifactRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                let mut conversation_id__ = None;
                let mut title__ = None;
                let mut summary__ = None;
                let mut authoring_kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = map_.next_value()?;
                        }
                        GeneratedField::ConversationId => {
                            if conversation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationId"));
                            }
                            conversation_id__ = map_.next_value()?;
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = map_.next_value()?;
                        }
                        GeneratedField::Summary => {
                            if summary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            summary__ = map_.next_value()?;
                        }
                        GeneratedField::AuthoringKind => {
                            if authoring_kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoringKind"));
                            }
                            authoring_kind__ = map_.next_value::<::std::option::Option<ArtifactAuthoringKind>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(CreateArtifactRequest {
                    artifact_id: artifact_id__,
                    conversation_id: conversation_id__,
                    title: title__,
                    summary: summary__,
                    authoring_kind: authoring_kind__,
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.CreateArtifactRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateArtifactResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.artifact.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.CreateArtifactResponse", len)?;
        if let Some(v) = self.artifact.as_ref() {
            struct_ser.serialize_field("artifact", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateArtifactResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Artifact,
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
                            "artifact" => Ok(GeneratedField::Artifact),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateArtifactResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.CreateArtifactResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateArtifactResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Artifact => {
                            if artifact__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifact"));
                            }
                            artifact__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateArtifactResponse {
                    artifact: artifact__,
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.CreateArtifactResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetArtifactRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        if self.artifact_version_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.GetArtifactRequest", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        if let Some(v) = self.artifact_version_id.as_ref() {
            struct_ser.serialize_field("artifactVersionId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetArtifactRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
            "artifact_version_id",
            "artifactVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
            ArtifactVersionId,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            "artifactVersionId" | "artifact_version_id" => Ok(GeneratedField::ArtifactVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetArtifactRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.GetArtifactRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetArtifactRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                let mut artifact_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArtifactVersionId => {
                            if artifact_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactVersionId"));
                            }
                            artifact_version_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetArtifactRequest {
                    artifact_id: artifact_id__.unwrap_or_default(),
                    artifact_version_id: artifact_version_id__,
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.GetArtifactRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetArtifactResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.artifact.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.GetArtifactResponse", len)?;
        if let Some(v) = self.artifact.as_ref() {
            struct_ser.serialize_field("artifact", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetArtifactResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Artifact,
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
                            "artifact" => Ok(GeneratedField::Artifact),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetArtifactResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.GetArtifactResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetArtifactResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Artifact => {
                            if artifact__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifact"));
                            }
                            artifact__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetArtifactResponse {
                    artifact: artifact__,
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.GetArtifactResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinkArtifactToConversationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        if !self.conversation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.LinkArtifactToConversationRequest", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        if !self.conversation_id.is_empty() {
            struct_ser.serialize_field("conversationId", &self.conversation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkArtifactToConversationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
            "conversation_id",
            "conversationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
            ConversationId,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            "conversationId" | "conversation_id" => Ok(GeneratedField::ConversationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LinkArtifactToConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.LinkArtifactToConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LinkArtifactToConversationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                let mut conversation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConversationId => {
                            if conversation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationId"));
                            }
                            conversation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LinkArtifactToConversationRequest {
                    artifact_id: artifact_id__.unwrap_or_default(),
                    conversation_id: conversation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.LinkArtifactToConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LinkArtifactToConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.artifacts.v1.LinkArtifactToConversationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LinkArtifactToConversationResponse {
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
            type Value = LinkArtifactToConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.LinkArtifactToConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LinkArtifactToConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(LinkArtifactToConversationResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.LinkArtifactToConversationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListArtifactVersionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.ListArtifactVersionsRequest", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
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
impl<'de> serde::Deserialize<'de> for ListArtifactVersionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
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
            type Value = ListArtifactVersionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.ListArtifactVersionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListArtifactVersionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
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
                Ok(ListArtifactVersionsRequest {
                    artifact_id: artifact_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.ListArtifactVersionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListArtifactVersionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.versions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.ListArtifactVersionsResponse", len)?;
        if !self.versions.is_empty() {
            struct_ser.serialize_field("versions", &self.versions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListArtifactVersionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "versions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Versions,
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
                            "versions" => Ok(GeneratedField::Versions),
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
            type Value = ListArtifactVersionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.ListArtifactVersionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListArtifactVersionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut versions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Versions => {
                            if versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            versions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListArtifactVersionsResponse {
                    versions: versions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.ListArtifactVersionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListArtifactsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.conversation_id.is_some() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if self.include_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.ListArtifactsRequest", len)?;
        if let Some(v) = self.conversation_id.as_ref() {
            struct_ser.serialize_field("conversationId", v)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if self.include_archived {
            struct_ser.serialize_field("includeArchived", &self.include_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListArtifactsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "conversation_id",
            "conversationId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "include_archived",
            "includeArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConversationId,
            PageSize,
            PageToken,
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
                            "conversationId" | "conversation_id" => Ok(GeneratedField::ConversationId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
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
            type Value = ListArtifactsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.ListArtifactsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListArtifactsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut conversation_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut include_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConversationId => {
                            if conversation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationId"));
                            }
                            conversation_id__ = map_.next_value()?;
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
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListArtifactsRequest {
                    conversation_id: conversation_id__,
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.ListArtifactsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListArtifactsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifacts.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.ListArtifactsResponse", len)?;
        if !self.artifacts.is_empty() {
            struct_ser.serialize_field("artifacts", &self.artifacts)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListArtifactsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifacts",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Artifacts,
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
                            "artifacts" => Ok(GeneratedField::Artifacts),
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
            type Value = ListArtifactsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.ListArtifactsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListArtifactsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifacts__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Artifacts => {
                            if artifacts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifacts"));
                            }
                            artifacts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListArtifactsResponse {
                    artifacts: artifacts__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.ListArtifactsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveArtifactRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.UnarchiveArtifactRequest", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveArtifactRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnarchiveArtifactRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.UnarchiveArtifactRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveArtifactRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnarchiveArtifactRequest {
                    artifact_id: artifact_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.UnarchiveArtifactRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnarchiveArtifactResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.artifacts.v1.UnarchiveArtifactResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnarchiveArtifactResponse {
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
            type Value = UnarchiveArtifactResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.UnarchiveArtifactResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnarchiveArtifactResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnarchiveArtifactResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.UnarchiveArtifactResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnlinkArtifactFromConversationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.artifact_id.is_empty() {
            len += 1;
        }
        if !self.conversation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.artifacts.v1.UnlinkArtifactFromConversationRequest", len)?;
        if !self.artifact_id.is_empty() {
            struct_ser.serialize_field("artifactId", &self.artifact_id)?;
        }
        if !self.conversation_id.is_empty() {
            struct_ser.serialize_field("conversationId", &self.conversation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnlinkArtifactFromConversationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_id",
            "artifactId",
            "conversation_id",
            "conversationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactId,
            ConversationId,
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
                            "artifactId" | "artifact_id" => Ok(GeneratedField::ArtifactId),
                            "conversationId" | "conversation_id" => Ok(GeneratedField::ConversationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnlinkArtifactFromConversationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.UnlinkArtifactFromConversationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnlinkArtifactFromConversationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_id__ = None;
                let mut conversation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ArtifactId => {
                            if artifact_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactId"));
                            }
                            artifact_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConversationId => {
                            if conversation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conversationId"));
                            }
                            conversation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnlinkArtifactFromConversationRequest {
                    artifact_id: artifact_id__.unwrap_or_default(),
                    conversation_id: conversation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.UnlinkArtifactFromConversationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnlinkArtifactFromConversationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.artifacts.v1.UnlinkArtifactFromConversationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnlinkArtifactFromConversationResponse {
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
            type Value = UnlinkArtifactFromConversationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.artifacts.v1.UnlinkArtifactFromConversationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnlinkArtifactFromConversationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnlinkArtifactFromConversationResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.artifacts.v1.UnlinkArtifactFromConversationResponse", FIELDS, GeneratedVisitor)
    }
}
