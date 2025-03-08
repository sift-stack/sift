// @generated
impl serde::Serialize for Tag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("sift.tags.v1.Tag", len)?;
        if !self.tag_id.is_empty() {
            struct_ser.serialize_field("tagId", &self.tag_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
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
impl<'de> serde::Deserialize<'de> for Tag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag_id",
            "tagId",
            "name",
            "organization_id",
            "organizationId",
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
            TagId,
            Name,
            OrganizationId,
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
                            "tagId" | "tag_id" => Ok(GeneratedField::TagId),
                            "name" => Ok(GeneratedField::Name),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
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
            type Value = Tag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.tags.v1.Tag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_id__ = None;
                let mut name__ = None;
                let mut organization_id__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TagId => {
                            if tag_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagId"));
                            }
                            tag_id__ = Some(map_.next_value()?);
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
                Ok(Tag {
                    tag_id: tag_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.tags.v1.Tag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.tags.v1.TagRef", len)?;
        if !self.tag_id.is_empty() {
            struct_ser.serialize_field("tagId", &self.tag_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag_id",
            "tagId",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagId,
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
                            "tagId" | "tag_id" => Ok(GeneratedField::TagId),
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
            type Value = TagRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.tags.v1.TagRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TagRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TagId => {
                            if tag_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagId"));
                            }
                            tag_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TagRef {
                    tag_id: tag_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.tags.v1.TagRef", FIELDS, GeneratedVisitor)
    }
}
