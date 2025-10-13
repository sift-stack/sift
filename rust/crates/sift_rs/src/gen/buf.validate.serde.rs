// @generated
impl serde::Serialize for AnyRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.AnyRules", len)?;
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnyRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "in",
            "not_in",
            "notIn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            In,
            NotIn,
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
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnyRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.AnyRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnyRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#in__ = None;
                let mut not_in__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnyRules {
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.AnyRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BoolRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.BoolRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BoolRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "example",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Example,
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
                            "const" => Ok(GeneratedField::Const),
                            "example" => Ok(GeneratedField::Example),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BoolRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.BoolRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BoolRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut example__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BoolRules {
                    r#const: r#const__,
                    example: example__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.BoolRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BytesRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.len.is_some() {
            len += 1;
        }
        if self.min_len.is_some() {
            len += 1;
        }
        if self.max_len.is_some() {
            len += 1;
        }
        if self.pattern.is_some() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        if self.suffix.is_some() {
            len += 1;
        }
        if self.contains.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.well_known.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.BytesRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("const", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.len.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.min_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minLen", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxLen", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.pattern.as_ref() {
            struct_ser.serialize_field("pattern", v)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("prefix", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.suffix.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("suffix", pbjson::private::base64::encode(&v).as_str())?;
        }
        if let Some(v) = self.contains.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("contains", pbjson::private::base64::encode(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.well_known.as_ref() {
            match v {
                bytes_rules::WellKnown::Ip(v) => {
                    struct_ser.serialize_field("ip", v)?;
                }
                bytes_rules::WellKnown::Ipv4(v) => {
                    struct_ser.serialize_field("ipv4", v)?;
                }
                bytes_rules::WellKnown::Ipv6(v) => {
                    struct_ser.serialize_field("ipv6", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BytesRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "len",
            "min_len",
            "minLen",
            "max_len",
            "maxLen",
            "pattern",
            "prefix",
            "suffix",
            "contains",
            "in",
            "not_in",
            "notIn",
            "example",
            "ip",
            "ipv4",
            "ipv6",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Len,
            MinLen,
            MaxLen,
            Pattern,
            Prefix,
            Suffix,
            Contains,
            In,
            NotIn,
            Example,
            Ip,
            Ipv4,
            Ipv6,
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
                            "const" => Ok(GeneratedField::Const),
                            "len" => Ok(GeneratedField::Len),
                            "minLen" | "min_len" => Ok(GeneratedField::MinLen),
                            "maxLen" | "max_len" => Ok(GeneratedField::MaxLen),
                            "pattern" => Ok(GeneratedField::Pattern),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "suffix" => Ok(GeneratedField::Suffix),
                            "contains" => Ok(GeneratedField::Contains),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "ip" => Ok(GeneratedField::Ip),
                            "ipv4" => Ok(GeneratedField::Ipv4),
                            "ipv6" => Ok(GeneratedField::Ipv6),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BytesRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.BytesRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BytesRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut len__ = None;
                let mut min_len__ = None;
                let mut max_len__ = None;
                let mut pattern__ = None;
                let mut prefix__ = None;
                let mut suffix__ = None;
                let mut contains__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut well_known__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Len => {
                            if len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("len"));
                            }
                            len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinLen => {
                            if min_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLen"));
                            }
                            min_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxLen => {
                            if max_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLen"));
                            }
                            max_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = map_.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Suffix => {
                            if suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            suffix__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Contains => {
                            if contains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contains"));
                            }
                            contains__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Ip => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(bytes_rules::WellKnown::Ip);
                        }
                        GeneratedField::Ipv4 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(bytes_rules::WellKnown::Ipv4);
                        }
                        GeneratedField::Ipv6 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(bytes_rules::WellKnown::Ipv6);
                        }
                    }
                }
                Ok(BytesRules {
                    r#const: r#const__,
                    len: len__,
                    min_len: min_len__,
                    max_len: max_len__,
                    pattern: pattern__,
                    prefix: prefix__,
                    suffix: suffix__,
                    contains: contains__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    well_known: well_known__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.BytesRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DoubleRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.finite.is_some() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.DoubleRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if let Some(v) = self.finite.as_ref() {
            struct_ser.serialize_field("finite", v)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                double_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                double_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                double_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                double_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DoubleRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "finite",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Finite,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "finite" => Ok(GeneratedField::Finite),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DoubleRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.DoubleRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DoubleRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut finite__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Finite => {
                            if finite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finite"));
                            }
                            finite__ = map_.next_value()?;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| double_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| double_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| double_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| double_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(DoubleRules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    finite: finite__,
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.DoubleRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DurationRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.DurationRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                duration_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                duration_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                duration_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                duration_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DurationRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DurationRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.DurationRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DurationRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<_>>()?.map(duration_rules::LessThan::Lt)
;
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<_>>()?.map(duration_rules::LessThan::Lte)
;
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<_>>()?.map(duration_rules::GreaterThan::Gt)
;
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<_>>()?.map(duration_rules::GreaterThan::Gte)
;
                        }
                    }
                }
                Ok(DurationRules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.DurationRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnumRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.defined_only.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.EnumRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.defined_only.as_ref() {
            struct_ser.serialize_field("definedOnly", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnumRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "defined_only",
            "definedOnly",
            "in",
            "not_in",
            "notIn",
            "example",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            DefinedOnly,
            In,
            NotIn,
            Example,
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
                            "const" => Ok(GeneratedField::Const),
                            "definedOnly" | "defined_only" => Ok(GeneratedField::DefinedOnly),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnumRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.EnumRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnumRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut defined_only__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::DefinedOnly => {
                            if defined_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definedOnly"));
                            }
                            defined_only__ = map_.next_value()?;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(EnumRules {
                    r#const: r#const__,
                    defined_only: defined_only__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.EnumRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldPath {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.elements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.FieldPath", len)?;
        if !self.elements.is_empty() {
            struct_ser.serialize_field("elements", &self.elements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldPath {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "elements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Elements,
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
                            "elements" => Ok(GeneratedField::Elements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldPath;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.FieldPath")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldPath, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut elements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Elements => {
                            if elements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elements"));
                            }
                            elements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FieldPath {
                    elements: elements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.FieldPath", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldPathElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field_number.is_some() {
            len += 1;
        }
        if self.field_name.is_some() {
            len += 1;
        }
        if self.field_type.is_some() {
            len += 1;
        }
        if self.key_type.is_some() {
            len += 1;
        }
        if self.value_type.is_some() {
            len += 1;
        }
        if self.subscript.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.FieldPathElement", len)?;
        if let Some(v) = self.field_number.as_ref() {
            struct_ser.serialize_field("fieldNumber", v)?;
        }
        if let Some(v) = self.field_name.as_ref() {
            struct_ser.serialize_field("fieldName", v)?;
        }
        if let Some(v) = self.field_type.as_ref() {
            let v = ::pbjson_types::field_descriptor_proto::Type::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("fieldType", &v)?;
        }
        if let Some(v) = self.key_type.as_ref() {
            let v = ::pbjson_types::field_descriptor_proto::Type::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("keyType", &v)?;
        }
        if let Some(v) = self.value_type.as_ref() {
            let v = ::pbjson_types::field_descriptor_proto::Type::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("valueType", &v)?;
        }
        if let Some(v) = self.subscript.as_ref() {
            match v {
                field_path_element::Subscript::Index(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("index", ToString::to_string(&v).as_str())?;
                }
                field_path_element::Subscript::BoolKey(v) => {
                    struct_ser.serialize_field("boolKey", v)?;
                }
                field_path_element::Subscript::IntKey(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("intKey", ToString::to_string(&v).as_str())?;
                }
                field_path_element::Subscript::UintKey(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("uintKey", ToString::to_string(&v).as_str())?;
                }
                field_path_element::Subscript::StringKey(v) => {
                    struct_ser.serialize_field("stringKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldPathElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_number",
            "fieldNumber",
            "field_name",
            "fieldName",
            "field_type",
            "fieldType",
            "key_type",
            "keyType",
            "value_type",
            "valueType",
            "index",
            "bool_key",
            "boolKey",
            "int_key",
            "intKey",
            "uint_key",
            "uintKey",
            "string_key",
            "stringKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldNumber,
            FieldName,
            FieldType,
            KeyType,
            ValueType,
            Index,
            BoolKey,
            IntKey,
            UintKey,
            StringKey,
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
                            "fieldNumber" | "field_number" => Ok(GeneratedField::FieldNumber),
                            "fieldName" | "field_name" => Ok(GeneratedField::FieldName),
                            "fieldType" | "field_type" => Ok(GeneratedField::FieldType),
                            "keyType" | "key_type" => Ok(GeneratedField::KeyType),
                            "valueType" | "value_type" => Ok(GeneratedField::ValueType),
                            "index" => Ok(GeneratedField::Index),
                            "boolKey" | "bool_key" => Ok(GeneratedField::BoolKey),
                            "intKey" | "int_key" => Ok(GeneratedField::IntKey),
                            "uintKey" | "uint_key" => Ok(GeneratedField::UintKey),
                            "stringKey" | "string_key" => Ok(GeneratedField::StringKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldPathElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.FieldPathElement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldPathElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_number__ = None;
                let mut field_name__ = None;
                let mut field_type__ = None;
                let mut key_type__ = None;
                let mut value_type__ = None;
                let mut subscript__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FieldNumber => {
                            if field_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldNumber"));
                            }
                            field_number__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::FieldName => {
                            if field_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldName"));
                            }
                            field_name__ = map_.next_value()?;
                        }
                        GeneratedField::FieldType => {
                            if field_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldType"));
                            }
                            field_type__ = map_.next_value::<::std::option::Option<::pbjson_types::field_descriptor_proto::Type>>()?.map(|x| x as i32);
                        }
                        GeneratedField::KeyType => {
                            if key_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyType"));
                            }
                            key_type__ = map_.next_value::<::std::option::Option<::pbjson_types::field_descriptor_proto::Type>>()?.map(|x| x as i32);
                        }
                        GeneratedField::ValueType => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueType"));
                            }
                            value_type__ = map_.next_value::<::std::option::Option<::pbjson_types::field_descriptor_proto::Type>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Index => {
                            if subscript__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            subscript__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| field_path_element::Subscript::Index(x.0));
                        }
                        GeneratedField::BoolKey => {
                            if subscript__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolKey"));
                            }
                            subscript__ = map_.next_value::<::std::option::Option<_>>()?.map(field_path_element::Subscript::BoolKey);
                        }
                        GeneratedField::IntKey => {
                            if subscript__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intKey"));
                            }
                            subscript__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| field_path_element::Subscript::IntKey(x.0));
                        }
                        GeneratedField::UintKey => {
                            if subscript__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uintKey"));
                            }
                            subscript__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| field_path_element::Subscript::UintKey(x.0));
                        }
                        GeneratedField::StringKey => {
                            if subscript__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringKey"));
                            }
                            subscript__ = map_.next_value::<::std::option::Option<_>>()?.map(field_path_element::Subscript::StringKey);
                        }
                    }
                }
                Ok(FieldPathElement {
                    field_number: field_number__,
                    field_name: field_name__,
                    field_type: field_type__,
                    key_type: key_type__,
                    value_type: value_type__,
                    subscript: subscript__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.FieldPathElement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cel.is_empty() {
            len += 1;
        }
        if self.required.is_some() {
            len += 1;
        }
        if self.ignore.is_some() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.FieldRules", len)?;
        if !self.cel.is_empty() {
            struct_ser.serialize_field("cel", &self.cel)?;
        }
        if let Some(v) = self.required.as_ref() {
            struct_ser.serialize_field("required", v)?;
        }
        if let Some(v) = self.ignore.as_ref() {
            let v = Ignore::try_from(*v)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("ignore", &v)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                field_rules::Type::Float(v) => {
                    struct_ser.serialize_field("float", v)?;
                }
                field_rules::Type::Double(v) => {
                    struct_ser.serialize_field("double", v)?;
                }
                field_rules::Type::Int32(v) => {
                    struct_ser.serialize_field("int32", v)?;
                }
                field_rules::Type::Int64(v) => {
                    struct_ser.serialize_field("int64", v)?;
                }
                field_rules::Type::Uint32(v) => {
                    struct_ser.serialize_field("uint32", v)?;
                }
                field_rules::Type::Uint64(v) => {
                    struct_ser.serialize_field("uint64", v)?;
                }
                field_rules::Type::Sint32(v) => {
                    struct_ser.serialize_field("sint32", v)?;
                }
                field_rules::Type::Sint64(v) => {
                    struct_ser.serialize_field("sint64", v)?;
                }
                field_rules::Type::Fixed32(v) => {
                    struct_ser.serialize_field("fixed32", v)?;
                }
                field_rules::Type::Fixed64(v) => {
                    struct_ser.serialize_field("fixed64", v)?;
                }
                field_rules::Type::Sfixed32(v) => {
                    struct_ser.serialize_field("sfixed32", v)?;
                }
                field_rules::Type::Sfixed64(v) => {
                    struct_ser.serialize_field("sfixed64", v)?;
                }
                field_rules::Type::Bool(v) => {
                    struct_ser.serialize_field("bool", v)?;
                }
                field_rules::Type::String(v) => {
                    struct_ser.serialize_field("string", v)?;
                }
                field_rules::Type::Bytes(v) => {
                    struct_ser.serialize_field("bytes", v)?;
                }
                field_rules::Type::Enum(v) => {
                    struct_ser.serialize_field("enum", v)?;
                }
                field_rules::Type::Repeated(v) => {
                    struct_ser.serialize_field("repeated", v)?;
                }
                field_rules::Type::Map(v) => {
                    struct_ser.serialize_field("map", v)?;
                }
                field_rules::Type::Any(v) => {
                    struct_ser.serialize_field("any", v)?;
                }
                field_rules::Type::Duration(v) => {
                    struct_ser.serialize_field("duration", v)?;
                }
                field_rules::Type::Timestamp(v) => {
                    struct_ser.serialize_field("timestamp", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FieldRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cel",
            "required",
            "ignore",
            "float",
            "double",
            "int32",
            "int64",
            "uint32",
            "uint64",
            "sint32",
            "sint64",
            "fixed32",
            "fixed64",
            "sfixed32",
            "sfixed64",
            "bool",
            "string",
            "bytes",
            "enum",
            "repeated",
            "map",
            "any",
            "duration",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cel,
            Required,
            Ignore,
            Float,
            Double,
            Int32,
            Int64,
            Uint32,
            Uint64,
            Sint32,
            Sint64,
            Fixed32,
            Fixed64,
            Sfixed32,
            Sfixed64,
            Bool,
            String,
            Bytes,
            Enum,
            Repeated,
            Map,
            Any,
            Duration,
            Timestamp,
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
                            "cel" => Ok(GeneratedField::Cel),
                            "required" => Ok(GeneratedField::Required),
                            "ignore" => Ok(GeneratedField::Ignore),
                            "float" => Ok(GeneratedField::Float),
                            "double" => Ok(GeneratedField::Double),
                            "int32" => Ok(GeneratedField::Int32),
                            "int64" => Ok(GeneratedField::Int64),
                            "uint32" => Ok(GeneratedField::Uint32),
                            "uint64" => Ok(GeneratedField::Uint64),
                            "sint32" => Ok(GeneratedField::Sint32),
                            "sint64" => Ok(GeneratedField::Sint64),
                            "fixed32" => Ok(GeneratedField::Fixed32),
                            "fixed64" => Ok(GeneratedField::Fixed64),
                            "sfixed32" => Ok(GeneratedField::Sfixed32),
                            "sfixed64" => Ok(GeneratedField::Sfixed64),
                            "bool" => Ok(GeneratedField::Bool),
                            "string" => Ok(GeneratedField::String),
                            "bytes" => Ok(GeneratedField::Bytes),
                            "enum" => Ok(GeneratedField::Enum),
                            "repeated" => Ok(GeneratedField::Repeated),
                            "map" => Ok(GeneratedField::Map),
                            "any" => Ok(GeneratedField::Any),
                            "duration" => Ok(GeneratedField::Duration),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.FieldRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FieldRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cel__ = None;
                let mut required__ = None;
                let mut ignore__ = None;
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cel => {
                            if cel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cel"));
                            }
                            cel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = map_.next_value()?;
                        }
                        GeneratedField::Ignore => {
                            if ignore__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignore"));
                            }
                            ignore__ = map_.next_value::<::std::option::Option<Ignore>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Float => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("float"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Float)
;
                        }
                        GeneratedField::Double => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("double"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Double)
;
                        }
                        GeneratedField::Int32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Int32)
;
                        }
                        GeneratedField::Int64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Int64)
;
                        }
                        GeneratedField::Uint32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Uint32)
;
                        }
                        GeneratedField::Uint64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Uint64)
;
                        }
                        GeneratedField::Sint32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sint32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sint32)
;
                        }
                        GeneratedField::Sint64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sint64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sint64)
;
                        }
                        GeneratedField::Fixed32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixed32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Fixed32)
;
                        }
                        GeneratedField::Fixed64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixed64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Fixed64)
;
                        }
                        GeneratedField::Sfixed32 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sfixed32"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sfixed32)
;
                        }
                        GeneratedField::Sfixed64 => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sfixed64"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Sfixed64)
;
                        }
                        GeneratedField::Bool => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bool"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Bool)
;
                        }
                        GeneratedField::String => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("string"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::String)
;
                        }
                        GeneratedField::Bytes => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytes"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Bytes)
;
                        }
                        GeneratedField::Enum => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enum"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Enum)
;
                        }
                        GeneratedField::Repeated => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repeated"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Repeated)
;
                        }
                        GeneratedField::Map => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("map"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Map)
;
                        }
                        GeneratedField::Any => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("any"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Any)
;
                        }
                        GeneratedField::Duration => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Duration)
;
                        }
                        GeneratedField::Timestamp => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            r#type__ = map_.next_value::<::std::option::Option<_>>()?.map(field_rules::Type::Timestamp)
;
                        }
                    }
                }
                Ok(FieldRules {
                    cel: cel__.unwrap_or_default(),
                    required: required__,
                    ignore: ignore__,
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.FieldRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fixed32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.Fixed32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                fixed32_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                fixed32_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                fixed32_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                fixed32_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fixed32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fixed32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.Fixed32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Fixed32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed32_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed32_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed32_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed32_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(Fixed32Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.Fixed32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fixed64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.Fixed64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                fixed64_rules::LessThan::Lt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
                }
                fixed64_rules::LessThan::Lte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                fixed64_rules::GreaterThan::Gt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
                }
                fixed64_rules::GreaterThan::Gte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fixed64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fixed64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.Fixed64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Fixed64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed64_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed64_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed64_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fixed64_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(Fixed64Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.Fixed64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FloatRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.finite.is_some() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.FloatRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if let Some(v) = self.finite.as_ref() {
            struct_ser.serialize_field("finite", v)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                float_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                float_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                float_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                float_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FloatRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "finite",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Finite,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "finite" => Ok(GeneratedField::Finite),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FloatRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.FloatRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FloatRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut finite__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Finite => {
                            if finite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finite"));
                            }
                            finite__ = map_.next_value()?;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| float_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| float_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| float_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| float_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(FloatRules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    finite: finite__,
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.FloatRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Ignore {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "IGNORE_UNSPECIFIED",
            Self::IfZeroValue => "IGNORE_IF_ZERO_VALUE",
            Self::Always => "IGNORE_ALWAYS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Ignore {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IGNORE_UNSPECIFIED",
            "IGNORE_IF_ZERO_VALUE",
            "IGNORE_ALWAYS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Ignore;

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
                    "IGNORE_UNSPECIFIED" => Ok(Ignore::Unspecified),
                    "IGNORE_IF_ZERO_VALUE" => Ok(Ignore::IfZeroValue),
                    "IGNORE_ALWAYS" => Ok(Ignore::Always),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Int32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.Int32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                int32_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                int32_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                int32_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                int32_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Int32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Int32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.Int32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Int32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int32_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int32_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int32_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int32_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(Int32Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.Int32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Int64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.Int64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                int64_rules::LessThan::Lt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
                }
                int64_rules::LessThan::Lte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                int64_rules::GreaterThan::Gt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
                }
                int64_rules::GreaterThan::Gte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Int64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Int64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.Int64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Int64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int64_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int64_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int64_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| int64_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(Int64Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.Int64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KnownRegex {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "KNOWN_REGEX_UNSPECIFIED",
            Self::HttpHeaderName => "KNOWN_REGEX_HTTP_HEADER_NAME",
            Self::HttpHeaderValue => "KNOWN_REGEX_HTTP_HEADER_VALUE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for KnownRegex {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "KNOWN_REGEX_UNSPECIFIED",
            "KNOWN_REGEX_HTTP_HEADER_NAME",
            "KNOWN_REGEX_HTTP_HEADER_VALUE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KnownRegex;

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
                    "KNOWN_REGEX_UNSPECIFIED" => Ok(KnownRegex::Unspecified),
                    "KNOWN_REGEX_HTTP_HEADER_NAME" => Ok(KnownRegex::HttpHeaderName),
                    "KNOWN_REGEX_HTTP_HEADER_VALUE" => Ok(KnownRegex::HttpHeaderValue),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MapRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_pairs.is_some() {
            len += 1;
        }
        if self.max_pairs.is_some() {
            len += 1;
        }
        if self.keys.is_some() {
            len += 1;
        }
        if self.values.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.MapRules", len)?;
        if let Some(v) = self.min_pairs.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minPairs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_pairs.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxPairs", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.keys.as_ref() {
            struct_ser.serialize_field("keys", v)?;
        }
        if let Some(v) = self.values.as_ref() {
            struct_ser.serialize_field("values", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MapRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_pairs",
            "minPairs",
            "max_pairs",
            "maxPairs",
            "keys",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinPairs,
            MaxPairs,
            Keys,
            Values,
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
                            "minPairs" | "min_pairs" => Ok(GeneratedField::MinPairs),
                            "maxPairs" | "max_pairs" => Ok(GeneratedField::MaxPairs),
                            "keys" => Ok(GeneratedField::Keys),
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MapRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.MapRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MapRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_pairs__ = None;
                let mut max_pairs__ = None;
                let mut keys__ = None;
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinPairs => {
                            if min_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minPairs"));
                            }
                            min_pairs__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxPairs => {
                            if max_pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPairs"));
                            }
                            max_pairs__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = map_.next_value()?;
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MapRules {
                    min_pairs: min_pairs__,
                    max_pairs: max_pairs__,
                    keys: keys__,
                    values: values__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.MapRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageOneofRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fields.is_empty() {
            len += 1;
        }
        if self.required.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.MessageOneofRule", len)?;
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        if let Some(v) = self.required.as_ref() {
            struct_ser.serialize_field("required", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageOneofRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fields",
            "required",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fields,
            Required,
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
                            "fields" => Ok(GeneratedField::Fields),
                            "required" => Ok(GeneratedField::Required),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageOneofRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.MessageOneofRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageOneofRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fields__ = None;
                let mut required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MessageOneofRule {
                    fields: fields__.unwrap_or_default(),
                    required: required__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.MessageOneofRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MessageRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cel.is_empty() {
            len += 1;
        }
        if !self.oneof.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.MessageRules", len)?;
        if !self.cel.is_empty() {
            struct_ser.serialize_field("cel", &self.cel)?;
        }
        if !self.oneof.is_empty() {
            struct_ser.serialize_field("oneof", &self.oneof)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MessageRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cel",
            "oneof",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cel,
            Oneof,
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
                            "cel" => Ok(GeneratedField::Cel),
                            "oneof" => Ok(GeneratedField::Oneof),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MessageRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.MessageRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MessageRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cel__ = None;
                let mut oneof__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cel => {
                            if cel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cel"));
                            }
                            cel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Oneof => {
                            if oneof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oneof"));
                            }
                            oneof__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MessageRules {
                    cel: cel__.unwrap_or_default(),
                    oneof: oneof__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.MessageRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OneofRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.required.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.OneofRules", len)?;
        if let Some(v) = self.required.as_ref() {
            struct_ser.serialize_field("required", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OneofRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "required",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Required,
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
                            "required" => Ok(GeneratedField::Required),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OneofRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.OneofRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OneofRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut required__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Required => {
                            if required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("required"));
                            }
                            required__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OneofRules {
                    required: required__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.OneofRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PredefinedRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cel.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.PredefinedRules", len)?;
        if !self.cel.is_empty() {
            struct_ser.serialize_field("cel", &self.cel)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PredefinedRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cel,
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
                            "cel" => Ok(GeneratedField::Cel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PredefinedRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.PredefinedRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PredefinedRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cel__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cel => {
                            if cel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cel"));
                            }
                            cel__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PredefinedRules {
                    cel: cel__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.PredefinedRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RepeatedRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_items.is_some() {
            len += 1;
        }
        if self.max_items.is_some() {
            len += 1;
        }
        if self.unique.is_some() {
            len += 1;
        }
        if self.items.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.RepeatedRules", len)?;
        if let Some(v) = self.min_items.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minItems", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_items.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxItems", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.unique.as_ref() {
            struct_ser.serialize_field("unique", v)?;
        }
        if let Some(v) = self.items.as_ref() {
            struct_ser.serialize_field("items", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepeatedRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_items",
            "minItems",
            "max_items",
            "maxItems",
            "unique",
            "items",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinItems,
            MaxItems,
            Unique,
            Items,
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
                            "minItems" | "min_items" => Ok(GeneratedField::MinItems),
                            "maxItems" | "max_items" => Ok(GeneratedField::MaxItems),
                            "unique" => Ok(GeneratedField::Unique),
                            "items" => Ok(GeneratedField::Items),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RepeatedRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.RepeatedRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RepeatedRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_items__ = None;
                let mut max_items__ = None;
                let mut unique__ = None;
                let mut items__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinItems => {
                            if min_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minItems"));
                            }
                            min_items__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxItems => {
                            if max_items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxItems"));
                            }
                            max_items__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Unique => {
                            if unique__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unique"));
                            }
                            unique__ = map_.next_value()?;
                        }
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RepeatedRules {
                    min_items: min_items__,
                    max_items: max_items__,
                    unique: unique__,
                    items: items__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.RepeatedRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id.is_some() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.expression.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.Rule", len)?;
        if let Some(v) = self.id.as_ref() {
            struct_ser.serialize_field("id", v)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.expression.as_ref() {
            struct_ser.serialize_field("expression", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "message",
            "expression",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Message,
            Expression,
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
                            "id" => Ok(GeneratedField::Id),
                            "message" => Ok(GeneratedField::Message),
                            "expression" => Ok(GeneratedField::Expression),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.Rule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Rule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut message__ = None;
                let mut expression__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = map_.next_value()?;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Rule {
                    id: id__,
                    message: message__,
                    expression: expression__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.Rule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SFixed32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.SFixed32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                s_fixed32_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                s_fixed32_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                s_fixed32_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                s_fixed32_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SFixed32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SFixed32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.SFixed32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SFixed32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed32_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed32_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed32_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed32_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(SFixed32Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.SFixed32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SFixed64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.SFixed64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                s_fixed64_rules::LessThan::Lt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
                }
                s_fixed64_rules::LessThan::Lte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                s_fixed64_rules::GreaterThan::Gt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
                }
                s_fixed64_rules::GreaterThan::Gte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SFixed64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SFixed64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.SFixed64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SFixed64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed64_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed64_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed64_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_fixed64_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(SFixed64Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.SFixed64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SInt32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.SInt32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                s_int32_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                s_int32_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                s_int32_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                s_int32_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SInt32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SInt32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.SInt32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SInt32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int32_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int32_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int32_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int32_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(SInt32Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.SInt32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SInt64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.SInt64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                s_int64_rules::LessThan::Lt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
                }
                s_int64_rules::LessThan::Lte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                s_int64_rules::GreaterThan::Gt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
                }
                s_int64_rules::GreaterThan::Gte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SInt64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SInt64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.SInt64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SInt64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int64_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int64_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int64_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| s_int64_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(SInt64Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.SInt64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.len.is_some() {
            len += 1;
        }
        if self.min_len.is_some() {
            len += 1;
        }
        if self.max_len.is_some() {
            len += 1;
        }
        if self.len_bytes.is_some() {
            len += 1;
        }
        if self.min_bytes.is_some() {
            len += 1;
        }
        if self.max_bytes.is_some() {
            len += 1;
        }
        if self.pattern.is_some() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        if self.suffix.is_some() {
            len += 1;
        }
        if self.contains.is_some() {
            len += 1;
        }
        if self.not_contains.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if self.strict.is_some() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.well_known.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.StringRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.len.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("len", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.min_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minLen", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_len.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxLen", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.len_bytes.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("lenBytes", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.min_bytes.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("minBytes", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_bytes.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxBytes", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.pattern.as_ref() {
            struct_ser.serialize_field("pattern", v)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            struct_ser.serialize_field("prefix", v)?;
        }
        if let Some(v) = self.suffix.as_ref() {
            struct_ser.serialize_field("suffix", v)?;
        }
        if let Some(v) = self.contains.as_ref() {
            struct_ser.serialize_field("contains", v)?;
        }
        if let Some(v) = self.not_contains.as_ref() {
            struct_ser.serialize_field("notContains", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if let Some(v) = self.strict.as_ref() {
            struct_ser.serialize_field("strict", v)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.well_known.as_ref() {
            match v {
                string_rules::WellKnown::Email(v) => {
                    struct_ser.serialize_field("email", v)?;
                }
                string_rules::WellKnown::Hostname(v) => {
                    struct_ser.serialize_field("hostname", v)?;
                }
                string_rules::WellKnown::Ip(v) => {
                    struct_ser.serialize_field("ip", v)?;
                }
                string_rules::WellKnown::Ipv4(v) => {
                    struct_ser.serialize_field("ipv4", v)?;
                }
                string_rules::WellKnown::Ipv6(v) => {
                    struct_ser.serialize_field("ipv6", v)?;
                }
                string_rules::WellKnown::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
                string_rules::WellKnown::UriRef(v) => {
                    struct_ser.serialize_field("uriRef", v)?;
                }
                string_rules::WellKnown::Address(v) => {
                    struct_ser.serialize_field("address", v)?;
                }
                string_rules::WellKnown::Uuid(v) => {
                    struct_ser.serialize_field("uuid", v)?;
                }
                string_rules::WellKnown::Tuuid(v) => {
                    struct_ser.serialize_field("tuuid", v)?;
                }
                string_rules::WellKnown::IpWithPrefixlen(v) => {
                    struct_ser.serialize_field("ipWithPrefixlen", v)?;
                }
                string_rules::WellKnown::Ipv4WithPrefixlen(v) => {
                    struct_ser.serialize_field("ipv4WithPrefixlen", v)?;
                }
                string_rules::WellKnown::Ipv6WithPrefixlen(v) => {
                    struct_ser.serialize_field("ipv6WithPrefixlen", v)?;
                }
                string_rules::WellKnown::IpPrefix(v) => {
                    struct_ser.serialize_field("ipPrefix", v)?;
                }
                string_rules::WellKnown::Ipv4Prefix(v) => {
                    struct_ser.serialize_field("ipv4Prefix", v)?;
                }
                string_rules::WellKnown::Ipv6Prefix(v) => {
                    struct_ser.serialize_field("ipv6Prefix", v)?;
                }
                string_rules::WellKnown::HostAndPort(v) => {
                    struct_ser.serialize_field("hostAndPort", v)?;
                }
                string_rules::WellKnown::WellKnownRegex(v) => {
                    let v = KnownRegex::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("wellKnownRegex", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "len",
            "min_len",
            "minLen",
            "max_len",
            "maxLen",
            "len_bytes",
            "lenBytes",
            "min_bytes",
            "minBytes",
            "max_bytes",
            "maxBytes",
            "pattern",
            "prefix",
            "suffix",
            "contains",
            "not_contains",
            "notContains",
            "in",
            "not_in",
            "notIn",
            "strict",
            "example",
            "email",
            "hostname",
            "ip",
            "ipv4",
            "ipv6",
            "uri",
            "uri_ref",
            "uriRef",
            "address",
            "uuid",
            "tuuid",
            "ip_with_prefixlen",
            "ipWithPrefixlen",
            "ipv4_with_prefixlen",
            "ipv4WithPrefixlen",
            "ipv6_with_prefixlen",
            "ipv6WithPrefixlen",
            "ip_prefix",
            "ipPrefix",
            "ipv4_prefix",
            "ipv4Prefix",
            "ipv6_prefix",
            "ipv6Prefix",
            "host_and_port",
            "hostAndPort",
            "well_known_regex",
            "wellKnownRegex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Len,
            MinLen,
            MaxLen,
            LenBytes,
            MinBytes,
            MaxBytes,
            Pattern,
            Prefix,
            Suffix,
            Contains,
            NotContains,
            In,
            NotIn,
            Strict,
            Example,
            Email,
            Hostname,
            Ip,
            Ipv4,
            Ipv6,
            Uri,
            UriRef,
            Address,
            Uuid,
            Tuuid,
            IpWithPrefixlen,
            Ipv4WithPrefixlen,
            Ipv6WithPrefixlen,
            IpPrefix,
            Ipv4Prefix,
            Ipv6Prefix,
            HostAndPort,
            WellKnownRegex,
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
                            "const" => Ok(GeneratedField::Const),
                            "len" => Ok(GeneratedField::Len),
                            "minLen" | "min_len" => Ok(GeneratedField::MinLen),
                            "maxLen" | "max_len" => Ok(GeneratedField::MaxLen),
                            "lenBytes" | "len_bytes" => Ok(GeneratedField::LenBytes),
                            "minBytes" | "min_bytes" => Ok(GeneratedField::MinBytes),
                            "maxBytes" | "max_bytes" => Ok(GeneratedField::MaxBytes),
                            "pattern" => Ok(GeneratedField::Pattern),
                            "prefix" => Ok(GeneratedField::Prefix),
                            "suffix" => Ok(GeneratedField::Suffix),
                            "contains" => Ok(GeneratedField::Contains),
                            "notContains" | "not_contains" => Ok(GeneratedField::NotContains),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "strict" => Ok(GeneratedField::Strict),
                            "example" => Ok(GeneratedField::Example),
                            "email" => Ok(GeneratedField::Email),
                            "hostname" => Ok(GeneratedField::Hostname),
                            "ip" => Ok(GeneratedField::Ip),
                            "ipv4" => Ok(GeneratedField::Ipv4),
                            "ipv6" => Ok(GeneratedField::Ipv6),
                            "uri" => Ok(GeneratedField::Uri),
                            "uriRef" | "uri_ref" => Ok(GeneratedField::UriRef),
                            "address" => Ok(GeneratedField::Address),
                            "uuid" => Ok(GeneratedField::Uuid),
                            "tuuid" => Ok(GeneratedField::Tuuid),
                            "ipWithPrefixlen" | "ip_with_prefixlen" => Ok(GeneratedField::IpWithPrefixlen),
                            "ipv4WithPrefixlen" | "ipv4_with_prefixlen" => Ok(GeneratedField::Ipv4WithPrefixlen),
                            "ipv6WithPrefixlen" | "ipv6_with_prefixlen" => Ok(GeneratedField::Ipv6WithPrefixlen),
                            "ipPrefix" | "ip_prefix" => Ok(GeneratedField::IpPrefix),
                            "ipv4Prefix" | "ipv4_prefix" => Ok(GeneratedField::Ipv4Prefix),
                            "ipv6Prefix" | "ipv6_prefix" => Ok(GeneratedField::Ipv6Prefix),
                            "hostAndPort" | "host_and_port" => Ok(GeneratedField::HostAndPort),
                            "wellKnownRegex" | "well_known_regex" => Ok(GeneratedField::WellKnownRegex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.StringRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut len__ = None;
                let mut min_len__ = None;
                let mut max_len__ = None;
                let mut len_bytes__ = None;
                let mut min_bytes__ = None;
                let mut max_bytes__ = None;
                let mut pattern__ = None;
                let mut prefix__ = None;
                let mut suffix__ = None;
                let mut contains__ = None;
                let mut not_contains__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut strict__ = None;
                let mut example__ = None;
                let mut well_known__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                        GeneratedField::Len => {
                            if len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("len"));
                            }
                            len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinLen => {
                            if min_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minLen"));
                            }
                            min_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxLen => {
                            if max_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxLen"));
                            }
                            max_len__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LenBytes => {
                            if len_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lenBytes"));
                            }
                            len_bytes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MinBytes => {
                            if min_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minBytes"));
                            }
                            min_bytes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxBytes => {
                            if max_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBytes"));
                            }
                            max_bytes__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Pattern => {
                            if pattern__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pattern"));
                            }
                            pattern__ = map_.next_value()?;
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = map_.next_value()?;
                        }
                        GeneratedField::Suffix => {
                            if suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            suffix__ = map_.next_value()?;
                        }
                        GeneratedField::Contains => {
                            if contains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contains"));
                            }
                            contains__ = map_.next_value()?;
                        }
                        GeneratedField::NotContains => {
                            if not_contains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notContains"));
                            }
                            not_contains__ = map_.next_value()?;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Strict => {
                            if strict__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strict"));
                            }
                            strict__ = map_.next_value()?;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Email);
                        }
                        GeneratedField::Hostname => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Hostname);
                        }
                        GeneratedField::Ip => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ip"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ip);
                        }
                        GeneratedField::Ipv4 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv4);
                        }
                        GeneratedField::Ipv6 => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv6);
                        }
                        GeneratedField::Uri => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Uri);
                        }
                        GeneratedField::UriRef => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uriRef"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::UriRef);
                        }
                        GeneratedField::Address => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Address);
                        }
                        GeneratedField::Uuid => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Uuid);
                        }
                        GeneratedField::Tuuid => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuuid"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Tuuid);
                        }
                        GeneratedField::IpWithPrefixlen => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipWithPrefixlen"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::IpWithPrefixlen);
                        }
                        GeneratedField::Ipv4WithPrefixlen => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4WithPrefixlen"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv4WithPrefixlen);
                        }
                        GeneratedField::Ipv6WithPrefixlen => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6WithPrefixlen"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv6WithPrefixlen);
                        }
                        GeneratedField::IpPrefix => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipPrefix"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::IpPrefix);
                        }
                        GeneratedField::Ipv4Prefix => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4Prefix"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv4Prefix);
                        }
                        GeneratedField::Ipv6Prefix => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6Prefix"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::Ipv6Prefix);
                        }
                        GeneratedField::HostAndPort => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostAndPort"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<_>>()?.map(string_rules::WellKnown::HostAndPort);
                        }
                        GeneratedField::WellKnownRegex => {
                            if well_known__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wellKnownRegex"));
                            }
                            well_known__ = map_.next_value::<::std::option::Option<KnownRegex>>()?.map(|x| string_rules::WellKnown::WellKnownRegex(x as i32));
                        }
                    }
                }
                Ok(StringRules {
                    r#const: r#const__,
                    len: len__,
                    min_len: min_len__,
                    max_len: max_len__,
                    len_bytes: len_bytes__,
                    min_bytes: min_bytes__,
                    max_bytes: max_bytes__,
                    pattern: pattern__,
                    prefix: prefix__,
                    suffix: suffix__,
                    contains: contains__,
                    not_contains: not_contains__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    strict: strict__,
                    example: example__.unwrap_or_default(),
                    well_known: well_known__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.StringRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampRules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if self.within.is_some() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.TimestampRules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if let Some(v) = self.within.as_ref() {
            struct_ser.serialize_field("within", v)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                timestamp_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                timestamp_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
                timestamp_rules::LessThan::LtNow(v) => {
                    struct_ser.serialize_field("ltNow", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                timestamp_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                timestamp_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
                timestamp_rules::GreaterThan::GtNow(v) => {
                    struct_ser.serialize_field("gtNow", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampRules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "within",
            "example",
            "lt",
            "lte",
            "lt_now",
            "ltNow",
            "gt",
            "gte",
            "gt_now",
            "gtNow",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            Within,
            Example,
            Lt,
            Lte,
            LtNow,
            Gt,
            Gte,
            GtNow,
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
                            "const" => Ok(GeneratedField::Const),
                            "within" => Ok(GeneratedField::Within),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "ltNow" | "lt_now" => Ok(GeneratedField::LtNow),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            "gtNow" | "gt_now" => Ok(GeneratedField::GtNow),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimestampRules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.TimestampRules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimestampRules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut within__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = map_.next_value()?;
                        }
                        GeneratedField::Within => {
                            if within__.is_some() {
                                return Err(serde::de::Error::duplicate_field("within"));
                            }
                            within__ = map_.next_value()?;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<_>>()?.map(timestamp_rules::LessThan::Lt)
;
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<_>>()?.map(timestamp_rules::LessThan::Lte)
;
                        }
                        GeneratedField::LtNow => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ltNow"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<_>>()?.map(timestamp_rules::LessThan::LtNow);
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<_>>()?.map(timestamp_rules::GreaterThan::Gt)
;
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<_>>()?.map(timestamp_rules::GreaterThan::Gte)
;
                        }
                        GeneratedField::GtNow => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gtNow"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<_>>()?.map(timestamp_rules::GreaterThan::GtNow);
                        }
                    }
                }
                Ok(TimestampRules {
                    r#const: r#const__,
                    within: within__,
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.TimestampRules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UInt32Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.UInt32Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            struct_ser.serialize_field("const", v)?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in)?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in)?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example)?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                u_int32_rules::LessThan::Lt(v) => {
                    struct_ser.serialize_field("lt", v)?;
                }
                u_int32_rules::LessThan::Lte(v) => {
                    struct_ser.serialize_field("lte", v)?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                u_int32_rules::GreaterThan::Gt(v) => {
                    struct_ser.serialize_field("gt", v)?;
                }
                u_int32_rules::GreaterThan::Gte(v) => {
                    struct_ser.serialize_field("gte", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UInt32Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UInt32Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.UInt32Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UInt32Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int32_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int32_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int32_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int32_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(UInt32Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.UInt32Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UInt64Rules {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#const.is_some() {
            len += 1;
        }
        if !self.r#in.is_empty() {
            len += 1;
        }
        if !self.not_in.is_empty() {
            len += 1;
        }
        if !self.example.is_empty() {
            len += 1;
        }
        if self.less_than.is_some() {
            len += 1;
        }
        if self.greater_than.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.UInt64Rules", len)?;
        if let Some(v) = self.r#const.as_ref() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("const", ToString::to_string(&v).as_str())?;
        }
        if !self.r#in.is_empty() {
            struct_ser.serialize_field("in", &self.r#in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.not_in.is_empty() {
            struct_ser.serialize_field("notIn", &self.not_in.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.example.is_empty() {
            struct_ser.serialize_field("example", &self.example.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.less_than.as_ref() {
            match v {
                u_int64_rules::LessThan::Lt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lt", ToString::to_string(&v).as_str())?;
                }
                u_int64_rules::LessThan::Lte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("lte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        if let Some(v) = self.greater_than.as_ref() {
            match v {
                u_int64_rules::GreaterThan::Gt(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gt", ToString::to_string(&v).as_str())?;
                }
                u_int64_rules::GreaterThan::Gte(v) => {
                    #[allow(clippy::needless_borrow)]
                    struct_ser.serialize_field("gte", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UInt64Rules {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "const",
            "in",
            "not_in",
            "notIn",
            "example",
            "lt",
            "lte",
            "gt",
            "gte",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Const,
            In,
            NotIn,
            Example,
            Lt,
            Lte,
            Gt,
            Gte,
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
                            "const" => Ok(GeneratedField::Const),
                            "in" => Ok(GeneratedField::In),
                            "notIn" | "not_in" => Ok(GeneratedField::NotIn),
                            "example" => Ok(GeneratedField::Example),
                            "lt" => Ok(GeneratedField::Lt),
                            "lte" => Ok(GeneratedField::Lte),
                            "gt" => Ok(GeneratedField::Gt),
                            "gte" => Ok(GeneratedField::Gte),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UInt64Rules;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.UInt64Rules")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UInt64Rules, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#const__ = None;
                let mut r#in__ = None;
                let mut not_in__ = None;
                let mut example__ = None;
                let mut less_than__ = None;
                let mut greater_than__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Const => {
                            if r#const__.is_some() {
                                return Err(serde::de::Error::duplicate_field("const"));
                            }
                            r#const__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::In => {
                            if r#in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("in"));
                            }
                            r#in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::NotIn => {
                            if not_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notIn"));
                            }
                            not_in__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Example => {
                            if example__.is_some() {
                                return Err(serde::de::Error::duplicate_field("example"));
                            }
                            example__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Lt => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lt"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int64_rules::LessThan::Lt(x.0));
                        }
                        GeneratedField::Lte => {
                            if less_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lte"));
                            }
                            less_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int64_rules::LessThan::Lte(x.0));
                        }
                        GeneratedField::Gt => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gt"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int64_rules::GreaterThan::Gt(x.0));
                        }
                        GeneratedField::Gte => {
                            if greater_than__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gte"));
                            }
                            greater_than__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| u_int64_rules::GreaterThan::Gte(x.0));
                        }
                    }
                }
                Ok(UInt64Rules {
                    r#const: r#const__,
                    r#in: r#in__.unwrap_or_default(),
                    not_in: not_in__.unwrap_or_default(),
                    example: example__.unwrap_or_default(),
                    less_than: less_than__,
                    greater_than: greater_than__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.UInt64Rules", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Violation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field.is_some() {
            len += 1;
        }
        if self.rule.is_some() {
            len += 1;
        }
        if self.rule_id.is_some() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.for_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.Violation", len)?;
        if let Some(v) = self.field.as_ref() {
            struct_ser.serialize_field("field", v)?;
        }
        if let Some(v) = self.rule.as_ref() {
            struct_ser.serialize_field("rule", v)?;
        }
        if let Some(v) = self.rule_id.as_ref() {
            struct_ser.serialize_field("ruleId", v)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.for_key.as_ref() {
            struct_ser.serialize_field("forKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Violation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "rule",
            "rule_id",
            "ruleId",
            "message",
            "for_key",
            "forKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Rule,
            RuleId,
            Message,
            ForKey,
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
                            "field" => Ok(GeneratedField::Field),
                            "rule" => Ok(GeneratedField::Rule),
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "message" => Ok(GeneratedField::Message),
                            "forKey" | "for_key" => Ok(GeneratedField::ForKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Violation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.Violation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Violation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut rule__ = None;
                let mut rule_id__ = None;
                let mut message__ = None;
                let mut for_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = map_.next_value()?;
                        }
                        GeneratedField::Rule => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            rule__ = map_.next_value()?;
                        }
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = map_.next_value()?;
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map_.next_value()?;
                        }
                        GeneratedField::ForKey => {
                            if for_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forKey"));
                            }
                            for_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Violation {
                    field: field__,
                    rule: rule__,
                    rule_id: rule_id__,
                    message: message__,
                    for_key: for_key__,
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.Violation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Violations {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("buf.validate.Violations", len)?;
        if !self.violations.is_empty() {
            struct_ser.serialize_field("violations", &self.violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Violations {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violations,
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
                            "violations" => Ok(GeneratedField::Violations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Violations;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct buf.validate.Violations")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Violations, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Violations => {
                            if violations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("violations"));
                            }
                            violations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Violations {
                    violations: violations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("buf.validate.Violations", FIELDS, GeneratedVisitor)
    }
}
