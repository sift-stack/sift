// @generated
impl serde::Serialize for CreateViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.CreateViewRequest", len)?;
        if let Some(v) = self.view.as_ref() {
            struct_ser.serialize_field("view", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            View,
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
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.CreateViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateViewRequest {
                    view: view__,
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.CreateViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.CreateViewResponse", len)?;
        if let Some(v) = self.view.as_ref() {
            struct_ser.serialize_field("view", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateViewResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            View,
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
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.CreateViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateViewResponse {
                    view: view__,
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.CreateViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.view_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.DeleteViewRequest", len)?;
        if !self.view_id.is_empty() {
            struct_ser.serialize_field("viewId", &self.view_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view_id",
            "viewId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ViewId,
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
                            "viewId" | "view_id" => Ok(GeneratedField::ViewId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.DeleteViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ViewId => {
                            if view_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewId"));
                            }
                            view_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteViewRequest {
                    view_id: view_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.DeleteViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.views.v1.DeleteViewResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteViewResponse {
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
            type Value = DeleteViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.DeleteViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteViewResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.DeleteViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.view_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.GetViewRequest", len)?;
        if !self.view_id.is_empty() {
            struct_ser.serialize_field("viewId", &self.view_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view_id",
            "viewId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ViewId,
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
                            "viewId" | "view_id" => Ok(GeneratedField::ViewId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.GetViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ViewId => {
                            if view_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewId"));
                            }
                            view_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetViewRequest {
                    view_id: view_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.GetViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.GetViewResponse", len)?;
        if let Some(v) = self.view.as_ref() {
            struct_ser.serialize_field("view", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetViewResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            View,
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
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.GetViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetViewResponse {
                    view: view__,
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.GetViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApplicableViewsRequest {
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
        if !self.asset_ids.is_empty() {
            len += 1;
        }
        if !self.run_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.ListApplicableViewsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.asset_ids.is_empty() {
            struct_ser.serialize_field("assetIds", &self.asset_ids)?;
        }
        if !self.run_ids.is_empty() {
            struct_ser.serialize_field("runIds", &self.run_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApplicableViewsRequest {
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
            "asset_ids",
            "assetIds",
            "run_ids",
            "runIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            AssetIds,
            RunIds,
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
                            "assetIds" | "asset_ids" => Ok(GeneratedField::AssetIds),
                            "runIds" | "run_ids" => Ok(GeneratedField::RunIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListApplicableViewsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.ListApplicableViewsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApplicableViewsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut asset_ids__ = None;
                let mut run_ids__ = None;
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
                    }
                }
                Ok(ListApplicableViewsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    asset_ids: asset_ids__.unwrap_or_default(),
                    run_ids: run_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.ListApplicableViewsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListApplicableViewsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.views.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.ListApplicableViewsResponse", len)?;
        if !self.views.is_empty() {
            struct_ser.serialize_field("views", &self.views)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListApplicableViewsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "views",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Views,
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
                            "views" => Ok(GeneratedField::Views),
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
            type Value = ListApplicableViewsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.ListApplicableViewsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListApplicableViewsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut views__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Views => {
                            if views__.is_some() {
                                return Err(serde::de::Error::duplicate_field("views"));
                            }
                            views__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListApplicableViewsResponse {
                    views: views__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.ListApplicableViewsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.ListViewsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListViewsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.ListViewsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
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
                    }
                }
                Ok(ListViewsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.ListViewsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.views.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.ListViewsResponse", len)?;
        if !self.views.is_empty() {
            struct_ser.serialize_field("views", &self.views)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "views",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Views,
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
                            "views" => Ok(GeneratedField::Views),
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
            type Value = ListViewsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.ListViewsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut views__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Views => {
                            if views__.is_some() {
                                return Err(serde::de::Error::duplicate_field("views"));
                            }
                            views__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListViewsResponse {
                    views: views__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.ListViewsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PinViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.view_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.PinViewRequest", len)?;
        if !self.view_id.is_empty() {
            struct_ser.serialize_field("viewId", &self.view_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PinViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view_id",
            "viewId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ViewId,
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
                            "viewId" | "view_id" => Ok(GeneratedField::ViewId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PinViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.PinViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PinViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ViewId => {
                            if view_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewId"));
                            }
                            view_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PinViewRequest {
                    view_id: view_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.PinViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PinViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.views.v1.PinViewResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PinViewResponse {
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
            type Value = PinViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.PinViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PinViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PinViewResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.PinViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnpinViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.view_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.UnpinViewRequest", len)?;
        if !self.view_id.is_empty() {
            struct_ser.serialize_field("viewId", &self.view_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnpinViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view_id",
            "viewId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ViewId,
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
                            "viewId" | "view_id" => Ok(GeneratedField::ViewId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnpinViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.UnpinViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnpinViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ViewId => {
                            if view_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewId"));
                            }
                            view_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnpinViewRequest {
                    view_id: view_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.UnpinViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnpinViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.views.v1.UnpinViewResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnpinViewResponse {
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
            type Value = UnpinViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.UnpinViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnpinViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UnpinViewResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.UnpinViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.UpdateViewRequest", len)?;
        if let Some(v) = self.view.as_ref() {
            struct_ser.serialize_field("view", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            View,
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
                            "view" => Ok(GeneratedField::View),
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
            type Value = UpdateViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.UpdateViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateViewRequest {
                    view: view__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.UpdateViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.UpdateViewResponse", len)?;
        if let Some(v) = self.view.as_ref() {
            struct_ser.serialize_field("view", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateViewResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            View,
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
                            "view" => Ok(GeneratedField::View),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.UpdateViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateViewResponse {
                    view: view__,
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.UpdateViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for View {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.view_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.axis_groups.is_some() {
            len += 1;
        }
        if !self.channels.is_empty() {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.is_pinned {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.View", len)?;
        if !self.view_id.is_empty() {
            struct_ser.serialize_field("viewId", &self.view_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.axis_groups.as_ref() {
            struct_ser.serialize_field("axisGroups", v)?;
        }
        if !self.channels.is_empty() {
            struct_ser.serialize_field("channels", &self.channels)?;
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
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if self.is_pinned {
            struct_ser.serialize_field("isPinned", &self.is_pinned)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for View {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view_id",
            "viewId",
            "name",
            "axis_groups",
            "axisGroups",
            "channels",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "organization_id",
            "organizationId",
            "modified_by_user_id",
            "modifiedByUserId",
            "is_pinned",
            "isPinned",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ViewId,
            Name,
            AxisGroups,
            Channels,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            OrganizationId,
            ModifiedByUserId,
            IsPinned,
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
                            "viewId" | "view_id" => Ok(GeneratedField::ViewId),
                            "name" => Ok(GeneratedField::Name),
                            "axisGroups" | "axis_groups" => Ok(GeneratedField::AxisGroups),
                            "channels" => Ok(GeneratedField::Channels),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "isPinned" | "is_pinned" => Ok(GeneratedField::IsPinned),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = View;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.View")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<View, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view_id__ = None;
                let mut name__ = None;
                let mut axis_groups__ = None;
                let mut channels__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut organization_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut is_pinned__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ViewId => {
                            if view_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewId"));
                            }
                            view_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AxisGroups => {
                            if axis_groups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("axisGroups"));
                            }
                            axis_groups__ = map_.next_value()?;
                        }
                        GeneratedField::Channels => {
                            if channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channels"));
                            }
                            channels__ = Some(map_.next_value()?);
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModifiedByUserId => {
                            if modified_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifiedByUserId"));
                            }
                            modified_by_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsPinned => {
                            if is_pinned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isPinned"));
                            }
                            is_pinned__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(View {
                    view_id: view_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    axis_groups: axis_groups__,
                    channels: channels__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    is_pinned: is_pinned__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.View", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for view::AxisGroups {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.left.is_empty() {
            len += 1;
        }
        if !self.right.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.View.AxisGroups", len)?;
        if !self.left.is_empty() {
            struct_ser.serialize_field("left", &self.left)?;
        }
        if !self.right.is_empty() {
            struct_ser.serialize_field("right", &self.right)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for view::AxisGroups {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "left",
            "right",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Left,
            Right,
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
                            "left" => Ok(GeneratedField::Left),
                            "right" => Ok(GeneratedField::Right),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = view::AxisGroups;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.View.AxisGroups")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<view::AxisGroups, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut left__ = None;
                let mut right__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Left => {
                            if left__.is_some() {
                                return Err(serde::de::Error::duplicate_field("left"));
                            }
                            left__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Right => {
                            if right__.is_some() {
                                return Err(serde::de::Error::duplicate_field("right"));
                            }
                            right__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(view::AxisGroups {
                    left: left__.unwrap_or_default(),
                    right: right__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.View.AxisGroups", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for view::Channel {
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
        if !self.component.is_empty() {
            len += 1;
        }
        if !self.data_type.is_empty() {
            len += 1;
        }
        if !self.axis_group.is_empty() {
            len += 1;
        }
        if !self.bit_field_names.is_empty() {
            len += 1;
        }
        if self.calculated_channel_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.View.Channel", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.component.is_empty() {
            struct_ser.serialize_field("component", &self.component)?;
        }
        if !self.data_type.is_empty() {
            struct_ser.serialize_field("dataType", &self.data_type)?;
        }
        if !self.axis_group.is_empty() {
            struct_ser.serialize_field("axisGroup", &self.axis_group)?;
        }
        if !self.bit_field_names.is_empty() {
            struct_ser.serialize_field("bitFieldNames", &self.bit_field_names)?;
        }
        if let Some(v) = self.calculated_channel_config.as_ref() {
            struct_ser.serialize_field("calculatedChannelConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for view::Channel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "component",
            "data_type",
            "dataType",
            "axis_group",
            "axisGroup",
            "bit_field_names",
            "bitFieldNames",
            "calculated_channel_config",
            "calculatedChannelConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Component,
            DataType,
            AxisGroup,
            BitFieldNames,
            CalculatedChannelConfig,
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
                            "component" => Ok(GeneratedField::Component),
                            "dataType" | "data_type" => Ok(GeneratedField::DataType),
                            "axisGroup" | "axis_group" => Ok(GeneratedField::AxisGroup),
                            "bitFieldNames" | "bit_field_names" => Ok(GeneratedField::BitFieldNames),
                            "calculatedChannelConfig" | "calculated_channel_config" => Ok(GeneratedField::CalculatedChannelConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = view::Channel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.View.Channel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<view::Channel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut component__ = None;
                let mut data_type__ = None;
                let mut axis_group__ = None;
                let mut bit_field_names__ = None;
                let mut calculated_channel_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Component => {
                            if component__.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            component__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DataType => {
                            if data_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataType"));
                            }
                            data_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AxisGroup => {
                            if axis_group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("axisGroup"));
                            }
                            axis_group__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BitFieldNames => {
                            if bit_field_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bitFieldNames"));
                            }
                            bit_field_names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CalculatedChannelConfig => {
                            if calculated_channel_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannelConfig"));
                            }
                            calculated_channel_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(view::Channel {
                    name: name__.unwrap_or_default(),
                    component: component__.unwrap_or_default(),
                    data_type: data_type__.unwrap_or_default(),
                    axis_group: axis_group__.unwrap_or_default(),
                    bit_field_names: bit_field_names__.unwrap_or_default(),
                    calculated_channel_config: calculated_channel_config__,
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.View.Channel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for view::channel::CalculatedChannelConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.channel_key.is_empty() {
            len += 1;
        }
        if !self.channel_references.is_empty() {
            len += 1;
        }
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.View.Channel.CalculatedChannelConfig", len)?;
        if !self.channel_key.is_empty() {
            struct_ser.serialize_field("channelKey", &self.channel_key)?;
        }
        if !self.channel_references.is_empty() {
            struct_ser.serialize_field("channelReferences", &self.channel_references)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for view::channel::CalculatedChannelConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "channel_key",
            "channelKey",
            "channel_references",
            "channelReferences",
            "expression",
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChannelKey,
            ChannelReferences,
            Expression,
            Unit,
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
                            "channelKey" | "channel_key" => Ok(GeneratedField::ChannelKey),
                            "channelReferences" | "channel_references" => Ok(GeneratedField::ChannelReferences),
                            "expression" => Ok(GeneratedField::Expression),
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = view::channel::CalculatedChannelConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.View.Channel.CalculatedChannelConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<view::channel::CalculatedChannelConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut channel_key__ = None;
                let mut channel_references__ = None;
                let mut expression__ = None;
                let mut unit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChannelKey => {
                            if channel_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelKey"));
                            }
                            channel_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelReferences => {
                            if channel_references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelReferences"));
                            }
                            channel_references__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(view::channel::CalculatedChannelConfig {
                    channel_key: channel_key__.unwrap_or_default(),
                    channel_references: channel_references__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.View.Channel.CalculatedChannelConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for view::channel::calculated_channel_config::ChannelReference {
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
        if !self.component.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.views.v1.View.Channel.CalculatedChannelConfig.ChannelReference", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.component.is_empty() {
            struct_ser.serialize_field("component", &self.component)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for view::channel::calculated_channel_config::ChannelReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "component",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Component,
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
                            "component" => Ok(GeneratedField::Component),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = view::channel::calculated_channel_config::ChannelReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.views.v1.View.Channel.CalculatedChannelConfig.ChannelReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<view::channel::calculated_channel_config::ChannelReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut component__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Component => {
                            if component__.is_some() {
                                return Err(serde::de::Error::duplicate_field("component"));
                            }
                            component__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(view::channel::calculated_channel_config::ChannelReference {
                    name: name__.unwrap_or_default(),
                    component: component__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.views.v1.View.Channel.CalculatedChannelConfig.ChannelReference", FIELDS, GeneratedVisitor)
    }
}
