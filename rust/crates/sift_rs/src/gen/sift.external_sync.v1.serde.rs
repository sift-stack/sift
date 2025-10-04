// @generated
impl serde::Serialize for ExternalSync {
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
        if self.most_recent_sync_date.is_some() {
            len += 1;
        }
        if self.most_recent_sync_by_user_id.is_some() {
            len += 1;
        }
        if !self.scim_server_url.is_empty() {
            len += 1;
        }
        if self.token_created_date.is_some() {
            len += 1;
        }
        if self.token_lifetime_seconds != 0 {
            len += 1;
        }
        if self.most_recent_token_by_user_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.external_sync.v1.ExternalSync", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if let Some(v) = self.most_recent_sync_date.as_ref() {
            struct_ser.serialize_field("mostRecentSyncDate", v)?;
        }
        if let Some(v) = self.most_recent_sync_by_user_id.as_ref() {
            struct_ser.serialize_field("mostRecentSyncByUserId", v)?;
        }
        if !self.scim_server_url.is_empty() {
            struct_ser.serialize_field("scimServerUrl", &self.scim_server_url)?;
        }
        if let Some(v) = self.token_created_date.as_ref() {
            struct_ser.serialize_field("tokenCreatedDate", v)?;
        }
        if self.token_lifetime_seconds != 0 {
            struct_ser.serialize_field("tokenLifetimeSeconds", &self.token_lifetime_seconds)?;
        }
        if let Some(v) = self.most_recent_token_by_user_id.as_ref() {
            struct_ser.serialize_field("mostRecentTokenByUserId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternalSync {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
            "most_recent_sync_date",
            "mostRecentSyncDate",
            "most_recent_sync_by_user_id",
            "mostRecentSyncByUserId",
            "scim_server_url",
            "scimServerUrl",
            "token_created_date",
            "tokenCreatedDate",
            "token_lifetime_seconds",
            "tokenLifetimeSeconds",
            "most_recent_token_by_user_id",
            "mostRecentTokenByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
            MostRecentSyncDate,
            MostRecentSyncByUserId,
            ScimServerUrl,
            TokenCreatedDate,
            TokenLifetimeSeconds,
            MostRecentTokenByUserId,
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
                            "mostRecentSyncDate" | "most_recent_sync_date" => Ok(GeneratedField::MostRecentSyncDate),
                            "mostRecentSyncByUserId" | "most_recent_sync_by_user_id" => Ok(GeneratedField::MostRecentSyncByUserId),
                            "scimServerUrl" | "scim_server_url" => Ok(GeneratedField::ScimServerUrl),
                            "tokenCreatedDate" | "token_created_date" => Ok(GeneratedField::TokenCreatedDate),
                            "tokenLifetimeSeconds" | "token_lifetime_seconds" => Ok(GeneratedField::TokenLifetimeSeconds),
                            "mostRecentTokenByUserId" | "most_recent_token_by_user_id" => Ok(GeneratedField::MostRecentTokenByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternalSync;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.ExternalSync")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExternalSync, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                let mut most_recent_sync_date__ = None;
                let mut most_recent_sync_by_user_id__ = None;
                let mut scim_server_url__ = None;
                let mut token_created_date__ = None;
                let mut token_lifetime_seconds__ = None;
                let mut most_recent_token_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MostRecentSyncDate => {
                            if most_recent_sync_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mostRecentSyncDate"));
                            }
                            most_recent_sync_date__ = map_.next_value()?;
                        }
                        GeneratedField::MostRecentSyncByUserId => {
                            if most_recent_sync_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mostRecentSyncByUserId"));
                            }
                            most_recent_sync_by_user_id__ = map_.next_value()?;
                        }
                        GeneratedField::ScimServerUrl => {
                            if scim_server_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scimServerUrl"));
                            }
                            scim_server_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TokenCreatedDate => {
                            if token_created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenCreatedDate"));
                            }
                            token_created_date__ = map_.next_value()?;
                        }
                        GeneratedField::TokenLifetimeSeconds => {
                            if token_lifetime_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenLifetimeSeconds"));
                            }
                            token_lifetime_seconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MostRecentTokenByUserId => {
                            if most_recent_token_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mostRecentTokenByUserId"));
                            }
                            most_recent_token_by_user_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExternalSync {
                    organization_id: organization_id__.unwrap_or_default(),
                    most_recent_sync_date: most_recent_sync_date__,
                    most_recent_sync_by_user_id: most_recent_sync_by_user_id__,
                    scim_server_url: scim_server_url__.unwrap_or_default(),
                    token_created_date: token_created_date__,
                    token_lifetime_seconds: token_lifetime_seconds__.unwrap_or_default(),
                    most_recent_token_by_user_id: most_recent_token_by_user_id__,
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.ExternalSync", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExternalSyncToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token_id.is_empty() {
            len += 1;
        }
        if self.lifetime_seconds != 0 {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.external_sync.v1.ExternalSyncToken", len)?;
        if !self.token_id.is_empty() {
            struct_ser.serialize_field("tokenId", &self.token_id)?;
        }
        if self.lifetime_seconds != 0 {
            struct_ser.serialize_field("lifetimeSeconds", &self.lifetime_seconds)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternalSyncToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token_id",
            "tokenId",
            "lifetime_seconds",
            "lifetimeSeconds",
            "created_date",
            "createdDate",
            "created_by_user_id",
            "createdByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenId,
            LifetimeSeconds,
            CreatedDate,
            CreatedByUserId,
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
                            "tokenId" | "token_id" => Ok(GeneratedField::TokenId),
                            "lifetimeSeconds" | "lifetime_seconds" => Ok(GeneratedField::LifetimeSeconds),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternalSyncToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.ExternalSyncToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExternalSyncToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token_id__ = None;
                let mut lifetime_seconds__ = None;
                let mut created_date__ = None;
                let mut created_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TokenId => {
                            if token_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenId"));
                            }
                            token_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LifetimeSeconds => {
                            if lifetime_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifetimeSeconds"));
                            }
                            lifetime_seconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
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
                    }
                }
                Ok(ExternalSyncToken {
                    token_id: token_id__.unwrap_or_default(),
                    lifetime_seconds: lifetime_seconds__.unwrap_or_default(),
                    created_date: created_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.ExternalSyncToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateTokenRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.external_sync.v1.GenerateTokenRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateTokenRequest {
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
            type Value = GenerateTokenRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.GenerateTokenRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateTokenRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GenerateTokenRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.GenerateTokenRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateTokenResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.external_sync.is_some() {
            len += 1;
        }
        if !self.token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.external_sync.v1.GenerateTokenResponse", len)?;
        if let Some(v) = self.external_sync.as_ref() {
            struct_ser.serialize_field("externalSync", v)?;
        }
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateTokenResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "external_sync",
            "externalSync",
            "token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternalSync,
            Token,
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
                            "externalSync" | "external_sync" => Ok(GeneratedField::ExternalSync),
                            "token" => Ok(GeneratedField::Token),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateTokenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.GenerateTokenResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateTokenResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut external_sync__ = None;
                let mut token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExternalSync => {
                            if external_sync__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSync"));
                            }
                            external_sync__ = map_.next_value()?;
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateTokenResponse {
                    external_sync: external_sync__,
                    token: token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.GenerateTokenResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExternalSyncRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.external_sync.v1.GetExternalSyncRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExternalSyncRequest {
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
            type Value = GetExternalSyncRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.GetExternalSyncRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetExternalSyncRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetExternalSyncRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.GetExternalSyncRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExternalSyncResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.success {
            len += 1;
        }
        if self.external_sync.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.external_sync.v1.GetExternalSyncResponse", len)?;
        if self.success {
            struct_ser.serialize_field("success", &self.success)?;
        }
        if let Some(v) = self.external_sync.as_ref() {
            struct_ser.serialize_field("externalSync", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExternalSyncResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "external_sync",
            "externalSync",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
            ExternalSync,
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
                            "success" => Ok(GeneratedField::Success),
                            "externalSync" | "external_sync" => Ok(GeneratedField::ExternalSync),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExternalSyncResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.GetExternalSyncResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetExternalSyncResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut success__ = None;
                let mut external_sync__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if success__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            success__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExternalSync => {
                            if external_sync__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSync"));
                            }
                            external_sync__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetExternalSyncResponse {
                    success: success__.unwrap_or_default(),
                    external_sync: external_sync__,
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.GetExternalSyncResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExternalSyncTokensRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.external_sync.v1.ListExternalSyncTokensRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListExternalSyncTokensRequest {
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
            type Value = ListExternalSyncTokensRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.ListExternalSyncTokensRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExternalSyncTokensRequest, V::Error>
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
                Ok(ListExternalSyncTokensRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.ListExternalSyncTokensRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExternalSyncTokensResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.external_sync_tokens.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.external_sync.v1.ListExternalSyncTokensResponse", len)?;
        if !self.external_sync_tokens.is_empty() {
            struct_ser.serialize_field("externalSyncTokens", &self.external_sync_tokens)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExternalSyncTokensResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "external_sync_tokens",
            "externalSyncTokens",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternalSyncTokens,
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
                            "externalSyncTokens" | "external_sync_tokens" => Ok(GeneratedField::ExternalSyncTokens),
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
            type Value = ListExternalSyncTokensResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.ListExternalSyncTokensResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListExternalSyncTokensResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut external_sync_tokens__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExternalSyncTokens => {
                            if external_sync_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSyncTokens"));
                            }
                            external_sync_tokens__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListExternalSyncTokensResponse {
                    external_sync_tokens: external_sync_tokens__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.ListExternalSyncTokensResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SyncOrganizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.external_sync.v1.SyncOrganizationRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SyncOrganizationRequest {
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
            type Value = SyncOrganizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.SyncOrganizationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SyncOrganizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SyncOrganizationRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.SyncOrganizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SyncOrganizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.external_sync.is_some() {
            len += 1;
        }
        if self.existing_user_count != 0 {
            len += 1;
        }
        if !self.added_to_organization_user_ids.is_empty() {
            len += 1;
        }
        if !self.created_users.is_empty() {
            len += 1;
        }
        if !self.deactivated_user_ids.is_empty() {
            len += 1;
        }
        if self.existing_group_count != 0 {
            len += 1;
        }
        if !self.created_user_groups.is_empty() {
            len += 1;
        }
        if !self.deleted_user_group_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.external_sync.v1.SyncOrganizationResponse", len)?;
        if let Some(v) = self.external_sync.as_ref() {
            struct_ser.serialize_field("externalSync", v)?;
        }
        if self.existing_user_count != 0 {
            struct_ser.serialize_field("existingUserCount", &self.existing_user_count)?;
        }
        if !self.added_to_organization_user_ids.is_empty() {
            struct_ser.serialize_field("addedToOrganizationUserIds", &self.added_to_organization_user_ids)?;
        }
        if !self.created_users.is_empty() {
            struct_ser.serialize_field("createdUsers", &self.created_users)?;
        }
        if !self.deactivated_user_ids.is_empty() {
            struct_ser.serialize_field("deactivatedUserIds", &self.deactivated_user_ids)?;
        }
        if self.existing_group_count != 0 {
            struct_ser.serialize_field("existingGroupCount", &self.existing_group_count)?;
        }
        if !self.created_user_groups.is_empty() {
            struct_ser.serialize_field("createdUserGroups", &self.created_user_groups)?;
        }
        if !self.deleted_user_group_names.is_empty() {
            struct_ser.serialize_field("deletedUserGroupNames", &self.deleted_user_group_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SyncOrganizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "external_sync",
            "externalSync",
            "existing_user_count",
            "existingUserCount",
            "added_to_organization_user_ids",
            "addedToOrganizationUserIds",
            "created_users",
            "createdUsers",
            "deactivated_user_ids",
            "deactivatedUserIds",
            "existing_group_count",
            "existingGroupCount",
            "created_user_groups",
            "createdUserGroups",
            "deleted_user_group_names",
            "deletedUserGroupNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternalSync,
            ExistingUserCount,
            AddedToOrganizationUserIds,
            CreatedUsers,
            DeactivatedUserIds,
            ExistingGroupCount,
            CreatedUserGroups,
            DeletedUserGroupNames,
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
                            "externalSync" | "external_sync" => Ok(GeneratedField::ExternalSync),
                            "existingUserCount" | "existing_user_count" => Ok(GeneratedField::ExistingUserCount),
                            "addedToOrganizationUserIds" | "added_to_organization_user_ids" => Ok(GeneratedField::AddedToOrganizationUserIds),
                            "createdUsers" | "created_users" => Ok(GeneratedField::CreatedUsers),
                            "deactivatedUserIds" | "deactivated_user_ids" => Ok(GeneratedField::DeactivatedUserIds),
                            "existingGroupCount" | "existing_group_count" => Ok(GeneratedField::ExistingGroupCount),
                            "createdUserGroups" | "created_user_groups" => Ok(GeneratedField::CreatedUserGroups),
                            "deletedUserGroupNames" | "deleted_user_group_names" => Ok(GeneratedField::DeletedUserGroupNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SyncOrganizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.external_sync.v1.SyncOrganizationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SyncOrganizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut external_sync__ = None;
                let mut existing_user_count__ = None;
                let mut added_to_organization_user_ids__ = None;
                let mut created_users__ = None;
                let mut deactivated_user_ids__ = None;
                let mut existing_group_count__ = None;
                let mut created_user_groups__ = None;
                let mut deleted_user_group_names__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExternalSync => {
                            if external_sync__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalSync"));
                            }
                            external_sync__ = map_.next_value()?;
                        }
                        GeneratedField::ExistingUserCount => {
                            if existing_user_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("existingUserCount"));
                            }
                            existing_user_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AddedToOrganizationUserIds => {
                            if added_to_organization_user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addedToOrganizationUserIds"));
                            }
                            added_to_organization_user_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedUsers => {
                            if created_users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdUsers"));
                            }
                            created_users__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeactivatedUserIds => {
                            if deactivated_user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deactivatedUserIds"));
                            }
                            deactivated_user_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExistingGroupCount => {
                            if existing_group_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("existingGroupCount"));
                            }
                            existing_group_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreatedUserGroups => {
                            if created_user_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdUserGroups"));
                            }
                            created_user_groups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeletedUserGroupNames => {
                            if deleted_user_group_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedUserGroupNames"));
                            }
                            deleted_user_group_names__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SyncOrganizationResponse {
                    external_sync: external_sync__,
                    existing_user_count: existing_user_count__.unwrap_or_default(),
                    added_to_organization_user_ids: added_to_organization_user_ids__.unwrap_or_default(),
                    created_users: created_users__.unwrap_or_default(),
                    deactivated_user_ids: deactivated_user_ids__.unwrap_or_default(),
                    existing_group_count: existing_group_count__.unwrap_or_default(),
                    created_user_groups: created_user_groups__.unwrap_or_default(),
                    deleted_user_group_names: deleted_user_group_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.external_sync.v1.SyncOrganizationResponse", FIELDS, GeneratedVisitor)
    }
}
