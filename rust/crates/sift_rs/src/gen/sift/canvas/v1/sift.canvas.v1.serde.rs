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
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
