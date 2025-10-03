// @generated
impl serde::Serialize for BatchResolveCalculatedChannelsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.BatchResolveCalculatedChannelsRequest", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchResolveCalculatedChannelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchResolveCalculatedChannelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.BatchResolveCalculatedChannelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchResolveCalculatedChannelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchResolveCalculatedChannelsRequest {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.BatchResolveCalculatedChannelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchResolveCalculatedChannelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.responses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.BatchResolveCalculatedChannelsResponse", len)?;
        if !self.responses.is_empty() {
            struct_ser.serialize_field("responses", &self.responses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchResolveCalculatedChannelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "responses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Responses,
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
                            "responses" => Ok(GeneratedField::Responses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchResolveCalculatedChannelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.BatchResolveCalculatedChannelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchResolveCalculatedChannelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut responses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Responses => {
                            if responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responses"));
                            }
                            responses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchResolveCalculatedChannelsResponse {
                    responses: responses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.BatchResolveCalculatedChannelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.calculated_channel_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.client_key.is_some() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if !self.version_id.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.change_message.is_empty() {
            len += 1;
        }
        if !self.user_notes.is_empty() {
            len += 1;
        }
        if self.units.is_some() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if self.calculated_channel_configuration.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if !self.function_dependencies.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannel", len)?;
        if !self.calculated_channel_id.is_empty() {
            struct_ser.serialize_field("calculatedChannelId", &self.calculated_channel_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.client_key.as_ref() {
            struct_ser.serialize_field("clientKey", v)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if !self.version_id.is_empty() {
            struct_ser.serialize_field("versionId", &self.version_id)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.change_message.is_empty() {
            struct_ser.serialize_field("changeMessage", &self.change_message)?;
        }
        if !self.user_notes.is_empty() {
            struct_ser.serialize_field("userNotes", &self.user_notes)?;
        }
        if let Some(v) = self.units.as_ref() {
            struct_ser.serialize_field("units", v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if let Some(v) = self.calculated_channel_configuration.as_ref() {
            struct_ser.serialize_field("calculatedChannelConfiguration", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if !self.function_dependencies.is_empty() {
            struct_ser.serialize_field("functionDependencies", &self.function_dependencies)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel_id",
            "calculatedChannelId",
            "organization_id",
            "organizationId",
            "client_key",
            "clientKey",
            "archived_date",
            "archivedDate",
            "version_id",
            "versionId",
            "version",
            "name",
            "description",
            "change_message",
            "changeMessage",
            "user_notes",
            "userNotes",
            "units",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "calculated_channel_configuration",
            "calculatedChannelConfiguration",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "function_dependencies",
            "functionDependencies",
            "metadata",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannelId,
            OrganizationId,
            ClientKey,
            ArchivedDate,
            VersionId,
            Version,
            Name,
            Description,
            ChangeMessage,
            UserNotes,
            Units,
            CreatedDate,
            ModifiedDate,
            CalculatedChannelConfiguration,
            CreatedByUserId,
            ModifiedByUserId,
            FunctionDependencies,
            Metadata,
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
                            "calculatedChannelId" | "calculated_channel_id" => Ok(GeneratedField::CalculatedChannelId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "versionId" | "version_id" => Ok(GeneratedField::VersionId),
                            "version" => Ok(GeneratedField::Version),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "changeMessage" | "change_message" => Ok(GeneratedField::ChangeMessage),
                            "userNotes" | "user_notes" => Ok(GeneratedField::UserNotes),
                            "units" => Ok(GeneratedField::Units),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "calculatedChannelConfiguration" | "calculated_channel_configuration" => Ok(GeneratedField::CalculatedChannelConfiguration),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "functionDependencies" | "function_dependencies" => Ok(GeneratedField::FunctionDependencies),
                            "metadata" => Ok(GeneratedField::Metadata),
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
            type Value = CalculatedChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel_id__ = None;
                let mut organization_id__ = None;
                let mut client_key__ = None;
                let mut archived_date__ = None;
                let mut version_id__ = None;
                let mut version__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut change_message__ = None;
                let mut user_notes__ = None;
                let mut units__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut calculated_channel_configuration__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut function_dependencies__ = None;
                let mut metadata__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannelId => {
                            if calculated_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelId"));
                            }
                            calculated_channel_id__ = Some(map_.next_value()?);
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
                            client_key__ = map_.next_value()?;
                        }
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                        GeneratedField::VersionId => {
                            if version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionId"));
                            }
                            version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChangeMessage => {
                            if change_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeMessage"));
                            }
                            change_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserNotes => {
                            if user_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNotes"));
                            }
                            user_notes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Units => {
                            if units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("units"));
                            }
                            units__ = map_.next_value()?;
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
                        GeneratedField::CalculatedChannelConfiguration => {
                            if calculated_channel_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelConfiguration"));
                            }
                            calculated_channel_configuration__ = map_.next_value()?;
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
                        GeneratedField::FunctionDependencies => {
                            if function_dependencies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionDependencies"));
                            }
                            function_dependencies__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CalculatedChannel {
                    calculated_channel_id: calculated_channel_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    client_key: client_key__,
                    archived_date: archived_date__,
                    version_id: version_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    change_message: change_message__.unwrap_or_default(),
                    user_notes: user_notes__.unwrap_or_default(),
                    units: units__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    calculated_channel_configuration: calculated_channel_configuration__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    function_dependencies: function_dependencies__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannelAbstractChannelReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_reference.is_empty() {
            len += 1;
        }
        if !self.channel_identifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelAbstractChannelReference", len)?;
        if !self.channel_reference.is_empty() {
            struct_ser.serialize_field("channelReference", &self.channel_reference)?;
        }
        if !self.channel_identifier.is_empty() {
            struct_ser.serialize_field("channelIdentifier", &self.channel_identifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannelAbstractChannelReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_reference",
            "channelReference",
            "channel_identifier",
            "channelIdentifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelReference,
            ChannelIdentifier,
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
                            "channelReference" | "channel_reference" => Ok(GeneratedField::ChannelReference),
                            "channelIdentifier" | "channel_identifier" => Ok(GeneratedField::ChannelIdentifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalculatedChannelAbstractChannelReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelAbstractChannelReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelAbstractChannelReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_reference__ = None;
                let mut channel_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelReference => {
                            if channel_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelReference"));
                            }
                            channel_reference__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelIdentifier => {
                            if channel_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelIdentifier"));
                            }
                            channel_identifier__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CalculatedChannelAbstractChannelReference {
                    channel_reference: channel_reference__.unwrap_or_default(),
                    channel_identifier: channel_identifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelAbstractChannelReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannelAssetConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.asset_scope.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelAssetConfiguration", len)?;
        if let Some(v) = self.asset_scope.as_ref() {
            match v {
                calculated_channel_asset_configuration::AssetScope::AllAssets(v) => {
                    struct_ser.serialize_field("allAssets", v)?;
                }
                calculated_channel_asset_configuration::AssetScope::Selection(v) => {
                    struct_ser.serialize_field("selection", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannelAssetConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "all_assets",
            "allAssets",
            "selection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllAssets,
            Selection,
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
                            "allAssets" | "all_assets" => Ok(GeneratedField::AllAssets),
                            "selection" => Ok(GeneratedField::Selection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalculatedChannelAssetConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelAssetConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelAssetConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_scope__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllAssets => {
                            if asset_scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allAssets"));
                            }
                            asset_scope__ = map_.next_value::<::std::option::Option<_>>()?.map(calculated_channel_asset_configuration::AssetScope::AllAssets);
                        }
                        GeneratedField::Selection => {
                            if asset_scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selection"));
                            }
                            asset_scope__ = map_.next_value::<::std::option::Option<_>>()?.map(calculated_channel_asset_configuration::AssetScope::Selection)
;
                        }
                    }
                }
                Ok(CalculatedChannelAssetConfiguration {
                    asset_scope: asset_scope__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelAssetConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for calculated_channel_asset_configuration::AssetSelection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.asset_ids.is_empty() {
            len += 1;
        }
        if !self.tag_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelAssetConfiguration.AssetSelection", len)?;
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if !self.tag_ids.is_empty() {
            struct_ser.serialize_field("tagIds", &self.tag_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for calculated_channel_asset_configuration::AssetSelection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_ids",
            "assetIds",
            "tag_ids",
            "tagIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetIds,
            TagIds,
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
                            "assetIds" | "asset_ids" => Ok(GeneratedField::AssetIds),
                            "tagIds" | "tag_ids" => Ok(GeneratedField::TagIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = calculated_channel_asset_configuration::AssetSelection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelAssetConfiguration.AssetSelection")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<calculated_channel_asset_configuration::AssetSelection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_ids__ = None;
                let mut tag_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TagIds => {
                            if tag_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagIds"));
                            }
                            tag_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(calculated_channel_asset_configuration::AssetSelection {
                    asset_ids: asset_ids__.unwrap_or_default(),
                    tag_ids: tag_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelAssetConfiguration.AssetSelection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannelConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.asset_configuration.is_some() {
            len += 1;
        }
        if self.query_configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelConfiguration", len)?;
        if let Some(v) = self.asset_configuration.as_ref() {
            struct_ser.serialize_field("assetConfiguration", v)?;
        }
        if let Some(v) = self.query_configuration.as_ref() {
            struct_ser.serialize_field("queryConfiguration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannelConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_configuration",
            "assetConfiguration",
            "query_configuration",
            "queryConfiguration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetConfiguration,
            QueryConfiguration,
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
                            "assetConfiguration" | "asset_configuration" => Ok(GeneratedField::AssetConfiguration),
                            "queryConfiguration" | "query_configuration" => Ok(GeneratedField::QueryConfiguration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalculatedChannelConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_configuration__ = None;
                let mut query_configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetConfiguration => {
                            if asset_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetConfiguration"));
                            }
                            asset_configuration__ = map_.next_value()?;
                        }
                        GeneratedField::QueryConfiguration => {
                            if query_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryConfiguration"));
                            }
                            query_configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CalculatedChannelConfiguration {
                    asset_configuration: asset_configuration__,
                    query_configuration: query_configuration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannelQueryConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.query.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelQueryConfiguration", len)?;
        if let Some(v) = self.query.as_ref() {
            match v {
                calculated_channel_query_configuration::Query::Sel(v) => {
                    struct_ser.serialize_field("sel", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannelQueryConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sel,
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
                            "sel" => Ok(GeneratedField::Sel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalculatedChannelQueryConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelQueryConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelQueryConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sel => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sel"));
                            }
                            query__ = map_.next_value::<::std::option::Option<_>>()?.map(calculated_channel_query_configuration::Query::Sel)
;
                        }
                    }
                }
                Ok(CalculatedChannelQueryConfiguration {
                    query: query__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelQueryConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for calculated_channel_query_configuration::Sel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.expression_channel_references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelQueryConfiguration.Sel", len)?;
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.expression_channel_references.is_empty() {
            struct_ser.serialize_field("expressionChannelReferences", &self.expression_channel_references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for calculated_channel_query_configuration::Sel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expression",
            "expression_channel_references",
            "expressionChannelReferences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expression,
            ExpressionChannelReferences,
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
                            "expression" => Ok(GeneratedField::Expression),
                            "expressionChannelReferences" | "expression_channel_references" => Ok(GeneratedField::ExpressionChannelReferences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = calculated_channel_query_configuration::Sel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelQueryConfiguration.Sel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<calculated_channel_query_configuration::Sel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expression__ = None;
                let mut expression_channel_references__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpressionChannelReferences => {
                            if expression_channel_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expressionChannelReferences"));
                            }
                            expression_channel_references__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(calculated_channel_query_configuration::Sel {
                    expression: expression__.unwrap_or_default(),
                    expression_channel_references: expression_channel_references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelQueryConfiguration.Sel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannelResolution {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.calculated_channel.is_some() {
            len += 1;
        }
        if !self.resolved.is_empty() {
            len += 1;
        }
        if !self.unresolved.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelResolution", len)?;
        if let Some(v) = self.calculated_channel.as_ref() {
            struct_ser.serialize_field("calculatedChannel", v)?;
        }
        if !self.resolved.is_empty() {
            struct_ser.serialize_field("resolved", &self.resolved)?;
        }
        if !self.unresolved.is_empty() {
            struct_ser.serialize_field("unresolved", &self.unresolved)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannelResolution {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel",
            "calculatedChannel",
            "resolved",
            "unresolved",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannel,
            Resolved,
            Unresolved,
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
                            "calculatedChannel" | "calculated_channel" => Ok(GeneratedField::CalculatedChannel),
                            "resolved" => Ok(GeneratedField::Resolved),
                            "unresolved" => Ok(GeneratedField::Unresolved),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalculatedChannelResolution;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelResolution")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelResolution, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel__ = None;
                let mut resolved__ = None;
                let mut unresolved__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannel => {
                            if calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannel"));
                            }
                            calculated_channel__ = map_.next_value()?;
                        }
                        GeneratedField::Resolved => {
                            if resolved__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolved"));
                            }
                            resolved__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unresolved => {
                            if unresolved__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unresolved"));
                            }
                            unresolved__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CalculatedChannelResolution {
                    calculated_channel: calculated_channel__,
                    resolved: resolved__.unwrap_or_default(),
                    unresolved: unresolved__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelResolution", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CalculatedChannelValidationResult {
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
        if self.asset_name.is_some() {
            len += 1;
        }
        if !self.tag_names.is_empty() {
            len += 1;
        }
        if !self.missing_channels.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CalculatedChannelValidationResult", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if let Some(v) = self.asset_name.as_ref() {
            struct_ser.serialize_field("assetName", v)?;
        }
        if !self.tag_names.is_empty() {
            struct_ser.serialize_field("tagNames", &self.tag_names)?;
        }
        if !self.missing_channels.is_empty() {
            struct_ser.serialize_field("missingChannels", &self.missing_channels)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CalculatedChannelValidationResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "asset_name",
            "assetName",
            "tag_names",
            "tagNames",
            "missing_channels",
            "missingChannels",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetId,
            AssetName,
            TagNames,
            MissingChannels,
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
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "tagNames" | "tag_names" => Ok(GeneratedField::TagNames),
                            "missingChannels" | "missing_channels" => Ok(GeneratedField::MissingChannels),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CalculatedChannelValidationResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CalculatedChannelValidationResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CalculatedChannelValidationResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut asset_name__ = None;
                let mut tag_names__ = None;
                let mut missing_channels__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = map_.next_value()?;
                        }
                        GeneratedField::TagNames => {
                            if tag_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagNames"));
                            }
                            tag_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MissingChannels => {
                            if missing_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("missingChannels"));
                            }
                            missing_channels__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CalculatedChannelValidationResult {
                    asset_id: asset_id__.unwrap_or_default(),
                    asset_name: asset_name__,
                    tag_names: tag_names__.unwrap_or_default(),
                    missing_channels: missing_channels__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CalculatedChannelValidationResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCalculatedChannelRequest {
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
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.user_notes.is_empty() {
            len += 1;
        }
        if self.units.is_some() {
            len += 1;
        }
        if self.client_key.is_some() {
            len += 1;
        }
        if self.calculated_channel_configuration.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CreateCalculatedChannelRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.user_notes.is_empty() {
            struct_ser.serialize_field("userNotes", &self.user_notes)?;
        }
        if let Some(v) = self.units.as_ref() {
            struct_ser.serialize_field("units", v)?;
        }
        if let Some(v) = self.client_key.as_ref() {
            struct_ser.serialize_field("clientKey", v)?;
        }
        if let Some(v) = self.calculated_channel_configuration.as_ref() {
            struct_ser.serialize_field("calculatedChannelConfiguration", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCalculatedChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "user_notes",
            "userNotes",
            "units",
            "client_key",
            "clientKey",
            "calculated_channel_configuration",
            "calculatedChannelConfiguration",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            UserNotes,
            Units,
            ClientKey,
            CalculatedChannelConfiguration,
            Metadata,
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
                            "description" => Ok(GeneratedField::Description),
                            "userNotes" | "user_notes" => Ok(GeneratedField::UserNotes),
                            "units" => Ok(GeneratedField::Units),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "calculatedChannelConfiguration" | "calculated_channel_configuration" => Ok(GeneratedField::CalculatedChannelConfiguration),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCalculatedChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CreateCalculatedChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCalculatedChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut user_notes__ = None;
                let mut units__ = None;
                let mut client_key__ = None;
                let mut calculated_channel_configuration__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserNotes => {
                            if user_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNotes"));
                            }
                            user_notes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Units => {
                            if units__.is_some() {
                                return Err(serde::de::Error::duplicate_field("units"));
                            }
                            units__ = map_.next_value()?;
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = map_.next_value()?;
                        }
                        GeneratedField::CalculatedChannelConfiguration => {
                            if calculated_channel_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelConfiguration"));
                            }
                            calculated_channel_configuration__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateCalculatedChannelRequest {
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    user_notes: user_notes__.unwrap_or_default(),
                    units: units__,
                    client_key: client_key__,
                    calculated_channel_configuration: calculated_channel_configuration__,
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CreateCalculatedChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateCalculatedChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.calculated_channel.is_some() {
            len += 1;
        }
        if !self.inapplicable_assets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.CreateCalculatedChannelResponse", len)?;
        if let Some(v) = self.calculated_channel.as_ref() {
            struct_ser.serialize_field("calculatedChannel", v)?;
        }
        if !self.inapplicable_assets.is_empty() {
            struct_ser.serialize_field("inapplicableAssets", &self.inapplicable_assets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateCalculatedChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel",
            "calculatedChannel",
            "inapplicable_assets",
            "inapplicableAssets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannel,
            InapplicableAssets,
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
                            "calculatedChannel" | "calculated_channel" => Ok(GeneratedField::CalculatedChannel),
                            "inapplicableAssets" | "inapplicable_assets" => Ok(GeneratedField::InapplicableAssets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateCalculatedChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.CreateCalculatedChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateCalculatedChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel__ = None;
                let mut inapplicable_assets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannel => {
                            if calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannel"));
                            }
                            calculated_channel__ = map_.next_value()?;
                        }
                        GeneratedField::InapplicableAssets => {
                            if inapplicable_assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inapplicableAssets"));
                            }
                            inapplicable_assets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateCalculatedChannelResponse {
                    calculated_channel: calculated_channel__,
                    inapplicable_assets: inapplicable_assets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.CreateCalculatedChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCalculatedChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.calculated_channel_id.is_empty() {
            len += 1;
        }
        if !self.client_key.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.GetCalculatedChannelRequest", len)?;
        if !self.calculated_channel_id.is_empty() {
            struct_ser.serialize_field("calculatedChannelId", &self.calculated_channel_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCalculatedChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel_id",
            "calculatedChannelId",
            "client_key",
            "clientKey",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannelId,
            ClientKey,
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
                            "calculatedChannelId" | "calculated_channel_id" => Ok(GeneratedField::CalculatedChannelId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
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
            type Value = GetCalculatedChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.GetCalculatedChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCalculatedChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel_id__ = None;
                let mut client_key__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannelId => {
                            if calculated_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelId"));
                            }
                            calculated_channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetCalculatedChannelRequest {
                    calculated_channel_id: calculated_channel_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.GetCalculatedChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCalculatedChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.calculated_channel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.GetCalculatedChannelResponse", len)?;
        if let Some(v) = self.calculated_channel.as_ref() {
            struct_ser.serialize_field("calculatedChannel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCalculatedChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel",
            "calculatedChannel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannel,
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
                            "calculatedChannel" | "calculated_channel" => Ok(GeneratedField::CalculatedChannel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCalculatedChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.GetCalculatedChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetCalculatedChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannel => {
                            if calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannel"));
                            }
                            calculated_channel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetCalculatedChannelResponse {
                    calculated_channel: calculated_channel__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.GetCalculatedChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCalculatedChannelVersionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.calculated_channel_id.is_empty() {
            len += 1;
        }
        if !self.client_key.is_empty() {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ListCalculatedChannelVersionsRequest", len)?;
        if !self.calculated_channel_id.is_empty() {
            struct_ser.serialize_field("calculatedChannelId", &self.calculated_channel_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
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
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCalculatedChannelVersionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel_id",
            "calculatedChannelId",
            "client_key",
            "clientKey",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
            "organization_id",
            "organizationId",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannelId,
            ClientKey,
            PageSize,
            PageToken,
            Filter,
            OrganizationId,
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
                            "calculatedChannelId" | "calculated_channel_id" => Ok(GeneratedField::CalculatedChannelId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
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
            type Value = ListCalculatedChannelVersionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ListCalculatedChannelVersionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCalculatedChannelVersionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel_id__ = None;
                let mut client_key__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut organization_id__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannelId => {
                            if calculated_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelId"));
                            }
                            calculated_channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListCalculatedChannelVersionsRequest {
                    calculated_channel_id: calculated_channel_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ListCalculatedChannelVersionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCalculatedChannelVersionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.calculated_channel_versions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ListCalculatedChannelVersionsResponse", len)?;
        if !self.calculated_channel_versions.is_empty() {
            struct_ser.serialize_field("calculatedChannelVersions", &self.calculated_channel_versions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCalculatedChannelVersionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel_versions",
            "calculatedChannelVersions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannelVersions,
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
                            "calculatedChannelVersions" | "calculated_channel_versions" => Ok(GeneratedField::CalculatedChannelVersions),
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
            type Value = ListCalculatedChannelVersionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ListCalculatedChannelVersionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCalculatedChannelVersionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel_versions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannelVersions => {
                            if calculated_channel_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelVersions"));
                            }
                            calculated_channel_versions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListCalculatedChannelVersionsResponse {
                    calculated_channel_versions: calculated_channel_versions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ListCalculatedChannelVersionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCalculatedChannelsRequest {
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
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ListCalculatedChannelsRequest", len)?;
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
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCalculatedChannelsRequest {
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
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrganizationId,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
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
            type Value = ListCalculatedChannelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ListCalculatedChannelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCalculatedChannelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut organization_id__ = None;
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListCalculatedChannelsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ListCalculatedChannelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListCalculatedChannelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.calculated_channels.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ListCalculatedChannelsResponse", len)?;
        if !self.calculated_channels.is_empty() {
            struct_ser.serialize_field("calculatedChannels", &self.calculated_channels)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListCalculatedChannelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channels",
            "calculatedChannels",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannels,
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
                            "calculatedChannels" | "calculated_channels" => Ok(GeneratedField::CalculatedChannels),
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
            type Value = ListCalculatedChannelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ListCalculatedChannelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListCalculatedChannelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channels__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannels => {
                            if calculated_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannels"));
                            }
                            calculated_channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListCalculatedChannelsResponse {
                    calculated_channels: calculated_channels__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ListCalculatedChannelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResolvedCalculatedChannelsRequest {
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
        if !self.run_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ListResolvedCalculatedChannelsRequest", len)?;
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResolvedCalculatedChannelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_id",
            "assetId",
            "run_id",
            "runId",
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
            AssetId,
            RunId,
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
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
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
            type Value = ListResolvedCalculatedChannelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ListResolvedCalculatedChannelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResolvedCalculatedChannelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_id__ = None;
                let mut run_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(ListResolvedCalculatedChannelsRequest {
                    asset_id: asset_id__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ListResolvedCalculatedChannelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListResolvedCalculatedChannelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.calculated_channel_resolutions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ListResolvedCalculatedChannelsResponse", len)?;
        if !self.calculated_channel_resolutions.is_empty() {
            struct_ser.serialize_field("calculatedChannelResolutions", &self.calculated_channel_resolutions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListResolvedCalculatedChannelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel_resolutions",
            "calculatedChannelResolutions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannelResolutions,
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
                            "calculatedChannelResolutions" | "calculated_channel_resolutions" => Ok(GeneratedField::CalculatedChannelResolutions),
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
            type Value = ListResolvedCalculatedChannelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ListResolvedCalculatedChannelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListResolvedCalculatedChannelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel_resolutions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannelResolutions => {
                            if calculated_channel_resolutions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelResolutions"));
                            }
                            calculated_channel_resolutions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListResolvedCalculatedChannelsResponse {
                    calculated_channel_resolutions: calculated_channel_resolutions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ListResolvedCalculatedChannelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveCalculatedChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.assets.is_some() {
            len += 1;
        }
        if self.run.is_some() {
            len += 1;
        }
        if self.calculated_channel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ResolveCalculatedChannelRequest", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.assets.as_ref() {
            struct_ser.serialize_field("assets", v)?;
        }
        if let Some(v) = self.run.as_ref() {
            struct_ser.serialize_field("run", v)?;
        }
        if let Some(v) = self.calculated_channel.as_ref() {
            match v {
                resolve_calculated_channel_request::CalculatedChannel::Identifier(v) => {
                    struct_ser.serialize_field("identifier", v)?;
                }
                resolve_calculated_channel_request::CalculatedChannel::CalculatedChannelConfiguration(v) => {
                    struct_ser.serialize_field("calculatedChannelConfiguration", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveCalculatedChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
            "assets",
            "run",
            "identifier",
            "calculated_channel_configuration",
            "calculatedChannelConfiguration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
            Assets,
            Run,
            Identifier,
            CalculatedChannelConfiguration,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "assets" => Ok(GeneratedField::Assets),
                            "run" => Ok(GeneratedField::Run),
                            "identifier" => Ok(GeneratedField::Identifier),
                            "calculatedChannelConfiguration" | "calculated_channel_configuration" => Ok(GeneratedField::CalculatedChannelConfiguration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveCalculatedChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ResolveCalculatedChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveCalculatedChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                let mut assets__ = None;
                let mut run__ = None;
                let mut calculated_channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = map_.next_value()?;
                        }
                        GeneratedField::Run => {
                            if run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            run__ = map_.next_value()?;
                        }
                        GeneratedField::Identifier => {
                            if calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            calculated_channel__ = map_.next_value::<::std::option::Option<_>>()?.map(resolve_calculated_channel_request::CalculatedChannel::Identifier)
;
                        }
                        GeneratedField::CalculatedChannelConfiguration => {
                            if calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelConfiguration"));
                            }
                            calculated_channel__ = map_.next_value::<::std::option::Option<_>>()?.map(resolve_calculated_channel_request::CalculatedChannel::CalculatedChannelConfiguration)
;
                        }
                    }
                }
                Ok(ResolveCalculatedChannelRequest {
                    organization_id: organization_id__.unwrap_or_default(),
                    assets: assets__,
                    run: run__,
                    calculated_channel: calculated_channel__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ResolveCalculatedChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveCalculatedChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.calculated_channel_id.is_some() {
            len += 1;
        }
        if !self.resolved.is_empty() {
            len += 1;
        }
        if !self.unresolved.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ResolveCalculatedChannelResponse", len)?;
        if let Some(v) = self.calculated_channel_id.as_ref() {
            struct_ser.serialize_field("calculatedChannelId", v)?;
        }
        if !self.resolved.is_empty() {
            struct_ser.serialize_field("resolved", &self.resolved)?;
        }
        if !self.unresolved.is_empty() {
            struct_ser.serialize_field("unresolved", &self.unresolved)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveCalculatedChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel_id",
            "calculatedChannelId",
            "resolved",
            "unresolved",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannelId,
            Resolved,
            Unresolved,
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
                            "calculatedChannelId" | "calculated_channel_id" => Ok(GeneratedField::CalculatedChannelId),
                            "resolved" => Ok(GeneratedField::Resolved),
                            "unresolved" => Ok(GeneratedField::Unresolved),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveCalculatedChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ResolveCalculatedChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveCalculatedChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel_id__ = None;
                let mut resolved__ = None;
                let mut unresolved__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannelId => {
                            if calculated_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelId"));
                            }
                            calculated_channel_id__ = map_.next_value()?;
                        }
                        GeneratedField::Resolved => {
                            if resolved__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolved"));
                            }
                            resolved__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unresolved => {
                            if unresolved__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unresolved"));
                            }
                            unresolved__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveCalculatedChannelResponse {
                    calculated_channel_id: calculated_channel_id__,
                    resolved: resolved__.unwrap_or_default(),
                    unresolved: unresolved__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ResolveCalculatedChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolvedCalculatedChannel {
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
        if self.expression_request.is_some() {
            len += 1;
        }
        if self.output_data_type != 0 {
            len += 1;
        }
        if !self.asset_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.ResolvedCalculatedChannel", len)?;
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if let Some(v) = self.expression_request.as_ref() {
            struct_ser.serialize_field("expressionRequest", v)?;
        }
        if self.output_data_type != 0 {
            let v = super::super::common::r#type::v1::ChannelDataType::try_from(self.output_data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.output_data_type)))?;
            struct_ser.serialize_field("outputDataType", &v)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolvedCalculatedChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_name",
            "assetName",
            "expression_request",
            "expressionRequest",
            "output_data_type",
            "outputDataType",
            "asset_id",
            "assetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetName,
            ExpressionRequest,
            OutputDataType,
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
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "expressionRequest" | "expression_request" => Ok(GeneratedField::ExpressionRequest),
                            "outputDataType" | "output_data_type" => Ok(GeneratedField::OutputDataType),
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
            type Value = ResolvedCalculatedChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.ResolvedCalculatedChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolvedCalculatedChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_name__ = None;
                let mut expression_request__ = None;
                let mut output_data_type__ = None;
                let mut asset_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpressionRequest => {
                            if expression_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expressionRequest"));
                            }
                            expression_request__ = map_.next_value()?;
                        }
                        GeneratedField::OutputDataType => {
                            if output_data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputDataType"));
                            }
                            output_data_type__ = Some(map_.next_value::<super::super::common::r#type::v1::ChannelDataType>()? as i32);
                        }
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolvedCalculatedChannel {
                    asset_name: asset_name__.unwrap_or_default(),
                    expression_request: expression_request__,
                    output_data_type: output_data_type__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.ResolvedCalculatedChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnresolvedCalculatedChannel {
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
        if !self.error_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.UnresolvedCalculatedChannel", len)?;
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnresolvedCalculatedChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_name",
            "assetName",
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetName,
            ErrorMessage,
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
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnresolvedCalculatedChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.UnresolvedCalculatedChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnresolvedCalculatedChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_name__ = None;
                let mut error_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnresolvedCalculatedChannel {
                    asset_name: asset_name__.unwrap_or_default(),
                    error_message: error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.UnresolvedCalculatedChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCalculatedChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.calculated_channel.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        if self.user_notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.UpdateCalculatedChannelRequest", len)?;
        if let Some(v) = self.calculated_channel.as_ref() {
            struct_ser.serialize_field("calculatedChannel", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        if let Some(v) = self.user_notes.as_ref() {
            struct_ser.serialize_field("userNotes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCalculatedChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel",
            "calculatedChannel",
            "update_mask",
            "updateMask",
            "user_notes",
            "userNotes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannel,
            UpdateMask,
            UserNotes,
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
                            "calculatedChannel" | "calculated_channel" => Ok(GeneratedField::CalculatedChannel),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            "userNotes" | "user_notes" => Ok(GeneratedField::UserNotes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCalculatedChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.UpdateCalculatedChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCalculatedChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel__ = None;
                let mut update_mask__ = None;
                let mut user_notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannel => {
                            if calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannel"));
                            }
                            calculated_channel__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                        GeneratedField::UserNotes => {
                            if user_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNotes"));
                            }
                            user_notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateCalculatedChannelRequest {
                    calculated_channel: calculated_channel__,
                    update_mask: update_mask__,
                    user_notes: user_notes__,
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.UpdateCalculatedChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateCalculatedChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.calculated_channel.is_some() {
            len += 1;
        }
        if !self.inapplicable_assets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.calculated_channels.v2.UpdateCalculatedChannelResponse", len)?;
        if let Some(v) = self.calculated_channel.as_ref() {
            struct_ser.serialize_field("calculatedChannel", v)?;
        }
        if !self.inapplicable_assets.is_empty() {
            struct_ser.serialize_field("inapplicableAssets", &self.inapplicable_assets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateCalculatedChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "calculated_channel",
            "calculatedChannel",
            "inapplicable_assets",
            "inapplicableAssets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CalculatedChannel,
            InapplicableAssets,
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
                            "calculatedChannel" | "calculated_channel" => Ok(GeneratedField::CalculatedChannel),
                            "inapplicableAssets" | "inapplicable_assets" => Ok(GeneratedField::InapplicableAssets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateCalculatedChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.calculated_channels.v2.UpdateCalculatedChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateCalculatedChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut calculated_channel__ = None;
                let mut inapplicable_assets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CalculatedChannel => {
                            if calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannel"));
                            }
                            calculated_channel__ = map_.next_value()?;
                        }
                        GeneratedField::InapplicableAssets => {
                            if inapplicable_assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inapplicableAssets"));
                            }
                            inapplicable_assets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateCalculatedChannelResponse {
                    calculated_channel: calculated_channel__,
                    inapplicable_assets: inapplicable_assets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.calculated_channels.v2.UpdateCalculatedChannelResponse", FIELDS, GeneratedVisitor)
    }
}
