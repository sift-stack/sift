// @generated
impl serde::Serialize for AddUserToUserGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_group_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.AddUserToUserGroupRequest", len)?;
        if !self.user_group_id.is_empty() {
            struct_ser.serialize_field("userGroupId", &self.user_group_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddUserToUserGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group_id",
            "userGroupId",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroupId,
            UserId,
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
                            "userGroupId" | "user_group_id" => Ok(GeneratedField::UserGroupId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddUserToUserGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.AddUserToUserGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddUserToUserGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group_id__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroupId => {
                            if user_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroupId"));
                            }
                            user_group_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddUserToUserGroupRequest {
                    user_group_id: user_group_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.AddUserToUserGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddUserToUserGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_groups.v2.AddUserToUserGroupResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddUserToUserGroupResponse {
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
            type Value = AddUserToUserGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.AddUserToUserGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddUserToUserGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddUserToUserGroupResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.AddUserToUserGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.CreateUserGroupRequest", len)?;
        if let Some(v) = self.user_group.as_ref() {
            struct_ser.serialize_field("userGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group",
            "userGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroup,
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
                            "userGroup" | "user_group" => Ok(GeneratedField::UserGroup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.CreateUserGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroup => {
                            if user_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroup"));
                            }
                            user_group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateUserGroupRequest {
                    user_group: user_group__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.CreateUserGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.CreateUserGroupResponse", len)?;
        if let Some(v) = self.user_group.as_ref() {
            struct_ser.serialize_field("userGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group",
            "userGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroup,
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
                            "userGroup" | "user_group" => Ok(GeneratedField::UserGroup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.CreateUserGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroup => {
                            if user_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroup"));
                            }
                            user_group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateUserGroupResponse {
                    user_group: user_group__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.CreateUserGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUserGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_group_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.DeleteUserGroupRequest", len)?;
        if !self.user_group_id.is_empty() {
            struct_ser.serialize_field("userGroupId", &self.user_group_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUserGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group_id",
            "userGroupId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroupId,
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
                            "userGroupId" | "user_group_id" => Ok(GeneratedField::UserGroupId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteUserGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.DeleteUserGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUserGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroupId => {
                            if user_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroupId"));
                            }
                            user_group_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteUserGroupRequest {
                    user_group_id: user_group_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.DeleteUserGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteUserGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_groups.v2.DeleteUserGroupResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteUserGroupResponse {
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
            type Value = DeleteUserGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.DeleteUserGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteUserGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteUserGroupResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.DeleteUserGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_group_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.GetUserGroupRequest", len)?;
        if !self.user_group_id.is_empty() {
            struct_ser.serialize_field("userGroupId", &self.user_group_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group_id",
            "userGroupId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroupId,
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
                            "userGroupId" | "user_group_id" => Ok(GeneratedField::UserGroupId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.GetUserGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroupId => {
                            if user_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroupId"));
                            }
                            user_group_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserGroupRequest {
                    user_group_id: user_group_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.GetUserGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.GetUserGroupResponse", len)?;
        if let Some(v) = self.user_group.as_ref() {
            struct_ser.serialize_field("userGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group",
            "userGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroup,
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
                            "userGroup" | "user_group" => Ok(GeneratedField::UserGroup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.GetUserGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroup => {
                            if user_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroup"));
                            }
                            user_group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserGroupResponse {
                    user_group: user_group__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.GetUserGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserGroupsForAssetsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.GetUserGroupsForAssetsRequest", len)?;
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserGroupsForAssetsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "asset_ids",
            "assetIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssetIds,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserGroupsForAssetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.GetUserGroupsForAssetsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserGroupsForAssetsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut asset_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserGroupsForAssetsRequest {
                    asset_ids: asset_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.GetUserGroupsForAssetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserGroupsForAssetsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_groups.is_empty() {
            len += 1;
        }
        if !self.roles.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.GetUserGroupsForAssetsResponse", len)?;
        if !self.user_groups.is_empty() {
            struct_ser.serialize_field("userGroups", &self.user_groups)?;
        }
        if !self.roles.is_empty() {
            struct_ser.serialize_field("roles", &self.roles)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserGroupsForAssetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_groups",
            "userGroups",
            "roles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroups,
            Roles,
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
                            "userGroups" | "user_groups" => Ok(GeneratedField::UserGroups),
                            "roles" => Ok(GeneratedField::Roles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserGroupsForAssetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.GetUserGroupsForAssetsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserGroupsForAssetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_groups__ = None;
                let mut roles__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroups => {
                            if user_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroups"));
                            }
                            user_groups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Roles => {
                            if roles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roles"));
                            }
                            roles__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserGroupsForAssetsResponse {
                    user_groups: user_groups__.unwrap_or_default(),
                    roles: roles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.GetUserGroupsForAssetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserGroupsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.ListUserGroupsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListUserGroupsRequest {
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
            type Value = ListUserGroupsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.ListUserGroupsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserGroupsRequest, V::Error>
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
                Ok(ListUserGroupsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.ListUserGroupsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserGroupsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_groups.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.ListUserGroupsResponse", len)?;
        if !self.user_groups.is_empty() {
            struct_ser.serialize_field("userGroups", &self.user_groups)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserGroupsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_groups",
            "userGroups",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroups,
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
                            "userGroups" | "user_groups" => Ok(GeneratedField::UserGroups),
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
            type Value = ListUserGroupsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.ListUserGroupsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserGroupsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_groups__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroups => {
                            if user_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroups"));
                            }
                            user_groups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUserGroupsResponse {
                    user_groups: user_groups__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.ListUserGroupsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveUserFromUserGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_group_id.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.RemoveUserFromUserGroupRequest", len)?;
        if !self.user_group_id.is_empty() {
            struct_ser.serialize_field("userGroupId", &self.user_group_id)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveUserFromUserGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group_id",
            "userGroupId",
            "user_id",
            "userId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroupId,
            UserId,
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
                            "userGroupId" | "user_group_id" => Ok(GeneratedField::UserGroupId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveUserFromUserGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.RemoveUserFromUserGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveUserFromUserGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group_id__ = None;
                let mut user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroupId => {
                            if user_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroupId"));
                            }
                            user_group_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveUserFromUserGroupRequest {
                    user_group_id: user_group_id__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.RemoveUserFromUserGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveUserFromUserGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_groups.v2.RemoveUserFromUserGroupResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveUserFromUserGroupResponse {
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
            type Value = RemoveUserFromUserGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.RemoveUserFromUserGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveUserFromUserGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveUserFromUserGroupResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.RemoveUserFromUserGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserGroupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_group.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.UpdateUserGroupRequest", len)?;
        if let Some(v) = self.user_group.as_ref() {
            struct_ser.serialize_field("userGroup", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserGroupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group",
            "userGroup",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroup,
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
                            "userGroup" | "user_group" => Ok(GeneratedField::UserGroup),
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
            type Value = UpdateUserGroupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.UpdateUserGroupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserGroupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroup => {
                            if user_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroup"));
                            }
                            user_group__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUserGroupRequest {
                    user_group: user_group__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.UpdateUserGroupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserGroupResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_group.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.UpdateUserGroupResponse", len)?;
        if let Some(v) = self.user_group.as_ref() {
            struct_ser.serialize_field("userGroup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserGroupResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group",
            "userGroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroup,
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
                            "userGroup" | "user_group" => Ok(GeneratedField::UserGroup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateUserGroupResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.UpdateUserGroupResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserGroupResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroup => {
                            if user_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroup"));
                            }
                            user_group__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUserGroupResponse {
                    user_group: user_group__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.UpdateUserGroupResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserUserGroupsRequest {
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
        if !self.user_group_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.UpdateUserUserGroupsRequest", len)?;
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("userId", &self.user_id)?;
        }
        if !self.user_group_ids.is_empty() {
            struct_ser.serialize_field("userGroupIds", &self.user_group_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserUserGroupsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_id",
            "userId",
            "user_group_ids",
            "userGroupIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            UserGroupIds,
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
                            "userGroupIds" | "user_group_ids" => Ok(GeneratedField::UserGroupIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateUserUserGroupsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.UpdateUserUserGroupsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserUserGroupsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut user_group_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserGroupIds => {
                            if user_group_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroupIds"));
                            }
                            user_group_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateUserUserGroupsRequest {
                    user_id: user_id__.unwrap_or_default(),
                    user_group_ids: user_group_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.UpdateUserUserGroupsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserUserGroupsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.user_groups.v2.UpdateUserUserGroupsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserUserGroupsResponse {
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
            type Value = UpdateUserUserGroupsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.UpdateUserUserGroupsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserUserGroupsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateUserUserGroupsResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.UpdateUserUserGroupsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserGroup {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_group_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.role_id.is_empty() {
            len += 1;
        }
        if self.is_default {
            len += 1;
        }
        if self.resources.is_some() {
            len += 1;
        }
        if !self.user_ids.is_empty() {
            len += 1;
        }
        if self.is_external {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.UserGroup", len)?;
        if !self.user_group_id.is_empty() {
            struct_ser.serialize_field("userGroupId", &self.user_group_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.role_id.is_empty() {
            struct_ser.serialize_field("roleId", &self.role_id)?;
        }
        if self.is_default {
            struct_ser.serialize_field("isDefault", &self.is_default)?;
        }
        if let Some(v) = self.resources.as_ref() {
            struct_ser.serialize_field("resources", v)?;
        }
        if !self.user_ids.is_empty() {
            struct_ser.serialize_field("userIds", &self.user_ids)?;
        }
        if self.is_external {
            struct_ser.serialize_field("isExternal", &self.is_external)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserGroup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_group_id",
            "userGroupId",
            "name",
            "role_id",
            "roleId",
            "is_default",
            "isDefault",
            "resources",
            "user_ids",
            "userIds",
            "is_external",
            "isExternal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserGroupId,
            Name,
            RoleId,
            IsDefault,
            Resources,
            UserIds,
            IsExternal,
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
                            "userGroupId" | "user_group_id" => Ok(GeneratedField::UserGroupId),
                            "name" => Ok(GeneratedField::Name),
                            "roleId" | "role_id" => Ok(GeneratedField::RoleId),
                            "isDefault" | "is_default" => Ok(GeneratedField::IsDefault),
                            "resources" => Ok(GeneratedField::Resources),
                            "userIds" | "user_ids" => Ok(GeneratedField::UserIds),
                            "isExternal" | "is_external" => Ok(GeneratedField::IsExternal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserGroup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.UserGroup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserGroup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_group_id__ = None;
                let mut name__ = None;
                let mut role_id__ = None;
                let mut is_default__ = None;
                let mut resources__ = None;
                let mut user_ids__ = None;
                let mut is_external__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserGroupId => {
                            if user_group_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userGroupId"));
                            }
                            user_group_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RoleId => {
                            if role_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roleId"));
                            }
                            role_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsDefault => {
                            if is_default__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDefault"));
                            }
                            is_default__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = map_.next_value()?;
                        }
                        GeneratedField::UserIds => {
                            if user_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userIds"));
                            }
                            user_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsExternal => {
                            if is_external__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExternal"));
                            }
                            is_external__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserGroup {
                    user_group_id: user_group_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    role_id: role_id__.unwrap_or_default(),
                    is_default: is_default__.unwrap_or_default(),
                    resources: resources__,
                    user_ids: user_ids__.unwrap_or_default(),
                    is_external: is_external__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.UserGroup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for user_group::Resource {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_groups.v2.UserGroup.Resource", len)?;
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if self.all_assets {
            struct_ser.serialize_field("allAssets", &self.all_assets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for user_group::Resource {
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
            type Value = user_group::Resource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_groups.v2.UserGroup.Resource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<user_group::Resource, V::Error>
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
                Ok(user_group::Resource {
                    asset_ids: asset_ids__.unwrap_or_default(),
                    all_assets: all_assets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_groups.v2.UserGroup.Resource", FIELDS, GeneratedVisitor)
    }
}
