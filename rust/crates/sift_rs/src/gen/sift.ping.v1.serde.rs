// @generated
impl serde::Serialize for PingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.ping.v1.PingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingRequest {
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
            type Value = PingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ping.v1.PingRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PingRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.ping.v1.PingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.response.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.ping.v1.PingResponse", len)?;
        if !self.response.is_empty() {
            struct_ser.serialize_field("response", &self.response)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Response,
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
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.ping.v1.PingResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PingResponse {
                    response: response__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.ping.v1.PingResponse", FIELDS, GeneratedVisitor)
    }
}
