// @generated
impl serde::Serialize for ChannelConfigurations {
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
        if self.r#type != 0 {
            len += 1;
        }
        if !self.color.is_empty() {
            len += 1;
        }
        if self.channel_settings.is_some() {
            len += 1;
        }
        if self.bit_field_index.is_some() {
            len += 1;
        }
        if self.bit_field_element.is_some() {
            len += 1;
        }
        if self.expression.is_some() {
            len += 1;
        }
        if !self.expression_channel_references.is_empty() {
            len += 1;
        }
        if self.data_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.ChannelConfigurations", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.r#type != 0 {
            let v = PlottedChannelType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.color.is_empty() {
            struct_ser.serialize_field("color", &self.color)?;
        }
        if let Some(v) = self.channel_settings.as_ref() {
            struct_ser.serialize_field("channelSettings", v)?;
        }
        if let Some(v) = self.bit_field_index.as_ref() {
            struct_ser.serialize_field("bitFieldIndex", v)?;
        }
        if let Some(v) = self.bit_field_element.as_ref() {
            struct_ser.serialize_field("bitFieldElement", v)?;
        }
        if let Some(v) = self.expression.as_ref() {
            struct_ser.serialize_field("expression", v)?;
        }
        if !self.expression_channel_references.is_empty() {
            struct_ser.serialize_field("expressionChannelReferences", &self.expression_channel_references)?;
        }
        if let Some(v) = self.data_type.as_ref() {
            let v = super::super::common::r#type::v1::ChannelDataType::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("dataType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChannelConfigurations {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "color",
            "channel_settings",
            "channelSettings",
            "bit_field_index",
            "bitFieldIndex",
            "bit_field_element",
            "bitFieldElement",
            "expression",
            "expression_channel_references",
            "expressionChannelReferences",
            "data_type",
            "dataType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            Color,
            ChannelSettings,
            BitFieldIndex,
            BitFieldElement,
            Expression,
            ExpressionChannelReferences,
            DataType,
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
                            "type" => Ok(GeneratedField::Type),
                            "color" => Ok(GeneratedField::Color),
                            "channelSettings" | "channel_settings" => Ok(GeneratedField::ChannelSettings),
                            "bitFieldIndex" | "bit_field_index" => Ok(GeneratedField::BitFieldIndex),
                            "bitFieldElement" | "bit_field_element" => Ok(GeneratedField::BitFieldElement),
                            "expression" => Ok(GeneratedField::Expression),
                            "expressionChannelReferences" | "expression_channel_references" => Ok(GeneratedField::ExpressionChannelReferences),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChannelConfigurations;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.ChannelConfigurations")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChannelConfigurations, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut color__ = None;
                let mut channel_settings__ = None;
                let mut bit_field_index__ = None;
                let mut bit_field_element__ = None;
                let mut expression__ = None;
                let mut expression_channel_references__ = None;
                let mut data_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<PlottedChannelType>()? as i32);
                        }
                        GeneratedField::Color => {
                            if color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("color"));
                            }
                            color__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelSettings => {
                            if channel_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelSettings"));
                            }
                            channel_settings__ = map_.next_value()?;
                        }
                        GeneratedField::BitFieldIndex => {
                            if bit_field_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldIndex"));
                            }
                            bit_field_index__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::BitFieldElement => {
                            if bit_field_element__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldElement"));
                            }
                            bit_field_element__ = map_.next_value()?;
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = map_.next_value()?;
                        }
                        GeneratedField::ExpressionChannelReferences => {
                            if expression_channel_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expressionChannelReferences"));
                            }
                            expression_channel_references__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = map_.next_value::<::std::option::Option<super::super::common::r#type::v1::ChannelDataType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(ChannelConfigurations {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    color: color__.unwrap_or_default(),
                    channel_settings: channel_settings__,
                    bit_field_index: bit_field_index__,
                    bit_field_element: bit_field_element__,
                    expression: expression__,
                    expression_channel_references: expression_channel_references__.unwrap_or_default(),
                    data_type: data_type__,
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.ChannelConfigurations", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePanelConfigurationRequest {
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
        if self.panel_type != 0 {
            len += 1;
        }
        if !self.channel_configurations.is_empty() {
            len += 1;
        }
        if self.chart_settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.CreatePanelConfigurationRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.panel_type != 0 {
            let v = PanelType::try_from(self.panel_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.panel_type)))?;
            struct_ser.serialize_field("panelType", &v)?;
        }
        if !self.channel_configurations.is_empty() {
            struct_ser.serialize_field("channelConfigurations", &self.channel_configurations)?;
        }
        if let Some(v) = self.chart_settings.as_ref() {
            struct_ser.serialize_field("chartSettings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePanelConfigurationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "panel_type",
            "panelType",
            "channel_configurations",
            "channelConfigurations",
            "chart_settings",
            "chartSettings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            PanelType,
            ChannelConfigurations,
            ChartSettings,
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
                            "panelType" | "panel_type" => Ok(GeneratedField::PanelType),
                            "channelConfigurations" | "channel_configurations" => Ok(GeneratedField::ChannelConfigurations),
                            "chartSettings" | "chart_settings" => Ok(GeneratedField::ChartSettings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePanelConfigurationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.CreatePanelConfigurationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePanelConfigurationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut panel_type__ = None;
                let mut channel_configurations__ = None;
                let mut chart_settings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PanelType => {
                            if panel_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelType"));
                            }
                            panel_type__ = Some(map_.next_value::<PanelType>()? as i32);
                        }
                        GeneratedField::ChannelConfigurations => {
                            if channel_configurations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelConfigurations"));
                            }
                            channel_configurations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChartSettings => {
                            if chart_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chartSettings"));
                            }
                            chart_settings__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePanelConfigurationRequest {
                    name: name__.unwrap_or_default(),
                    panel_type: panel_type__.unwrap_or_default(),
                    channel_configurations: channel_configurations__.unwrap_or_default(),
                    chart_settings: chart_settings__,
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.CreatePanelConfigurationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePanelConfigurationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.panel_configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.CreatePanelConfigurationResponse", len)?;
        if let Some(v) = self.panel_configuration.as_ref() {
            struct_ser.serialize_field("panelConfiguration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePanelConfigurationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "panel_configuration",
            "panelConfiguration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PanelConfiguration,
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
                            "panelConfiguration" | "panel_configuration" => Ok(GeneratedField::PanelConfiguration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePanelConfigurationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.CreatePanelConfigurationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePanelConfigurationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut panel_configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PanelConfiguration => {
                            if panel_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelConfiguration"));
                            }
                            panel_configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePanelConfigurationResponse {
                    panel_configuration: panel_configuration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.CreatePanelConfigurationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPanelConfigurationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.panel_configuration_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.GetPanelConfigurationRequest", len)?;
        if !self.panel_configuration_id.is_empty() {
            struct_ser.serialize_field("panelConfigurationId", &self.panel_configuration_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPanelConfigurationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "panel_configuration_id",
            "panelConfigurationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PanelConfigurationId,
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
                            "panelConfigurationId" | "panel_configuration_id" => Ok(GeneratedField::PanelConfigurationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPanelConfigurationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.GetPanelConfigurationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPanelConfigurationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut panel_configuration_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PanelConfigurationId => {
                            if panel_configuration_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelConfigurationId"));
                            }
                            panel_configuration_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPanelConfigurationRequest {
                    panel_configuration_id: panel_configuration_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.GetPanelConfigurationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPanelConfigurationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.panel_configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.GetPanelConfigurationResponse", len)?;
        if let Some(v) = self.panel_configuration.as_ref() {
            struct_ser.serialize_field("panelConfiguration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPanelConfigurationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "panel_configuration",
            "panelConfiguration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PanelConfiguration,
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
                            "panelConfiguration" | "panel_configuration" => Ok(GeneratedField::PanelConfiguration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPanelConfigurationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.GetPanelConfigurationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPanelConfigurationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut panel_configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PanelConfiguration => {
                            if panel_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelConfiguration"));
                            }
                            panel_configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPanelConfigurationResponse {
                    panel_configuration: panel_configuration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.GetPanelConfigurationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPanelConfigurationsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.ListPanelConfigurationsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListPanelConfigurationsRequest {
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
            type Value = ListPanelConfigurationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.ListPanelConfigurationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPanelConfigurationsRequest, V::Error>
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
                Ok(ListPanelConfigurationsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.ListPanelConfigurationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPanelConfigurationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.panel_configurations.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.ListPanelConfigurationsResponse", len)?;
        if !self.panel_configurations.is_empty() {
            struct_ser.serialize_field("panelConfigurations", &self.panel_configurations)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPanelConfigurationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "panel_configurations",
            "panelConfigurations",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PanelConfigurations,
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
                            "panelConfigurations" | "panel_configurations" => Ok(GeneratedField::PanelConfigurations),
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
            type Value = ListPanelConfigurationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.ListPanelConfigurationsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPanelConfigurationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut panel_configurations__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PanelConfigurations => {
                            if panel_configurations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelConfigurations"));
                            }
                            panel_configurations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPanelConfigurationsResponse {
                    panel_configurations: panel_configurations__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.ListPanelConfigurationsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PanelConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.panel_configuration_id.is_empty() {
            len += 1;
        }
        if !self.version_id.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if !self.change_message.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.panel_type != 0 {
            len += 1;
        }
        if !self.channel_configurations.is_empty() {
            len += 1;
        }
        if self.chart_settings.is_some() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.PanelConfiguration", len)?;
        if !self.panel_configuration_id.is_empty() {
            struct_ser.serialize_field("panelConfigurationId", &self.panel_configuration_id)?;
        }
        if !self.version_id.is_empty() {
            struct_ser.serialize_field("versionId", &self.version_id)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.change_message.is_empty() {
            struct_ser.serialize_field("changeMessage", &self.change_message)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.panel_type != 0 {
            let v = PanelType::try_from(self.panel_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.panel_type)))?;
            struct_ser.serialize_field("panelType", &v)?;
        }
        if !self.channel_configurations.is_empty() {
            struct_ser.serialize_field("channelConfigurations", &self.channel_configurations)?;
        }
        if let Some(v) = self.chart_settings.as_ref() {
            struct_ser.serialize_field("chartSettings", v)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PanelConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "panel_configuration_id",
            "panelConfigurationId",
            "version_id",
            "versionId",
            "version",
            "change_message",
            "changeMessage",
            "name",
            "panel_type",
            "panelType",
            "channel_configurations",
            "channelConfigurations",
            "chart_settings",
            "chartSettings",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "archived_date",
            "archivedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PanelConfigurationId,
            VersionId,
            Version,
            ChangeMessage,
            Name,
            PanelType,
            ChannelConfigurations,
            ChartSettings,
            CreatedDate,
            ModifiedDate,
            ArchivedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "panelConfigurationId" | "panel_configuration_id" => Ok(GeneratedField::PanelConfigurationId),
                            "versionId" | "version_id" => Ok(GeneratedField::VersionId),
                            "version" => Ok(GeneratedField::Version),
                            "changeMessage" | "change_message" => Ok(GeneratedField::ChangeMessage),
                            "name" => Ok(GeneratedField::Name),
                            "panelType" | "panel_type" => Ok(GeneratedField::PanelType),
                            "channelConfigurations" | "channel_configurations" => Ok(GeneratedField::ChannelConfigurations),
                            "chartSettings" | "chart_settings" => Ok(GeneratedField::ChartSettings),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
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
            type Value = PanelConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.PanelConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PanelConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut panel_configuration_id__ = None;
                let mut version_id__ = None;
                let mut version__ = None;
                let mut change_message__ = None;
                let mut name__ = None;
                let mut panel_type__ = None;
                let mut channel_configurations__ = None;
                let mut chart_settings__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut archived_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PanelConfigurationId => {
                            if panel_configuration_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelConfigurationId"));
                            }
                            panel_configuration_id__ = Some(map_.next_value()?);
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
                        GeneratedField::ChangeMessage => {
                            if change_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeMessage"));
                            }
                            change_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PanelType => {
                            if panel_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelType"));
                            }
                            panel_type__ = Some(map_.next_value::<PanelType>()? as i32);
                        }
                        GeneratedField::ChannelConfigurations => {
                            if channel_configurations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelConfigurations"));
                            }
                            channel_configurations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChartSettings => {
                            if chart_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chartSettings"));
                            }
                            chart_settings__ = map_.next_value()?;
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
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
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
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PanelConfiguration {
                    panel_configuration_id: panel_configuration_id__.unwrap_or_default(),
                    version_id: version_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    change_message: change_message__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    panel_type: panel_type__.unwrap_or_default(),
                    channel_configurations: channel_configurations__.unwrap_or_default(),
                    chart_settings: chart_settings__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    archived_date: archived_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.PanelConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PanelType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PANEL_TYPE_UNSPECIFIED",
            Self::Timeseries => "PANEL_TYPE_TIMESERIES",
            Self::Table => "PANEL_TYPE_TABLE",
            Self::Fft => "PANEL_TYPE_FFT",
            Self::GeoMap => "PANEL_TYPE_GEO_MAP",
            Self::ScatterPlot => "PANEL_TYPE_SCATTER_PLOT",
            Self::FileViewer => "PANEL_TYPE_FILE_VIEWER",
            Self::Histogram => "PANEL_TYPE_HISTOGRAM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PanelType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PANEL_TYPE_UNSPECIFIED",
            "PANEL_TYPE_TIMESERIES",
            "PANEL_TYPE_TABLE",
            "PANEL_TYPE_FFT",
            "PANEL_TYPE_GEO_MAP",
            "PANEL_TYPE_SCATTER_PLOT",
            "PANEL_TYPE_FILE_VIEWER",
            "PANEL_TYPE_HISTOGRAM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PanelType;

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
                    "PANEL_TYPE_UNSPECIFIED" => Ok(PanelType::Unspecified),
                    "PANEL_TYPE_TIMESERIES" => Ok(PanelType::Timeseries),
                    "PANEL_TYPE_TABLE" => Ok(PanelType::Table),
                    "PANEL_TYPE_FFT" => Ok(PanelType::Fft),
                    "PANEL_TYPE_GEO_MAP" => Ok(PanelType::GeoMap),
                    "PANEL_TYPE_SCATTER_PLOT" => Ok(PanelType::ScatterPlot),
                    "PANEL_TYPE_FILE_VIEWER" => Ok(PanelType::FileViewer),
                    "PANEL_TYPE_HISTOGRAM" => Ok(PanelType::Histogram),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PlottedChannelType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PLOTTED_CHANNEL_TYPE_UNSPECIFIED",
            Self::Regular => "PLOTTED_CHANNEL_TYPE_REGULAR",
            Self::CalculatedChannel => "PLOTTED_CHANNEL_TYPE_CALCULATED_CHANNEL",
            Self::BitFieldElement => "PLOTTED_CHANNEL_TYPE_BIT_FIELD_ELEMENT",
            Self::Enum => "PLOTTED_CHANNEL_TYPE_ENUM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for PlottedChannelType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PLOTTED_CHANNEL_TYPE_UNSPECIFIED",
            "PLOTTED_CHANNEL_TYPE_REGULAR",
            "PLOTTED_CHANNEL_TYPE_CALCULATED_CHANNEL",
            "PLOTTED_CHANNEL_TYPE_BIT_FIELD_ELEMENT",
            "PLOTTED_CHANNEL_TYPE_ENUM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PlottedChannelType;

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
                    "PLOTTED_CHANNEL_TYPE_UNSPECIFIED" => Ok(PlottedChannelType::Unspecified),
                    "PLOTTED_CHANNEL_TYPE_REGULAR" => Ok(PlottedChannelType::Regular),
                    "PLOTTED_CHANNEL_TYPE_CALCULATED_CHANNEL" => Ok(PlottedChannelType::CalculatedChannel),
                    "PLOTTED_CHANNEL_TYPE_BIT_FIELD_ELEMENT" => Ok(PlottedChannelType::BitFieldElement),
                    "PLOTTED_CHANNEL_TYPE_ENUM" => Ok(PlottedChannelType::Enum),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePanelConfigurationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.panel_configuration.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.UpdatePanelConfigurationRequest", len)?;
        if let Some(v) = self.panel_configuration.as_ref() {
            struct_ser.serialize_field("panelConfiguration", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePanelConfigurationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "panel_configuration",
            "panelConfiguration",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PanelConfiguration,
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
                            "panelConfiguration" | "panel_configuration" => Ok(GeneratedField::PanelConfiguration),
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
            type Value = UpdatePanelConfigurationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.UpdatePanelConfigurationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePanelConfigurationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut panel_configuration__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PanelConfiguration => {
                            if panel_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelConfiguration"));
                            }
                            panel_configuration__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePanelConfigurationRequest {
                    panel_configuration: panel_configuration__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.UpdatePanelConfigurationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePanelConfigurationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.panel_configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.panel_configurations.v1.UpdatePanelConfigurationResponse", len)?;
        if let Some(v) = self.panel_configuration.as_ref() {
            struct_ser.serialize_field("panelConfiguration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePanelConfigurationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "panel_configuration",
            "panelConfiguration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PanelConfiguration,
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
                            "panelConfiguration" | "panel_configuration" => Ok(GeneratedField::PanelConfiguration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePanelConfigurationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.panel_configurations.v1.UpdatePanelConfigurationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePanelConfigurationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut panel_configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PanelConfiguration => {
                            if panel_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panelConfiguration"));
                            }
                            panel_configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePanelConfigurationResponse {
                    panel_configuration: panel_configuration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.panel_configurations.v1.UpdatePanelConfigurationResponse", FIELDS, GeneratedVisitor)
    }
}
