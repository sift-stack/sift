// @generated
impl serde::Serialize for CanvasCellExecutionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CANVAS_CELL_EXECUTION_STATUS_UNSPECIFIED",
            Self::InProgress => "CANVAS_CELL_EXECUTION_STATUS_IN_PROGRESS",
            Self::AutoPassed => "CANVAS_CELL_EXECUTION_STATUS_AUTO_PASSED",
            Self::AutoFailed => "CANVAS_CELL_EXECUTION_STATUS_AUTO_FAILED",
            Self::ManualPassed => "CANVAS_CELL_EXECUTION_STATUS_MANUAL_PASSED",
            Self::ManualFailed => "CANVAS_CELL_EXECUTION_STATUS_MANUAL_FAILED",
            Self::Error => "CANVAS_CELL_EXECUTION_STATUS_ERROR",
            Self::Skipped => "CANVAS_CELL_EXECUTION_STATUS_SKIPPED",
            Self::Open => "CANVAS_CELL_EXECUTION_STATUS_OPEN",
            Self::ManuallyCancelled => "CANVAS_CELL_EXECUTION_STATUS_MANUALLY_CANCELLED",
            Self::InQueue => "CANVAS_CELL_EXECUTION_STATUS_IN_QUEUE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CanvasCellExecutionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CANVAS_CELL_EXECUTION_STATUS_UNSPECIFIED",
            "CANVAS_CELL_EXECUTION_STATUS_IN_PROGRESS",
            "CANVAS_CELL_EXECUTION_STATUS_AUTO_PASSED",
            "CANVAS_CELL_EXECUTION_STATUS_AUTO_FAILED",
            "CANVAS_CELL_EXECUTION_STATUS_MANUAL_PASSED",
            "CANVAS_CELL_EXECUTION_STATUS_MANUAL_FAILED",
            "CANVAS_CELL_EXECUTION_STATUS_ERROR",
            "CANVAS_CELL_EXECUTION_STATUS_SKIPPED",
            "CANVAS_CELL_EXECUTION_STATUS_OPEN",
            "CANVAS_CELL_EXECUTION_STATUS_MANUALLY_CANCELLED",
            "CANVAS_CELL_EXECUTION_STATUS_IN_QUEUE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CanvasCellExecutionStatus;

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
                    "CANVAS_CELL_EXECUTION_STATUS_UNSPECIFIED" => Ok(CanvasCellExecutionStatus::Unspecified),
                    "CANVAS_CELL_EXECUTION_STATUS_IN_PROGRESS" => Ok(CanvasCellExecutionStatus::InProgress),
                    "CANVAS_CELL_EXECUTION_STATUS_AUTO_PASSED" => Ok(CanvasCellExecutionStatus::AutoPassed),
                    "CANVAS_CELL_EXECUTION_STATUS_AUTO_FAILED" => Ok(CanvasCellExecutionStatus::AutoFailed),
                    "CANVAS_CELL_EXECUTION_STATUS_MANUAL_PASSED" => Ok(CanvasCellExecutionStatus::ManualPassed),
                    "CANVAS_CELL_EXECUTION_STATUS_MANUAL_FAILED" => Ok(CanvasCellExecutionStatus::ManualFailed),
                    "CANVAS_CELL_EXECUTION_STATUS_ERROR" => Ok(CanvasCellExecutionStatus::Error),
                    "CANVAS_CELL_EXECUTION_STATUS_SKIPPED" => Ok(CanvasCellExecutionStatus::Skipped),
                    "CANVAS_CELL_EXECUTION_STATUS_OPEN" => Ok(CanvasCellExecutionStatus::Open),
                    "CANVAS_CELL_EXECUTION_STATUS_MANUALLY_CANCELLED" => Ok(CanvasCellExecutionStatus::ManuallyCancelled),
                    "CANVAS_CELL_EXECUTION_STATUS_IN_QUEUE" => Ok(CanvasCellExecutionStatus::InQueue),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExportCanvasRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.canvas_id.is_empty() {
            len += 1;
        }
        if !self.canvas_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.canvas.v1.ExportCanvasRequest", len)?;
        if !self.canvas_id.is_empty() {
            struct_ser.serialize_field("canvasId", &self.canvas_id)?;
        }
        if !self.canvas_version_id.is_empty() {
            struct_ser.serialize_field("canvasVersionId", &self.canvas_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportCanvasRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "canvas_id",
            "canvasId",
            "canvas_version_id",
            "canvasVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CanvasId,
            CanvasVersionId,
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
                            "canvasId" | "canvas_id" => Ok(GeneratedField::CanvasId),
                            "canvasVersionId" | "canvas_version_id" => Ok(GeneratedField::CanvasVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportCanvasRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.canvas.v1.ExportCanvasRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportCanvasRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canvas_id__ = None;
                let mut canvas_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CanvasId => {
                            if canvas_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canvasId"));
                            }
                            canvas_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CanvasVersionId => {
                            if canvas_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canvasVersionId"));
                            }
                            canvas_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExportCanvasRequest {
                    canvas_id: canvas_id__.unwrap_or_default(),
                    canvas_version_id: canvas_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.canvas.v1.ExportCanvasRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportCanvasResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.download_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.canvas.v1.ExportCanvasResponse", len)?;
        if !self.download_url.is_empty() {
            struct_ser.serialize_field("downloadUrl", &self.download_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportCanvasResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "download_url",
            "downloadUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DownloadUrl,
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
                            "downloadUrl" | "download_url" => Ok(GeneratedField::DownloadUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportCanvasResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.canvas.v1.ExportCanvasResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportCanvasResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut download_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DownloadUrl => {
                            if download_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downloadUrl"));
                            }
                            download_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExportCanvasResponse {
                    download_url: download_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.canvas.v1.ExportCanvasResponse", FIELDS, GeneratedVisitor)
    }
}
