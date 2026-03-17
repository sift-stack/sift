// @generated
impl serde::Serialize for ArchivePolicyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policy_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.ArchivePolicyRequest", len)?;
        if !self.policy_id.is_empty() {
            struct_ser.serialize_field("policyId", &self.policy_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy_id",
            "policyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolicyId,
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
                            "policyId" | "policy_id" => Ok(GeneratedField::PolicyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchivePolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.ArchivePolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PolicyId => {
                            if policy_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyId"));
                            }
                            policy_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ArchivePolicyRequest {
                    policy_id: policy_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.ArchivePolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArchivePolicyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.ArchivePolicyResponse", len)?;
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArchivePolicyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policy,
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
                            "policy" => Ok(GeneratedField::Policy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArchivePolicyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.ArchivePolicyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ArchivePolicyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ArchivePolicyResponse {
                    policy: policy__,
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.ArchivePolicyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePolicyRequest {
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
        if self.description.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.version_notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.CreatePolicyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if let Some(v) = self.version_notes.as_ref() {
            struct_ser.serialize_field("versionNotes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "configuration",
            "version_notes",
            "versionNotes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Configuration,
            VersionNotes,
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
                            "description" => Ok(GeneratedField::Description),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "versionNotes" | "version_notes" => Ok(GeneratedField::VersionNotes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.CreatePolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut configuration__ = None;
                let mut version_notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::VersionNotes => {
                            if version_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNotes"));
                            }
                            version_notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePolicyRequest {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    configuration: configuration__,
                    version_notes: version_notes__,
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.CreatePolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreatePolicyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.CreatePolicyResponse", len)?;
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreatePolicyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policy,
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
                            "policy" => Ok(GeneratedField::Policy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreatePolicyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.CreatePolicyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreatePolicyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreatePolicyResponse {
                    policy: policy__,
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.CreatePolicyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPolicyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policy_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.GetPolicyRequest", len)?;
        if !self.policy_id.is_empty() {
            struct_ser.serialize_field("policyId", &self.policy_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy_id",
            "policyId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolicyId,
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
                            "policyId" | "policy_id" => Ok(GeneratedField::PolicyId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.GetPolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PolicyId => {
                            if policy_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyId"));
                            }
                            policy_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetPolicyRequest {
                    policy_id: policy_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.GetPolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPolicyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.GetPolicyResponse", len)?;
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPolicyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policy,
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
                            "policy" => Ok(GeneratedField::Policy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPolicyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.GetPolicyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetPolicyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetPolicyResponse {
                    policy: policy__,
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.GetPolicyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPoliciesRequest {
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
        if self.include_archived {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.ListPoliciesRequest", len)?;
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
        if self.include_archived {
            struct_ser.serialize_field("includeArchived", &self.include_archived)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPoliciesRequest {
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
            "include_archived",
            "includeArchived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            PageToken,
            Filter,
            OrderBy,
            IncludeArchived,
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
                            "includeArchived" | "include_archived" => Ok(GeneratedField::IncludeArchived),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListPoliciesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.ListPoliciesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPoliciesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                let mut include_archived__ = None;
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
                        GeneratedField::IncludeArchived => {
                            if include_archived__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeArchived"));
                            }
                            include_archived__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPoliciesRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                    include_archived: include_archived__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.ListPoliciesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListPoliciesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policies.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.ListPoliciesResponse", len)?;
        if !self.policies.is_empty() {
            struct_ser.serialize_field("policies", &self.policies)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListPoliciesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policies",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policies,
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
                            "policies" => Ok(GeneratedField::Policies),
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
            type Value = ListPoliciesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.ListPoliciesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListPoliciesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policies__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policies => {
                            if policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            policies__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListPoliciesResponse {
                    policies: policies__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.ListPoliciesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Policy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policy_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
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
        if self.configuration.is_some() {
            len += 1;
        }
        if !self.policy_version_id.is_empty() {
            len += 1;
        }
        if self.archived_date.is_some() {
            len += 1;
        }
        if self.is_archived {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.version_notes.is_some() {
            len += 1;
        }
        if self.generated_change_message.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.Policy", len)?;
        if !self.policy_id.is_empty() {
            struct_ser.serialize_field("policyId", &self.policy_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
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
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if !self.policy_version_id.is_empty() {
            struct_ser.serialize_field("policyVersionId", &self.policy_version_id)?;
        }
        if let Some(v) = self.archived_date.as_ref() {
            struct_ser.serialize_field("archivedDate", v)?;
        }
        if self.is_archived {
            struct_ser.serialize_field("isArchived", &self.is_archived)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.version_notes.as_ref() {
            struct_ser.serialize_field("versionNotes", v)?;
        }
        if let Some(v) = self.generated_change_message.as_ref() {
            struct_ser.serialize_field("generatedChangeMessage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy_id",
            "policyId",
            "name",
            "description",
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
            "configuration",
            "policy_version_id",
            "policyVersionId",
            "archived_date",
            "archivedDate",
            "is_archived",
            "isArchived",
            "version",
            "version_notes",
            "versionNotes",
            "generated_change_message",
            "generatedChangeMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PolicyId,
            Name,
            Description,
            OrganizationId,
            CreatedByUserId,
            ModifiedByUserId,
            CreatedDate,
            ModifiedDate,
            Configuration,
            PolicyVersionId,
            ArchivedDate,
            IsArchived,
            Version,
            VersionNotes,
            GeneratedChangeMessage,
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
                            "policyId" | "policy_id" => Ok(GeneratedField::PolicyId),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "organizationId" | "organization_id" => Ok(GeneratedField::OrganizationId),
                            "createdByUserId" | "created_by_user_id" => Ok(GeneratedField::CreatedByUserId),
                            "modifiedByUserId" | "modified_by_user_id" => Ok(GeneratedField::ModifiedByUserId),
                            "createdDate" | "created_date" => Ok(GeneratedField::CreatedDate),
                            "modifiedDate" | "modified_date" => Ok(GeneratedField::ModifiedDate),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "policyVersionId" | "policy_version_id" => Ok(GeneratedField::PolicyVersionId),
                            "archivedDate" | "archived_date" => Ok(GeneratedField::ArchivedDate),
                            "isArchived" | "is_archived" => Ok(GeneratedField::IsArchived),
                            "version" => Ok(GeneratedField::Version),
                            "versionNotes" | "version_notes" => Ok(GeneratedField::VersionNotes),
                            "generatedChangeMessage" | "generated_change_message" => Ok(GeneratedField::GeneratedChangeMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.Policy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_id__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut organization_id__ = None;
                let mut created_by_user_id__ = None;
                let mut modified_by_user_id__ = None;
                let mut created_date__ = None;
                let mut modified_date__ = None;
                let mut configuration__ = None;
                let mut policy_version_id__ = None;
                let mut archived_date__ = None;
                let mut is_archived__ = None;
                let mut version__ = None;
                let mut version_notes__ = None;
                let mut generated_change_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PolicyId => {
                            if policy_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyId"));
                            }
                            policy_id__ = Some(map_.next_value()?);
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
                            description__ = map_.next_value()?;
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
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map_.next_value()?;
                        }
                        GeneratedField::PolicyVersionId => {
                            if policy_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyVersionId"));
                            }
                            policy_version_id__ = Some(map_.next_value()?);
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
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::VersionNotes => {
                            if version_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNotes"));
                            }
                            version_notes__ = map_.next_value()?;
                        }
                        GeneratedField::GeneratedChangeMessage => {
                            if generated_change_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generatedChangeMessage"));
                            }
                            generated_change_message__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Policy {
                    policy_id: policy_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__,
                    organization_id: organization_id__.unwrap_or_default(),
                    created_by_user_id: created_by_user_id__.unwrap_or_default(),
                    modified_by_user_id: modified_by_user_id__.unwrap_or_default(),
                    created_date: created_date__,
                    modified_date: modified_date__,
                    configuration: configuration__,
                    policy_version_id: policy_version_id__.unwrap_or_default(),
                    archived_date: archived_date__,
                    is_archived: is_archived__.unwrap_or_default(),
                    version: version__,
                    version_notes: version_notes__,
                    generated_change_message: generated_change_message__,
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PolicyConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cedar_policy.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.PolicyConfiguration", len)?;
        if !self.cedar_policy.is_empty() {
            struct_ser.serialize_field("cedarPolicy", &self.cedar_policy)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PolicyConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cedar_policy",
            "cedarPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CedarPolicy,
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
                            "cedarPolicy" | "cedar_policy" => Ok(GeneratedField::CedarPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PolicyConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.PolicyConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PolicyConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cedar_policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CedarPolicy => {
                            if cedar_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cedarPolicy"));
                            }
                            cedar_policy__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PolicyConfiguration {
                    cedar_policy: cedar_policy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.PolicyConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePolicyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        if self.version_notes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.UpdatePolicyRequest", len)?;
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        if let Some(v) = self.version_notes.as_ref() {
            struct_ser.serialize_field("versionNotes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy",
            "update_mask",
            "updateMask",
            "version_notes",
            "versionNotes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policy,
            UpdateMask,
            VersionNotes,
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
                            "policy" => Ok(GeneratedField::Policy),
                            "updateMask" | "update_mask" => Ok(GeneratedField::UpdateMask),
                            "versionNotes" | "version_notes" => Ok(GeneratedField::VersionNotes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.UpdatePolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy__ = None;
                let mut update_mask__ = None;
                let mut version_notes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                        GeneratedField::VersionNotes => {
                            if version_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionNotes"));
                            }
                            version_notes__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePolicyRequest {
                    policy: policy__,
                    update_mask: update_mask__,
                    version_notes: version_notes__,
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.UpdatePolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdatePolicyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.policies.v1.UpdatePolicyResponse", len)?;
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdatePolicyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policy,
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
                            "policy" => Ok(GeneratedField::Policy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdatePolicyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.policies.v1.UpdatePolicyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdatePolicyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdatePolicyResponse {
                    policy: policy__,
                })
            }
        }
        deserializer.deserialize_struct("sift.policies.v1.UpdatePolicyResponse", FIELDS, GeneratedVisitor)
    }
}
