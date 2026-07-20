// @generated
impl serde::Serialize for AbstractWindowType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignment_name.is_empty() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.AbstractWindowType", len)?;
        if !self.family_alignment_name.is_empty() {
            struct_ser.serialize_field("familyAlignmentName", &self.family_alignment_name)?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AbstractWindowType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignment_name",
            "familyAlignmentName",
            "duration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignmentName,
            Duration,
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
                            "familyAlignmentName" | "family_alignment_name" => Ok(GeneratedField::FamilyAlignmentName),
                            "duration" => Ok(GeneratedField::Duration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbstractWindowType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.AbstractWindowType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AbstractWindowType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignment_name__ = None;
                let mut duration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignmentName => {
                            if family_alignment_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentName"));
                            }
                            family_alignment_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AbstractWindowType {
                    family_alignment_name: family_alignment_name__.unwrap_or_default(),
                    duration: duration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.AbstractWindowType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationAlignment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_id.is_empty() {
            len += 1;
        }
        if self.bound != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.AnnotationAlignment", len)?;
        if !self.annotation_id.is_empty() {
            struct_ser.serialize_field("annotationId", &self.annotation_id)?;
        }
        if self.bound != 0 {
            let v = TimeRangeBound::try_from(self.bound)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.bound)))?;
            struct_ser.serialize_field("bound", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationAlignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_id",
            "annotationId",
            "bound",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationId,
            Bound,
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
                            "annotationId" | "annotation_id" => Ok(GeneratedField::AnnotationId),
                            "bound" => Ok(GeneratedField::Bound),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationAlignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.AnnotationAlignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationAlignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_id__ = None;
                let mut bound__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationId => {
                            if annotation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationId"));
                            }
                            annotation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bound => {
                            if bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bound"));
                            }
                            bound__ = Some(map_.next_value::<TimeRangeBound>()? as i32);
                        }
                    }
                }
                Ok(AnnotationAlignment {
                    annotation_id: annotation_id__.unwrap_or_default(),
                    bound: bound__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.AnnotationAlignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AnnotationOccurrence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ANNOTATION_OCCURRENCE_UNSPECIFIED",
            Self::First => "ANNOTATION_OCCURRENCE_FIRST",
            Self::Last => "ANNOTATION_OCCURRENCE_LAST",
            Self::Every => "ANNOTATION_OCCURRENCE_EVERY",
            Self::Nth => "ANNOTATION_OCCURRENCE_NTH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationOccurrence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANNOTATION_OCCURRENCE_UNSPECIFIED",
            "ANNOTATION_OCCURRENCE_FIRST",
            "ANNOTATION_OCCURRENCE_LAST",
            "ANNOTATION_OCCURRENCE_EVERY",
            "ANNOTATION_OCCURRENCE_NTH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationOccurrence;

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
                    "ANNOTATION_OCCURRENCE_UNSPECIFIED" => Ok(AnnotationOccurrence::Unspecified),
                    "ANNOTATION_OCCURRENCE_FIRST" => Ok(AnnotationOccurrence::First),
                    "ANNOTATION_OCCURRENCE_LAST" => Ok(AnnotationOccurrence::Last),
                    "ANNOTATION_OCCURRENCE_EVERY" => Ok(AnnotationOccurrence::Every),
                    "ANNOTATION_OCCURRENCE_NTH" => Ok(AnnotationOccurrence::Nth),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateFamilyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family.is_some() {
            len += 1;
        }
        if self.family_version.is_some() {
            len += 1;
        }
        if !self.family_runs.is_empty() {
            len += 1;
        }
        if !self.family_alignments.is_empty() {
            len += 1;
        }
        if !self.family_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.CreateFamilyRequest", len)?;
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        if let Some(v) = self.family_version.as_ref() {
            struct_ser.serialize_field("familyVersion", v)?;
        }
        if !self.family_runs.is_empty() {
            struct_ser.serialize_field("familyRuns", &self.family_runs)?;
        }
        if !self.family_alignments.is_empty() {
            struct_ser.serialize_field("familyAlignments", &self.family_alignments)?;
        }
        if !self.family_stats.is_empty() {
            struct_ser.serialize_field("familyStats", &self.family_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateFamilyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family",
            "family_version",
            "familyVersion",
            "family_runs",
            "familyRuns",
            "family_alignments",
            "familyAlignments",
            "family_stats",
            "familyStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Family,
            FamilyVersion,
            FamilyRuns,
            FamilyAlignments,
            FamilyStats,
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
                            "family" => Ok(GeneratedField::Family),
                            "familyVersion" | "family_version" => Ok(GeneratedField::FamilyVersion),
                            "familyRuns" | "family_runs" => Ok(GeneratedField::FamilyRuns),
                            "familyAlignments" | "family_alignments" => Ok(GeneratedField::FamilyAlignments),
                            "familyStats" | "family_stats" => Ok(GeneratedField::FamilyStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateFamilyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.CreateFamilyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFamilyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family__ = None;
                let mut family_version__ = None;
                let mut family_runs__ = None;
                let mut family_alignments__ = None;
                let mut family_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyVersion => {
                            if family_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersion"));
                            }
                            family_version__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyRuns => {
                            if family_runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyRuns"));
                            }
                            family_runs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyAlignments => {
                            if family_alignments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignments"));
                            }
                            family_alignments__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStats => {
                            if family_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStats"));
                            }
                            family_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateFamilyRequest {
                    family: family__,
                    family_version: family_version__,
                    family_runs: family_runs__.unwrap_or_default(),
                    family_alignments: family_alignments__.unwrap_or_default(),
                    family_stats: family_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.CreateFamilyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateFamilyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_id.is_empty() {
            len += 1;
        }
        if self.family.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.CreateFamilyResponse", len)?;
        if !self.family_id.is_empty() {
            struct_ser.serialize_field("familyId", &self.family_id)?;
        }
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateFamilyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_id",
            "familyId",
            "family",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyId,
            Family,
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
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "family" => Ok(GeneratedField::Family),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateFamilyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.CreateFamilyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFamilyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_id__ = None;
                let mut family__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyId => {
                            if family_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateFamilyResponse {
                    family_id: family_id__.unwrap_or_default(),
                    family: family__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.CreateFamilyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateFamilyStatRangesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_stat_ids.is_empty() {
            len += 1;
        }
        if !self.family_stat_ranges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.CreateFamilyStatRangesRequest", len)?;
        if !self.family_stat_ids.is_empty() {
            struct_ser.serialize_field("familyStatIds", &self.family_stat_ids)?;
        }
        if !self.family_stat_ranges.is_empty() {
            struct_ser.serialize_field("familyStatRanges", &self.family_stat_ranges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateFamilyStatRangesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat_ids",
            "familyStatIds",
            "family_stat_ranges",
            "familyStatRanges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatIds,
            FamilyStatRanges,
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
                            "familyStatIds" | "family_stat_ids" => Ok(GeneratedField::FamilyStatIds),
                            "familyStatRanges" | "family_stat_ranges" => Ok(GeneratedField::FamilyStatRanges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateFamilyStatRangesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.CreateFamilyStatRangesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFamilyStatRangesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat_ids__ = None;
                let mut family_stat_ranges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatIds => {
                            if family_stat_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatIds"));
                            }
                            family_stat_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStatRanges => {
                            if family_stat_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatRanges"));
                            }
                            family_stat_ranges__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateFamilyStatRangesRequest {
                    family_stat_ids: family_stat_ids__.unwrap_or_default(),
                    family_stat_ranges: family_stat_ranges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.CreateFamilyStatRangesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateFamilyStatRangesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if self.family.is_some() {
            len += 1;
        }
        if !self.family_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.CreateFamilyStatRangesResponse", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        if !self.family_stats.is_empty() {
            struct_ser.serialize_field("familyStats", &self.family_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateFamilyStatRangesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
            "family",
            "family_stats",
            "familyStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
            Family,
            FamilyStats,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "family" => Ok(GeneratedField::Family),
                            "familyStats" | "family_stats" => Ok(GeneratedField::FamilyStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateFamilyStatRangesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.CreateFamilyStatRangesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateFamilyStatRangesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                let mut family__ = None;
                let mut family_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyStats => {
                            if family_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStats"));
                            }
                            family_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateFamilyStatRangesResponse {
                    family_version_id: family_version_id__.unwrap_or_default(),
                    family: family__,
                    family_stats: family_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.CreateFamilyStatRangesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DefaultAnnotationAlignment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.annotation_name.is_empty() {
            len += 1;
        }
        if self.bound != 0 {
            len += 1;
        }
        if self.occurrence != 0 {
            len += 1;
        }
        if self.occurrence_index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.DefaultAnnotationAlignment", len)?;
        if !self.annotation_name.is_empty() {
            struct_ser.serialize_field("annotationName", &self.annotation_name)?;
        }
        if self.bound != 0 {
            let v = TimeRangeBound::try_from(self.bound)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.bound)))?;
            struct_ser.serialize_field("bound", &v)?;
        }
        if self.occurrence != 0 {
            let v = AnnotationOccurrence::try_from(self.occurrence)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.occurrence)))?;
            struct_ser.serialize_field("occurrence", &v)?;
        }
        if self.occurrence_index != 0 {
            struct_ser.serialize_field("occurrenceIndex", &self.occurrence_index)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DefaultAnnotationAlignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "annotation_name",
            "annotationName",
            "bound",
            "occurrence",
            "occurrence_index",
            "occurrenceIndex",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AnnotationName,
            Bound,
            Occurrence,
            OccurrenceIndex,
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
                            "annotationName" | "annotation_name" => Ok(GeneratedField::AnnotationName),
                            "bound" => Ok(GeneratedField::Bound),
                            "occurrence" => Ok(GeneratedField::Occurrence),
                            "occurrenceIndex" | "occurrence_index" => Ok(GeneratedField::OccurrenceIndex),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DefaultAnnotationAlignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.DefaultAnnotationAlignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DefaultAnnotationAlignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut annotation_name__ = None;
                let mut bound__ = None;
                let mut occurrence__ = None;
                let mut occurrence_index__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AnnotationName => {
                            if annotation_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotationName"));
                            }
                            annotation_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Bound => {
                            if bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bound"));
                            }
                            bound__ = Some(map_.next_value::<TimeRangeBound>()? as i32);
                        }
                        GeneratedField::Occurrence => {
                            if occurrence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrence"));
                            }
                            occurrence__ = Some(map_.next_value::<AnnotationOccurrence>()? as i32);
                        }
                        GeneratedField::OccurrenceIndex => {
                            if occurrence_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrenceIndex"));
                            }
                            occurrence_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DefaultAnnotationAlignment {
                    annotation_name: annotation_name__.unwrap_or_default(),
                    bound: bound__.unwrap_or_default(),
                    occurrence: occurrence__.unwrap_or_default(),
                    occurrence_index: occurrence_index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.DefaultAnnotationAlignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportFamilyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ExportFamilyRequest", len)?;
        if let Some(v) = self.family_identifier.as_ref() {
            match v {
                export_family_request::FamilyIdentifier::FamilyId(v) => {
                    struct_ser.serialize_field("familyId", v)?;
                }
                export_family_request::FamilyIdentifier::ClientKey(v) => {
                    struct_ser.serialize_field("clientKey", v)?;
                }
                export_family_request::FamilyIdentifier::FamilyVersionId(v) => {
                    struct_ser.serialize_field("familyVersionId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportFamilyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_id",
            "familyId",
            "client_key",
            "clientKey",
            "family_version_id",
            "familyVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyId,
            ClientKey,
            FamilyVersionId,
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
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportFamilyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ExportFamilyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportFamilyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyId => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(export_family_request::FamilyIdentifier::FamilyId);
                        }
                        GeneratedField::ClientKey => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(export_family_request::FamilyIdentifier::ClientKey);
                        }
                        GeneratedField::FamilyVersionId => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(export_family_request::FamilyIdentifier::FamilyVersionId);
                        }
                    }
                }
                Ok(ExportFamilyRequest {
                    family_identifier: family_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ExportFamilyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportFamilyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exported_family.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ExportFamilyResponse", len)?;
        if !self.exported_family.is_empty() {
            struct_ser.serialize_field("exportedFamily", &self.exported_family)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportFamilyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exported_family",
            "exportedFamily",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExportedFamily,
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
                            "exportedFamily" | "exported_family" => Ok(GeneratedField::ExportedFamily),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportFamilyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ExportFamilyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExportFamilyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exported_family__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExportedFamily => {
                            if exported_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exportedFamily"));
                            }
                            exported_family__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExportFamilyResponse {
                    exported_family: exported_family__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ExportFamilyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Family {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_id.is_empty() {
            len += 1;
        }
        if !self.client_key.is_empty() {
            len += 1;
        }
        if !self.current_version_id.is_empty() {
            len += 1;
        }
        if self.is_archived {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.Family", len)?;
        if !self.family_id.is_empty() {
            struct_ser.serialize_field("familyId", &self.family_id)?;
        }
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        if !self.current_version_id.is_empty() {
            struct_ser.serialize_field("currentVersionId", &self.current_version_id)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
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
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Family {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_id",
            "familyId",
            "client_key",
            "clientKey",
            "current_version_id",
            "currentVersionId",
            "is_archived",
            "isArchived",
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
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyId,
            ClientKey,
            CurrentVersionId,
            IsArchived,
            CreatedDate,
            ModifiedDate,
            ArchivedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            "currentVersionId" | "current_version_id" => Ok(GeneratedField::CurrentVersionId),
                            "isArchived" | "is_archived" => Ok(GeneratedField::IsArchived),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
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
            type Value = Family;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.Family")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Family, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_id__ = None;
                let mut client_key__ = None;
                let mut current_version_id__ = None;
                let mut is_archived__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut archived_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyId => {
                            if family_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CurrentVersionId => {
                            if current_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentVersionId"));
                            }
                            current_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Family {
                    family_id: family_id__.unwrap_or_default(),
                    client_key: client_key__.unwrap_or_default(),
                    current_version_id: current_version_id__.unwrap_or_default(),
                    is_archived: is_archived__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    archived_date: archived_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.Family", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyAlignment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignment_id.is_empty() {
            len += 1;
        }
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.family_alignment_points.is_empty() {
            len += 1;
        }
        if self.default_alignment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyAlignment", len)?;
        if !self.family_alignment_id.is_empty() {
            struct_ser.serialize_field("familyAlignmentId", &self.family_alignment_id)?;
        }
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.family_alignment_points.is_empty() {
            struct_ser.serialize_field("familyAlignmentPoints", &self.family_alignment_points)?;
        }
        if let Some(v) = self.default_alignment.as_ref() {
            match v {
                family_alignment::DefaultAlignment::Run(v) => {
                    struct_ser.serialize_field("run", v)?;
                }
                family_alignment::DefaultAlignment::Annotation(v) => {
                    struct_ser.serialize_field("annotation", v)?;
                }
                family_alignment::DefaultAlignment::Timestamp(v) => {
                    struct_ser.serialize_field("timestamp", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyAlignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignment_id",
            "familyAlignmentId",
            "family_version_id",
            "familyVersionId",
            "name",
            "description",
            "family_alignment_points",
            "familyAlignmentPoints",
            "run",
            "annotation",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignmentId,
            FamilyVersionId,
            Name,
            Description,
            FamilyAlignmentPoints,
            Run,
            Annotation,
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
                            "familyAlignmentId" | "family_alignment_id" => Ok(GeneratedField::FamilyAlignmentId),
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "familyAlignmentPoints" | "family_alignment_points" => Ok(GeneratedField::FamilyAlignmentPoints),
                            "run" => Ok(GeneratedField::Run),
                            "annotation" => Ok(GeneratedField::Annotation),
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
            type Value = FamilyAlignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyAlignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyAlignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignment_id__ = None;
                let mut family_version_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut family_alignment_points__ = None;
                let mut default_alignment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignmentId => {
                            if family_alignment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentId"));
                            }
                            family_alignment_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyAlignmentPoints => {
                            if family_alignment_points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentPoints"));
                            }
                            family_alignment_points__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Run => {
                            if default_alignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            default_alignment__ = map_.next_value::<::std::option::Option<_>>()?.map(family_alignment::DefaultAlignment::Run)
;
                        }
                        GeneratedField::Annotation => {
                            if default_alignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            default_alignment__ = map_.next_value::<::std::option::Option<_>>()?.map(family_alignment::DefaultAlignment::Annotation)
;
                        }
                        GeneratedField::Timestamp => {
                            if default_alignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            default_alignment__ = map_.next_value::<::std::option::Option<_>>()?.map(family_alignment::DefaultAlignment::Timestamp)
;
                        }
                    }
                }
                Ok(FamilyAlignment {
                    family_alignment_id: family_alignment_id__.unwrap_or_default(),
                    family_version_id: family_version_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    family_alignment_points: family_alignment_points__.unwrap_or_default(),
                    default_alignment: default_alignment__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyAlignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyAlignmentPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignment_point_id.is_empty() {
            len += 1;
        }
        if !self.family_run_id.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        if !self.family_alignment_id.is_empty() {
            len += 1;
        }
        if self.alignment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyAlignmentPoint", len)?;
        if !self.family_alignment_point_id.is_empty() {
            struct_ser.serialize_field("familyAlignmentPointId", &self.family_alignment_point_id)?;
        }
        if !self.family_run_id.is_empty() {
            struct_ser.serialize_field("familyRunId", &self.family_run_id)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if !self.family_alignment_id.is_empty() {
            struct_ser.serialize_field("familyAlignmentId", &self.family_alignment_id)?;
        }
        if let Some(v) = self.alignment.as_ref() {
            match v {
                family_alignment_point::Alignment::Run(v) => {
                    struct_ser.serialize_field("run", v)?;
                }
                family_alignment_point::Alignment::Annotation(v) => {
                    struct_ser.serialize_field("annotation", v)?;
                }
                family_alignment_point::Alignment::Timestamp(v) => {
                    struct_ser.serialize_field("timestamp", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyAlignmentPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignment_point_id",
            "familyAlignmentPointId",
            "family_run_id",
            "familyRunId",
            "run_id",
            "runId",
            "family_alignment_id",
            "familyAlignmentId",
            "run",
            "annotation",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignmentPointId,
            FamilyRunId,
            RunId,
            FamilyAlignmentId,
            Run,
            Annotation,
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
                            "familyAlignmentPointId" | "family_alignment_point_id" => Ok(GeneratedField::FamilyAlignmentPointId),
                            "familyRunId" | "family_run_id" => Ok(GeneratedField::FamilyRunId),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "familyAlignmentId" | "family_alignment_id" => Ok(GeneratedField::FamilyAlignmentId),
                            "run" => Ok(GeneratedField::Run),
                            "annotation" => Ok(GeneratedField::Annotation),
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
            type Value = FamilyAlignmentPoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyAlignmentPoint")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyAlignmentPoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignment_point_id__ = None;
                let mut family_run_id__ = None;
                let mut run_id__ = None;
                let mut family_alignment_id__ = None;
                let mut alignment__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignmentPointId => {
                            if family_alignment_point_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentPointId"));
                            }
                            family_alignment_point_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyRunId => {
                            if family_run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyRunId"));
                            }
                            family_run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyAlignmentId => {
                            if family_alignment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentId"));
                            }
                            family_alignment_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Run => {
                            if alignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            alignment__ = map_.next_value::<::std::option::Option<_>>()?.map(family_alignment_point::Alignment::Run)
;
                        }
                        GeneratedField::Annotation => {
                            if alignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotation"));
                            }
                            alignment__ = map_.next_value::<::std::option::Option<_>>()?.map(family_alignment_point::Alignment::Annotation)
;
                        }
                        GeneratedField::Timestamp => {
                            if alignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            alignment__ = map_.next_value::<::std::option::Option<_>>()?.map(family_alignment_point::Alignment::Timestamp)
;
                        }
                    }
                }
                Ok(FamilyAlignmentPoint {
                    family_alignment_point_id: family_alignment_point_id__.unwrap_or_default(),
                    family_run_id: family_run_id__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    family_alignment_id: family_alignment_id__.unwrap_or_default(),
                    alignment: alignment__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyAlignmentPoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family.is_some() {
            len += 1;
        }
        if self.family_version.is_some() {
            len += 1;
        }
        if !self.family_runs.is_empty() {
            len += 1;
        }
        if !self.family_alignments.is_empty() {
            len += 1;
        }
        if !self.family_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyDetails", len)?;
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        if let Some(v) = self.family_version.as_ref() {
            struct_ser.serialize_field("familyVersion", v)?;
        }
        if !self.family_runs.is_empty() {
            struct_ser.serialize_field("familyRuns", &self.family_runs)?;
        }
        if !self.family_alignments.is_empty() {
            struct_ser.serialize_field("familyAlignments", &self.family_alignments)?;
        }
        if !self.family_stats.is_empty() {
            struct_ser.serialize_field("familyStats", &self.family_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family",
            "family_version",
            "familyVersion",
            "family_runs",
            "familyRuns",
            "family_alignments",
            "familyAlignments",
            "family_stats",
            "familyStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Family,
            FamilyVersion,
            FamilyRuns,
            FamilyAlignments,
            FamilyStats,
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
                            "family" => Ok(GeneratedField::Family),
                            "familyVersion" | "family_version" => Ok(GeneratedField::FamilyVersion),
                            "familyRuns" | "family_runs" => Ok(GeneratedField::FamilyRuns),
                            "familyAlignments" | "family_alignments" => Ok(GeneratedField::FamilyAlignments),
                            "familyStats" | "family_stats" => Ok(GeneratedField::FamilyStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FamilyDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family__ = None;
                let mut family_version__ = None;
                let mut family_runs__ = None;
                let mut family_alignments__ = None;
                let mut family_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyVersion => {
                            if family_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersion"));
                            }
                            family_version__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyRuns => {
                            if family_runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyRuns"));
                            }
                            family_runs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyAlignments => {
                            if family_alignments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignments"));
                            }
                            family_alignments__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStats => {
                            if family_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStats"));
                            }
                            family_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FamilyDetails {
                    family: family__,
                    family_version: family_version__,
                    family_runs: family_runs__.unwrap_or_default(),
                    family_alignments: family_alignments__.unwrap_or_default(),
                    family_stats: family_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyRuleDependency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.stat_reference.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyRuleDependency", len)?;
        if !self.rule_id.is_empty() {
            struct_ser.serialize_field("ruleId", &self.rule_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.stat_reference.as_ref() {
            struct_ser.serialize_field("statReference", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyRuleDependency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_id",
            "ruleId",
            "name",
            "stat_reference",
            "statReference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleId,
            Name,
            StatReference,
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
                            "ruleId" | "rule_id" => Ok(GeneratedField::RuleId),
                            "name" => Ok(GeneratedField::Name),
                            "statReference" | "stat_reference" => Ok(GeneratedField::StatReference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FamilyRuleDependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyRuleDependency")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyRuleDependency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_id__ = None;
                let mut name__ = None;
                let mut stat_reference__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleId => {
                            if rule_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleId"));
                            }
                            rule_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StatReference => {
                            if stat_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statReference"));
                            }
                            stat_reference__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FamilyRuleDependency {
                    rule_id: rule_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    stat_reference: stat_reference__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyRuleDependency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyRun {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_run_id.is_empty() {
            len += 1;
        }
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        if self.is_exclusion {
            len += 1;
        }
        if !self.rationale.is_empty() {
            len += 1;
        }
        if self.added_date.is_some() {
            len += 1;
        }
        if !self.added_by_user_id.is_empty() {
            len += 1;
        }
        if !self.added_in_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyRun", len)?;
        if !self.family_run_id.is_empty() {
            struct_ser.serialize_field("familyRunId", &self.family_run_id)?;
        }
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        if self.is_exclusion {
            struct_ser.serialize_field("isExclusion", &self.is_exclusion)?;
        }
        if !self.rationale.is_empty() {
            struct_ser.serialize_field("rationale", &self.rationale)?;
        }
        if let Some(v) = self.added_date.as_ref() {
            struct_ser.serialize_field("addedDate", v)?;
        }
        if !self.added_by_user_id.is_empty() {
            struct_ser.serialize_field("addedByUserId", &self.added_by_user_id)?;
        }
        if !self.added_in_version_id.is_empty() {
            struct_ser.serialize_field("addedInVersionId", &self.added_in_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyRun {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_run_id",
            "familyRunId",
            "family_version_id",
            "familyVersionId",
            "run_id",
            "runId",
            "is_exclusion",
            "isExclusion",
            "rationale",
            "added_date",
            "addedDate",
            "added_by_user_id",
            "addedByUserId",
            "added_in_version_id",
            "addedInVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyRunId,
            FamilyVersionId,
            RunId,
            IsExclusion,
            Rationale,
            AddedDate,
            AddedByUserId,
            AddedInVersionId,
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
                            "familyRunId" | "family_run_id" => Ok(GeneratedField::FamilyRunId),
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "isExclusion" | "is_exclusion" => Ok(GeneratedField::IsExclusion),
                            "rationale" => Ok(GeneratedField::Rationale),
                            "addedDate" | "added_date" => Ok(GeneratedField::AddedDate),
                            "addedByUserId" | "added_by_user_id" => Ok(GeneratedField::AddedByUserId),
                            "addedInVersionId" | "added_in_version_id" => Ok(GeneratedField::AddedInVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FamilyRun;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyRun")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyRun, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_run_id__ = None;
                let mut family_version_id__ = None;
                let mut run_id__ = None;
                let mut is_exclusion__ = None;
                let mut rationale__ = None;
                let mut added_date__ = None;
                let mut added_by_user_id__ = None;
                let mut added_in_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyRunId => {
                            if family_run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyRunId"));
                            }
                            family_run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsExclusion => {
                            if is_exclusion__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isExclusion"));
                            }
                            is_exclusion__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rationale => {
                            if rationale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rationale"));
                            }
                            rationale__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddedDate => {
                            if added_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addedDate"));
                            }
                            added_date__ = map_.next_value()?;
                        }
                        GeneratedField::AddedByUserId => {
                            if added_by_user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addedByUserId"));
                            }
                            added_by_user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AddedInVersionId => {
                            if added_in_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addedInVersionId"));
                            }
                            added_in_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FamilyRun {
                    family_run_id: family_run_id__.unwrap_or_default(),
                    family_version_id: family_version_id__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                    is_exclusion: is_exclusion__.unwrap_or_default(),
                    rationale: rationale__.unwrap_or_default(),
                    added_date: added_date__,
                    added_by_user_id: added_by_user_id__.unwrap_or_default(),
                    added_in_version_id: added_in_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyRun", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyStat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_stat_id.is_empty() {
            len += 1;
        }
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.sample_rate != 0. {
            len += 1;
        }
        if !self.default_channel_names.is_empty() {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyStat", len)?;
        if !self.family_stat_id.is_empty() {
            struct_ser.serialize_field("familyStatId", &self.family_stat_id)?;
        }
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.sample_rate != 0. {
            struct_ser.serialize_field("sampleRate", &self.sample_rate)?;
        }
        if !self.default_channel_names.is_empty() {
            struct_ser.serialize_field("defaultChannelNames", &self.default_channel_names)?;
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
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyStat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat_id",
            "familyStatId",
            "family_version_id",
            "familyVersionId",
            "name",
            "description",
            "sample_rate",
            "sampleRate",
            "default_channel_names",
            "defaultChannelNames",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatId,
            FamilyVersionId,
            Name,
            Description,
            SampleRate,
            DefaultChannelNames,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "familyStatId" | "family_stat_id" => Ok(GeneratedField::FamilyStatId),
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "sampleRate" | "sample_rate" => Ok(GeneratedField::SampleRate),
                            "defaultChannelNames" | "default_channel_names" => Ok(GeneratedField::DefaultChannelNames),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
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
            type Value = FamilyStat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyStat")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyStat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat_id__ = None;
                let mut family_version_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut sample_rate__ = None;
                let mut default_channel_names__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatId => {
                            if family_stat_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatId"));
                            }
                            family_stat_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SampleRate => {
                            if sample_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampleRate"));
                            }
                            sample_rate__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DefaultChannelNames => {
                            if default_channel_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultChannelNames"));
                            }
                            default_channel_names__ = Some(map_.next_value()?);
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FamilyStat {
                    family_stat_id: family_stat_id__.unwrap_or_default(),
                    family_version_id: family_version_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    sample_rate: sample_rate__.unwrap_or_default(),
                    default_channel_names: default_channel_names__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyStat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyStatChannel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_stat_channel_id.is_empty() {
            len += 1;
        }
        if !self.family_stat_id.is_empty() {
            len += 1;
        }
        if !self.channel_id.is_empty() {
            len += 1;
        }
        if !self.alignment_point_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyStatChannel", len)?;
        if !self.family_stat_channel_id.is_empty() {
            struct_ser.serialize_field("familyStatChannelId", &self.family_stat_channel_id)?;
        }
        if !self.family_stat_id.is_empty() {
            struct_ser.serialize_field("familyStatId", &self.family_stat_id)?;
        }
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        if !self.alignment_point_id.is_empty() {
            struct_ser.serialize_field("alignmentPointId", &self.alignment_point_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyStatChannel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat_channel_id",
            "familyStatChannelId",
            "family_stat_id",
            "familyStatId",
            "channel_id",
            "channelId",
            "alignment_point_id",
            "alignmentPointId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatChannelId,
            FamilyStatId,
            ChannelId,
            AlignmentPointId,
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
                            "familyStatChannelId" | "family_stat_channel_id" => Ok(GeneratedField::FamilyStatChannelId),
                            "familyStatId" | "family_stat_id" => Ok(GeneratedField::FamilyStatId),
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            "alignmentPointId" | "alignment_point_id" => Ok(GeneratedField::AlignmentPointId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FamilyStatChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyStatChannel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyStatChannel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat_channel_id__ = None;
                let mut family_stat_id__ = None;
                let mut channel_id__ = None;
                let mut alignment_point_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatChannelId => {
                            if family_stat_channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatChannelId"));
                            }
                            family_stat_channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStatId => {
                            if family_stat_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatId"));
                            }
                            family_stat_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AlignmentPointId => {
                            if alignment_point_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alignmentPointId"));
                            }
                            alignment_point_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FamilyStatChannel {
                    family_stat_channel_id: family_stat_channel_id__.unwrap_or_default(),
                    family_stat_id: family_stat_id__.unwrap_or_default(),
                    channel_id: channel_id__.unwrap_or_default(),
                    alignment_point_id: alignment_point_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyStatChannel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyStatDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family_stat.is_some() {
            len += 1;
        }
        if !self.family_stat_expressions.is_empty() {
            len += 1;
        }
        if !self.family_stat_channels.is_empty() {
            len += 1;
        }
        if !self.family_stat_ranges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyStatDetails", len)?;
        if let Some(v) = self.family_stat.as_ref() {
            struct_ser.serialize_field("familyStat", v)?;
        }
        if !self.family_stat_expressions.is_empty() {
            struct_ser.serialize_field("familyStatExpressions", &self.family_stat_expressions)?;
        }
        if !self.family_stat_channels.is_empty() {
            struct_ser.serialize_field("familyStatChannels", &self.family_stat_channels)?;
        }
        if !self.family_stat_ranges.is_empty() {
            struct_ser.serialize_field("familyStatRanges", &self.family_stat_ranges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyStatDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat",
            "familyStat",
            "family_stat_expressions",
            "familyStatExpressions",
            "family_stat_channels",
            "familyStatChannels",
            "family_stat_ranges",
            "familyStatRanges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStat,
            FamilyStatExpressions,
            FamilyStatChannels,
            FamilyStatRanges,
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
                            "familyStat" | "family_stat" => Ok(GeneratedField::FamilyStat),
                            "familyStatExpressions" | "family_stat_expressions" => Ok(GeneratedField::FamilyStatExpressions),
                            "familyStatChannels" | "family_stat_channels" => Ok(GeneratedField::FamilyStatChannels),
                            "familyStatRanges" | "family_stat_ranges" => Ok(GeneratedField::FamilyStatRanges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FamilyStatDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyStatDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyStatDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat__ = None;
                let mut family_stat_expressions__ = None;
                let mut family_stat_channels__ = None;
                let mut family_stat_ranges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStat => {
                            if family_stat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStat"));
                            }
                            family_stat__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyStatExpressions => {
                            if family_stat_expressions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatExpressions"));
                            }
                            family_stat_expressions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStatChannels => {
                            if family_stat_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatChannels"));
                            }
                            family_stat_channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStatRanges => {
                            if family_stat_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatRanges"));
                            }
                            family_stat_ranges__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FamilyStatDetails {
                    family_stat: family_stat__,
                    family_stat_expressions: family_stat_expressions__.unwrap_or_default(),
                    family_stat_channels: family_stat_channels__.unwrap_or_default(),
                    family_stat_ranges: family_stat_ranges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyStatDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyStatExpression {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_stat_expression_id.is_empty() {
            len += 1;
        }
        if !self.family_stat_id.is_empty() {
            len += 1;
        }
        if self.expression.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyStatExpression", len)?;
        if !self.family_stat_expression_id.is_empty() {
            struct_ser.serialize_field("familyStatExpressionId", &self.family_stat_expression_id)?;
        }
        if !self.family_stat_id.is_empty() {
            struct_ser.serialize_field("familyStatId", &self.family_stat_id)?;
        }
        if let Some(v) = self.expression.as_ref() {
            match v {
                family_stat_expression::Expression::Avg(v) => {
                    struct_ser.serialize_field("avg", v)?;
                }
                family_stat_expression::Expression::Median(v) => {
                    struct_ser.serialize_field("median", v)?;
                }
                family_stat_expression::Expression::Min(v) => {
                    struct_ser.serialize_field("min", v)?;
                }
                family_stat_expression::Expression::Max(v) => {
                    struct_ser.serialize_field("max", v)?;
                }
                family_stat_expression::Expression::Stdev(v) => {
                    struct_ser.serialize_field("stdev", v)?;
                }
                family_stat_expression::Expression::Sum(v) => {
                    struct_ser.serialize_field("sum", v)?;
                }
                family_stat_expression::Expression::InputCount(v) => {
                    struct_ser.serialize_field("inputCount", v)?;
                }
                family_stat_expression::Expression::Sigma(v) => {
                    struct_ser.serialize_field("sigma", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyStatExpression {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat_expression_id",
            "familyStatExpressionId",
            "family_stat_id",
            "familyStatId",
            "avg",
            "median",
            "min",
            "max",
            "stdev",
            "sum",
            "input_count",
            "inputCount",
            "sigma",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatExpressionId,
            FamilyStatId,
            Avg,
            Median,
            Min,
            Max,
            Stdev,
            Sum,
            InputCount,
            Sigma,
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
                            "familyStatExpressionId" | "family_stat_expression_id" => Ok(GeneratedField::FamilyStatExpressionId),
                            "familyStatId" | "family_stat_id" => Ok(GeneratedField::FamilyStatId),
                            "avg" => Ok(GeneratedField::Avg),
                            "median" => Ok(GeneratedField::Median),
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            "stdev" => Ok(GeneratedField::Stdev),
                            "sum" => Ok(GeneratedField::Sum),
                            "inputCount" | "input_count" => Ok(GeneratedField::InputCount),
                            "sigma" => Ok(GeneratedField::Sigma),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FamilyStatExpression;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyStatExpression")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyStatExpression, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat_expression_id__ = None;
                let mut family_stat_id__ = None;
                let mut expression__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatExpressionId => {
                            if family_stat_expression_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatExpressionId"));
                            }
                            family_stat_expression_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStatId => {
                            if family_stat_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatId"));
                            }
                            family_stat_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Avg => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::Avg)
;
                        }
                        GeneratedField::Median => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("median"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::Median)
;
                        }
                        GeneratedField::Min => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::Min)
;
                        }
                        GeneratedField::Max => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::Max)
;
                        }
                        GeneratedField::Stdev => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdev"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::Stdev)
;
                        }
                        GeneratedField::Sum => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::Sum)
;
                        }
                        GeneratedField::InputCount => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputCount"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::InputCount)
;
                        }
                        GeneratedField::Sigma => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sigma"));
                            }
                            expression__ = map_.next_value::<::std::option::Option<_>>()?.map(family_stat_expression::Expression::Sigma)
;
                        }
                    }
                }
                Ok(FamilyStatExpression {
                    family_stat_expression_id: family_stat_expression_id__.unwrap_or_default(),
                    family_stat_id: family_stat_id__.unwrap_or_default(),
                    expression: expression__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyStatExpression", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyStatRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_stat_range_id.is_empty() {
            len += 1;
        }
        if !self.family_stat_id.is_empty() {
            len += 1;
        }
        if !self.family_alignment_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.window_start.is_some() {
            len += 1;
        }
        if self.window_end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyStatRange", len)?;
        if !self.family_stat_range_id.is_empty() {
            struct_ser.serialize_field("familyStatRangeId", &self.family_stat_range_id)?;
        }
        if !self.family_stat_id.is_empty() {
            struct_ser.serialize_field("familyStatId", &self.family_stat_id)?;
        }
        if !self.family_alignment_id.is_empty() {
            struct_ser.serialize_field("familyAlignmentId", &self.family_alignment_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.window_start.as_ref() {
            struct_ser.serialize_field("windowStart", v)?;
        }
        if let Some(v) = self.window_end.as_ref() {
            struct_ser.serialize_field("windowEnd", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyStatRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat_range_id",
            "familyStatRangeId",
            "family_stat_id",
            "familyStatId",
            "family_alignment_id",
            "familyAlignmentId",
            "name",
            "window_start",
            "windowStart",
            "window_end",
            "windowEnd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatRangeId,
            FamilyStatId,
            FamilyAlignmentId,
            Name,
            WindowStart,
            WindowEnd,
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
                            "familyStatRangeId" | "family_stat_range_id" => Ok(GeneratedField::FamilyStatRangeId),
                            "familyStatId" | "family_stat_id" => Ok(GeneratedField::FamilyStatId),
                            "familyAlignmentId" | "family_alignment_id" => Ok(GeneratedField::FamilyAlignmentId),
                            "name" => Ok(GeneratedField::Name),
                            "windowStart" | "window_start" => Ok(GeneratedField::WindowStart),
                            "windowEnd" | "window_end" => Ok(GeneratedField::WindowEnd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FamilyStatRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyStatRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyStatRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat_range_id__ = None;
                let mut family_stat_id__ = None;
                let mut family_alignment_id__ = None;
                let mut name__ = None;
                let mut window_start__ = None;
                let mut window_end__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatRangeId => {
                            if family_stat_range_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatRangeId"));
                            }
                            family_stat_range_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyStatId => {
                            if family_stat_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatId"));
                            }
                            family_stat_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyAlignmentId => {
                            if family_alignment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentId"));
                            }
                            family_alignment_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WindowStart => {
                            if window_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowStart"));
                            }
                            window_start__ = map_.next_value()?;
                        }
                        GeneratedField::WindowEnd => {
                            if window_end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowEnd"));
                            }
                            window_end__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FamilyStatRange {
                    family_stat_range_id: family_stat_range_id__.unwrap_or_default(),
                    family_stat_id: family_stat_id__.unwrap_or_default(),
                    family_alignment_id: family_alignment_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    window_start: window_start__,
                    window_end: window_end__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyStatRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FamilyVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if !self.family_id.is_empty() {
            len += 1;
        }
        if self.version != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.run_query.is_empty() {
            len += 1;
        }
        if !self.user_notes.is_empty() {
            len += 1;
        }
        if !self.change_message.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.sigma_default_values.is_empty() {
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
        if !self.organization_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.FamilyVersion", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if !self.family_id.is_empty() {
            struct_ser.serialize_field("familyId", &self.family_id)?;
        }
        if self.version != 0 {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.run_query.is_empty() {
            struct_ser.serialize_field("runQuery", &self.run_query)?;
        }
        if !self.user_notes.is_empty() {
            struct_ser.serialize_field("userNotes", &self.user_notes)?;
        }
        if !self.change_message.is_empty() {
            struct_ser.serialize_field("changeMessage", &self.change_message)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.sigma_default_values.is_empty() {
            struct_ser.serialize_field("sigmaDefaultValues", &self.sigma_default_values)?;
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
        if !self.organization_id.is_empty() {
            struct_ser.serialize_field("organizationId", &self.organization_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FamilyVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
            "family_id",
            "familyId",
            "version",
            "name",
            "description",
            "run_query",
            "runQuery",
            "user_notes",
            "userNotes",
            "change_message",
            "changeMessage",
            "metadata",
            "sigma_default_values",
            "sigmaDefaultValues",
            "created_date",
            "createdDate",
            "modified_date",
            "modifiedDate",
            "created_by_user_id",
            "createdByUserId",
            "modified_by_user_id",
            "modifiedByUserId",
            "organization_id",
            "organizationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
            FamilyId,
            Version,
            Name,
            Description,
            RunQuery,
            UserNotes,
            ChangeMessage,
            Metadata,
            SigmaDefaultValues,
            CreatedDate,
            ModifiedDate,
            CreatedByUserId,
            ModifiedByUserId,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "version" => Ok(GeneratedField::Version),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "runQuery" | "run_query" => Ok(GeneratedField::RunQuery),
                            "userNotes" | "user_notes" => Ok(GeneratedField::UserNotes),
                            "changeMessage" | "change_message" => Ok(GeneratedField::ChangeMessage),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "sigmaDefaultValues" | "sigma_default_values" => Ok(GeneratedField::SigmaDefaultValues),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
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
            type Value = FamilyVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.FamilyVersion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FamilyVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                let mut family_id__ = None;
                let mut version__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut run_query__ = None;
                let mut user_notes__ = None;
                let mut change_message__ = None;
                let mut metadata__ = None;
                let mut sigma_default_values__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut organization_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FamilyId => {
                            if family_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunQuery => {
                            if run_query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runQuery"));
                            }
                            run_query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserNotes => {
                            if user_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNotes"));
                            }
                            user_notes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ChangeMessage => {
                            if change_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changeMessage"));
                            }
                            change_message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SigmaDefaultValues => {
                            if sigma_default_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sigmaDefaultValues"));
                            }
                            sigma_default_values__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
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
                        GeneratedField::OrganizationId => {
                            if organization_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("organizationId"));
                            }
                            organization_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FamilyVersion {
                    family_version_id: family_version_id__.unwrap_or_default(),
                    family_id: family_id__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    run_query: run_query__.unwrap_or_default(),
                    user_notes: user_notes__.unwrap_or_default(),
                    change_message: change_message__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    sigma_default_values: sigma_default_values__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    organization_id: organization_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.FamilyVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateFamilyStatRangeNameRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignment_name.is_empty() {
            len += 1;
        }
        if self.window_start.is_some() {
            len += 1;
        }
        if self.window_end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GenerateFamilyStatRangeNameRequest", len)?;
        if !self.family_alignment_name.is_empty() {
            struct_ser.serialize_field("familyAlignmentName", &self.family_alignment_name)?;
        }
        if let Some(v) = self.window_start.as_ref() {
            struct_ser.serialize_field("windowStart", v)?;
        }
        if let Some(v) = self.window_end.as_ref() {
            struct_ser.serialize_field("windowEnd", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateFamilyStatRangeNameRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignment_name",
            "familyAlignmentName",
            "window_start",
            "windowStart",
            "window_end",
            "windowEnd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignmentName,
            WindowStart,
            WindowEnd,
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
                            "familyAlignmentName" | "family_alignment_name" => Ok(GeneratedField::FamilyAlignmentName),
                            "windowStart" | "window_start" => Ok(GeneratedField::WindowStart),
                            "windowEnd" | "window_end" => Ok(GeneratedField::WindowEnd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateFamilyStatRangeNameRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GenerateFamilyStatRangeNameRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateFamilyStatRangeNameRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignment_name__ = None;
                let mut window_start__ = None;
                let mut window_end__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignmentName => {
                            if family_alignment_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentName"));
                            }
                            family_alignment_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WindowStart => {
                            if window_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowStart"));
                            }
                            window_start__ = map_.next_value()?;
                        }
                        GeneratedField::WindowEnd => {
                            if window_end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowEnd"));
                            }
                            window_end__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GenerateFamilyStatRangeNameRequest {
                    family_alignment_name: family_alignment_name__.unwrap_or_default(),
                    window_start: window_start__,
                    window_end: window_end__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GenerateFamilyStatRangeNameRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateFamilyStatRangeNameResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GenerateFamilyStatRangeNameResponse", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateFamilyStatRangeNameResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateFamilyStatRangeNameResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GenerateFamilyStatRangeNameResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateFamilyStatRangeNameResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateFamilyStatRangeNameResponse {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GenerateFamilyStatRangeNameResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamiliesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_ids.is_empty() {
            len += 1;
        }
        if !self.client_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamiliesRequest", len)?;
        if !self.family_ids.is_empty() {
            struct_ser.serialize_field("familyIds", &self.family_ids)?;
        }
        if !self.client_keys.is_empty() {
            struct_ser.serialize_field("clientKeys", &self.client_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamiliesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_ids",
            "familyIds",
            "client_keys",
            "clientKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyIds,
            ClientKeys,
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
                            "familyIds" | "family_ids" => Ok(GeneratedField::FamilyIds),
                            "clientKeys" | "client_keys" => Ok(GeneratedField::ClientKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamiliesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamiliesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamiliesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_ids__ = None;
                let mut client_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyIds => {
                            if family_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyIds"));
                            }
                            family_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientKeys => {
                            if client_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKeys"));
                            }
                            client_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamiliesRequest {
                    family_ids: family_ids__.unwrap_or_default(),
                    client_keys: client_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamiliesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamiliesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.families.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamiliesResponse", len)?;
        if !self.families.is_empty() {
            struct_ser.serialize_field("families", &self.families)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamiliesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "families",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Families,
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
                            "families" => Ok(GeneratedField::Families),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamiliesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamiliesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamiliesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut families__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Families => {
                            if families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("families"));
                            }
                            families__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamiliesResponse {
                    families: families__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamiliesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyCandidateRunCountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyCandidateRunCountRequest", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyCandidateRunCountRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyCandidateRunCountRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyCandidateRunCountRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyCandidateRunCountRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyCandidateRunCountRequest {
                    family_version_id: family_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyCandidateRunCountRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyCandidateRunCountResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyCandidateRunCountResponse", len)?;
        if self.count != 0 {
            struct_ser.serialize_field("count", &self.count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyCandidateRunCountResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "count",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Count,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyCandidateRunCountResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyCandidateRunCountResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyCandidateRunCountResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut count__ = None;
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
                    }
                }
                Ok(GetFamilyCandidateRunCountResponse {
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyCandidateRunCountResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyCandidateRunFilterFieldsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyCandidateRunFilterFieldsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyCandidateRunFilterFieldsRequest {
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
            type Value = GetFamilyCandidateRunFilterFieldsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyCandidateRunFilterFieldsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyCandidateRunFilterFieldsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetFamilyCandidateRunFilterFieldsRequest {
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyCandidateRunFilterFieldsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyCandidateRunFilterFieldsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filter_fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyCandidateRunFilterFieldsResponse", len)?;
        if !self.filter_fields.is_empty() {
            struct_ser.serialize_field("filterFields", &self.filter_fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyCandidateRunFilterFieldsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter_fields",
            "filterFields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilterFields,
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
                            "filterFields" | "filter_fields" => Ok(GeneratedField::FilterFields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyCandidateRunFilterFieldsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyCandidateRunFilterFieldsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyCandidateRunFilterFieldsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilterFields => {
                            if filter_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterFields"));
                            }
                            filter_fields__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyCandidateRunFilterFieldsResponse {
                    filter_fields: filter_fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyCandidateRunFilterFieldsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyCandidateRunsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyCandidateRunsRequest", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyCandidateRunsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyCandidateRunsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyCandidateRunsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyCandidateRunsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyCandidateRunsRequest {
                    family_version_id: family_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyCandidateRunsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyCandidateRunsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.run_query.is_empty() {
            len += 1;
        }
        if !self.candidate_run_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyCandidateRunsResponse", len)?;
        if !self.run_query.is_empty() {
            struct_ser.serialize_field("runQuery", &self.run_query)?;
        }
        if !self.candidate_run_ids.is_empty() {
            struct_ser.serialize_field("candidateRunIds", &self.candidate_run_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyCandidateRunsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_query",
            "runQuery",
            "candidate_run_ids",
            "candidateRunIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunQuery,
            CandidateRunIds,
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
                            "runQuery" | "run_query" => Ok(GeneratedField::RunQuery),
                            "candidateRunIds" | "candidate_run_ids" => Ok(GeneratedField::CandidateRunIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyCandidateRunsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyCandidateRunsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyCandidateRunsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_query__ = None;
                let mut candidate_run_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RunQuery => {
                            if run_query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runQuery"));
                            }
                            run_query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CandidateRunIds => {
                            if candidate_run_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("candidateRunIds"));
                            }
                            candidate_run_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyCandidateRunsResponse {
                    run_query: run_query__.unwrap_or_default(),
                    candidate_run_ids: candidate_run_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyCandidateRunsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyRequest", len)?;
        if let Some(v) = self.family_identifier.as_ref() {
            match v {
                get_family_request::FamilyIdentifier::FamilyId(v) => {
                    struct_ser.serialize_field("familyId", v)?;
                }
                get_family_request::FamilyIdentifier::ClientKey(v) => {
                    struct_ser.serialize_field("clientKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_id",
            "familyId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyId,
            ClientKey,
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
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyId => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_family_request::FamilyIdentifier::FamilyId);
                        }
                        GeneratedField::ClientKey => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(get_family_request::FamilyIdentifier::ClientKey);
                        }
                    }
                }
                Ok(GetFamilyRequest {
                    family_identifier: family_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyResponse", len)?;
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Family,
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
                            "family" => Ok(GeneratedField::Family),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetFamilyResponse {
                    family: family__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyRuleDependenciesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyRuleDependenciesRequest", len)?;
        if !self.family_id.is_empty() {
            struct_ser.serialize_field("familyId", &self.family_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyRuleDependenciesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_id",
            "familyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyId,
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
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyRuleDependenciesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyRuleDependenciesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyRuleDependenciesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyId => {
                            if family_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyRuleDependenciesRequest {
                    family_id: family_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyRuleDependenciesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyRuleDependenciesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rule_dependencies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyRuleDependenciesResponse", len)?;
        if !self.rule_dependencies.is_empty() {
            struct_ser.serialize_field("ruleDependencies", &self.rule_dependencies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyRuleDependenciesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rule_dependencies",
            "ruleDependencies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuleDependencies,
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
                            "ruleDependencies" | "rule_dependencies" => Ok(GeneratedField::RuleDependencies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyRuleDependenciesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyRuleDependenciesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyRuleDependenciesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule_dependencies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RuleDependencies => {
                            if rule_dependencies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ruleDependencies"));
                            }
                            rule_dependencies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyRuleDependenciesResponse {
                    rule_dependencies: rule_dependencies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyRuleDependenciesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyStatRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_stat_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyStatRequest", len)?;
        if !self.family_stat_id.is_empty() {
            struct_ser.serialize_field("familyStatId", &self.family_stat_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyStatRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat_id",
            "familyStatId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatId,
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
                            "familyStatId" | "family_stat_id" => Ok(GeneratedField::FamilyStatId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyStatRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyStatRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyStatRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatId => {
                            if family_stat_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatId"));
                            }
                            family_stat_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyStatRequest {
                    family_stat_id: family_stat_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyStatRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyStatResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family_stat_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyStatResponse", len)?;
        if let Some(v) = self.family_stat_details.as_ref() {
            struct_ser.serialize_field("familyStatDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyStatResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stat_details",
            "familyStatDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatDetails,
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
                            "familyStatDetails" | "family_stat_details" => Ok(GeneratedField::FamilyStatDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyStatResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyStatResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyStatResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stat_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatDetails => {
                            if family_stat_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatDetails"));
                            }
                            family_stat_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetFamilyStatResponse {
                    family_stat_details: family_stat_details__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyStatResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyStatsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyStatsRequest", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyStatsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyStatsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyStatsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyStatsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyStatsRequest {
                    family_version_id: family_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyStatsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyStatsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_stats_details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyStatsResponse", len)?;
        if !self.family_stats_details.is_empty() {
            struct_ser.serialize_field("familyStatsDetails", &self.family_stats_details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyStatsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_stats_details",
            "familyStatsDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyStatsDetails,
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
                            "familyStatsDetails" | "family_stats_details" => Ok(GeneratedField::FamilyStatsDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyStatsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyStatsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyStatsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_stats_details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyStatsDetails => {
                            if family_stats_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStatsDetails"));
                            }
                            family_stats_details__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyStatsResponse {
                    family_stats_details: family_stats_details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyStatsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyVersionRequest", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyVersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetFamilyVersionRequest {
                    family_version_id: family_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetFamilyVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.GetFamilyVersionResponse", len)?;
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetFamilyVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Family,
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
                            "family" => Ok(GeneratedField::Family),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetFamilyVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.GetFamilyVersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetFamilyVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetFamilyVersionResponse {
                    family: family__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.GetFamilyVersionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportFamilyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_yaml.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ImportFamilyRequest", len)?;
        if !self.family_yaml.is_empty() {
            struct_ser.serialize_field("familyYaml", &self.family_yaml)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportFamilyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_yaml",
            "familyYaml",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyYaml,
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
                            "familyYaml" | "family_yaml" => Ok(GeneratedField::FamilyYaml),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportFamilyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ImportFamilyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportFamilyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_yaml__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyYaml => {
                            if family_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyYaml"));
                            }
                            family_yaml__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ImportFamilyRequest {
                    family_yaml: family_yaml__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ImportFamilyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportFamilyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_id.is_empty() {
            len += 1;
        }
        if self.family.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ImportFamilyResponse", len)?;
        if !self.family_id.is_empty() {
            struct_ser.serialize_field("familyId", &self.family_id)?;
        }
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportFamilyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_id",
            "familyId",
            "family",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyId,
            Family,
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
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "family" => Ok(GeneratedField::Family),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportFamilyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ImportFamilyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportFamilyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_id__ = None;
                let mut family__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyId => {
                            if family_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ImportFamilyResponse {
                    family_id: family_id__.unwrap_or_default(),
                    family: family__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ImportFamilyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportUpdateFamilyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_yaml.is_empty() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        if self.family_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ImportUpdateFamilyRequest", len)?;
        if !self.family_yaml.is_empty() {
            struct_ser.serialize_field("familyYaml", &self.family_yaml)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        if let Some(v) = self.family_identifier.as_ref() {
            match v {
                import_update_family_request::FamilyIdentifier::FamilyId(v) => {
                    struct_ser.serialize_field("familyId", v)?;
                }
                import_update_family_request::FamilyIdentifier::ClientKey(v) => {
                    struct_ser.serialize_field("clientKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportUpdateFamilyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_yaml",
            "familyYaml",
            "update_mask",
            "updateMask",
            "family_id",
            "familyId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyYaml,
            UpdateMask,
            FamilyId,
            ClientKey,
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
                            "familyYaml" | "family_yaml" => Ok(GeneratedField::FamilyYaml),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportUpdateFamilyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ImportUpdateFamilyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportUpdateFamilyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_yaml__ = None;
                let mut update_mask__ = None;
                let mut family_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyYaml => {
                            if family_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyYaml"));
                            }
                            family_yaml__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyId => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(import_update_family_request::FamilyIdentifier::FamilyId);
                        }
                        GeneratedField::ClientKey => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(import_update_family_request::FamilyIdentifier::ClientKey);
                        }
                    }
                }
                Ok(ImportUpdateFamilyRequest {
                    family_yaml: family_yaml__.unwrap_or_default(),
                    update_mask: update_mask__,
                    family_identifier: family_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ImportUpdateFamilyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportUpdateFamilyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if self.family.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ImportUpdateFamilyResponse", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportUpdateFamilyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
            "family",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
            Family,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "family" => Ok(GeneratedField::Family),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportUpdateFamilyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ImportUpdateFamilyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportUpdateFamilyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                let mut family__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ImportUpdateFamilyResponse {
                    family_version_id: family_version_id__.unwrap_or_default(),
                    family: family__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ImportUpdateFamilyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamiliesRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamiliesRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListFamiliesRequest {
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
            type Value = ListFamiliesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamiliesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamiliesRequest, V::Error>
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
                Ok(ListFamiliesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamiliesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamiliesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.families.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamiliesResponse", len)?;
        if !self.families.is_empty() {
            struct_ser.serialize_field("families", &self.families)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamiliesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "families",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Families,
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
                            "families" => Ok(GeneratedField::Families),
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
            type Value = ListFamiliesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamiliesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamiliesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut families__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Families => {
                            if families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("families"));
                            }
                            families__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamiliesResponse {
                    families: families__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamiliesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamily {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family.is_some() {
            len += 1;
        }
        if self.family_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamily", len)?;
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        if let Some(v) = self.family_version.as_ref() {
            struct_ser.serialize_field("familyVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamily {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family",
            "family_version",
            "familyVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Family,
            FamilyVersion,
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
                            "family" => Ok(GeneratedField::Family),
                            "familyVersion" | "family_version" => Ok(GeneratedField::FamilyVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFamily;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamily")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamily, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family__ = None;
                let mut family_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyVersion => {
                            if family_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersion"));
                            }
                            family_version__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListFamily {
                    family: family__,
                    family_version: family_version__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamily", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyAlignmentPointsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignment_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyAlignmentPointsRequest", len)?;
        if !self.family_alignment_id.is_empty() {
            struct_ser.serialize_field("familyAlignmentId", &self.family_alignment_id)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyAlignmentPointsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignment_id",
            "familyAlignmentId",
            "page_size",
            "pageSize",
            "filter",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignmentId,
            PageSize,
            Filter,
            PageToken,
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
                            "familyAlignmentId" | "family_alignment_id" => Ok(GeneratedField::FamilyAlignmentId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "filter" => Ok(GeneratedField::Filter),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFamilyAlignmentPointsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyAlignmentPointsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyAlignmentPointsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignment_id__ = None;
                let mut page_size__ = None;
                let mut filter__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignmentId => {
                            if family_alignment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentId"));
                            }
                            family_alignment_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyAlignmentPointsRequest {
                    family_alignment_id: family_alignment_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyAlignmentPointsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyAlignmentPointsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignment_points.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyAlignmentPointsResponse", len)?;
        if !self.family_alignment_points.is_empty() {
            struct_ser.serialize_field("familyAlignmentPoints", &self.family_alignment_points)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyAlignmentPointsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignment_points",
            "familyAlignmentPoints",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignmentPoints,
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
                            "familyAlignmentPoints" | "family_alignment_points" => Ok(GeneratedField::FamilyAlignmentPoints),
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
            type Value = ListFamilyAlignmentPointsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyAlignmentPointsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyAlignmentPointsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignment_points__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignmentPoints => {
                            if family_alignment_points__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentPoints"));
                            }
                            family_alignment_points__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyAlignmentPointsResponse {
                    family_alignment_points: family_alignment_points__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyAlignmentPointsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyAlignmentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyAlignmentsRequest", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyAlignmentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
            "page_size",
            "pageSize",
            "filter",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
            PageSize,
            Filter,
            PageToken,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "filter" => Ok(GeneratedField::Filter),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFamilyAlignmentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyAlignmentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyAlignmentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                let mut page_size__ = None;
                let mut filter__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyAlignmentsRequest {
                    family_version_id: family_version_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyAlignmentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyAlignmentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignments.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyAlignmentsResponse", len)?;
        if !self.family_alignments.is_empty() {
            struct_ser.serialize_field("familyAlignments", &self.family_alignments)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyAlignmentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignments",
            "familyAlignments",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignments,
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
                            "familyAlignments" | "family_alignments" => Ok(GeneratedField::FamilyAlignments),
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
            type Value = ListFamilyAlignmentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyAlignmentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyAlignmentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignments__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignments => {
                            if family_alignments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignments"));
                            }
                            family_alignments__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyAlignmentsResponse {
                    family_alignments: family_alignments__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyAlignmentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyCandidateRunsRequest {
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
        if !self.family_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyCandidateRunsRequest", len)?;
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
        if !self.family_id.is_empty() {
            struct_ser.serialize_field("familyId", &self.family_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyCandidateRunsRequest {
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
            "family_id",
            "familyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrderBy,
            FamilyId,
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
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFamilyCandidateRunsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyCandidateRunsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyCandidateRunsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut family_id__ = None;
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
                        GeneratedField::FamilyId => {
                            if family_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyCandidateRunsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    family_id: family_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyCandidateRunsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyCandidateRunsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.runs.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyCandidateRunsResponse", len)?;
        if !self.runs.is_empty() {
            struct_ser.serialize_field("runs", &self.runs)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyCandidateRunsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "runs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Runs,
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
                            "runs" => Ok(GeneratedField::Runs),
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
            type Value = ListFamilyCandidateRunsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyCandidateRunsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyCandidateRunsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut runs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Runs => {
                            if runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runs"));
                            }
                            runs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyCandidateRunsResponse {
                    runs: runs__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyCandidateRunsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyMembersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyMembersRequest", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyMembersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
            "page_size",
            "pageSize",
            "filter",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
            PageSize,
            Filter,
            PageToken,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "filter" => Ok(GeneratedField::Filter),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListFamilyMembersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyMembersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyMembersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                let mut page_size__ = None;
                let mut filter__ = None;
                let mut page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyMembersRequest {
                    family_version_id: family_version_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyMembersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyMembersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_runs.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyMembersResponse", len)?;
        if !self.family_runs.is_empty() {
            struct_ser.serialize_field("familyRuns", &self.family_runs)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyMembersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_runs",
            "familyRuns",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyRuns,
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
                            "familyRuns" | "family_runs" => Ok(GeneratedField::FamilyRuns),
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
            type Value = ListFamilyMembersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyMembersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyMembersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_runs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyRuns => {
                            if family_runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyRuns"));
                            }
                            family_runs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyMembersResponse {
                    family_runs: family_runs__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyMembersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyVersionsRequest {
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
        if !self.filter.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyVersionsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyVersionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "filter",
            "page_token",
            "pageToken",
            "order_by",
            "orderBy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            Filter,
            PageToken,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
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
            type Value = ListFamilyVersionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyVersionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyVersionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut filter__ = None;
                let mut page_token__ = None;
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
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyVersionsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyVersionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListFamilyVersionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_versions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ListFamilyVersionsResponse", len)?;
        if !self.family_versions.is_empty() {
            struct_ser.serialize_field("familyVersions", &self.family_versions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListFamilyVersionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_versions",
            "familyVersions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersions,
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
                            "familyVersions" | "family_versions" => Ok(GeneratedField::FamilyVersions),
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
            type Value = ListFamilyVersionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ListFamilyVersionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListFamilyVersionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_versions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersions => {
                            if family_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersions"));
                            }
                            family_versions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListFamilyVersionsResponse {
                    family_versions: family_versions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ListFamilyVersionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunAlignment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bound != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.RunAlignment", len)?;
        if self.bound != 0 {
            let v = TimeRangeBound::try_from(self.bound)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.bound)))?;
            struct_ser.serialize_field("bound", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunAlignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bound",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bound,
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
                            "bound" => Ok(GeneratedField::Bound),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunAlignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.RunAlignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RunAlignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bound__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bound => {
                            if bound__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bound"));
                            }
                            bound__ = Some(map_.next_value::<TimeRangeBound>()? as i32);
                        }
                    }
                }
                Ok(RunAlignment {
                    bound: bound__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.RunAlignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeRangeBound {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TIME_RANGE_BOUND_UNSPECIFIED",
            Self::Start => "TIME_RANGE_BOUND_START",
            Self::End => "TIME_RANGE_BOUND_END",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TimeRangeBound {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TIME_RANGE_BOUND_UNSPECIFIED",
            "TIME_RANGE_BOUND_START",
            "TIME_RANGE_BOUND_END",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeRangeBound;

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
                    "TIME_RANGE_BOUND_UNSPECIFIED" => Ok(TimeRangeBound::Unspecified),
                    "TIME_RANGE_BOUND_START" => Ok(TimeRangeBound::Start),
                    "TIME_RANGE_BOUND_END" => Ok(TimeRangeBound::End),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TimestampAlignment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.TimestampAlignment", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimestampAlignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = TimestampAlignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.TimestampAlignment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TimestampAlignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TimestampAlignment {
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.TimestampAlignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateFamilyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.family.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        if self.family_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.UpdateFamilyRequest", len)?;
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        if let Some(v) = self.family_identifier.as_ref() {
            match v {
                update_family_request::FamilyIdentifier::FamilyId(v) => {
                    struct_ser.serialize_field("familyId", v)?;
                }
                update_family_request::FamilyIdentifier::ClientKey(v) => {
                    struct_ser.serialize_field("clientKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateFamilyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family",
            "update_mask",
            "updateMask",
            "family_id",
            "familyId",
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Family,
            UpdateMask,
            FamilyId,
            ClientKey,
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
                            "family" => Ok(GeneratedField::Family),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            "familyId" | "family_id" => Ok(GeneratedField::FamilyId),
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateFamilyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.UpdateFamilyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateFamilyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family__ = None;
                let mut update_mask__ = None;
                let mut family_identifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyId => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyId"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(update_family_request::FamilyIdentifier::FamilyId);
                        }
                        GeneratedField::ClientKey => {
                            if family_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            family_identifier__ = map_.next_value::<::std::option::Option<_>>()?.map(update_family_request::FamilyIdentifier::ClientKey);
                        }
                    }
                }
                Ok(UpdateFamilyRequest {
                    family: family__,
                    update_mask: update_mask__,
                    family_identifier: family_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.UpdateFamilyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateFamilyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_version_id.is_empty() {
            len += 1;
        }
        if self.family.is_some() {
            len += 1;
        }
        if !self.family_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.UpdateFamilyResponse", len)?;
        if !self.family_version_id.is_empty() {
            struct_ser.serialize_field("familyVersionId", &self.family_version_id)?;
        }
        if let Some(v) = self.family.as_ref() {
            struct_ser.serialize_field("family", v)?;
        }
        if !self.family_stats.is_empty() {
            struct_ser.serialize_field("familyStats", &self.family_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateFamilyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_version_id",
            "familyVersionId",
            "family",
            "family_stats",
            "familyStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyVersionId,
            Family,
            FamilyStats,
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
                            "familyVersionId" | "family_version_id" => Ok(GeneratedField::FamilyVersionId),
                            "family" => Ok(GeneratedField::Family),
                            "familyStats" | "family_stats" => Ok(GeneratedField::FamilyStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateFamilyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.UpdateFamilyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateFamilyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_version_id__ = None;
                let mut family__ = None;
                let mut family_stats__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyVersionId => {
                            if family_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyVersionId"));
                            }
                            family_version_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Family => {
                            if family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            family__ = map_.next_value()?;
                        }
                        GeneratedField::FamilyStats => {
                            if family_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyStats"));
                            }
                            family_stats__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateFamilyResponse {
                    family_version_id: family_version_id__.unwrap_or_default(),
                    family: family__,
                    family_stats: family_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.UpdateFamilyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateFamilyClientKeyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ValidateFamilyClientKeyRequest", len)?;
        if !self.client_key.is_empty() {
            struct_ser.serialize_field("clientKey", &self.client_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateFamilyClientKeyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_key",
            "clientKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientKey,
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
                            "clientKey" | "client_key" => Ok(GeneratedField::ClientKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateFamilyClientKeyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ValidateFamilyClientKeyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateFamilyClientKeyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_key__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientKey => {
                            if client_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientKey"));
                            }
                            client_key__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidateFamilyClientKeyRequest {
                    client_key: client_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ValidateFamilyClientKeyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateFamilyClientKeyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ValidateFamilyClientKeyResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            match v {
                validate_family_client_key_response::Result::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                validate_family_client_key_response::Result::ErrorMessage(v) => {
                    struct_ser.serialize_field("errorMessage", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateFamilyClientKeyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
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
                            "success" => Ok(GeneratedField::Success),
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
            type Value = ValidateFamilyClientKeyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ValidateFamilyClientKeyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateFamilyClientKeyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_family_client_key_response::Result::Success);
                        }
                        GeneratedField::ErrorMessage => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_family_client_key_response::Result::ErrorMessage);
                        }
                    }
                }
                Ok(ValidateFamilyClientKeyResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ValidateFamilyClientKeyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateFamilyNameRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ValidateFamilyNameRequest", len)?;
        if !self.family_name.is_empty() {
            struct_ser.serialize_field("familyName", &self.family_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateFamilyNameRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_name",
            "familyName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyName,
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
                            "familyName" | "family_name" => Ok(GeneratedField::FamilyName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateFamilyNameRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ValidateFamilyNameRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateFamilyNameRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyName => {
                            if family_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyName"));
                            }
                            family_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidateFamilyNameRequest {
                    family_name: family_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ValidateFamilyNameRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateFamilyNameResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.ValidateFamilyNameResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            match v {
                validate_family_name_response::Result::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
                validate_family_name_response::Result::ErrorMessage(v) => {
                    struct_ser.serialize_field("errorMessage", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateFamilyNameResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "success",
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Success,
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
                            "success" => Ok(GeneratedField::Success),
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
            type Value = ValidateFamilyNameResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.ValidateFamilyNameResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateFamilyNameResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Success => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_family_name_response::Result::Success);
                        }
                        GeneratedField::ErrorMessage => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_family_name_response::Result::ErrorMessage);
                        }
                    }
                }
                Ok(ValidateFamilyNameResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.ValidateFamilyNameResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WindowType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.family_alignment_id.is_empty() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.families.v1.WindowType", len)?;
        if !self.family_alignment_id.is_empty() {
            struct_ser.serialize_field("familyAlignmentId", &self.family_alignment_id)?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WindowType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "family_alignment_id",
            "familyAlignmentId",
            "duration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FamilyAlignmentId,
            Duration,
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
                            "familyAlignmentId" | "family_alignment_id" => Ok(GeneratedField::FamilyAlignmentId),
                            "duration" => Ok(GeneratedField::Duration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WindowType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.families.v1.WindowType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WindowType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut family_alignment_id__ = None;
                let mut duration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FamilyAlignmentId => {
                            if family_alignment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("familyAlignmentId"));
                            }
                            family_alignment_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WindowType {
                    family_alignment_id: family_alignment_id__.unwrap_or_default(),
                    duration: duration__,
                })
            }
        }
        deserializer.deserialize_struct("sift.families.v1.WindowType", FIELDS, GeneratedVisitor)
    }
}
