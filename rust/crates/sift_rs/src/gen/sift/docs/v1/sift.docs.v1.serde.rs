// @generated
impl serde::Serialize for DocHit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if self.score != 0 {
            len += 1;
        }
        if self.match_line != 0 {
            len += 1;
        }
        if self.total_lines != 0 {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.docs.v1.DocHit", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if self.score != 0 {
            struct_ser.serialize_field("score", &self.score)?;
        }
        if self.match_line != 0 {
            struct_ser.serialize_field("matchLine", &self.match_line)?;
        }
        if self.total_lines != 0 {
            struct_ser.serialize_field("totalLines", &self.total_lines)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DocHit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "title",
            "score",
            "match_line",
            "matchLine",
            "total_lines",
            "totalLines",
            "content",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Title,
            Score,
            MatchLine,
            TotalLines,
            Content,
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
                            "path" => Ok(GeneratedField::Path),
                            "title" => Ok(GeneratedField::Title),
                            "score" => Ok(GeneratedField::Score),
                            "matchLine" | "match_line" => Ok(GeneratedField::MatchLine),
                            "totalLines" | "total_lines" => Ok(GeneratedField::TotalLines),
                            "content" => Ok(GeneratedField::Content),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DocHit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.docs.v1.DocHit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DocHit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut title__ = None;
                let mut score__ = None;
                let mut match_line__ = None;
                let mut total_lines__ = None;
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Score => {
                            if score__.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            score__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MatchLine => {
                            if match_line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchLine"));
                            }
                            match_line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalLines => {
                            if total_lines__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalLines"));
                            }
                            total_lines__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DocHit {
                    path: path__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    score: score__.unwrap_or_default(),
                    match_line: match_line__.unwrap_or_default(),
                    total_lines: total_lines__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.docs.v1.DocHit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadDocRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.offset != 0 {
            len += 1;
        }
        if self.limit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.docs.v1.ReadDocRequest", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if self.offset != 0 {
            struct_ser.serialize_field("offset", &self.offset)?;
        }
        if self.limit != 0 {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadDocRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "offset",
            "limit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Offset,
            Limit,
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
                            "path" => Ok(GeneratedField::Path),
                            "offset" => Ok(GeneratedField::Offset),
                            "limit" => Ok(GeneratedField::Limit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadDocRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.docs.v1.ReadDocRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadDocRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut offset__ = None;
                let mut limit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Offset => {
                            if offset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("offset"));
                            }
                            offset__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ReadDocRequest {
                    path: path__.unwrap_or_default(),
                    offset: offset__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.docs.v1.ReadDocRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadDocResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.title.is_empty() {
            len += 1;
        }
        if self.total_lines != 0 {
            len += 1;
        }
        if self.start_line != 0 {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.docs.v1.ReadDocResponse", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if self.total_lines != 0 {
            struct_ser.serialize_field("totalLines", &self.total_lines)?;
        }
        if self.start_line != 0 {
            struct_ser.serialize_field("startLine", &self.start_line)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadDocResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "title",
            "total_lines",
            "totalLines",
            "start_line",
            "startLine",
            "content",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Title,
            TotalLines,
            StartLine,
            Content,
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
                            "path" => Ok(GeneratedField::Path),
                            "title" => Ok(GeneratedField::Title),
                            "totalLines" | "total_lines" => Ok(GeneratedField::TotalLines),
                            "startLine" | "start_line" => Ok(GeneratedField::StartLine),
                            "content" => Ok(GeneratedField::Content),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadDocResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.docs.v1.ReadDocResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadDocResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut title__ = None;
                let mut total_lines__ = None;
                let mut start_line__ = None;
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalLines => {
                            if total_lines__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalLines"));
                            }
                            total_lines__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartLine => {
                            if start_line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startLine"));
                            }
                            start_line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadDocResponse {
                    path: path__.unwrap_or_default(),
                    title: title__.unwrap_or_default(),
                    total_lines: total_lines__.unwrap_or_default(),
                    start_line: start_line__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.docs.v1.ReadDocResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchDocsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        if self.max_results != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.docs.v1.SearchDocsRequest", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if self.max_results != 0 {
            struct_ser.serialize_field("maxResults", &self.max_results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchDocsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
            "max_results",
            "maxResults",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
            MaxResults,
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
                            "query" => Ok(GeneratedField::Query),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchDocsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.docs.v1.SearchDocsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchDocsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                let mut max_results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SearchDocsRequest {
                    query: query__.unwrap_or_default(),
                    max_results: max_results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.docs.v1.SearchDocsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchDocsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hits.is_empty() {
            len += 1;
        }
        if self.total_scanned != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.docs.v1.SearchDocsResponse", len)?;
        if !self.hits.is_empty() {
            struct_ser.serialize_field("hits", &self.hits)?;
        }
        if self.total_scanned != 0 {
            struct_ser.serialize_field("totalScanned", &self.total_scanned)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchDocsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hits",
            "total_scanned",
            "totalScanned",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hits,
            TotalScanned,
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
                            "hits" => Ok(GeneratedField::Hits),
                            "totalScanned" | "total_scanned" => Ok(GeneratedField::TotalScanned),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchDocsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.docs.v1.SearchDocsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SearchDocsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hits__ = None;
                let mut total_scanned__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Hits => {
                            if hits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            hits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalScanned => {
                            if total_scanned__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalScanned"));
                            }
                            total_scanned__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SearchDocsResponse {
                    hits: hits__.unwrap_or_default(),
                    total_scanned: total_scanned__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.docs.v1.SearchDocsResponse", FIELDS, GeneratedVisitor)
    }
}
