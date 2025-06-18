// @generated
impl serde::Serialize for ErrorSummary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.count != 0 {
            len += 1;
        }
        if self.asset_name.is_some() {
            len += 1;
        }
        if self.min_timestamp.is_some() {
            len += 1;
        }
        if self.max_timestamp.is_some() {
            len += 1;
        }
        if !self.dlq_parquet_file_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.dlq_errors.v2.ErrorSummary", len)?;
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        if let Some(v) = self.asset_name.as_ref() {
            struct_ser.serialize_field("assetName", v)?;
        }
        if let Some(v) = self.min_timestamp.as_ref() {
            struct_ser.serialize_field("minTimestamp", v)?;
        }
        if let Some(v) = self.max_timestamp.as_ref() {
            struct_ser.serialize_field("maxTimestamp", v)?;
        }
        if !self.dlq_parquet_file_id.is_empty() {
            struct_ser.serialize_field("dlqParquetFileId", &self.dlq_parquet_file_id)?;
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
impl<'de> serde::Deserialize<'de> for ErrorSummary {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
            "asset_name",
            "assetName",
            "min_timestamp",
            "minTimestamp",
            "max_timestamp",
            "maxTimestamp",
            "dlq_parquet_file_id",
            "dlqParquetFileId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
            AssetName,
            MinTimestamp,
            MaxTimestamp,
            DlqParquetFileId,
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
                            "count" => Ok(GeneratedField::Count),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "minTimestamp" | "min_timestamp" => Ok(GeneratedField::MinTimestamp),
                            "maxTimestamp" | "max_timestamp" => Ok(GeneratedField::MaxTimestamp),
                            "dlqParquetFileId" | "dlq_parquet_file_id" => Ok(GeneratedField::DlqParquetFileId),
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
            type Value = ErrorSummary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.dlq_errors.v2.ErrorSummary")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorSummary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
                let mut asset_name__ = None;
                let mut min_timestamp__ = None;
                let mut max_timestamp__ = None;
                let mut dlq_parquet_file_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Count => {
                            if count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = map_.next_value()?;
                        }
                        GeneratedField::MinTimestamp => {
                            if min_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minTimestamp"));
                            }
                            min_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::MaxTimestamp => {
                            if max_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTimestamp"));
                            }
                            max_timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::DlqParquetFileId => {
                            if dlq_parquet_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dlqParquetFileId"));
                            }
                            dlq_parquet_file_id__ = Some(map_.next_value()?);
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
                Ok(ErrorSummary {
                    count: count__.unwrap_or_default(),
                    asset_name: asset_name__,
                    min_timestamp: min_timestamp__,
                    max_timestamp: max_timestamp__,
                    dlq_parquet_file_id: dlq_parquet_file_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.dlq_errors.v2.ErrorSummary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDlqErrorsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.dlq_errors.v2.ListDlqErrorsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListDlqErrorsRequest {
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
            type Value = ListDlqErrorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.dlq_errors.v2.ListDlqErrorsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDlqErrorsRequest, V::Error>
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
                Ok(ListDlqErrorsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.dlq_errors.v2.ListDlqErrorsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDlqErrorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.error_summaries.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.dlq_errors.v2.ListDlqErrorsResponse", len)?;
        if !self.error_summaries.is_empty() {
            struct_ser.serialize_field("errorSummaries", &self.error_summaries)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDlqErrorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error_summaries",
            "errorSummaries",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorSummaries,
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
                            "errorSummaries" | "error_summaries" => Ok(GeneratedField::ErrorSummaries),
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
            type Value = ListDlqErrorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.dlq_errors.v2.ListDlqErrorsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDlqErrorsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error_summaries__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ErrorSummaries => {
                            if error_summaries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorSummaries"));
                            }
                            error_summaries__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDlqErrorsResponse {
                    error_summaries: error_summaries__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.dlq_errors.v2.ListDlqErrorsResponse", FIELDS, GeneratedVisitor)
    }
}
