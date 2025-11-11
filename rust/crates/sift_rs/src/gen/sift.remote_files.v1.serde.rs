// @generated
impl serde::Serialize for AudioMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.duration_seconds != 0. {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.AudioMetadata", len)?;
        if self.duration_seconds != 0. {
            struct_ser.serialize_field("durationSeconds", &self.duration_seconds)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AudioMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "duration_seconds",
            "durationSeconds",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DurationSeconds,
            Timestamp,
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
                            "durationSeconds" | "duration_seconds" => Ok(GeneratedField::DurationSeconds),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AudioMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.AudioMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AudioMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut duration_seconds__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DurationSeconds => {
                            if duration_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationSeconds"));
                            }
                            duration_seconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AudioMetadata {
                    duration_seconds: duration_seconds__.unwrap_or_default(),
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.AudioMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchDeleteRemoteFilesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_file_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.BatchDeleteRemoteFilesRequest", len)?;
        if !self.remote_file_ids.is_empty() {
            struct_ser.serialize_field("remoteFileIds", &self.remote_file_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteRemoteFilesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file_ids",
            "remoteFileIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFileIds,
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
                            "remoteFileIds" | "remote_file_ids" => Ok(GeneratedField::RemoteFileIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchDeleteRemoteFilesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.BatchDeleteRemoteFilesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteRemoteFilesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFileIds => {
                            if remote_file_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileIds"));
                            }
                            remote_file_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchDeleteRemoteFilesRequest {
                    remote_file_ids: remote_file_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.BatchDeleteRemoteFilesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchDeleteRemoteFilesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.remote_files.v1.BatchDeleteRemoteFilesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchDeleteRemoteFilesResponse {
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
            type Value = BatchDeleteRemoteFilesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.BatchDeleteRemoteFilesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchDeleteRemoteFilesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchDeleteRemoteFilesResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.BatchDeleteRemoteFilesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRemoteFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_name.is_empty() {
            len += 1;
        }
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if self.entity_type != 0 {
            len += 1;
        }
        if !self.file_mime_type.is_empty() {
            len += 1;
        }
        if !self.file_content_encoding.is_empty() {
            len += 1;
        }
        if self.file_size != 0 {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.custom_uuid.is_some() {
            len += 1;
        }
        if !self.metadata_values.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.CreateRemoteFileRequest", len)?;
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if self.entity_type != 0 {
            let v = EntityType::try_from(self.entity_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.entity_type)))?;
            struct_ser.serialize_field("entityType", &v)?;
        }
        if !self.file_mime_type.is_empty() {
            struct_ser.serialize_field("fileMimeType", &self.file_mime_type)?;
        }
        if !self.file_content_encoding.is_empty() {
            struct_ser.serialize_field("fileContentEncoding", &self.file_content_encoding)?;
        }
        if self.file_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("fileSize", ToString::to_string(&self.file_size).as_str())?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.custom_uuid.as_ref() {
            struct_ser.serialize_field("customUuid", v)?;
        }
        if !self.metadata_values.is_empty() {
            struct_ser.serialize_field("metadataValues", &self.metadata_values)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            match v {
                create_remote_file_request::Metadata::VideoMetadata(v) => {
                    struct_ser.serialize_field("videoMetadata", v)?;
                }
                create_remote_file_request::Metadata::ImageMetadata(v) => {
                    struct_ser.serialize_field("imageMetadata", v)?;
                }
                create_remote_file_request::Metadata::AudioMetadata(v) => {
                    struct_ser.serialize_field("audioMetadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRemoteFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_name",
            "fileName",
            "entity_id",
            "entityId",
            "entity_type",
            "entityType",
            "file_mime_type",
            "fileMimeType",
            "file_content_encoding",
            "fileContentEncoding",
            "file_size",
            "fileSize",
            "description",
            "organization_id",
            "organizationId",
            "custom_uuid",
            "customUuid",
            "metadata_values",
            "metadataValues",
            "video_metadata",
            "videoMetadata",
            "image_metadata",
            "imageMetadata",
            "audio_metadata",
            "audioMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FileName,
            EntityId,
            EntityType,
            FileMimeType,
            FileContentEncoding,
            FileSize,
            Description,
            OrganizationId,
            CustomUuid,
            MetadataValues,
            VideoMetadata,
            ImageMetadata,
            AudioMetadata,
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
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "entityType" | "entity_type" => Ok(GeneratedField::EntityType),
                            "fileMimeType" | "file_mime_type" => Ok(GeneratedField::FileMimeType),
                            "fileContentEncoding" | "file_content_encoding" => Ok(GeneratedField::FileContentEncoding),
                            "fileSize" | "file_size" => Ok(GeneratedField::FileSize),
                            "description" => Ok(GeneratedField::Description),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "customUuid" | "custom_uuid" => Ok(GeneratedField::CustomUuid),
                            "metadataValues" | "metadata_values" => Ok(GeneratedField::MetadataValues),
                            "videoMetadata" | "video_metadata" => Ok(GeneratedField::VideoMetadata),
                            "imageMetadata" | "image_metadata" => Ok(GeneratedField::ImageMetadata),
                            "audioMetadata" | "audio_metadata" => Ok(GeneratedField::AudioMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRemoteFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.CreateRemoteFileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRemoteFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_name__ = None;
                let mut entity_id__ = None;
                let mut entity_type__ = None;
                let mut file_mime_type__ = None;
                let mut file_content_encoding__ = None;
                let mut file_size__ = None;
                let mut description__ = None;
                let mut organization_id__ = None;
                let mut custom_uuid__ = None;
                let mut metadata_values__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map_.next_value()?);
                        }
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
                            entity_type__ = Some(map_.next_value::<EntityType>()? as i32);
                        }
                        GeneratedField::FileMimeType => {
                            if file_mime_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileMimeType"));
                            }
                            file_mime_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileContentEncoding => {
                            if file_content_encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileContentEncoding"));
                            }
                            file_content_encoding__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileSize => {
                            if file_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSize"));
                            }
                            file_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CustomUuid => {
                            if custom_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customUuid"));
                            }
                            custom_uuid__ = map_.next_value()?;
                        }
                        GeneratedField::MetadataValues => {
                            if metadata_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataValues"));
                            }
                            metadata_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VideoMetadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("videoMetadata"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(create_remote_file_request::Metadata::VideoMetadata)
;
                        }
                        GeneratedField::ImageMetadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageMetadata"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(create_remote_file_request::Metadata::ImageMetadata)
;
                        }
                        GeneratedField::AudioMetadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audioMetadata"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(create_remote_file_request::Metadata::AudioMetadata)
;
                        }
                    }
                }
                Ok(CreateRemoteFileRequest {
                    file_name: file_name__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                    entity_type: entity_type__.unwrap_or_default(),
                    file_mime_type: file_mime_type__.unwrap_or_default(),
                    file_content_encoding: file_content_encoding__.unwrap_or_default(),
                    file_size: file_size__.unwrap_or_default(),
                    description: description__,
                    organization_id: organization_id__.unwrap_or_default(),
                    custom_uuid: custom_uuid__,
                    metadata_values: metadata_values__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.CreateRemoteFileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRemoteFileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote_file.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.CreateRemoteFileResponse", len)?;
        if let Some(v) = self.remote_file.as_ref() {
            struct_ser.serialize_field("remoteFile", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRemoteFileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file",
            "remoteFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFile,
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
                            "remoteFile" | "remote_file" => Ok(GeneratedField::RemoteFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRemoteFileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.CreateRemoteFileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateRemoteFileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFile => {
                            if remote_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFile"));
                            }
                            remote_file__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateRemoteFileResponse {
                    remote_file: remote_file__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.CreateRemoteFileResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRemoteFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_file_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.DeleteRemoteFileRequest", len)?;
        if !self.remote_file_id.is_empty() {
            struct_ser.serialize_field("remoteFileId", &self.remote_file_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRemoteFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file_id",
            "remoteFileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFileId,
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
                            "remoteFileId" | "remote_file_id" => Ok(GeneratedField::RemoteFileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRemoteFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.DeleteRemoteFileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRemoteFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFileId => {
                            if remote_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileId"));
                            }
                            remote_file_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteRemoteFileRequest {
                    remote_file_id: remote_file_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.DeleteRemoteFileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRemoteFileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.remote_files.v1.DeleteRemoteFileResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRemoteFileResponse {
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
            type Value = DeleteRemoteFileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.DeleteRemoteFileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteRemoteFileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteRemoteFileResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.DeleteRemoteFileResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntityType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ENTITY_TYPE_UNSPECIFIED",
            Self::Run => "ENTITY_TYPE_RUN",
            Self::Annotation => "ENTITY_TYPE_ANNOTATION",
            Self::Asset => "ENTITY_TYPE_ASSET",
            Self::AnnotationLog => "ENTITY_TYPE_ANNOTATION_LOG",
            Self::TestReport => "ENTITY_TYPE_TEST_REPORT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EntityType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENTITY_TYPE_UNSPECIFIED",
            "ENTITY_TYPE_RUN",
            "ENTITY_TYPE_ANNOTATION",
            "ENTITY_TYPE_ASSET",
            "ENTITY_TYPE_ANNOTATION_LOG",
            "ENTITY_TYPE_TEST_REPORT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntityType;

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
                    "ENTITY_TYPE_UNSPECIFIED" => Ok(EntityType::Unspecified),
                    "ENTITY_TYPE_RUN" => Ok(EntityType::Run),
                    "ENTITY_TYPE_ANNOTATION" => Ok(EntityType::Annotation),
                    "ENTITY_TYPE_ASSET" => Ok(EntityType::Asset),
                    "ENTITY_TYPE_ANNOTATION_LOG" => Ok(EntityType::AnnotationLog),
                    "ENTITY_TYPE_TEST_REPORT" => Ok(EntityType::TestReport),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetRemoteFileDownloadUrlRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_file_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.GetRemoteFileDownloadUrlRequest", len)?;
        if !self.remote_file_id.is_empty() {
            struct_ser.serialize_field("remoteFileId", &self.remote_file_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRemoteFileDownloadUrlRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file_id",
            "remoteFileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFileId,
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
                            "remoteFileId" | "remote_file_id" => Ok(GeneratedField::RemoteFileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRemoteFileDownloadUrlRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.GetRemoteFileDownloadUrlRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRemoteFileDownloadUrlRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFileId => {
                            if remote_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileId"));
                            }
                            remote_file_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetRemoteFileDownloadUrlRequest {
                    remote_file_id: remote_file_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.GetRemoteFileDownloadUrlRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRemoteFileDownloadUrlResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.download_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.GetRemoteFileDownloadUrlResponse", len)?;
        if !self.download_url.is_empty() {
            struct_ser.serialize_field("downloadUrl", &self.download_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRemoteFileDownloadUrlResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "download_url",
            "downloadUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DownloadUrl,
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
                            "downloadUrl" | "download_url" => Ok(GeneratedField::DownloadUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRemoteFileDownloadUrlResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.GetRemoteFileDownloadUrlResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRemoteFileDownloadUrlResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut download_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DownloadUrl => {
                            if download_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downloadUrl"));
                            }
                            download_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetRemoteFileDownloadUrlResponse {
                    download_url: download_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.GetRemoteFileDownloadUrlResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRemoteFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_file_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.GetRemoteFileRequest", len)?;
        if !self.remote_file_id.is_empty() {
            struct_ser.serialize_field("remoteFileId", &self.remote_file_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRemoteFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file_id",
            "remoteFileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFileId,
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
                            "remoteFileId" | "remote_file_id" => Ok(GeneratedField::RemoteFileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRemoteFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.GetRemoteFileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRemoteFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFileId => {
                            if remote_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileId"));
                            }
                            remote_file_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetRemoteFileRequest {
                    remote_file_id: remote_file_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.GetRemoteFileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRemoteFileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote_file.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.GetRemoteFileResponse", len)?;
        if let Some(v) = self.remote_file.as_ref() {
            struct_ser.serialize_field("remoteFile", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRemoteFileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file",
            "remoteFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFile,
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
                            "remoteFile" | "remote_file" => Ok(GeneratedField::RemoteFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRemoteFileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.GetRemoteFileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetRemoteFileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFile => {
                            if remote_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFile"));
                            }
                            remote_file__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetRemoteFileResponse {
                    remote_file: remote_file__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.GetRemoteFileResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImageMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if self.width != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.ImageMetadata", len)?;
        if self.height != 0 {
            struct_ser.serialize_field("height", &self.height)?;
        }
        if self.width != 0 {
            struct_ser.serialize_field("width", &self.width)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImageMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "width",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            Width,
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
                            "height" => Ok(GeneratedField::Height),
                            "width" => Ok(GeneratedField::Width),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImageMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.ImageMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImageMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut width__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ImageMetadata {
                    height: height__.unwrap_or_default(),
                    width: width__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.ImageMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRemoteFilesRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.ListRemoteFilesRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListRemoteFilesRequest {
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
            type Value = ListRemoteFilesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.ListRemoteFilesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRemoteFilesRequest, V::Error>
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
                Ok(ListRemoteFilesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.ListRemoteFilesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRemoteFilesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_files.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.ListRemoteFilesResponse", len)?;
        if !self.remote_files.is_empty() {
            struct_ser.serialize_field("remoteFiles", &self.remote_files)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRemoteFilesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_files",
            "remoteFiles",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFiles,
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
                            "remoteFiles" | "remote_files" => Ok(GeneratedField::RemoteFiles),
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
            type Value = ListRemoteFilesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.ListRemoteFilesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListRemoteFilesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_files__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFiles => {
                            if remote_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFiles"));
                            }
                            remote_files__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListRemoteFilesResponse {
                    remote_files: remote_files__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.ListRemoteFilesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoteFile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_file_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.entity_id.is_empty() {
            len += 1;
        }
        if self.entity_type != 0 {
            len += 1;
        }
        if !self.file_name.is_empty() {
            len += 1;
        }
        if !self.file_mime_type.is_empty() {
            len += 1;
        }
        if !self.file_content_encoding.is_empty() {
            len += 1;
        }
        if !self.storage_key.is_empty() {
            len += 1;
        }
        if self.file_size != 0 {
            len += 1;
        }
        if self.description.is_some() {
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
        if !self.metadata_values.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.RemoteFile", len)?;
        if !self.remote_file_id.is_empty() {
            struct_ser.serialize_field("remoteFileId", &self.remote_file_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.entity_id.is_empty() {
            struct_ser.serialize_field("entityId", &self.entity_id)?;
        }
        if self.entity_type != 0 {
            let v = EntityType::try_from(self.entity_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.entity_type)))?;
            struct_ser.serialize_field("entityType", &v)?;
        }
        if !self.file_name.is_empty() {
            struct_ser.serialize_field("fileName", &self.file_name)?;
        }
        if !self.file_mime_type.is_empty() {
            struct_ser.serialize_field("fileMimeType", &self.file_mime_type)?;
        }
        if !self.file_content_encoding.is_empty() {
            struct_ser.serialize_field("fileContentEncoding", &self.file_content_encoding)?;
        }
        if !self.storage_key.is_empty() {
            struct_ser.serialize_field("storageKey", &self.storage_key)?;
        }
        if self.file_size != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("fileSize", ToString::to_string(&self.file_size).as_str())?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
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
        if !self.metadata_values.is_empty() {
            struct_ser.serialize_field("metadataValues", &self.metadata_values)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            match v {
                remote_file::Metadata::VideoMetadata(v) => {
                    struct_ser.serialize_field("videoMetadata", v)?;
                }
                remote_file::Metadata::ImageMetadata(v) => {
                    struct_ser.serialize_field("imageMetadata", v)?;
                }
                remote_file::Metadata::AudioMetadata(v) => {
                    struct_ser.serialize_field("audioMetadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoteFile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file_id",
            "remoteFileId",
            "organization_id",
            "organizationId",
            "entity_id",
            "entityId",
            "entity_type",
            "entityType",
            "file_name",
            "fileName",
            "file_mime_type",
            "fileMimeType",
            "file_content_encoding",
            "fileContentEncoding",
            "storage_key",
            "storageKey",
            "file_size",
            "fileSize",
            "description",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "metadata_values",
            "metadataValues",
            "video_metadata",
            "videoMetadata",
            "image_metadata",
            "imageMetadata",
            "audio_metadata",
            "audioMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFileId,
            OrganizationId,
            EntityId,
            EntityType,
            FileName,
            FileMimeType,
            FileContentEncoding,
            StorageKey,
            FileSize,
            Description,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
            MetadataValues,
            VideoMetadata,
            ImageMetadata,
            AudioMetadata,
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
                            "remoteFileId" | "remote_file_id" => Ok(GeneratedField::RemoteFileId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "entityType" | "entity_type" => Ok(GeneratedField::EntityType),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "fileMimeType" | "file_mime_type" => Ok(GeneratedField::FileMimeType),
                            "fileContentEncoding" | "file_content_encoding" => Ok(GeneratedField::FileContentEncoding),
                            "storageKey" | "storage_key" => Ok(GeneratedField::StorageKey),
                            "fileSize" | "file_size" => Ok(GeneratedField::FileSize),
                            "description" => Ok(GeneratedField::Description),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "metadataValues" | "metadata_values" => Ok(GeneratedField::MetadataValues),
                            "videoMetadata" | "video_metadata" => Ok(GeneratedField::VideoMetadata),
                            "imageMetadata" | "image_metadata" => Ok(GeneratedField::ImageMetadata),
                            "audioMetadata" | "audio_metadata" => Ok(GeneratedField::AudioMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoteFile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.RemoteFile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoteFile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file_id__ = None;
                let mut organization_id__ = None;
                let mut entity_id__ = None;
                let mut entity_type__ = None;
                let mut file_name__ = None;
                let mut file_mime_type__ = None;
                let mut file_content_encoding__ = None;
                let mut storage_key__ = None;
                let mut file_size__ = None;
                let mut description__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut metadata_values__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFileId => {
                            if remote_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileId"));
                            }
                            remote_file_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
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
                            entity_type__ = Some(map_.next_value::<EntityType>()? as i32);
                        }
                        GeneratedField::FileName => {
                            if file_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            file_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileMimeType => {
                            if file_mime_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileMimeType"));
                            }
                            file_mime_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileContentEncoding => {
                            if file_content_encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileContentEncoding"));
                            }
                            file_content_encoding__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageKey => {
                            if storage_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageKey"));
                            }
                            storage_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileSize => {
                            if file_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSize"));
                            }
                            file_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
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
                        GeneratedField::MetadataValues => {
                            if metadata_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataValues"));
                            }
                            metadata_values__ = Some(map_.next_value()?);
                        }
                        GeneratedField::VideoMetadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("videoMetadata"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(remote_file::Metadata::VideoMetadata)
;
                        }
                        GeneratedField::ImageMetadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imageMetadata"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(remote_file::Metadata::ImageMetadata)
;
                        }
                        GeneratedField::AudioMetadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audioMetadata"));
                            }
                            metadata__ = map_.next_value::<::std::option::Option<_>>()?.map(remote_file::Metadata::AudioMetadata)
;
                        }
                    }
                }
                Ok(RemoteFile {
                    remote_file_id: remote_file_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    entity_id: entity_id__.unwrap_or_default(),
                    entity_type: entity_type__.unwrap_or_default(),
                    file_name: file_name__.unwrap_or_default(),
                    file_mime_type: file_mime_type__.unwrap_or_default(),
                    file_content_encoding: file_content_encoding__.unwrap_or_default(),
                    storage_key: storage_key__.unwrap_or_default(),
                    file_size: file_size__.unwrap_or_default(),
                    description: description__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    metadata_values: metadata_values__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.RemoteFile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRemoteFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote_file.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.UpdateRemoteFileRequest", len)?;
        if let Some(v) = self.remote_file.as_ref() {
            struct_ser.serialize_field("remoteFile", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRemoteFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file",
            "remoteFile",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFile,
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
                            "remoteFile" | "remote_file" => Ok(GeneratedField::RemoteFile),
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
            type Value = UpdateRemoteFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.UpdateRemoteFileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRemoteFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFile => {
                            if remote_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFile"));
                            }
                            remote_file__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateRemoteFileRequest {
                    remote_file: remote_file__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.UpdateRemoteFileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRemoteFileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.remote_file.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.UpdateRemoteFileResponse", len)?;
        if let Some(v) = self.remote_file.as_ref() {
            struct_ser.serialize_field("remoteFile", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRemoteFileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file",
            "remoteFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFile,
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
                            "remoteFile" | "remote_file" => Ok(GeneratedField::RemoteFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRemoteFileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.UpdateRemoteFileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateRemoteFileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFile => {
                            if remote_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFile"));
                            }
                            remote_file__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateRemoteFileResponse {
                    remote_file: remote_file__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.UpdateRemoteFileResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VideoMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if self.width != 0 {
            len += 1;
        }
        if self.duration_seconds != 0. {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.remote_files.v1.VideoMetadata", len)?;
        if self.height != 0 {
            struct_ser.serialize_field("height", &self.height)?;
        }
        if self.width != 0 {
            struct_ser.serialize_field("width", &self.width)?;
        }
        if self.duration_seconds != 0. {
            struct_ser.serialize_field("durationSeconds", &self.duration_seconds)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VideoMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "width",
            "duration_seconds",
            "durationSeconds",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            Width,
            DurationSeconds,
            Timestamp,
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
                            "height" => Ok(GeneratedField::Height),
                            "width" => Ok(GeneratedField::Width),
                            "durationSeconds" | "duration_seconds" => Ok(GeneratedField::DurationSeconds),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VideoMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.remote_files.v1.VideoMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<VideoMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut width__ = None;
                let mut duration_seconds__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Width => {
                            if width__.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            width__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DurationSeconds => {
                            if duration_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationSeconds"));
                            }
                            duration_seconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(VideoMetadata {
                    height: height__.unwrap_or_default(),
                    width: width__.unwrap_or_default(),
                    duration_seconds: duration_seconds__.unwrap_or_default(),
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("sift.remote_files.v1.VideoMetadata", FIELDS, GeneratedVisitor)
    }
}
