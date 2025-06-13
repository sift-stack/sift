// @generated
impl serde::Serialize for GetMeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.me.v2.GetMeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMeRequest {
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
            type Value = GetMeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.me.v2.GetMeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetMeRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.me.v2.GetMeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_id.is_empty() {
            len += 1;
        }
        if !self.user_email.is_empty() {
            len += 1;
        }
        if !self.organizations.is_empty() {
            len += 1;
        }
        if self.is_admin {
            len += 1;
        }
        if self.permissions.is_some() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if !self.hash_based_message_authentication_code.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.me.v2.GetMeResponse", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_email.is_empty() {
            struct_ser.serialize_field("userEmail", &self.user_email)?;
        }
        if !self.organizations.is_empty() {
            struct_ser.serialize_field("organizations", &self.organizations)?;
        }
        if self.is_admin {
            struct_ser.serialize_field("isAdmin", &self.is_admin)?;
        }
        if let Some(v) = self.permissions.as_ref() {
            struct_ser.serialize_field("permissions", v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if !self.hash_based_message_authentication_code.is_empty() {
            struct_ser.serialize_field("hashBasedMessageAuthenticationCode", &self.hash_based_message_authentication_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_email",
            "userEmail",
            "organizations",
            "is_admin",
            "isAdmin",
            "permissions",
            "created_date",
            "createdDate",
            "hash_based_message_authentication_code",
            "hashBasedMessageAuthenticationCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserEmail,
            Organizations,
            IsAdmin,
            Permissions,
            CreatedDate,
            HashBasedMessageAuthenticationCode,
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
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "userEmail" | "user_email" => Ok(GeneratedField::UserEmail),
                            "organizations" => Ok(GeneratedField::Organizations),
                            "isAdmin" | "is_admin" => Ok(GeneratedField::IsAdmin),
                            "permissions" => Ok(GeneratedField::Permissions),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "hashBasedMessageAuthenticationCode" | "hash_based_message_authentication_code" => Ok(GeneratedField::HashBasedMessageAuthenticationCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.me.v2.GetMeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetMeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_email__ = None;
                let mut organizations__ = None;
                let mut is_admin__ = None;
                let mut permissions__ = None;
                let mut created_date__ = None;
                let mut hash_based_message_authentication_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserEmail => {
                            if user_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userEmail"));
                            }
                            user_email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Organizations => {
                            if organizations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizations"));
                            }
                            organizations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAdmin => {
                            if is_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAdmin"));
                            }
                            is_admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedDate => {
                            if created_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdDate"));
                            }
                            created_date__ = map_.next_value()?;
                        }
                        GeneratedField::HashBasedMessageAuthenticationCode => {
                            if hash_based_message_authentication_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashBasedMessageAuthenticationCode"));
                            }
                            hash_based_message_authentication_code__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetMeResponse {
                    user_id: user_id__.unwrap_or_default(),
                    user_email: user_email__.unwrap_or_default(),
                    organizations: organizations__.unwrap_or_default(),
                    is_admin: is_admin__.unwrap_or_default(),
                    permissions: permissions__,
                    created_date: created_date__,
                    hash_based_message_authentication_code: hash_based_message_authentication_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.me.v2.GetMeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.permission_resources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.me.v2.PermissionResources", len)?;
        if !self.permission_resources.is_empty() {
            struct_ser.serialize_field("permissionResources", &self.permission_resources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permission_resources",
            "permissionResources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PermissionResources,
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
                            "permissionResources" | "permission_resources" => Ok(GeneratedField::PermissionResources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.me.v2.PermissionResources")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PermissionResources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permission_resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PermissionResources => {
                            if permission_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionResources"));
                            }
                            permission_resources__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(PermissionResources {
                    permission_resources: permission_resources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.me.v2.PermissionResources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Permissions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_permission_resources.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.me.v2.Permissions", len)?;
        if !self.organization_permission_resources.is_empty() {
            struct_ser.serialize_field("organizationPermissionResources", &self.organization_permission_resources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Permissions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_permission_resources",
            "organizationPermissionResources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationPermissionResources,
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
                            "organizationPermissionResources" | "organization_permission_resources" => Ok(GeneratedField::OrganizationPermissionResources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Permissions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.me.v2.Permissions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Permissions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_permission_resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationPermissionResources => {
                            if organization_permission_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationPermissionResources"));
                            }
                            organization_permission_resources__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Permissions {
                    organization_permission_resources: organization_permission_resources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.me.v2.Permissions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Resources {
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
        if self.all_assets {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.me.v2.Resources", len)?;
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if self.all_assets {
            struct_ser.serialize_field("allAssets", &self.all_assets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Resources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_ids",
            "assetIds",
            "all_assets",
            "allAssets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetIds,
            AllAssets,
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
                            "allAssets" | "all_assets" => Ok(GeneratedField::AllAssets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Resources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.me.v2.Resources")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Resources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_ids__ = None;
                let mut all_assets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllAssets => {
                            if all_assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allAssets"));
                            }
                            all_assets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Resources {
                    asset_ids: asset_ids__.unwrap_or_default(),
                    all_assets: all_assets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.me.v2.Resources", FIELDS, GeneratedVisitor)
    }
}
