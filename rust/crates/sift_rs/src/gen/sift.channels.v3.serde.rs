// @generated
impl serde::Serialize for Channel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.data_type != 0 {
            len += 1;
        }
        if !self.enum_types.is_empty() {
            len += 1;
        }
        if !self.bit_field_elements.is_empty() {
            len += 1;
        }
        if !self.display_description.is_empty() {
            len += 1;
        }
        if !self.display_unit_id.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.Channel", len)?;
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if self.data_type != 0 {
            let v = super::super::common::r#type::v1::ChannelDataType::try_from(self.data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        if !self.enum_types.is_empty() {
            struct_ser.serialize_field("enumTypes", &self.enum_types)?;
        }
        if !self.bit_field_elements.is_empty() {
            struct_ser.serialize_field("bitFieldElements", &self.bit_field_elements)?;
        }
        if !self.display_description.is_empty() {
            struct_ser.serialize_field("displayDescription", &self.display_description)?;
        }
        if !self.display_unit_id.is_empty() {
            struct_ser.serialize_field("displayUnitId", &self.display_unit_id)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Channel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_id",
            "channelId",
            "name",
            "asset_id",
            "assetId",
            "description",
            "unit_id",
            "unitId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "data_type",
            "dataType",
            "enum_types",
            "enumTypes",
            "bit_field_elements",
            "bitFieldElements",
            "display_description",
            "displayDescription",
            "display_unit_id",
            "displayUnitId",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelId,
            Name,
            AssetId,
            Description,
            UnitId,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
            DataType,
            EnumTypes,
            BitFieldElements,
            DisplayDescription,
            DisplayUnitId,
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
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            "name" => Ok(GeneratedField::Name),
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "description" => Ok(GeneratedField::Description),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "enumTypes" | "enum_types" => Ok(GeneratedField::EnumTypes),
                            "bitFieldElements" | "bit_field_elements" => Ok(GeneratedField::BitFieldElements),
                            "displayDescription" | "display_description" => Ok(GeneratedField::DisplayDescription),
                            "displayUnitId" | "display_unit_id" => Ok(GeneratedField::DisplayUnitId),
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
            type Value = Channel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.Channel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Channel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_id__ = None;
                let mut name__ = None;
                let mut asset_id__ = None;
                let mut description__ = None;
                let mut unit_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut data_type__ = None;
                let mut enum_types__ = None;
                let mut bit_field_elements__ = None;
                let mut display_description__ = None;
                let mut display_unit_id__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetId => {
                            if asset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetId"));
                            }
                            asset_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
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
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value::<super::super::common::r#type::v1::ChannelDataType>()? as i32);
                        }
                        GeneratedField::EnumTypes => {
                            if enum_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumTypes"));
                            }
                            enum_types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BitFieldElements => {
                            if bit_field_elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldElements"));
                            }
                            bit_field_elements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayDescription => {
                            if display_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayDescription"));
                            }
                            display_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayUnitId => {
                            if display_unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayUnitId"));
                            }
                            display_unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Channel {
                    channel_id: channel_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    unit_id: unit_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    enum_types: enum_types__.unwrap_or_default(),
                    bit_field_elements: bit_field_elements__.unwrap_or_default(),
                    display_description: display_description__.unwrap_or_default(),
                    display_unit_id: display_unit_id__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.Channel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.asset_id.is_empty() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.display_description.is_empty() {
            len += 1;
        }
        if !self.unit_id.is_empty() {
            len += 1;
        }
        if !self.display_unit_id.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if !self.display_unit.is_empty() {
            len += 1;
        }
        if self.data_type != 0 {
            len += 1;
        }
        if !self.enum_types.is_empty() {
            len += 1;
        }
        if !self.bit_field_elements.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.FilterChannel", len)?;
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.asset_id.is_empty() {
            struct_ser.serialize_field("assetId", &self.asset_id)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.display_description.is_empty() {
            struct_ser.serialize_field("displayDescription", &self.display_description)?;
        }
        if !self.unit_id.is_empty() {
            struct_ser.serialize_field("unitId", &self.unit_id)?;
        }
        if !self.display_unit_id.is_empty() {
            struct_ser.serialize_field("displayUnitId", &self.display_unit_id)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if !self.display_unit.is_empty() {
            struct_ser.serialize_field("displayUnit", &self.display_unit)?;
        }
        if self.data_type != 0 {
            let v = super::super::common::r#type::v1::ChannelDataType::try_from(self.data_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.data_type)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        if !self.enum_types.is_empty() {
            struct_ser.serialize_field("enumTypes", &self.enum_types)?;
        }
        if !self.bit_field_elements.is_empty() {
            struct_ser.serialize_field("bitFieldElements", &self.bit_field_elements)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_id",
            "channelId",
            "name",
            "organization_id",
            "organizationId",
            "asset_id",
            "assetId",
            "asset_name",
            "assetName",
            "description",
            "display_description",
            "displayDescription",
            "unit_id",
            "unitId",
            "display_unit_id",
            "displayUnitId",
            "unit",
            "display_unit",
            "displayUnit",
            "data_type",
            "dataType",
            "enum_types",
            "enumTypes",
            "bit_field_elements",
            "bitFieldElements",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelId,
            Name,
            OrganizationId,
            AssetId,
            AssetName,
            Description,
            DisplayDescription,
            UnitId,
            DisplayUnitId,
            Unit,
            DisplayUnit,
            DataType,
            EnumTypes,
            BitFieldElements,
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
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            "name" => Ok(GeneratedField::Name),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "assetId" | "asset_id" => Ok(GeneratedField::AssetId),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "description" => Ok(GeneratedField::Description),
                            "displayDescription" | "display_description" => Ok(GeneratedField::DisplayDescription),
                            "unitId" | "unit_id" => Ok(GeneratedField::UnitId),
                            "displayUnitId" | "display_unit_id" => Ok(GeneratedField::DisplayUnitId),
                            "unit" => Ok(GeneratedField::Unit),
                            "displayUnit" | "display_unit" => Ok(GeneratedField::DisplayUnit),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "enumTypes" | "enum_types" => Ok(GeneratedField::EnumTypes),
                            "bitFieldElements" | "bit_field_elements" => Ok(GeneratedField::BitFieldElements),
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
            type Value = FilterChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.FilterChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_id__ = None;
                let mut name__ = None;
                let mut organization_id__ = None;
                let mut asset_id__ = None;
                let mut asset_name__ = None;
                let mut description__ = None;
                let mut display_description__ = None;
                let mut unit_id__ = None;
                let mut display_unit_id__ = None;
                let mut unit__ = None;
                let mut display_unit__ = None;
                let mut data_type__ = None;
                let mut enum_types__ = None;
                let mut bit_field_elements__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map_.next_value()?);
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
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayDescription => {
                            if display_description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayDescription"));
                            }
                            display_description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnitId => {
                            if unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitId"));
                            }
                            unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayUnitId => {
                            if display_unit_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayUnitId"));
                            }
                            display_unit_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayUnit => {
                            if display_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayUnit"));
                            }
                            display_unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value::<super::super::common::r#type::v1::ChannelDataType>()? as i32);
                        }
                        GeneratedField::EnumTypes => {
                            if enum_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enumTypes"));
                            }
                            enum_types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BitFieldElements => {
                            if bit_field_elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldElements"));
                            }
                            bit_field_elements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FilterChannel {
                    channel_id: channel_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    asset_id: asset_id__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    display_description: display_description__.unwrap_or_default(),
                    unit_id: unit_id__.unwrap_or_default(),
                    display_unit_id: display_unit_id__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    display_unit: display_unit__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    enum_types: enum_types__.unwrap_or_default(),
                    bit_field_elements: bit_field_elements__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.FilterChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterChannelsRequest {
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
        if !self.search_term.is_empty() {
            len += 1;
        }
        if self.is_search_case_sensitive {
            len += 1;
        }
        if self.is_search_regexp {
            len += 1;
        }
        if !self.asset_ids.is_empty() {
            len += 1;
        }
        if !self.run_ids.is_empty() {
            len += 1;
        }
        if !self.channel_ids.is_empty() {
            len += 1;
        }
        if !self.asset_tag_ids.is_empty() {
            len += 1;
        }
        if !self.data_types.is_empty() {
            len += 1;
        }
        if !self.metadata_keys.is_empty() {
            len += 1;
        }
        if !self.metadata_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.FilterChannelsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.search_term.is_empty() {
            struct_ser.serialize_field("searchTerm", &self.search_term)?;
        }
        if self.is_search_case_sensitive {
            struct_ser.serialize_field("isSearchCaseSensitive", &self.is_search_case_sensitive)?;
        }
        if self.is_search_regexp {
            struct_ser.serialize_field("isSearchRegexp", &self.is_search_regexp)?;
        }
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if !self.run_ids.is_empty() {
            struct_ser.serialize_field("runIds", &self.run_ids)?;
        }
        if !self.channel_ids.is_empty() {
            struct_ser.serialize_field("channelIds", &self.channel_ids)?;
        }
        if !self.asset_tag_ids.is_empty() {
            struct_ser.serialize_field("assetTagIds", &self.asset_tag_ids)?;
        }
        if !self.data_types.is_empty() {
            let v = self.data_types.iter().cloned().map(|v| {
                super::super::common::r#type::v1::ChannelDataType::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("dataTypes", &v)?;
        }
        if !self.metadata_keys.is_empty() {
            struct_ser.serialize_field("metadataKeys", &self.metadata_keys)?;
        }
        if !self.metadata_values.is_empty() {
            struct_ser.serialize_field("metadataValues", &self.metadata_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterChannelsRequest {
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
            "search_term",
            "searchTerm",
            "is_search_case_sensitive",
            "isSearchCaseSensitive",
            "is_search_regexp",
            "isSearchRegexp",
            "asset_ids",
            "assetIds",
            "run_ids",
            "runIds",
            "channel_ids",
            "channelIds",
            "asset_tag_ids",
            "assetTagIds",
            "data_types",
            "dataTypes",
            "metadata_keys",
            "metadataKeys",
            "metadata_values",
            "metadataValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            SearchTerm,
            IsSearchCaseSensitive,
            IsSearchRegexp,
            AssetIds,
            RunIds,
            ChannelIds,
            AssetTagIds,
            DataTypes,
            MetadataKeys,
            MetadataValues,
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
                            "searchTerm" | "search_term" => Ok(GeneratedField::SearchTerm),
                            "isSearchCaseSensitive" | "is_search_case_sensitive" => Ok(GeneratedField::IsSearchCaseSensitive),
                            "isSearchRegexp" | "is_search_regexp" => Ok(GeneratedField::IsSearchRegexp),
                            "assetIds" | "asset_ids" => Ok(GeneratedField::AssetIds),
                            "runIds" | "run_ids" => Ok(GeneratedField::RunIds),
                            "channelIds" | "channel_ids" => Ok(GeneratedField::ChannelIds),
                            "assetTagIds" | "asset_tag_ids" => Ok(GeneratedField::AssetTagIds),
                            "dataTypes" | "data_types" => Ok(GeneratedField::DataTypes),
                            "metadataKeys" | "metadata_keys" => Ok(GeneratedField::MetadataKeys),
                            "metadataValues" | "metadata_values" => Ok(GeneratedField::MetadataValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterChannelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.FilterChannelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterChannelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut search_term__ = None;
                let mut is_search_case_sensitive__ = None;
                let mut is_search_regexp__ = None;
                let mut asset_ids__ = None;
                let mut run_ids__ = None;
                let mut channel_ids__ = None;
                let mut asset_tag_ids__ = None;
                let mut data_types__ = None;
                let mut metadata_keys__ = None;
                let mut metadata_values__ = None;
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
                        GeneratedField::SearchTerm => {
                            if search_term__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searchTerm"));
                            }
                            search_term__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsSearchCaseSensitive => {
                            if is_search_case_sensitive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSearchCaseSensitive"));
                            }
                            is_search_case_sensitive__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsSearchRegexp => {
                            if is_search_regexp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSearchRegexp"));
                            }
                            is_search_regexp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetIds => {
                            if asset_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetIds"));
                            }
                            asset_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunIds => {
                            if run_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runIds"));
                            }
                            run_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelIds => {
                            if channel_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelIds"));
                            }
                            channel_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetTagIds => {
                            if asset_tag_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetTagIds"));
                            }
                            asset_tag_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataTypes => {
                            if data_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataTypes"));
                            }
                            data_types__ = Some(map_.next_value::<Vec<super::super::common::r#type::v1::ChannelDataType>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::MetadataKeys => {
                            if metadata_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataKeys"));
                            }
                            metadata_keys__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MetadataValues => {
                            if metadata_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataValues"));
                            }
                            metadata_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FilterChannelsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    search_term: search_term__.unwrap_or_default(),
                    is_search_case_sensitive: is_search_case_sensitive__.unwrap_or_default(),
                    is_search_regexp: is_search_regexp__.unwrap_or_default(),
                    asset_ids: asset_ids__.unwrap_or_default(),
                    run_ids: run_ids__.unwrap_or_default(),
                    channel_ids: channel_ids__.unwrap_or_default(),
                    asset_tag_ids: asset_tag_ids__.unwrap_or_default(),
                    data_types: data_types__.unwrap_or_default(),
                    metadata_keys: metadata_keys__.unwrap_or_default(),
                    metadata_values: metadata_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.FilterChannelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterChannelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channels.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.FilterChannelsResponse", len)?;
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterChannelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channels",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channels,
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
                            "channels" => Ok(GeneratedField::Channels),
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
            type Value = FilterChannelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.FilterChannelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FilterChannelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channels__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FilterChannelsResponse {
                    channels: channels__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.FilterChannelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.GetChannelRequest", len)?;
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_id",
            "channelId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelId,
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
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.GetChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetChannelRequest {
                    channel_id: channel_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.GetChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.GetChannelResponse", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
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
                            "channel" => Ok(GeneratedField::Channel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.GetChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetChannelResponse {
                    channel: channel__,
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.GetChannelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListChannelsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.ListChannelsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListChannelsRequest {
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
            type Value = ListChannelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.ListChannelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListChannelsRequest, V::Error>
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
                Ok(ListChannelsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.ListChannelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListChannelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channels.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.ListChannelsResponse", len)?;
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListChannelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channels",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channels,
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
                            "channels" => Ok(GeneratedField::Channels),
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
            type Value = ListChannelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.ListChannelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListChannelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channels__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListChannelsResponse {
                    channels: channels__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.ListChannelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateChannelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.UpdateChannelRequest", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateChannelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
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
                            "channel" => Ok(GeneratedField::Channel),
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
            type Value = UpdateChannelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.UpdateChannelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateChannelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateChannelRequest {
                    channel: channel__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.UpdateChannelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateChannelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.channel.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.channels.v3.UpdateChannelResponse", len)?;
        if let Some(v) = self.channel.as_ref() {
            struct_ser.serialize_field("channel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateChannelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Channel,
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
                            "channel" => Ok(GeneratedField::Channel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateChannelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.channels.v3.UpdateChannelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateChannelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Channel => {
                            if channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channel"));
                            }
                            channel__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateChannelResponse {
                    channel: channel__,
                })
            }
        }
        deserializer.deserialize_struct("sift.channels.v3.UpdateChannelResponse", FIELDS, GeneratedVisitor)
    }
}
