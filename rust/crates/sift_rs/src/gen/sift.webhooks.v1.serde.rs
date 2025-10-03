// @generated
impl serde::Serialize for BatchCreateWebhookLogsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.BatchCreateWebhookLogsRequest", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateWebhookLogsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
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
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCreateWebhookLogsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.BatchCreateWebhookLogsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateWebhookLogsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCreateWebhookLogsRequest {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.BatchCreateWebhookLogsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCreateWebhookLogsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.webhooks.v1.BatchCreateWebhookLogsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCreateWebhookLogsResponse {
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
            type Value = BatchCreateWebhookLogsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.BatchCreateWebhookLogsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCreateWebhookLogsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BatchCreateWebhookLogsResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.BatchCreateWebhookLogsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebhookLogRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.webhook_id.is_empty() {
            len += 1;
        }
        if !self.event_id.is_empty() {
            len += 1;
        }
        if self.retry_attempt_number != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        if self.error_reason.is_some() {
            len += 1;
        }
        if self.sent_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.CreateWebhookLogRequest", len)?;
        if !self.webhook_id.is_empty() {
            struct_ser.serialize_field("webhookId", &self.webhook_id)?;
        }
        if !self.event_id.is_empty() {
            struct_ser.serialize_field("eventId", &self.event_id)?;
        }
        if self.retry_attempt_number != 0 {
            struct_ser.serialize_field("retryAttemptNumber", &self.retry_attempt_number)?;
        }
        if self.status != 0 {
            let v = WebhookLogStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            struct_ser.serialize_field("payload", v)?;
        }
        if let Some(v) = self.error_reason.as_ref() {
            struct_ser.serialize_field("errorReason", v)?;
        }
        if let Some(v) = self.sent_date.as_ref() {
            struct_ser.serialize_field("sentDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebhookLogRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook_id",
            "webhookId",
            "event_id",
            "eventId",
            "retry_attempt_number",
            "retryAttemptNumber",
            "status",
            "payload",
            "error_reason",
            "errorReason",
            "sent_date",
            "sentDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebhookId,
            EventId,
            RetryAttemptNumber,
            Status,
            Payload,
            ErrorReason,
            SentDate,
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
                            "webhookId" | "webhook_id" => Ok(GeneratedField::WebhookId),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "retryAttemptNumber" | "retry_attempt_number" => Ok(GeneratedField::RetryAttemptNumber),
                            "status" => Ok(GeneratedField::Status),
                            "payload" => Ok(GeneratedField::Payload),
                            "errorReason" | "error_reason" => Ok(GeneratedField::ErrorReason),
                            "sentDate" | "sent_date" => Ok(GeneratedField::SentDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebhookLogRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.CreateWebhookLogRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebhookLogRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook_id__ = None;
                let mut event_id__ = None;
                let mut retry_attempt_number__ = None;
                let mut status__ = None;
                let mut payload__ = None;
                let mut error_reason__ = None;
                let mut sent_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebhookId => {
                            if webhook_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhookId"));
                            }
                            webhook_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetryAttemptNumber => {
                            if retry_attempt_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryAttemptNumber"));
                            }
                            retry_attempt_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<WebhookLogStatus>()? as i32);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorReason => {
                            if error_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorReason"));
                            }
                            error_reason__ = map_.next_value()?;
                        }
                        GeneratedField::SentDate => {
                            if sent_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sentDate"));
                            }
                            sent_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateWebhookLogRequest {
                    webhook_id: webhook_id__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    retry_attempt_number: retry_attempt_number__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    payload: payload__,
                    error_reason: error_reason__,
                    sent_date: sent_date__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.CreateWebhookLogRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebhookRequest {
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
        if !self.target_url.is_empty() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        if !self.http_headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.CreateWebhookRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.target_url.is_empty() {
            struct_ser.serialize_field("targetUrl", &self.target_url)?;
        }
        if self.event_type != 0 {
            let v = WebhookEventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            struct_ser.serialize_field("payload", v)?;
        }
        if !self.http_headers.is_empty() {
            struct_ser.serialize_field("httpHeaders", &self.http_headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebhookRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "target_url",
            "targetUrl",
            "event_type",
            "eventType",
            "payload",
            "http_headers",
            "httpHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TargetUrl,
            EventType,
            Payload,
            HttpHeaders,
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
                            "targetUrl" | "target_url" => Ok(GeneratedField::TargetUrl),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "payload" => Ok(GeneratedField::Payload),
                            "httpHeaders" | "http_headers" => Ok(GeneratedField::HttpHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebhookRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.CreateWebhookRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebhookRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut target_url__ = None;
                let mut event_type__ = None;
                let mut payload__ = None;
                let mut http_headers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetUrl => {
                            if target_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUrl"));
                            }
                            target_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<WebhookEventType>()? as i32);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = map_.next_value()?;
                        }
                        GeneratedField::HttpHeaders => {
                            if http_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpHeaders"));
                            }
                            http_headers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateWebhookRequest {
                    name: name__.unwrap_or_default(),
                    target_url: target_url__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    payload: payload__,
                    http_headers: http_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.CreateWebhookRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebhookResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.webhook.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.CreateWebhookResponse", len)?;
        if let Some(v) = self.webhook.as_ref() {
            struct_ser.serialize_field("webhook", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebhookResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Webhook,
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
                            "webhook" => Ok(GeneratedField::Webhook),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebhookResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.CreateWebhookResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebhookResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Webhook => {
                            if webhook__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhook"));
                            }
                            webhook__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateWebhookResponse {
                    webhook: webhook__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.CreateWebhookResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebhookSignatureKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.webhooks.v1.CreateWebhookSignatureKeyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebhookSignatureKeyRequest {
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
            type Value = CreateWebhookSignatureKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.CreateWebhookSignatureKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebhookSignatureKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CreateWebhookSignatureKeyRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.CreateWebhookSignatureKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWebhookSignatureKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.signature_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.CreateWebhookSignatureKeyResponse", len)?;
        if let Some(v) = self.signature_key.as_ref() {
            struct_ser.serialize_field("signatureKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWebhookSignatureKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature_key",
            "signatureKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignatureKey,
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
                            "signatureKey" | "signature_key" => Ok(GeneratedField::SignatureKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWebhookSignatureKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.CreateWebhookSignatureKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWebhookSignatureKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signature_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SignatureKey => {
                            if signature_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureKey"));
                            }
                            signature_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateWebhookSignatureKeyResponse {
                    signature_key: signature_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.CreateWebhookSignatureKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWebhookRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.webhook_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.GetWebhookRequest", len)?;
        if !self.webhook_id.is_empty() {
            struct_ser.serialize_field("webhookId", &self.webhook_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWebhookRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook_id",
            "webhookId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebhookId,
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
                            "webhookId" | "webhook_id" => Ok(GeneratedField::WebhookId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWebhookRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.GetWebhookRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWebhookRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebhookId => {
                            if webhook_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhookId"));
                            }
                            webhook_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetWebhookRequest {
                    webhook_id: webhook_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.GetWebhookRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWebhookResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.webhook.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.GetWebhookResponse", len)?;
        if let Some(v) = self.webhook.as_ref() {
            struct_ser.serialize_field("webhook", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWebhookResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Webhook,
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
                            "webhook" => Ok(GeneratedField::Webhook),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWebhookResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.GetWebhookResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWebhookResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Webhook => {
                            if webhook__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhook"));
                            }
                            webhook__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWebhookResponse {
                    webhook: webhook__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.GetWebhookResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWebhookSignatureKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.GetWebhookSignatureKeyRequest", len)?;
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWebhookSignatureKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrganizationId,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWebhookSignatureKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.GetWebhookSignatureKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWebhookSignatureKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetWebhookSignatureKeyRequest {
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.GetWebhookSignatureKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWebhookSignatureKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.signature_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.GetWebhookSignatureKeyResponse", len)?;
        if let Some(v) = self.signature_key.as_ref() {
            struct_ser.serialize_field("signatureKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWebhookSignatureKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature_key",
            "signatureKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignatureKey,
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
                            "signatureKey" | "signature_key" => Ok(GeneratedField::SignatureKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWebhookSignatureKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.GetWebhookSignatureKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWebhookSignatureKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signature_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SignatureKey => {
                            if signature_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureKey"));
                            }
                            signature_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWebhookSignatureKeyResponse {
                    signature_key: signature_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.GetWebhookSignatureKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebhookLogsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.ListWebhookLogsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListWebhookLogsRequest {
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
            type Value = ListWebhookLogsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.ListWebhookLogsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebhookLogsRequest, V::Error>
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
                Ok(ListWebhookLogsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.ListWebhookLogsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebhookLogsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.logs.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.ListWebhookLogsResponse", len)?;
        if !self.logs.is_empty() {
            struct_ser.serialize_field("logs", &self.logs)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebhookLogsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "logs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Logs,
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
                            "logs" => Ok(GeneratedField::Logs),
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
            type Value = ListWebhookLogsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.ListWebhookLogsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebhookLogsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut logs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Logs => {
                            if logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logs"));
                            }
                            logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListWebhookLogsResponse {
                    logs: logs__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.ListWebhookLogsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebhooksRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.ListWebhooksRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListWebhooksRequest {
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
            type Value = ListWebhooksRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.ListWebhooksRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebhooksRequest, V::Error>
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
                Ok(ListWebhooksRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.ListWebhooksRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWebhooksResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.webhooks.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.ListWebhooksResponse", len)?;
        if !self.webhooks.is_empty() {
            struct_ser.serialize_field("webhooks", &self.webhooks)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWebhooksResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhooks",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Webhooks,
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
                            "webhooks" => Ok(GeneratedField::Webhooks),
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
            type Value = ListWebhooksResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.ListWebhooksResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWebhooksResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhooks__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Webhooks => {
                            if webhooks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhooks"));
                            }
                            webhooks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListWebhooksResponse {
                    webhooks: webhooks__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.ListWebhooksResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RotateWebhookSignatureKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.webhooks.v1.RotateWebhookSignatureKeyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RotateWebhookSignatureKeyRequest {
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
            type Value = RotateWebhookSignatureKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.RotateWebhookSignatureKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RotateWebhookSignatureKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RotateWebhookSignatureKeyRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.RotateWebhookSignatureKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RotateWebhookSignatureKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.signature_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.RotateWebhookSignatureKeyResponse", len)?;
        if let Some(v) = self.signature_key.as_ref() {
            struct_ser.serialize_field("signatureKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RotateWebhookSignatureKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature_key",
            "signatureKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignatureKey,
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
                            "signatureKey" | "signature_key" => Ok(GeneratedField::SignatureKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RotateWebhookSignatureKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.RotateWebhookSignatureKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RotateWebhookSignatureKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signature_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SignatureKey => {
                            if signature_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureKey"));
                            }
                            signature_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RotateWebhookSignatureKeyResponse {
                    signature_key: signature_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.RotateWebhookSignatureKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestWebhookRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.form.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.TestWebhookRequest", len)?;
        if let Some(v) = self.form.as_ref() {
            match v {
                test_webhook_request::Form::WebhookId(v) => {
                    struct_ser.serialize_field("webhookId", v)?;
                }
                test_webhook_request::Form::Webhook(v) => {
                    struct_ser.serialize_field("webhook", v)?;
                }
                test_webhook_request::Form::CreateRequest(v) => {
                    struct_ser.serialize_field("createRequest", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestWebhookRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook_id",
            "webhookId",
            "webhook",
            "create_request",
            "createRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebhookId,
            Webhook,
            CreateRequest,
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
                            "webhookId" | "webhook_id" => Ok(GeneratedField::WebhookId),
                            "webhook" => Ok(GeneratedField::Webhook),
                            "createRequest" | "create_request" => Ok(GeneratedField::CreateRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestWebhookRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.TestWebhookRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestWebhookRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut form__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebhookId => {
                            if form__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhookId"));
                            }
                            form__ = map_.next_value::<::std::option::Option<_>>()?.map(test_webhook_request::Form::WebhookId);
                        }
                        GeneratedField::Webhook => {
                            if form__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhook"));
                            }
                            form__ = map_.next_value::<::std::option::Option<_>>()?.map(test_webhook_request::Form::Webhook)
;
                        }
                        GeneratedField::CreateRequest => {
                            if form__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createRequest"));
                            }
                            form__ = map_.next_value::<::std::option::Option<_>>()?.map(test_webhook_request::Form::CreateRequest)
;
                        }
                    }
                }
                Ok(TestWebhookRequest {
                    form: form__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.TestWebhookRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestWebhookResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_response_code != 0 {
            len += 1;
        }
        if !self.http_response_body.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.TestWebhookResponse", len)?;
        if self.http_response_code != 0 {
            struct_ser.serialize_field("httpResponseCode", &self.http_response_code)?;
        }
        if !self.http_response_body.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("httpResponseBody", pbjson::private::base64::encode(&self.http_response_body).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestWebhookResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_response_code",
            "httpResponseCode",
            "http_response_body",
            "httpResponseBody",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpResponseCode,
            HttpResponseBody,
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
                            "httpResponseCode" | "http_response_code" => Ok(GeneratedField::HttpResponseCode),
                            "httpResponseBody" | "http_response_body" => Ok(GeneratedField::HttpResponseBody),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestWebhookResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.TestWebhookResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestWebhookResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_response_code__ = None;
                let mut http_response_body__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HttpResponseCode => {
                            if http_response_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpResponseCode"));
                            }
                            http_response_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HttpResponseBody => {
                            if http_response_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpResponseBody"));
                            }
                            http_response_body__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TestWebhookResponse {
                    http_response_code: http_response_code__.unwrap_or_default(),
                    http_response_body: http_response_body__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.TestWebhookResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ToggleWebhookSignatureKeyActivationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enable {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.ToggleWebhookSignatureKeyActivationRequest", len)?;
        if self.enable {
            struct_ser.serialize_field("enable", &self.enable)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToggleWebhookSignatureKeyActivationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enable,
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
                            "enable" => Ok(GeneratedField::Enable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ToggleWebhookSignatureKeyActivationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.ToggleWebhookSignatureKeyActivationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ToggleWebhookSignatureKeyActivationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enable__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Enable => {
                            if enable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enable"));
                            }
                            enable__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ToggleWebhookSignatureKeyActivationRequest {
                    enable: enable__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.ToggleWebhookSignatureKeyActivationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ToggleWebhookSignatureKeyActivationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.signature_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.ToggleWebhookSignatureKeyActivationResponse", len)?;
        if let Some(v) = self.signature_key.as_ref() {
            struct_ser.serialize_field("signatureKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ToggleWebhookSignatureKeyActivationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature_key",
            "signatureKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignatureKey,
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
                            "signatureKey" | "signature_key" => Ok(GeneratedField::SignatureKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ToggleWebhookSignatureKeyActivationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.ToggleWebhookSignatureKeyActivationResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ToggleWebhookSignatureKeyActivationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signature_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SignatureKey => {
                            if signature_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureKey"));
                            }
                            signature_key__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ToggleWebhookSignatureKeyActivationResponse {
                    signature_key: signature_key__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.ToggleWebhookSignatureKeyActivationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateWebhookRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.webhook.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.UpdateWebhookRequest", len)?;
        if let Some(v) = self.webhook.as_ref() {
            struct_ser.serialize_field("webhook", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateWebhookRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Webhook,
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
                            "webhook" => Ok(GeneratedField::Webhook),
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
            type Value = UpdateWebhookRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.UpdateWebhookRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateWebhookRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Webhook => {
                            if webhook__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhook"));
                            }
                            webhook__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateWebhookRequest {
                    webhook: webhook__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.UpdateWebhookRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateWebhookResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.webhook.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.UpdateWebhookResponse", len)?;
        if let Some(v) = self.webhook.as_ref() {
            struct_ser.serialize_field("webhook", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateWebhookResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Webhook,
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
                            "webhook" => Ok(GeneratedField::Webhook),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateWebhookResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.UpdateWebhookResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateWebhookResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Webhook => {
                            if webhook__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhook"));
                            }
                            webhook__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateWebhookResponse {
                    webhook: webhook__,
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.UpdateWebhookResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Webhook {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.webhook_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.target_url.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if self.payload.is_some() {
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
        if !self.http_headers.is_empty() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.Webhook", len)?;
        if !self.webhook_id.is_empty() {
            struct_ser.serialize_field("webhookId", &self.webhook_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.target_url.is_empty() {
            struct_ser.serialize_field("targetUrl", &self.target_url)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.event_type != 0 {
            let v = WebhookEventType::try_from(self.event_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.event_type)))?;
            struct_ser.serialize_field("eventType", &v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            struct_ser.serialize_field("payload", v)?;
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
        if !self.http_headers.is_empty() {
            struct_ser.serialize_field("httpHeaders", &self.http_headers)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Webhook {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook_id",
            "webhookId",
            "organization_id",
            "organizationId",
            "target_url",
            "targetUrl",
            "name",
            "event_type",
            "eventType",
            "payload",
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
            "http_headers",
            "httpHeaders",
            "is_archived",
            "isArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebhookId,
            OrganizationId,
            TargetUrl,
            Name,
            EventType,
            Payload,
            CreatedDate,
            ModifiedDate,
            ArchivedDate,
            CreatedByUserId,
            ModifiedByUserId,
            HttpHeaders,
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
                            "webhookId" | "webhook_id" => Ok(GeneratedField::WebhookId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "targetUrl" | "target_url" => Ok(GeneratedField::TargetUrl),
                            "name" => Ok(GeneratedField::Name),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "payload" => Ok(GeneratedField::Payload),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "httpHeaders" | "http_headers" => Ok(GeneratedField::HttpHeaders),
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
            type Value = Webhook;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.Webhook")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Webhook, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook_id__ = None;
                let mut organization_id__ = None;
                let mut target_url__ = None;
                let mut name__ = None;
                let mut event_type__ = None;
                let mut payload__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut archived_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut http_headers__ = None;
                let mut is_archived__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebhookId => {
                            if webhook_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhookId"));
                            }
                            webhook_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetUrl => {
                            if target_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUrl"));
                            }
                            target_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = Some(map_.next_value::<WebhookEventType>()? as i32);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = map_.next_value()?;
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
                        GeneratedField::HttpHeaders => {
                            if http_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpHeaders"));
                            }
                            http_headers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Webhook {
                    webhook_id: webhook_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    target_url: target_url__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    payload: payload__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    archived_date: archived_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    http_headers: http_headers__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.Webhook", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebhookEventType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "WEBHOOK_EVENT_TYPE_UNSPECIFIED",
            Self::RuleViolation => "WEBHOOK_EVENT_TYPE_RULE_VIOLATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for WebhookEventType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WEBHOOK_EVENT_TYPE_UNSPECIFIED",
            "WEBHOOK_EVENT_TYPE_RULE_VIOLATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebhookEventType;

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
                    "WEBHOOK_EVENT_TYPE_UNSPECIFIED" => Ok(WebhookEventType::Unspecified),
                    "WEBHOOK_EVENT_TYPE_RULE_VIOLATION" => Ok(WebhookEventType::RuleViolation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WebhookHttpHeader {
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
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.WebhookHttpHeader", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebhookHttpHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebhookHttpHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.WebhookHttpHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebhookHttpHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WebhookHttpHeader {
                    name: name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.WebhookHttpHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebhookLog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.webhook_log_id.is_empty() {
            len += 1;
        }
        if !self.webhook_id.is_empty() {
            len += 1;
        }
        if !self.event_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        if self.retry_attempt_number != 0 {
            len += 1;
        }
        if self.error_reason.is_some() {
            len += 1;
        }
        if self.sent_date.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.WebhookLog", len)?;
        if !self.webhook_log_id.is_empty() {
            struct_ser.serialize_field("webhookLogId", &self.webhook_log_id)?;
        }
        if !self.webhook_id.is_empty() {
            struct_ser.serialize_field("webhookId", &self.webhook_id)?;
        }
        if !self.event_id.is_empty() {
            struct_ser.serialize_field("eventId", &self.event_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if self.status != 0 {
            let v = WebhookLogStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.payload.as_ref() {
            struct_ser.serialize_field("payload", v)?;
        }
        if self.retry_attempt_number != 0 {
            struct_ser.serialize_field("retryAttemptNumber", &self.retry_attempt_number)?;
        }
        if let Some(v) = self.error_reason.as_ref() {
            struct_ser.serialize_field("errorReason", v)?;
        }
        if let Some(v) = self.sent_date.as_ref() {
            struct_ser.serialize_field("sentDate", v)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebhookLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "webhook_log_id",
            "webhookLogId",
            "webhook_id",
            "webhookId",
            "event_id",
            "eventId",
            "organization_id",
            "organizationId",
            "status",
            "payload",
            "retry_attempt_number",
            "retryAttemptNumber",
            "error_reason",
            "errorReason",
            "sent_date",
            "sentDate",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WebhookLogId,
            WebhookId,
            EventId,
            OrganizationId,
            Status,
            Payload,
            RetryAttemptNumber,
            ErrorReason,
            SentDate,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "webhookLogId" | "webhook_log_id" => Ok(GeneratedField::WebhookLogId),
                            "webhookId" | "webhook_id" => Ok(GeneratedField::WebhookId),
                            "eventId" | "event_id" => Ok(GeneratedField::EventId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "status" => Ok(GeneratedField::Status),
                            "payload" => Ok(GeneratedField::Payload),
                            "retryAttemptNumber" | "retry_attempt_number" => Ok(GeneratedField::RetryAttemptNumber),
                            "errorReason" | "error_reason" => Ok(GeneratedField::ErrorReason),
                            "sentDate" | "sent_date" => Ok(GeneratedField::SentDate),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebhookLog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.WebhookLog")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebhookLog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut webhook_log_id__ = None;
                let mut webhook_id__ = None;
                let mut event_id__ = None;
                let mut organization_id__ = None;
                let mut status__ = None;
                let mut payload__ = None;
                let mut retry_attempt_number__ = None;
                let mut error_reason__ = None;
                let mut sent_date__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WebhookLogId => {
                            if webhook_log_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhookLogId"));
                            }
                            webhook_log_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WebhookId => {
                            if webhook_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("webhookId"));
                            }
                            webhook_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventId => {
                            if event_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventId"));
                            }
                            event_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<WebhookLogStatus>()? as i32);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = map_.next_value()?;
                        }
                        GeneratedField::RetryAttemptNumber => {
                            if retry_attempt_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryAttemptNumber"));
                            }
                            retry_attempt_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ErrorReason => {
                            if error_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorReason"));
                            }
                            error_reason__ = map_.next_value()?;
                        }
                        GeneratedField::SentDate => {
                            if sent_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sentDate"));
                            }
                            sent_date__ = map_.next_value()?;
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
                    }
                }
                Ok(WebhookLog {
                    webhook_log_id: webhook_log_id__.unwrap_or_default(),
                    webhook_id: webhook_id__.unwrap_or_default(),
                    event_id: event_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    payload: payload__,
                    retry_attempt_number: retry_attempt_number__.unwrap_or_default(),
                    error_reason: error_reason__,
                    sent_date: sent_date__,
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.WebhookLog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WebhookLogStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "WEBHOOK_LOG_STATUS_UNSPECIFIED",
            Self::Sent => "WEBHOOK_LOG_STATUS_SENT",
            Self::Failed => "WEBHOOK_LOG_STATUS_FAILED",
            Self::Retrying => "WEBHOOK_LOG_STATUS_RETRYING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for WebhookLogStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WEBHOOK_LOG_STATUS_UNSPECIFIED",
            "WEBHOOK_LOG_STATUS_SENT",
            "WEBHOOK_LOG_STATUS_FAILED",
            "WEBHOOK_LOG_STATUS_RETRYING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebhookLogStatus;

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
                    "WEBHOOK_LOG_STATUS_UNSPECIFIED" => Ok(WebhookLogStatus::Unspecified),
                    "WEBHOOK_LOG_STATUS_SENT" => Ok(WebhookLogStatus::Sent),
                    "WEBHOOK_LOG_STATUS_FAILED" => Ok(WebhookLogStatus::Failed),
                    "WEBHOOK_LOG_STATUS_RETRYING" => Ok(WebhookLogStatus::Retrying),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WebhookSignatureKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signature_key.is_empty() {
            len += 1;
        }
        if self.active {
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
        let mut struct_ser = serializer.serialize_struct("sift.webhooks.v1.WebhookSignatureKey", len)?;
        if !self.signature_key.is_empty() {
            struct_ser.serialize_field("signatureKey", &self.signature_key)?;
        }
        if self.active {
            struct_ser.serialize_field("active", &self.active)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WebhookSignatureKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature_key",
            "signatureKey",
            "active",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignatureKey,
            Active,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "signatureKey" | "signature_key" => Ok(GeneratedField::SignatureKey),
                            "active" => Ok(GeneratedField::Active),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WebhookSignatureKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.webhooks.v1.WebhookSignatureKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WebhookSignatureKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signature_key__ = None;
                let mut active__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SignatureKey => {
                            if signature_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatureKey"));
                            }
                            signature_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Active => {
                            if active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            active__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(WebhookSignatureKey {
                    signature_key: signature_key__.unwrap_or_default(),
                    active: active__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.webhooks.v1.WebhookSignatureKey", FIELDS, GeneratedVisitor)
    }
}
