// @generated
impl serde::Serialize for CountTestMeasurementsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CountTestMeasurementsRequest", len)?;
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountTestMeasurementsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CountTestMeasurementsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CountTestMeasurementsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CountTestMeasurementsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CountTestMeasurementsRequest {
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CountTestMeasurementsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountTestMeasurementsResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CountTestMeasurementsResponse", len)?;
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountTestMeasurementsResponse {
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
            type Value = CountTestMeasurementsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CountTestMeasurementsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CountTestMeasurementsResponse, V::Error>
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
                Ok(CountTestMeasurementsResponse {
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CountTestMeasurementsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountTestStepsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CountTestStepsRequest", len)?;
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountTestStepsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CountTestStepsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CountTestStepsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CountTestStepsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CountTestStepsRequest {
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CountTestStepsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CountTestStepsResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CountTestStepsResponse", len)?;
        if self.count != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("count", ToString::to_string(&self.count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CountTestStepsResponse {
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
            type Value = CountTestStepsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CountTestStepsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CountTestStepsResponse, V::Error>
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
                Ok(CountTestStepsResponse {
                    count: count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CountTestStepsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestMeasurementRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_measurement.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestMeasurementRequest", len)?;
        if let Some(v) = self.test_measurement.as_ref() {
            struct_ser.serialize_field("testMeasurement", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestMeasurementRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_measurement",
            "testMeasurement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestMeasurement,
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
                            "testMeasurement" | "test_measurement" => Ok(GeneratedField::TestMeasurement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestMeasurementRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestMeasurementRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestMeasurementRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_measurement__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestMeasurement => {
                            if test_measurement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testMeasurement"));
                            }
                            test_measurement__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTestMeasurementRequest {
                    test_measurement: test_measurement__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestMeasurementRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestMeasurementResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_measurement.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestMeasurementResponse", len)?;
        if let Some(v) = self.test_measurement.as_ref() {
            struct_ser.serialize_field("testMeasurement", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestMeasurementResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_measurement",
            "testMeasurement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestMeasurement,
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
                            "testMeasurement" | "test_measurement" => Ok(GeneratedField::TestMeasurement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestMeasurementResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestMeasurementResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestMeasurementResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_measurement__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestMeasurement => {
                            if test_measurement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testMeasurement"));
                            }
                            test_measurement__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTestMeasurementResponse {
                    test_measurement: test_measurement__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestMeasurementResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestMeasurementsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_measurements.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestMeasurementsRequest", len)?;
        if !self.test_measurements.is_empty() {
            struct_ser.serialize_field("testMeasurements", &self.test_measurements)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestMeasurementsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_measurements",
            "testMeasurements",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestMeasurements,
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
                            "testMeasurements" | "test_measurements" => Ok(GeneratedField::TestMeasurements),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestMeasurementsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestMeasurementsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestMeasurementsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_measurements__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestMeasurements => {
                            if test_measurements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testMeasurements"));
                            }
                            test_measurements__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTestMeasurementsRequest {
                    test_measurements: test_measurements__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestMeasurementsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestMeasurementsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.measurements_created_count != 0 {
            len += 1;
        }
        if !self.measurement_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestMeasurementsResponse", len)?;
        if self.measurements_created_count != 0 {
            struct_ser.serialize_field("measurementsCreatedCount", &self.measurements_created_count)?;
        }
        if !self.measurement_ids.is_empty() {
            struct_ser.serialize_field("measurementIds", &self.measurement_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestMeasurementsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "measurements_created_count",
            "measurementsCreatedCount",
            "measurement_ids",
            "measurementIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MeasurementsCreatedCount,
            MeasurementIds,
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
                            "measurementsCreatedCount" | "measurements_created_count" => Ok(GeneratedField::MeasurementsCreatedCount),
                            "measurementIds" | "measurement_ids" => Ok(GeneratedField::MeasurementIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestMeasurementsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestMeasurementsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestMeasurementsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut measurements_created_count__ = None;
                let mut measurement_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MeasurementsCreatedCount => {
                            if measurements_created_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurementsCreatedCount"));
                            }
                            measurements_created_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MeasurementIds => {
                            if measurement_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurementIds"));
                            }
                            measurement_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTestMeasurementsResponse {
                    measurements_created_count: measurements_created_count__.unwrap_or_default(),
                    measurement_ids: measurement_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestMeasurementsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.test_system_name.is_empty() {
            len += 1;
        }
        if !self.test_case.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.serial_number.is_empty() {
            len += 1;
        }
        if !self.part_number.is_empty() {
            len += 1;
        }
        if !self.system_operator.is_empty() {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestReportRequest", len)?;
        if self.status != 0 {
            let v = TestStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.test_system_name.is_empty() {
            struct_ser.serialize_field("testSystemName", &self.test_system_name)?;
        }
        if !self.test_case.is_empty() {
            struct_ser.serialize_field("testCase", &self.test_case)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.serial_number.is_empty() {
            struct_ser.serialize_field("serialNumber", &self.serial_number)?;
        }
        if !self.part_number.is_empty() {
            struct_ser.serialize_field("partNumber", &self.part_number)?;
        }
        if !self.system_operator.is_empty() {
            struct_ser.serialize_field("systemOperator", &self.system_operator)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "name",
            "test_system_name",
            "testSystemName",
            "test_case",
            "testCase",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "metadata",
            "serial_number",
            "serialNumber",
            "part_number",
            "partNumber",
            "system_operator",
            "systemOperator",
            "run_id",
            "runId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            Name,
            TestSystemName,
            TestCase,
            StartTime,
            EndTime,
            Metadata,
            SerialNumber,
            PartNumber,
            SystemOperator,
            RunId,
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
                            "status" => Ok(GeneratedField::Status),
                            "name" => Ok(GeneratedField::Name),
                            "testSystemName" | "test_system_name" => Ok(GeneratedField::TestSystemName),
                            "testCase" | "test_case" => Ok(GeneratedField::TestCase),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "partNumber" | "part_number" => Ok(GeneratedField::PartNumber),
                            "systemOperator" | "system_operator" => Ok(GeneratedField::SystemOperator),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut name__ = None;
                let mut test_system_name__ = None;
                let mut test_case__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut metadata__ = None;
                let mut serial_number__ = None;
                let mut part_number__ = None;
                let mut system_operator__ = None;
                let mut run_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<TestStatus>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestSystemName => {
                            if test_system_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testSystemName"));
                            }
                            test_system_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestCase => {
                            if test_case__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testCase"));
                            }
                            test_case__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PartNumber => {
                            if part_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partNumber"));
                            }
                            part_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SystemOperator => {
                            if system_operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemOperator"));
                            }
                            system_operator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateTestReportRequest {
                    status: status__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    test_system_name: test_system_name__.unwrap_or_default(),
                    test_case: test_case__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    metadata: metadata__.unwrap_or_default(),
                    serial_number: serial_number__.unwrap_or_default(),
                    part_number: part_number__.unwrap_or_default(),
                    system_operator: system_operator__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestReportResponse", len)?;
        if let Some(v) = self.test_report.as_ref() {
            struct_ser.serialize_field("testReport", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report",
            "testReport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReport,
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
                            "testReport" | "test_report" => Ok(GeneratedField::TestReport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReport => {
                            if test_report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReport"));
                            }
                            test_report__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTestReportResponse {
                    test_report: test_report__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestStepRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_step.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestStepRequest", len)?;
        if let Some(v) = self.test_step.as_ref() {
            struct_ser.serialize_field("testStep", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestStepRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_step",
            "testStep",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestStep,
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
                            "testStep" | "test_step" => Ok(GeneratedField::TestStep),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestStepRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestStepRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestStepRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_step__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestStep => {
                            if test_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testStep"));
                            }
                            test_step__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTestStepRequest {
                    test_step: test_step__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestStepRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateTestStepResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_step.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.CreateTestStepResponse", len)?;
        if let Some(v) = self.test_step.as_ref() {
            struct_ser.serialize_field("testStep", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateTestStepResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_step",
            "testStep",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestStep,
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
                            "testStep" | "test_step" => Ok(GeneratedField::TestStep),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateTestStepResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.CreateTestStepResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateTestStepResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_step__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestStep => {
                            if test_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testStep"));
                            }
                            test_step__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateTestStepResponse {
                    test_step: test_step__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.CreateTestStepResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTestMeasurementRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.measurement_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.DeleteTestMeasurementRequest", len)?;
        if !self.measurement_id.is_empty() {
            struct_ser.serialize_field("measurementId", &self.measurement_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTestMeasurementRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "measurement_id",
            "measurementId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MeasurementId,
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
                            "measurementId" | "measurement_id" => Ok(GeneratedField::MeasurementId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteTestMeasurementRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.DeleteTestMeasurementRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTestMeasurementRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut measurement_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MeasurementId => {
                            if measurement_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurementId"));
                            }
                            measurement_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteTestMeasurementRequest {
                    measurement_id: measurement_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.DeleteTestMeasurementRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTestMeasurementResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.test_reports.v1.DeleteTestMeasurementResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTestMeasurementResponse {
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
            type Value = DeleteTestMeasurementResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.DeleteTestMeasurementResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTestMeasurementResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteTestMeasurementResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.DeleteTestMeasurementResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTestReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.DeleteTestReportRequest", len)?;
        if !self.test_report_id.is_empty() {
            struct_ser.serialize_field("testReportId", &self.test_report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTestReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report_id",
            "testReportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReportId,
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
                            "testReportId" | "test_report_id" => Ok(GeneratedField::TestReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteTestReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.DeleteTestReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTestReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReportId => {
                            if test_report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReportId"));
                            }
                            test_report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteTestReportRequest {
                    test_report_id: test_report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.DeleteTestReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTestReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.test_reports.v1.DeleteTestReportResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTestReportResponse {
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
            type Value = DeleteTestReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.DeleteTestReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTestReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteTestReportResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.DeleteTestReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTestStepRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_step_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.DeleteTestStepRequest", len)?;
        if !self.test_step_id.is_empty() {
            struct_ser.serialize_field("testStepId", &self.test_step_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTestStepRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_step_id",
            "testStepId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestStepId,
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
                            "testStepId" | "test_step_id" => Ok(GeneratedField::TestStepId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteTestStepRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.DeleteTestStepRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTestStepRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_step_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestStepId => {
                            if test_step_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testStepId"));
                            }
                            test_step_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteTestStepRequest {
                    test_step_id: test_step_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.DeleteTestStepRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTestStepResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sift.test_reports.v1.DeleteTestStepResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTestStepResponse {
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
            type Value = DeleteTestStepResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.DeleteTestStepResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteTestStepResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteTestStepResponse {
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.DeleteTestStepResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error_code != 0 {
            len += 1;
        }
        if !self.error_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ErrorInfo", len)?;
        if self.error_code != 0 {
            struct_ser.serialize_field("errorCode", &self.error_code)?;
        }
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error_code",
            "errorCode",
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorCode,
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
                            "errorCode" | "error_code" => Ok(GeneratedField::ErrorCode),
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
            type Value = ErrorInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ErrorInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error_code__ = None;
                let mut error_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ErrorCode => {
                            if error_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorCode"));
                            }
                            error_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ErrorInfo {
                    error_code: error_code__.unwrap_or_default(),
                    error_message: error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ErrorInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTestReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_report_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.GetTestReportRequest", len)?;
        if !self.test_report_id.is_empty() {
            struct_ser.serialize_field("testReportId", &self.test_report_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTestReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report_id",
            "testReportId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReportId,
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
                            "testReportId" | "test_report_id" => Ok(GeneratedField::TestReportId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTestReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.GetTestReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTestReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReportId => {
                            if test_report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReportId"));
                            }
                            test_report_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetTestReportRequest {
                    test_report_id: test_report_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.GetTestReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTestReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.GetTestReportResponse", len)?;
        if let Some(v) = self.test_report.as_ref() {
            struct_ser.serialize_field("testReport", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTestReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report",
            "testReport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReport,
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
                            "testReport" | "test_report" => Ok(GeneratedField::TestReport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTestReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.GetTestReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetTestReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReport => {
                            if test_report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReport"));
                            }
                            test_report__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetTestReportResponse {
                    test_report: test_report__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.GetTestReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportTestReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remote_file_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ImportTestReportRequest", len)?;
        if !self.remote_file_id.is_empty() {
            struct_ser.serialize_field("remoteFileId", &self.remote_file_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportTestReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "remote_file_id",
            "remoteFileId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemoteFileId,
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
                            "remoteFileId" | "remote_file_id" => Ok(GeneratedField::RemoteFileId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportTestReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ImportTestReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportTestReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut remote_file_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RemoteFileId => {
                            if remote_file_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteFileId"));
                            }
                            remote_file_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ImportTestReportRequest {
                    remote_file_id: remote_file_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ImportTestReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportTestReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ImportTestReportResponse", len)?;
        if let Some(v) = self.test_report.as_ref() {
            struct_ser.serialize_field("testReport", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportTestReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report",
            "testReport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReport,
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
                            "testReport" | "test_report" => Ok(GeneratedField::TestReport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportTestReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ImportTestReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportTestReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReport => {
                            if test_report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReport"));
                            }
                            test_report__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ImportTestReportResponse {
                    test_report: test_report__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ImportTestReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTestMeasurementsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ListTestMeasurementsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListTestMeasurementsRequest {
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
            type Value = ListTestMeasurementsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ListTestMeasurementsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTestMeasurementsRequest, V::Error>
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
                Ok(ListTestMeasurementsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ListTestMeasurementsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTestMeasurementsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_measurements.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ListTestMeasurementsResponse", len)?;
        if !self.test_measurements.is_empty() {
            struct_ser.serialize_field("testMeasurements", &self.test_measurements)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTestMeasurementsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_measurements",
            "testMeasurements",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestMeasurements,
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
                            "testMeasurements" | "test_measurements" => Ok(GeneratedField::TestMeasurements),
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
            type Value = ListTestMeasurementsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ListTestMeasurementsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTestMeasurementsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_measurements__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestMeasurements => {
                            if test_measurements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testMeasurements"));
                            }
                            test_measurements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTestMeasurementsResponse {
                    test_measurements: test_measurements__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ListTestMeasurementsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTestReportsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ListTestReportsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListTestReportsRequest {
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
            type Value = ListTestReportsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ListTestReportsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTestReportsRequest, V::Error>
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
                Ok(ListTestReportsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ListTestReportsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTestReportsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_reports.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ListTestReportsResponse", len)?;
        if !self.test_reports.is_empty() {
            struct_ser.serialize_field("testReports", &self.test_reports)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTestReportsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_reports",
            "testReports",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReports,
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
                            "testReports" | "test_reports" => Ok(GeneratedField::TestReports),
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
            type Value = ListTestReportsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ListTestReportsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTestReportsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_reports__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReports => {
                            if test_reports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReports"));
                            }
                            test_reports__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTestReportsResponse {
                    test_reports: test_reports__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ListTestReportsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTestStepsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ListTestStepsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListTestStepsRequest {
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
            type Value = ListTestStepsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ListTestStepsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTestStepsRequest, V::Error>
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
                Ok(ListTestStepsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ListTestStepsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTestStepsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_steps.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.ListTestStepsResponse", len)?;
        if !self.test_steps.is_empty() {
            struct_ser.serialize_field("testSteps", &self.test_steps)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTestStepsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_steps",
            "testSteps",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestSteps,
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
                            "testSteps" | "test_steps" => Ok(GeneratedField::TestSteps),
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
            type Value = ListTestStepsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.ListTestStepsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTestStepsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_steps__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestSteps => {
                            if test_steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testSteps"));
                            }
                            test_steps__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTestStepsResponse {
                    test_steps: test_steps__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.ListTestStepsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NumericBounds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min.is_some() {
            len += 1;
        }
        if self.max.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.NumericBounds", len)?;
        if let Some(v) = self.min.as_ref() {
            struct_ser.serialize_field("min", v)?;
        }
        if let Some(v) = self.max.as_ref() {
            struct_ser.serialize_field("max", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NumericBounds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            Max,
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
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NumericBounds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.NumericBounds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NumericBounds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min__ = None;
                let mut max__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Max => {
                            if max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(NumericBounds {
                    min: min__,
                    max: max__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.NumericBounds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringBounds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expected_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.StringBounds", len)?;
        if !self.expected_value.is_empty() {
            struct_ser.serialize_field("expectedValue", &self.expected_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringBounds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expected_value",
            "expectedValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpectedValue,
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
                            "expectedValue" | "expected_value" => Ok(GeneratedField::ExpectedValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringBounds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.StringBounds")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringBounds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expected_value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExpectedValue => {
                            if expected_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectedValue"));
                            }
                            expected_value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StringBounds {
                    expected_value: expected_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.StringBounds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestMeasurement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.measurement_id.is_empty() {
            len += 1;
        }
        if self.measurement_type != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.test_step_id.is_empty() {
            len += 1;
        }
        if !self.test_report_id.is_empty() {
            len += 1;
        }
        if self.unit.is_some() {
            len += 1;
        }
        if self.passed {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.bounds.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.TestMeasurement", len)?;
        if !self.measurement_id.is_empty() {
            struct_ser.serialize_field("measurementId", &self.measurement_id)?;
        }
        if self.measurement_type != 0 {
            let v = TestMeasurementType::try_from(self.measurement_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.measurement_type)))?;
            struct_ser.serialize_field("measurementType", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.test_step_id.is_empty() {
            struct_ser.serialize_field("testStepId", &self.test_step_id)?;
        }
        if !self.test_report_id.is_empty() {
            struct_ser.serialize_field("testReportId", &self.test_report_id)?;
        }
        if let Some(v) = self.unit.as_ref() {
            struct_ser.serialize_field("unit", v)?;
        }
        if self.passed {
            struct_ser.serialize_field("passed", &self.passed)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                test_measurement::Value::NumericValue(v) => {
                    struct_ser.serialize_field("numericValue", v)?;
                }
                test_measurement::Value::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                test_measurement::Value::BooleanValue(v) => {
                    struct_ser.serialize_field("booleanValue", v)?;
                }
            }
        }
        if let Some(v) = self.bounds.as_ref() {
            match v {
                test_measurement::Bounds::NumericBounds(v) => {
                    struct_ser.serialize_field("numericBounds", v)?;
                }
                test_measurement::Bounds::StringBounds(v) => {
                    struct_ser.serialize_field("stringBounds", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestMeasurement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "measurement_id",
            "measurementId",
            "measurement_type",
            "measurementType",
            "name",
            "test_step_id",
            "testStepId",
            "test_report_id",
            "testReportId",
            "unit",
            "passed",
            "timestamp",
            "numeric_value",
            "numericValue",
            "string_value",
            "stringValue",
            "boolean_value",
            "booleanValue",
            "numeric_bounds",
            "numericBounds",
            "string_bounds",
            "stringBounds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MeasurementId,
            MeasurementType,
            Name,
            TestStepId,
            TestReportId,
            Unit,
            Passed,
            Timestamp,
            NumericValue,
            StringValue,
            BooleanValue,
            NumericBounds,
            StringBounds,
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
                            "measurementId" | "measurement_id" => Ok(GeneratedField::MeasurementId),
                            "measurementType" | "measurement_type" => Ok(GeneratedField::MeasurementType),
                            "name" => Ok(GeneratedField::Name),
                            "testStepId" | "test_step_id" => Ok(GeneratedField::TestStepId),
                            "testReportId" | "test_report_id" => Ok(GeneratedField::TestReportId),
                            "unit" => Ok(GeneratedField::Unit),
                            "passed" => Ok(GeneratedField::Passed),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "numericValue" | "numeric_value" => Ok(GeneratedField::NumericValue),
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "booleanValue" | "boolean_value" => Ok(GeneratedField::BooleanValue),
                            "numericBounds" | "numeric_bounds" => Ok(GeneratedField::NumericBounds),
                            "stringBounds" | "string_bounds" => Ok(GeneratedField::StringBounds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestMeasurement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.TestMeasurement")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestMeasurement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut measurement_id__ = None;
                let mut measurement_type__ = None;
                let mut name__ = None;
                let mut test_step_id__ = None;
                let mut test_report_id__ = None;
                let mut unit__ = None;
                let mut passed__ = None;
                let mut timestamp__ = None;
                let mut value__ = None;
                let mut bounds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MeasurementId => {
                            if measurement_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurementId"));
                            }
                            measurement_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MeasurementType => {
                            if measurement_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurementType"));
                            }
                            measurement_type__ = Some(map_.next_value::<TestMeasurementType>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestStepId => {
                            if test_step_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testStepId"));
                            }
                            test_step_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestReportId => {
                            if test_report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReportId"));
                            }
                            test_report_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = map_.next_value()?;
                        }
                        GeneratedField::Passed => {
                            if passed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passed"));
                            }
                            passed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                        GeneratedField::NumericValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numericValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| test_measurement::Value::NumericValue(x.0));
                        }
                        GeneratedField::StringValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(test_measurement::Value::StringValue);
                        }
                        GeneratedField::BooleanValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("booleanValue"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(test_measurement::Value::BooleanValue);
                        }
                        GeneratedField::NumericBounds => {
                            if bounds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numericBounds"));
                            }
                            bounds__ = map_.next_value::<::std::option::Option<_>>()?.map(test_measurement::Bounds::NumericBounds)
;
                        }
                        GeneratedField::StringBounds => {
                            if bounds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringBounds"));
                            }
                            bounds__ = map_.next_value::<::std::option::Option<_>>()?.map(test_measurement::Bounds::StringBounds)
;
                        }
                    }
                }
                Ok(TestMeasurement {
                    measurement_id: measurement_id__.unwrap_or_default(),
                    measurement_type: measurement_type__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    test_step_id: test_step_id__.unwrap_or_default(),
                    test_report_id: test_report_id__.unwrap_or_default(),
                    unit: unit__,
                    passed: passed__.unwrap_or_default(),
                    timestamp: timestamp__,
                    value: value__,
                    bounds: bounds__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.TestMeasurement", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestMeasurementType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TEST_MEASUREMENT_TYPE_UNSPECIFIED",
            Self::Double => "TEST_MEASUREMENT_TYPE_DOUBLE",
            Self::String => "TEST_MEASUREMENT_TYPE_STRING",
            Self::Boolean => "TEST_MEASUREMENT_TYPE_BOOLEAN",
            Self::Limit => "TEST_MEASUREMENT_TYPE_LIMIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TestMeasurementType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TEST_MEASUREMENT_TYPE_UNSPECIFIED",
            "TEST_MEASUREMENT_TYPE_DOUBLE",
            "TEST_MEASUREMENT_TYPE_STRING",
            "TEST_MEASUREMENT_TYPE_BOOLEAN",
            "TEST_MEASUREMENT_TYPE_LIMIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestMeasurementType;

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
                    "TEST_MEASUREMENT_TYPE_UNSPECIFIED" => Ok(TestMeasurementType::Unspecified),
                    "TEST_MEASUREMENT_TYPE_DOUBLE" => Ok(TestMeasurementType::Double),
                    "TEST_MEASUREMENT_TYPE_STRING" => Ok(TestMeasurementType::String),
                    "TEST_MEASUREMENT_TYPE_BOOLEAN" => Ok(TestMeasurementType::Boolean),
                    "TEST_MEASUREMENT_TYPE_LIMIT" => Ok(TestMeasurementType::Limit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TestReport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_report_id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.test_system_name.is_empty() {
            len += 1;
        }
        if !self.test_case.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.serial_number.is_empty() {
            len += 1;
        }
        if !self.part_number.is_empty() {
            len += 1;
        }
        if !self.system_operator.is_empty() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        if !self.run_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.TestReport", len)?;
        if !self.test_report_id.is_empty() {
            struct_ser.serialize_field("testReportId", &self.test_report_id)?;
        }
        if self.status != 0 {
            let v = TestStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.test_system_name.is_empty() {
            struct_ser.serialize_field("testSystemName", &self.test_system_name)?;
        }
        if !self.test_case.is_empty() {
            struct_ser.serialize_field("testCase", &self.test_case)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.serial_number.is_empty() {
            struct_ser.serialize_field("serialNumber", &self.serial_number)?;
        }
        if !self.part_number.is_empty() {
            struct_ser.serialize_field("partNumber", &self.part_number)?;
        }
        if !self.system_operator.is_empty() {
            struct_ser.serialize_field("systemOperator", &self.system_operator)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        if !self.run_id.is_empty() {
            struct_ser.serialize_field("runId", &self.run_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestReport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report_id",
            "testReportId",
            "status",
            "name",
            "test_system_name",
            "testSystemName",
            "test_case",
            "testCase",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "metadata",
            "serial_number",
            "serialNumber",
            "part_number",
            "partNumber",
            "system_operator",
            "systemOperator",
            "archived_date",
            "archivedDate",
            "is_archived",
            "isArchived",
            "run_id",
            "runId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReportId,
            Status,
            Name,
            TestSystemName,
            TestCase,
            StartTime,
            EndTime,
            Metadata,
            SerialNumber,
            PartNumber,
            SystemOperator,
            ArchivedDate,
            IsArchived,
            RunId,
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
                            "testReportId" | "test_report_id" => Ok(GeneratedField::TestReportId),
                            "status" => Ok(GeneratedField::Status),
                            "name" => Ok(GeneratedField::Name),
                            "testSystemName" | "test_system_name" => Ok(GeneratedField::TestSystemName),
                            "testCase" | "test_case" => Ok(GeneratedField::TestCase),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "partNumber" | "part_number" => Ok(GeneratedField::PartNumber),
                            "systemOperator" | "system_operator" => Ok(GeneratedField::SystemOperator),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "isArchived" | "is_archived" => Ok(GeneratedField::IsArchived),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestReport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.TestReport")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestReport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report_id__ = None;
                let mut status__ = None;
                let mut name__ = None;
                let mut test_system_name__ = None;
                let mut test_case__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut metadata__ = None;
                let mut serial_number__ = None;
                let mut part_number__ = None;
                let mut system_operator__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                let mut run_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReportId => {
                            if test_report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReportId"));
                            }
                            test_report_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<TestStatus>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestSystemName => {
                            if test_system_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testSystemName"));
                            }
                            test_system_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestCase => {
                            if test_case__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testCase"));
                            }
                            test_case__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PartNumber => {
                            if part_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partNumber"));
                            }
                            part_number__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SystemOperator => {
                            if system_operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemOperator"));
                            }
                            system_operator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ArchivedDate => {
                            if archived_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archivedDate"));
                            }
                            archived_date__ = map_.next_value()?;
                        }
                        GeneratedField::IsArchived => {
                            if is_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isArchived"));
                            }
                            is_archived__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestReport {
                    test_report_id: test_report_id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    test_system_name: test_system_name__.unwrap_or_default(),
                    test_case: test_case__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    metadata: metadata__.unwrap_or_default(),
                    serial_number: serial_number__.unwrap_or_default(),
                    part_number: part_number__.unwrap_or_default(),
                    system_operator: system_operator__.unwrap_or_default(),
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                    run_id: run_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.TestReport", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TEST_STATUS_UNSPECIFIED",
            Self::Draft => "TEST_STATUS_DRAFT",
            Self::Passed => "TEST_STATUS_PASSED",
            Self::Failed => "TEST_STATUS_FAILED",
            Self::Aborted => "TEST_STATUS_ABORTED",
            Self::Error => "TEST_STATUS_ERROR",
            Self::InProgress => "TEST_STATUS_IN_PROGRESS",
            Self::Skipped => "TEST_STATUS_SKIPPED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TestStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TEST_STATUS_UNSPECIFIED",
            "TEST_STATUS_DRAFT",
            "TEST_STATUS_PASSED",
            "TEST_STATUS_FAILED",
            "TEST_STATUS_ABORTED",
            "TEST_STATUS_ERROR",
            "TEST_STATUS_IN_PROGRESS",
            "TEST_STATUS_SKIPPED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestStatus;

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
                    "TEST_STATUS_UNSPECIFIED" => Ok(TestStatus::Unspecified),
                    "TEST_STATUS_DRAFT" => Ok(TestStatus::Draft),
                    "TEST_STATUS_PASSED" => Ok(TestStatus::Passed),
                    "TEST_STATUS_FAILED" => Ok(TestStatus::Failed),
                    "TEST_STATUS_ABORTED" => Ok(TestStatus::Aborted),
                    "TEST_STATUS_ERROR" => Ok(TestStatus::Error),
                    "TEST_STATUS_IN_PROGRESS" => Ok(TestStatus::InProgress),
                    "TEST_STATUS_SKIPPED" => Ok(TestStatus::Skipped),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TestStep {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_step_id.is_empty() {
            len += 1;
        }
        if !self.test_report_id.is_empty() {
            len += 1;
        }
        if !self.parent_step_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.step_type != 0 {
            len += 1;
        }
        if !self.step_path.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.error_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.TestStep", len)?;
        if !self.test_step_id.is_empty() {
            struct_ser.serialize_field("testStepId", &self.test_step_id)?;
        }
        if !self.test_report_id.is_empty() {
            struct_ser.serialize_field("testReportId", &self.test_report_id)?;
        }
        if !self.parent_step_id.is_empty() {
            struct_ser.serialize_field("parentStepId", &self.parent_step_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.step_type != 0 {
            let v = TestStepType::try_from(self.step_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.step_type)))?;
            struct_ser.serialize_field("stepType", &v)?;
        }
        if !self.step_path.is_empty() {
            struct_ser.serialize_field("stepPath", &self.step_path)?;
        }
        if self.status != 0 {
            let v = TestStatus::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", v)?;
        }
        if let Some(v) = self.error_info.as_ref() {
            struct_ser.serialize_field("errorInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestStep {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_step_id",
            "testStepId",
            "test_report_id",
            "testReportId",
            "parent_step_id",
            "parentStepId",
            "name",
            "description",
            "step_type",
            "stepType",
            "step_path",
            "stepPath",
            "status",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "error_info",
            "errorInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestStepId,
            TestReportId,
            ParentStepId,
            Name,
            Description,
            StepType,
            StepPath,
            Status,
            StartTime,
            EndTime,
            ErrorInfo,
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
                            "testStepId" | "test_step_id" => Ok(GeneratedField::TestStepId),
                            "testReportId" | "test_report_id" => Ok(GeneratedField::TestReportId),
                            "parentStepId" | "parent_step_id" => Ok(GeneratedField::ParentStepId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "stepType" | "step_type" => Ok(GeneratedField::StepType),
                            "stepPath" | "step_path" => Ok(GeneratedField::StepPath),
                            "status" => Ok(GeneratedField::Status),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "errorInfo" | "error_info" => Ok(GeneratedField::ErrorInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestStep;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.TestStep")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestStep, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_step_id__ = None;
                let mut test_report_id__ = None;
                let mut parent_step_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut step_type__ = None;
                let mut step_path__ = None;
                let mut status__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut error_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestStepId => {
                            if test_step_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testStepId"));
                            }
                            test_step_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TestReportId => {
                            if test_report_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReportId"));
                            }
                            test_report_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentStepId => {
                            if parent_step_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentStepId"));
                            }
                            parent_step_id__ = Some(map_.next_value()?);
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
                        GeneratedField::StepType => {
                            if step_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stepType"));
                            }
                            step_type__ = Some(map_.next_value::<TestStepType>()? as i32);
                        }
                        GeneratedField::StepPath => {
                            if step_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stepPath"));
                            }
                            step_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<TestStatus>()? as i32);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorInfo => {
                            if error_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorInfo"));
                            }
                            error_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TestStep {
                    test_step_id: test_step_id__.unwrap_or_default(),
                    test_report_id: test_report_id__.unwrap_or_default(),
                    parent_step_id: parent_step_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    step_type: step_type__.unwrap_or_default(),
                    step_path: step_path__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    start_time: start_time__,
                    end_time: end_time__,
                    error_info: error_info__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.TestStep", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestStepType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TEST_STEP_TYPE_UNSPECIFIED",
            Self::Sequence => "TEST_STEP_TYPE_SEQUENCE",
            Self::Group => "TEST_STEP_TYPE_GROUP",
            Self::Action => "TEST_STEP_TYPE_ACTION",
            Self::FlowControl => "TEST_STEP_TYPE_FLOW_CONTROL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TestStepType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TEST_STEP_TYPE_UNSPECIFIED",
            "TEST_STEP_TYPE_SEQUENCE",
            "TEST_STEP_TYPE_GROUP",
            "TEST_STEP_TYPE_ACTION",
            "TEST_STEP_TYPE_FLOW_CONTROL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestStepType;

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
                    "TEST_STEP_TYPE_UNSPECIFIED" => Ok(TestStepType::Unspecified),
                    "TEST_STEP_TYPE_SEQUENCE" => Ok(TestStepType::Sequence),
                    "TEST_STEP_TYPE_GROUP" => Ok(TestStepType::Group),
                    "TEST_STEP_TYPE_ACTION" => Ok(TestStepType::Action),
                    "TEST_STEP_TYPE_FLOW_CONTROL" => Ok(TestStepType::FlowControl),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTestMeasurementRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_measurement.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.UpdateTestMeasurementRequest", len)?;
        if let Some(v) = self.test_measurement.as_ref() {
            struct_ser.serialize_field("testMeasurement", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTestMeasurementRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_measurement",
            "testMeasurement",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestMeasurement,
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
                            "testMeasurement" | "test_measurement" => Ok(GeneratedField::TestMeasurement),
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
            type Value = UpdateTestMeasurementRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.UpdateTestMeasurementRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTestMeasurementRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_measurement__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestMeasurement => {
                            if test_measurement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testMeasurement"));
                            }
                            test_measurement__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTestMeasurementRequest {
                    test_measurement: test_measurement__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.UpdateTestMeasurementRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTestMeasurementResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_measurement.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.UpdateTestMeasurementResponse", len)?;
        if let Some(v) = self.test_measurement.as_ref() {
            struct_ser.serialize_field("testMeasurement", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTestMeasurementResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_measurement",
            "testMeasurement",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestMeasurement,
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
                            "testMeasurement" | "test_measurement" => Ok(GeneratedField::TestMeasurement),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTestMeasurementResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.UpdateTestMeasurementResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTestMeasurementResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_measurement__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestMeasurement => {
                            if test_measurement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testMeasurement"));
                            }
                            test_measurement__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTestMeasurementResponse {
                    test_measurement: test_measurement__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.UpdateTestMeasurementResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTestReportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_report.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.UpdateTestReportRequest", len)?;
        if let Some(v) = self.test_report.as_ref() {
            struct_ser.serialize_field("testReport", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTestReportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report",
            "testReport",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReport,
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
                            "testReport" | "test_report" => Ok(GeneratedField::TestReport),
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
            type Value = UpdateTestReportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.UpdateTestReportRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTestReportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReport => {
                            if test_report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReport"));
                            }
                            test_report__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTestReportRequest {
                    test_report: test_report__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.UpdateTestReportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTestReportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_report.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.UpdateTestReportResponse", len)?;
        if let Some(v) = self.test_report.as_ref() {
            struct_ser.serialize_field("testReport", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTestReportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_report",
            "testReport",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestReport,
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
                            "testReport" | "test_report" => Ok(GeneratedField::TestReport),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTestReportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.UpdateTestReportResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTestReportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_report__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestReport => {
                            if test_report__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testReport"));
                            }
                            test_report__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTestReportResponse {
                    test_report: test_report__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.UpdateTestReportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTestStepRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_step.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.UpdateTestStepRequest", len)?;
        if let Some(v) = self.test_step.as_ref() {
            struct_ser.serialize_field("testStep", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTestStepRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_step",
            "testStep",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestStep,
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
                            "testStep" | "test_step" => Ok(GeneratedField::TestStep),
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
            type Value = UpdateTestStepRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.UpdateTestStepRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTestStepRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_step__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestStep => {
                            if test_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testStep"));
                            }
                            test_step__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTestStepRequest {
                    test_step: test_step__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.UpdateTestStepRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateTestStepResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_step.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.test_reports.v1.UpdateTestStepResponse", len)?;
        if let Some(v) = self.test_step.as_ref() {
            struct_ser.serialize_field("testStep", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTestStepResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_step",
            "testStep",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestStep,
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
                            "testStep" | "test_step" => Ok(GeneratedField::TestStep),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateTestStepResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.test_reports.v1.UpdateTestStepResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateTestStepResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_step__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestStep => {
                            if test_step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testStep"));
                            }
                            test_step__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateTestStepResponse {
                    test_step: test_step__,
                })
            }
        }
        deserializer.deserialize_struct("sift.test_reports.v1.UpdateTestStepResponse", FIELDS, GeneratedVisitor)
    }
}
