// @generated
impl serde::Serialize for CancelJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.CancelJobRequest", len)?;
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelJobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_id",
            "jobId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobId,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.CancelJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelJobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CancelJobRequest {
                    job_id: job_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.CancelJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelJobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.jobs.v1.CancelJobResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelJobResponse {
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
            type Value = CancelJobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.CancelJobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelJobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CancelJobResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.CancelJobResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataExportJobDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        if !self.storage_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.DataExportJobDetails", len)?;
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if !self.storage_key.is_empty() {
            struct_ser.serialize_field("storageKey", &self.storage_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataExportJobDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request",
            "storage_key",
            "storageKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Request,
            StorageKey,
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
                            "request" => Ok(GeneratedField::Request),
                            "storageKey" | "storage_key" => Ok(GeneratedField::StorageKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataExportJobDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.DataExportJobDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataExportJobDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                let mut storage_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map_.next_value()?;
                        }
                        GeneratedField::StorageKey => {
                            if storage_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageKey"));
                            }
                            storage_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DataExportJobDetails {
                    request: request__,
                    storage_key: storage_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.DataExportJobDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataExportStatusDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.error_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.DataExportStatusDetails", len)?;
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataExportStatusDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorMessage,
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
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataExportStatusDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.DataExportStatusDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataExportStatusDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DataExportStatusDetails {
                    error_message: error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.DataExportStatusDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataImportJobDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data_import_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.DataImportJobDetails", len)?;
        if !self.data_import_id.is_empty() {
            struct_ser.serialize_field("dataImportId", &self.data_import_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataImportJobDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_import_id",
            "dataImportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataImportId,
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
                            "dataImportId" | "data_import_id" => Ok(GeneratedField::DataImportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataImportJobDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.DataImportJobDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataImportJobDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_import_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DataImportId => {
                            if data_import_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImportId"));
                            }
                            data_import_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DataImportJobDetails {
                    data_import_id: data_import_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.DataImportJobDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataImportStatusDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.points_processed != 0 {
            len += 1;
        }
        if self.points_total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.DataImportStatusDetails", len)?;
        if self.points_processed != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("pointsProcessed", ToString::to_string(&self.points_processed).as_str())?;
        }
        if self.points_total != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("pointsTotal", ToString::to_string(&self.points_total).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataImportStatusDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "points_processed",
            "pointsProcessed",
            "points_total",
            "pointsTotal",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PointsProcessed,
            PointsTotal,
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
                            "pointsProcessed" | "points_processed" => Ok(GeneratedField::PointsProcessed),
                            "pointsTotal" | "points_total" => Ok(GeneratedField::PointsTotal),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataImportStatusDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.DataImportStatusDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataImportStatusDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut points_processed__ = None;
                let mut points_total__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PointsProcessed => {
                            if points_processed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pointsProcessed"));
                            }
                            points_processed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PointsTotal => {
                            if points_total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pointsTotal"));
                            }
                            points_total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DataImportStatusDetails {
                    points_processed: points_processed__.unwrap_or_default(),
                    points_total: points_total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.DataImportStatusDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Job {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_id.is_empty() {
            len += 1;
        }
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.created_by_user_id.is_empty() {
            len += 1;
        }
        if !self.modified_by_user_id.is_empty() {
            len += 1;
        }
        if self.created_date.is_some() {
            len += 1;
        }
        if self.modified_date.is_some() {
            len += 1;
        }
        if self.started_date.is_some() {
            len += 1;
        }
        if self.completed_date.is_some() {
            len += 1;
        }
        if self.job_type != 0 {
            len += 1;
        }
        if self.job_status != 0 {
            len += 1;
        }
        if self.job_status_details.is_some() {
            len += 1;
        }
        if self.job_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.Job", len)?;
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.created_by_user_id.is_empty() {
            struct_ser.serialize_field("createdByUserId", &self.created_by_user_id)?;
        }
        if !self.modified_by_user_id.is_empty() {
            struct_ser.serialize_field("modifiedByUserId", &self.modified_by_user_id)?;
        }
        if let Some(v) = self.created_date.as_ref() {
            struct_ser.serialize_field("createdDate", v)?;
        }
        if let Some(v) = self.modified_date.as_ref() {
            struct_ser.serialize_field("modifiedDate", v)?;
        }
        if let Some(v) = self.started_date.as_ref() {
            struct_ser.serialize_field("startedDate", v)?;
        }
        if let Some(v) = self.completed_date.as_ref() {
            struct_ser.serialize_field("completedDate", v)?;
        }
        if self.job_type != 0 {
            let v = JobType::try_from(self.job_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.job_type)))?;
            struct_ser.serialize_field("jobType", &v)?;
        }
        if self.job_status != 0 {
            let v = JobStatus::try_from(self.job_status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.job_status)))?;
            struct_ser.serialize_field("jobStatus", &v)?;
        }
        if let Some(v) = self.job_status_details.as_ref() {
            struct_ser.serialize_field("jobStatusDetails", v)?;
        }
        if let Some(v) = self.job_details.as_ref() {
            struct_ser.serialize_field("jobDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Job {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_id",
            "jobId",
            "organization_id",
            "organizationId",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "started_date",
            "startedDate",
            "completed_date",
            "completedDate",
            "job_type",
            "jobType",
            "job_status",
            "jobStatus",
            "job_status_details",
            "jobStatusDetails",
            "job_details",
            "jobDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobId,
            OrganizationId,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
            StartedDate,
            CompletedDate,
            JobType,
            JobStatus,
            JobStatusDetails,
            JobDetails,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "startedDate" | "started_date" => Ok(GeneratedField::StartedDate),
                            "completedDate" | "completed_date" => Ok(GeneratedField::CompletedDate),
                            "jobType" | "job_type" => Ok(GeneratedField::JobType),
                            "jobStatus" | "job_status" => Ok(GeneratedField::JobStatus),
                            "jobStatusDetails" | "job_status_details" => Ok(GeneratedField::JobStatusDetails),
                            "jobDetails" | "job_details" => Ok(GeneratedField::JobDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Job;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.Job")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Job, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_id__ = None;
                let mut organization_id__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut started_date__ = None;
                let mut completed_date__ = None;
                let mut job_type__ = None;
                let mut job_status__ = None;
                let mut job_status_details__ = None;
                let mut job_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
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
                        GeneratedField::StartedDate => {
                            if started_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startedDate"));
                            }
                            started_date__ = map_.next_value()?;
                        }
                        GeneratedField::CompletedDate => {
                            if completed_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completedDate"));
                            }
                            completed_date__ = map_.next_value()?;
                        }
                        GeneratedField::JobType => {
                            if job_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobType"));
                            }
                            job_type__ = Some(map_.next_value::<JobType>()? as i32);
                        }
                        GeneratedField::JobStatus => {
                            if job_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobStatus"));
                            }
                            job_status__ = Some(map_.next_value::<JobStatus>()? as i32);
                        }
                        GeneratedField::JobStatusDetails => {
                            if job_status_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobStatusDetails"));
                            }
                            job_status_details__ = map_.next_value()?;
                        }
                        GeneratedField::JobDetails => {
                            if job_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobDetails"));
                            }
                            job_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Job {
                    job_id: job_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    started_date: started_date__,
                    completed_date: completed_date__,
                    job_type: job_type__.unwrap_or_default(),
                    job_status: job_status__.unwrap_or_default(),
                    job_status_details: job_status_details__,
                    job_details: job_details__,
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.Job", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.JobDetails", len)?;
        if let Some(v) = self.details.as_ref() {
            match v {
                job_details::Details::RuleEvaluation(v) => {
                    struct_ser.serialize_field("ruleEvaluation", v)?;
                }
                job_details::Details::DataImport(v) => {
                    struct_ser.serialize_field("dataImport", v)?;
                }
                job_details::Details::DataExport(v) => {
                    struct_ser.serialize_field("dataExport", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JobDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_evaluation",
            "ruleEvaluation",
            "data_import",
            "dataImport",
            "data_export",
            "dataExport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleEvaluation,
            DataImport,
            DataExport,
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
                            "ruleEvaluation" | "rule_evaluation" => Ok(GeneratedField::RuleEvaluation),
                            "dataImport" | "data_import" => Ok(GeneratedField::DataImport),
                            "dataExport" | "data_export" => Ok(GeneratedField::DataExport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.JobDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JobDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleEvaluation => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleEvaluation"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(job_details::Details::RuleEvaluation)
;
                        }
                        GeneratedField::DataImport => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImport"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(job_details::Details::DataImport)
;
                        }
                        GeneratedField::DataExport => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataExport"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(job_details::Details::DataExport)
;
                        }
                    }
                }
                Ok(JobDetails {
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.JobDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "JOB_STATUS_UNSPECIFIED",
            Self::Created => "JOB_STATUS_CREATED",
            Self::Running => "JOB_STATUS_RUNNING",
            Self::Finished => "JOB_STATUS_FINISHED",
            Self::Failed => "JOB_STATUS_FAILED",
            Self::Cancelled => "JOB_STATUS_CANCELLED",
            Self::CancelRequested => "JOB_STATUS_CANCEL_REQUESTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for JobStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JOB_STATUS_UNSPECIFIED",
            "JOB_STATUS_CREATED",
            "JOB_STATUS_RUNNING",
            "JOB_STATUS_FINISHED",
            "JOB_STATUS_FAILED",
            "JOB_STATUS_CANCELLED",
            "JOB_STATUS_CANCEL_REQUESTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobStatus;

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
                    "JOB_STATUS_UNSPECIFIED" => Ok(JobStatus::Unspecified),
                    "JOB_STATUS_CREATED" => Ok(JobStatus::Created),
                    "JOB_STATUS_RUNNING" => Ok(JobStatus::Running),
                    "JOB_STATUS_FINISHED" => Ok(JobStatus::Finished),
                    "JOB_STATUS_FAILED" => Ok(JobStatus::Failed),
                    "JOB_STATUS_CANCELLED" => Ok(JobStatus::Cancelled),
                    "JOB_STATUS_CANCEL_REQUESTED" => Ok(JobStatus::CancelRequested),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for JobStatusDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.JobStatusDetails", len)?;
        if let Some(v) = self.status.as_ref() {
            match v {
                job_status_details::Status::RuleEvaluation(v) => {
                    struct_ser.serialize_field("ruleEvaluation", v)?;
                }
                job_status_details::Status::DataImport(v) => {
                    struct_ser.serialize_field("dataImport", v)?;
                }
                job_status_details::Status::DataExport(v) => {
                    struct_ser.serialize_field("dataExport", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JobStatusDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_evaluation",
            "ruleEvaluation",
            "data_import",
            "dataImport",
            "data_export",
            "dataExport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleEvaluation,
            DataImport,
            DataExport,
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
                            "ruleEvaluation" | "rule_evaluation" => Ok(GeneratedField::RuleEvaluation),
                            "dataImport" | "data_import" => Ok(GeneratedField::DataImport),
                            "dataExport" | "data_export" => Ok(GeneratedField::DataExport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobStatusDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.JobStatusDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JobStatusDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleEvaluation => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleEvaluation"));
                            }
                            status__ = map_.next_value::<::std::option::Option<_>>()?.map(job_status_details::Status::RuleEvaluation)
;
                        }
                        GeneratedField::DataImport => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataImport"));
                            }
                            status__ = map_.next_value::<::std::option::Option<_>>()?.map(job_status_details::Status::DataImport)
;
                        }
                        GeneratedField::DataExport => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataExport"));
                            }
                            status__ = map_.next_value::<::std::option::Option<_>>()?.map(job_status_details::Status::DataExport)
;
                        }
                    }
                }
                Ok(JobStatusDetails {
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.JobStatusDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "JOB_TYPE_UNSPECIFIED",
            Self::RuleEvaluation => "JOB_TYPE_RULE_EVALUATION",
            Self::DataImport => "JOB_TYPE_DATA_IMPORT",
            Self::DataExport => "JOB_TYPE_DATA_EXPORT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for JobType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JOB_TYPE_UNSPECIFIED",
            "JOB_TYPE_RULE_EVALUATION",
            "JOB_TYPE_DATA_IMPORT",
            "JOB_TYPE_DATA_EXPORT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobType;

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
                    "JOB_TYPE_UNSPECIFIED" => Ok(JobType::Unspecified),
                    "JOB_TYPE_RULE_EVALUATION" => Ok(JobType::RuleEvaluation),
                    "JOB_TYPE_DATA_IMPORT" => Ok(JobType::DataImport),
                    "JOB_TYPE_DATA_EXPORT" => Ok(JobType::DataExport),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ListJobsRequest {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.ListJobsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListJobsRequest {
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
            "organization_id",
            "organizationId",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrganizationId,
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
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
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
            type Value = ListJobsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.ListJobsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListJobsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut organization_id__ = None;
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListJobsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.ListJobsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListJobsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.jobs.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.ListJobsResponse", len)?;
        if !self.jobs.is_empty() {
            struct_ser.serialize_field("jobs", &self.jobs)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListJobsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "jobs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Jobs,
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
                            "jobs" => Ok(GeneratedField::Jobs),
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
            type Value = ListJobsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.ListJobsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListJobsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut jobs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Jobs => {
                            if jobs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobs"));
                            }
                            jobs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListJobsResponse {
                    jobs: jobs__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.ListJobsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.RetryJobRequest", len)?;
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryJobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_id",
            "jobId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobId,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.RetryJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryJobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RetryJobRequest {
                    job_id: job_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.RetryJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RetryJobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.job.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.RetryJobResponse", len)?;
        if let Some(v) = self.job.as_ref() {
            struct_ser.serialize_field("job", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryJobResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Job,
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
                            "job" => Ok(GeneratedField::Job),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryJobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.RetryJobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RetryJobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Job => {
                            if job__.is_some() {
                                return Err(serde::de::Error::duplicate_field("job"));
                            }
                            job__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RetryJobResponse {
                    job: job__,
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.RetryJobResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleEvaluationJobDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.jobs.v1.RuleEvaluationJobDetails", len)?;
        if !self.report_id.is_empty() {
            struct_ser.serialize_field("reportId", &self.report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleEvaluationJobDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "report_id",
            "reportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReportId,
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
                            "reportId" | "report_id" => Ok(GeneratedField::ReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuleEvaluationJobDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.RuleEvaluationJobDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleEvaluationJobDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReportId => {
                            if report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportId"));
                            }
                            report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RuleEvaluationJobDetails {
                    report_id: report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.RuleEvaluationJobDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuleEvaluationStatusDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.jobs.v1.RuleEvaluationStatusDetails", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuleEvaluationStatusDetails {
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
            type Value = RuleEvaluationStatusDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.jobs.v1.RuleEvaluationStatusDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RuleEvaluationStatusDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RuleEvaluationStatusDetails {
                })
            }
        }
        deserializer.deserialize_struct("sift.jobs.v1.RuleEvaluationStatusDetails", FIELDS, GeneratedVisitor)
    }
}
