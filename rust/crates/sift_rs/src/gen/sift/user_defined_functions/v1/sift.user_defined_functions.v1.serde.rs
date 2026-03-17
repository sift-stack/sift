// @generated
impl serde::Serialize for CheckUpdatableFieldsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_function_id.is_empty() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.CheckUpdatableFieldsRequest", len)?;
        if !self.user_defined_function_id.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionId", &self.user_defined_function_id)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckUpdatableFieldsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function_id",
            "userDefinedFunctionId",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctionId,
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
                            "userDefinedFunctionId" | "user_defined_function_id" => Ok(GeneratedField::UserDefinedFunctionId),
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
            type Value = CheckUpdatableFieldsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.CheckUpdatableFieldsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckUpdatableFieldsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function_id__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctionId => {
                            if user_defined_function_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionId"));
                            }
                            user_defined_function_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CheckUpdatableFieldsRequest {
                    user_defined_function_id: user_defined_function_id__.unwrap_or_default(),
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.CheckUpdatableFieldsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckUpdatableFieldsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed_fields.is_some() {
            len += 1;
        }
        if !self.disallowed_fields.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.CheckUpdatableFieldsResponse", len)?;
        if let Some(v) = self.allowed_fields.as_ref() {
            struct_ser.serialize_field("allowedFields", v)?;
        }
        if !self.disallowed_fields.is_empty() {
            struct_ser.serialize_field("disallowedFields", &self.disallowed_fields)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckUpdatableFieldsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_fields",
            "allowedFields",
            "disallowed_fields",
            "disallowedFields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedFields,
            DisallowedFields,
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
                            "allowedFields" | "allowed_fields" => Ok(GeneratedField::AllowedFields),
                            "disallowedFields" | "disallowed_fields" => Ok(GeneratedField::DisallowedFields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckUpdatableFieldsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.CheckUpdatableFieldsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckUpdatableFieldsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_fields__ = None;
                let mut disallowed_fields__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedFields => {
                            if allowed_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedFields"));
                            }
                            allowed_fields__ = map_.next_value()?;
                        }
                        GeneratedField::DisallowedFields => {
                            if disallowed_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disallowedFields"));
                            }
                            disallowed_fields__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(CheckUpdatableFieldsResponse {
                    allowed_fields: allowed_fields__,
                    disallowed_fields: disallowed_fields__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.CheckUpdatableFieldsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserDefinedFunctionRequest {
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
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.function_inputs.is_empty() {
            len += 1;
        }
        if self.user_notes.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.CreateUserDefinedFunctionRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.function_inputs.is_empty() {
            struct_ser.serialize_field("functionInputs", &self.function_inputs)?;
        }
        if let Some(v) = self.user_notes.as_ref() {
            struct_ser.serialize_field("userNotes", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserDefinedFunctionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "expression",
            "function_inputs",
            "functionInputs",
            "user_notes",
            "userNotes",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Expression,
            FunctionInputs,
            UserNotes,
            Metadata,
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
                            "expression" => Ok(GeneratedField::Expression),
                            "functionInputs" | "function_inputs" => Ok(GeneratedField::FunctionInputs),
                            "userNotes" | "user_notes" => Ok(GeneratedField::UserNotes),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserDefinedFunctionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.CreateUserDefinedFunctionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserDefinedFunctionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut expression__ = None;
                let mut function_inputs__ = None;
                let mut user_notes__ = None;
                let mut metadata__ = None;
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
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionInputs => {
                            if function_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionInputs"));
                            }
                            function_inputs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserNotes => {
                            if user_notes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userNotes"));
                            }
                            user_notes__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateUserDefinedFunctionRequest {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    expression: expression__.unwrap_or_default(),
                    function_inputs: function_inputs__.unwrap_or_default(),
                    user_notes: user_notes__,
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.CreateUserDefinedFunctionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateUserDefinedFunctionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_defined_function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.CreateUserDefinedFunctionResponse", len)?;
        if let Some(v) = self.user_defined_function.as_ref() {
            struct_ser.serialize_field("userDefinedFunction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateUserDefinedFunctionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function",
            "userDefinedFunction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunction,
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
                            "userDefinedFunction" | "user_defined_function" => Ok(GeneratedField::UserDefinedFunction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateUserDefinedFunctionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.CreateUserDefinedFunctionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateUserDefinedFunctionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunction => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunction"));
                            }
                            user_defined_function__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateUserDefinedFunctionResponse {
                    user_defined_function: user_defined_function__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.CreateUserDefinedFunctionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionDependentsRequest {
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
        if self.user_defined_function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionDependentsRequest", len)?;
        if self.page_size != 0 {
            struct_ser.serialize_field("pageSize", &self.page_size)?;
        }
        if let Some(v) = self.user_defined_function.as_ref() {
            match v {
                get_user_defined_function_dependents_request::UserDefinedFunction::UserDefinedFunctionId(v) => {
                    struct_ser.serialize_field("userDefinedFunctionId", v)?;
                }
                get_user_defined_function_dependents_request::UserDefinedFunction::UserDefinedFunctionName(v) => {
                    struct_ser.serialize_field("userDefinedFunctionName", v)?;
                }
                get_user_defined_function_dependents_request::UserDefinedFunction::UserDefinedFunctionVersionId(v) => {
                    struct_ser.serialize_field("userDefinedFunctionVersionId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionDependentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "pageSize",
            "user_defined_function_id",
            "userDefinedFunctionId",
            "user_defined_function_name",
            "userDefinedFunctionName",
            "user_defined_function_version_id",
            "userDefinedFunctionVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            UserDefinedFunctionId,
            UserDefinedFunctionName,
            UserDefinedFunctionVersionId,
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
                            "userDefinedFunctionId" | "user_defined_function_id" => Ok(GeneratedField::UserDefinedFunctionId),
                            "userDefinedFunctionName" | "user_defined_function_name" => Ok(GeneratedField::UserDefinedFunctionName),
                            "userDefinedFunctionVersionId" | "user_defined_function_version_id" => Ok(GeneratedField::UserDefinedFunctionVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserDefinedFunctionDependentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionDependentsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionDependentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut user_defined_function__ = None;
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
                        GeneratedField::UserDefinedFunctionId => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionId"));
                            }
                            user_defined_function__ = map_.next_value::<::std::option::Option<_>>()?.map(get_user_defined_function_dependents_request::UserDefinedFunction::UserDefinedFunctionId);
                        }
                        GeneratedField::UserDefinedFunctionName => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionName"));
                            }
                            user_defined_function__ = map_.next_value::<::std::option::Option<_>>()?.map(get_user_defined_function_dependents_request::UserDefinedFunction::UserDefinedFunctionName);
                        }
                        GeneratedField::UserDefinedFunctionVersionId => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionVersionId"));
                            }
                            user_defined_function__ = map_.next_value::<::std::option::Option<_>>()?.map(get_user_defined_function_dependents_request::UserDefinedFunction::UserDefinedFunctionVersionId);
                        }
                    }
                }
                Ok(GetUserDefinedFunctionDependentsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    user_defined_function: user_defined_function__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionDependentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionDependentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_functions.is_empty() {
            len += 1;
        }
        if !self.next_page_token_user_defined_function.is_empty() {
            len += 1;
        }
        if !self.calculated_channels.is_empty() {
            len += 1;
        }
        if !self.next_page_token_calculated_channel.is_empty() {
            len += 1;
        }
        if !self.rules.is_empty() {
            len += 1;
        }
        if !self.next_page_token_rule.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionDependentsResponse", len)?;
        if !self.user_defined_functions.is_empty() {
            struct_ser.serialize_field("userDefinedFunctions", &self.user_defined_functions)?;
        }
        if !self.next_page_token_user_defined_function.is_empty() {
            struct_ser.serialize_field("nextPageTokenUserDefinedFunction", &self.next_page_token_user_defined_function)?;
        }
        if !self.calculated_channels.is_empty() {
            struct_ser.serialize_field("calculatedChannels", &self.calculated_channels)?;
        }
        if !self.next_page_token_calculated_channel.is_empty() {
            struct_ser.serialize_field("nextPageTokenCalculatedChannel", &self.next_page_token_calculated_channel)?;
        }
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if !self.next_page_token_rule.is_empty() {
            struct_ser.serialize_field("nextPageTokenRule", &self.next_page_token_rule)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionDependentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_functions",
            "userDefinedFunctions",
            "next_page_token_user_defined_function",
            "nextPageTokenUserDefinedFunction",
            "calculated_channels",
            "calculatedChannels",
            "next_page_token_calculated_channel",
            "nextPageTokenCalculatedChannel",
            "rules",
            "next_page_token_rule",
            "nextPageTokenRule",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctions,
            NextPageTokenUserDefinedFunction,
            CalculatedChannels,
            NextPageTokenCalculatedChannel,
            Rules,
            NextPageTokenRule,
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
                            "userDefinedFunctions" | "user_defined_functions" => Ok(GeneratedField::UserDefinedFunctions),
                            "nextPageTokenUserDefinedFunction" | "next_page_token_user_defined_function" => Ok(GeneratedField::NextPageTokenUserDefinedFunction),
                            "calculatedChannels" | "calculated_channels" => Ok(GeneratedField::CalculatedChannels),
                            "nextPageTokenCalculatedChannel" | "next_page_token_calculated_channel" => Ok(GeneratedField::NextPageTokenCalculatedChannel),
                            "rules" => Ok(GeneratedField::Rules),
                            "nextPageTokenRule" | "next_page_token_rule" => Ok(GeneratedField::NextPageTokenRule),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserDefinedFunctionDependentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionDependentsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionDependentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_functions__ = None;
                let mut next_page_token_user_defined_function__ = None;
                let mut calculated_channels__ = None;
                let mut next_page_token_calculated_channel__ = None;
                let mut rules__ = None;
                let mut next_page_token_rule__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctions => {
                            if user_defined_functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctions"));
                            }
                            user_defined_functions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageTokenUserDefinedFunction => {
                            if next_page_token_user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageTokenUserDefinedFunction"));
                            }
                            next_page_token_user_defined_function__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CalculatedChannels => {
                            if calculated_channels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("calculatedChannels"));
                            }
                            calculated_channels__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageTokenCalculatedChannel => {
                            if next_page_token_calculated_channel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageTokenCalculatedChannel"));
                            }
                            next_page_token_calculated_channel__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageTokenRule => {
                            if next_page_token_rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageTokenRule"));
                            }
                            next_page_token_rule__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserDefinedFunctionDependentsResponse {
                    user_defined_functions: user_defined_functions__.unwrap_or_default(),
                    next_page_token_user_defined_function: next_page_token_user_defined_function__.unwrap_or_default(),
                    calculated_channels: calculated_channels__.unwrap_or_default(),
                    next_page_token_calculated_channel: next_page_token_calculated_channel__.unwrap_or_default(),
                    rules: rules__.unwrap_or_default(),
                    next_page_token_rule: next_page_token_rule__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionDependentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_function_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionRequest", len)?;
        if !self.user_defined_function_id.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionId", &self.user_defined_function_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function_id",
            "userDefinedFunctionId",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctionId,
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
                            "userDefinedFunctionId" | "user_defined_function_id" => Ok(GeneratedField::UserDefinedFunctionId),
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
            type Value = GetUserDefinedFunctionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function_id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctionId => {
                            if user_defined_function_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionId"));
                            }
                            user_defined_function_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserDefinedFunctionRequest {
                    user_defined_function_id: user_defined_function_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_defined_function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionResponse", len)?;
        if let Some(v) = self.user_defined_function.as_ref() {
            struct_ser.serialize_field("userDefinedFunction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function",
            "userDefinedFunction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunction,
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
                            "userDefinedFunction" | "user_defined_function" => Ok(GeneratedField::UserDefinedFunction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserDefinedFunctionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunction => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunction"));
                            }
                            user_defined_function__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserDefinedFunctionResponse {
                    user_defined_function: user_defined_function__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_function_version_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionRequest", len)?;
        if !self.user_defined_function_version_id.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionVersionId", &self.user_defined_function_version_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function_version_id",
            "userDefinedFunctionVersionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctionVersionId,
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
                            "userDefinedFunctionVersionId" | "user_defined_function_version_id" => Ok(GeneratedField::UserDefinedFunctionVersionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserDefinedFunctionVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionVersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function_version_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctionVersionId => {
                            if user_defined_function_version_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionVersionId"));
                            }
                            user_defined_function_version_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserDefinedFunctionVersionRequest {
                    user_defined_function_version_id: user_defined_function_version_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_defined_function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionResponse", len)?;
        if let Some(v) = self.user_defined_function.as_ref() {
            struct_ser.serialize_field("userDefinedFunction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function",
            "userDefinedFunction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunction,
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
                            "userDefinedFunction" | "user_defined_function" => Ok(GeneratedField::UserDefinedFunction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserDefinedFunctionVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionVersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunction => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunction"));
                            }
                            user_defined_function__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetUserDefinedFunctionVersionResponse {
                    user_defined_function: user_defined_function__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionVersionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_function_version_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionsRequest", len)?;
        if !self.user_defined_function_version_ids.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionVersionIds", &self.user_defined_function_version_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionVersionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function_version_ids",
            "userDefinedFunctionVersionIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctionVersionIds,
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
                            "userDefinedFunctionVersionIds" | "user_defined_function_version_ids" => Ok(GeneratedField::UserDefinedFunctionVersionIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserDefinedFunctionVersionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionVersionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionVersionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function_version_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctionVersionIds => {
                            if user_defined_function_version_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionVersionIds"));
                            }
                            user_defined_function_version_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserDefinedFunctionVersionsRequest {
                    user_defined_function_version_ids: user_defined_function_version_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetUserDefinedFunctionVersionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_functions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionsResponse", len)?;
        if !self.user_defined_functions.is_empty() {
            struct_ser.serialize_field("userDefinedFunctions", &self.user_defined_functions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetUserDefinedFunctionVersionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_functions",
            "userDefinedFunctions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctions,
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
                            "userDefinedFunctions" | "user_defined_functions" => Ok(GeneratedField::UserDefinedFunctions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetUserDefinedFunctionVersionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.GetUserDefinedFunctionVersionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetUserDefinedFunctionVersionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_functions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctions => {
                            if user_defined_functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctions"));
                            }
                            user_defined_functions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetUserDefinedFunctionVersionsResponse {
                    user_defined_functions: user_defined_functions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.GetUserDefinedFunctionVersionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserDefinedFunctionVersionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_function_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
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
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionVersionsRequest", len)?;
        if !self.user_defined_function_id.is_empty() {
            struct_ser.serialize_field("userDefinedFunctionId", &self.user_defined_function_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
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
impl<'de> serde::Deserialize<'de> for ListUserDefinedFunctionVersionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function_id",
            "userDefinedFunctionId",
            "name",
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
            UserDefinedFunctionId,
            Name,
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
                            "userDefinedFunctionId" | "user_defined_function_id" => Ok(GeneratedField::UserDefinedFunctionId),
                            "name" => Ok(GeneratedField::Name),
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
            type Value = ListUserDefinedFunctionVersionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ListUserDefinedFunctionVersionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserDefinedFunctionVersionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function_id__ = None;
                let mut name__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                let mut order_by__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctionId => {
                            if user_defined_function_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctionId"));
                            }
                            user_defined_function_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
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
                Ok(ListUserDefinedFunctionVersionsRequest {
                    user_defined_function_id: user_defined_function_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionVersionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserDefinedFunctionVersionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_functions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionVersionsResponse", len)?;
        if !self.user_defined_functions.is_empty() {
            struct_ser.serialize_field("userDefinedFunctions", &self.user_defined_functions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserDefinedFunctionVersionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_functions",
            "userDefinedFunctions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctions,
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
                            "userDefinedFunctions" | "user_defined_functions" => Ok(GeneratedField::UserDefinedFunctions),
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
            type Value = ListUserDefinedFunctionVersionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ListUserDefinedFunctionVersionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserDefinedFunctionVersionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_functions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctions => {
                            if user_defined_functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctions"));
                            }
                            user_defined_functions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUserDefinedFunctionVersionsResponse {
                    user_defined_functions: user_defined_functions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionVersionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserDefinedFunctionsRequest {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListUserDefinedFunctionsRequest {
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
            type Value = ListUserDefinedFunctionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ListUserDefinedFunctionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserDefinedFunctionsRequest, V::Error>
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
                Ok(ListUserDefinedFunctionsRequest {
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                    order_by: order_by__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserDefinedFunctionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_defined_functions.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionsResponse", len)?;
        if !self.user_defined_functions.is_empty() {
            struct_ser.serialize_field("userDefinedFunctions", &self.user_defined_functions)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserDefinedFunctionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_functions",
            "userDefinedFunctions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunctions,
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
                            "userDefinedFunctions" | "user_defined_functions" => Ok(GeneratedField::UserDefinedFunctions),
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
            type Value = ListUserDefinedFunctionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ListUserDefinedFunctionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserDefinedFunctionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_functions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunctions => {
                            if user_defined_functions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunctions"));
                            }
                            user_defined_functions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUserDefinedFunctionsResponse {
                    user_defined_functions: user_defined_functions__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ListUserDefinedFunctionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserDefinedFunctionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_defined_function.is_some() {
            len += 1;
        }
        if self.update_mask.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.UpdateUserDefinedFunctionRequest", len)?;
        if let Some(v) = self.user_defined_function.as_ref() {
            struct_ser.serialize_field("userDefinedFunction", v)?;
        }
        if let Some(v) = self.update_mask.as_ref() {
            struct_ser.serialize_field("updateMask", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserDefinedFunctionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function",
            "userDefinedFunction",
            "update_mask",
            "updateMask",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunction,
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
                            "userDefinedFunction" | "user_defined_function" => Ok(GeneratedField::UserDefinedFunction),
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
            type Value = UpdateUserDefinedFunctionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.UpdateUserDefinedFunctionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserDefinedFunctionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function__ = None;
                let mut update_mask__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunction => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunction"));
                            }
                            user_defined_function__ = map_.next_value()?;
                        }
                        GeneratedField::UpdateMask => {
                            if update_mask__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMask"));
                            }
                            update_mask__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUserDefinedFunctionRequest {
                    user_defined_function: user_defined_function__,
                    update_mask: update_mask__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.UpdateUserDefinedFunctionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserDefinedFunctionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_defined_function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.UpdateUserDefinedFunctionResponse", len)?;
        if let Some(v) = self.user_defined_function.as_ref() {
            struct_ser.serialize_field("userDefinedFunction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserDefinedFunctionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function",
            "userDefinedFunction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunction,
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
                            "userDefinedFunction" | "user_defined_function" => Ok(GeneratedField::UserDefinedFunction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateUserDefinedFunctionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.UpdateUserDefinedFunctionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateUserDefinedFunctionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunction => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunction"));
                            }
                            user_defined_function__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateUserDefinedFunctionResponse {
                    user_defined_function: user_defined_function__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.UpdateUserDefinedFunctionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateUserDefinedFunctionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.function_inputs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionRequest", len)?;
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.function_inputs.is_empty() {
            struct_ser.serialize_field("functionInputs", &self.function_inputs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateUserDefinedFunctionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expression",
            "function_inputs",
            "functionInputs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Expression,
            FunctionInputs,
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
                            "expression" => Ok(GeneratedField::Expression),
                            "functionInputs" | "function_inputs" => Ok(GeneratedField::FunctionInputs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateUserDefinedFunctionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ValidateUserDefinedFunctionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateUserDefinedFunctionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expression__ = None;
                let mut function_inputs__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FunctionInputs => {
                            if function_inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("functionInputs"));
                            }
                            function_inputs__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidateUserDefinedFunctionRequest {
                    expression: expression__.unwrap_or_default(),
                    function_inputs: function_inputs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateUserDefinedFunctionResponse {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            match v {
                validate_user_defined_function_response::Result::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                validate_user_defined_function_response::Result::Success(v) => {
                    struct_ser.serialize_field("success", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateUserDefinedFunctionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
            "success",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            Success,
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
                            "error" => Ok(GeneratedField::Error),
                            "success" => Ok(GeneratedField::Success),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateUserDefinedFunctionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateUserDefinedFunctionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_user_defined_function_response::Result::Error)
;
                        }
                        GeneratedField::Success => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("success"));
                            }
                            result__ = map_.next_value::<::std::option::Option<_>>()?.map(validate_user_defined_function_response::Result::Success)
;
                        }
                    }
                }
                Ok(ValidateUserDefinedFunctionResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for validate_user_defined_function_response::ErrorValidatingUserDefinedFunctionResult {
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
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse.ErrorValidatingUserDefinedFunctionResult", len)?;
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for validate_user_defined_function_response::ErrorValidatingUserDefinedFunctionResult {
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
            type Value = validate_user_defined_function_response::ErrorValidatingUserDefinedFunctionResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse.ErrorValidatingUserDefinedFunctionResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<validate_user_defined_function_response::ErrorValidatingUserDefinedFunctionResult, V::Error>
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
                Ok(validate_user_defined_function_response::ErrorValidatingUserDefinedFunctionResult {
                    error_message: error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse.ErrorValidatingUserDefinedFunctionResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for validate_user_defined_function_response::SuccessValidatingUserDefinedFunctionResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_defined_function.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse.SuccessValidatingUserDefinedFunctionResult", len)?;
        if let Some(v) = self.user_defined_function.as_ref() {
            struct_ser.serialize_field("userDefinedFunction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for validate_user_defined_function_response::SuccessValidatingUserDefinedFunctionResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_defined_function",
            "userDefinedFunction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserDefinedFunction,
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
                            "userDefinedFunction" | "user_defined_function" => Ok(GeneratedField::UserDefinedFunction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = validate_user_defined_function_response::SuccessValidatingUserDefinedFunctionResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse.SuccessValidatingUserDefinedFunctionResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<validate_user_defined_function_response::SuccessValidatingUserDefinedFunctionResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_defined_function__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserDefinedFunction => {
                            if user_defined_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userDefinedFunction"));
                            }
                            user_defined_function__ = map_.next_value()?;
                        }
                    }
                }
                Ok(validate_user_defined_function_response::SuccessValidatingUserDefinedFunctionResult {
                    user_defined_function: user_defined_function__,
                })
            }
        }
        deserializer.deserialize_struct("sift.user_defined_functions.v1.ValidateUserDefinedFunctionResponse.SuccessValidatingUserDefinedFunctionResult", FIELDS, GeneratedVisitor)
    }
}
