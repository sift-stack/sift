// @generated
/// `Constraint` represents a validation rule written in the Common Expression
/// Language (CEL) syntax. Each Constraint includes a unique identifier, an
/// optional error message, and the CEL expression to evaluate. For more
/// information on CEL, [see our documentation](<https://github.com/bufbuild/protovalidate/blob/main/docs/cel.md>).
///
/// ```proto
/// message Foo {
///    option (buf.validate.message).cel = {
///      id: "foo.bar"
///      message: "bar must be greater than 0"
///      expression: "this.bar > 0"
///    };
///    int32 bar = 1;
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraint {
    /// `id` is a string that serves as a machine-readable name for this Constraint.
    /// It should be unique within its scope, which could be either a message or a field.
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// `message` is an optional field that provides a human-readable error message
    /// for this Constraint when the CEL expression evaluates to false. If a
    /// non-empty message is provided, any strings resulting from the CEL
    /// expression evaluation are ignored.
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// `expression` is the actual CEL expression that will be evaluated for
    /// validation. This string must resolve to either a boolean or a string
    /// value. If the expression evaluates to false or a non-empty string, the
    /// validation is considered failed, and the message is rejected.
    #[prost(string, optional, tag="3")]
    pub expression: ::core::option::Option<::prost::alloc::string::String>,
}
/// MessageConstraints represents validation rules that are applied to the entire message.
/// It includes disabling options and a list of Constraint messages representing Common Expression Language (CEL) validation rules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageConstraints {
    /// `disabled` is a boolean flag that, when set to true, nullifies any validation rules for this message.
    /// This includes any fields within the message that would otherwise support validation.
    ///
    /// ```proto
    /// message MyMessage {
    ///    // validation will be bypassed for this message
    ///    option (buf.validate.message).disabled = true;
    /// }
    /// ```
    #[prost(bool, optional, tag="1")]
    pub disabled: ::core::option::Option<bool>,
    /// `cel` is a repeated field of type Constraint. Each Constraint specifies a validation rule to be applied to this message.
    /// These constraints are written in Common Expression Language (CEL) syntax. For more information on
    /// CEL, [see our documentation](<https://github.com/bufbuild/protovalidate/blob/main/docs/cel.md>).
    ///
    ///
    /// ```proto
    /// message MyMessage {
    ///    // The field `foo` must be greater than 42.
    ///    option (buf.validate.message).cel = {
    ///      id: "my_message.value",
    ///      message: "value must be greater than 42",
    ///      expression: "this.foo > 42",
    ///    };
    ///    optional int32 foo = 1;
    /// }
    /// ```
    #[prost(message, repeated, tag="3")]
    pub cel: ::prost::alloc::vec::Vec<Constraint>,
}
/// The `OneofConstraints` message type enables you to manage constraints for
/// oneof fields in your protobuf messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneofConstraints {
    /// If `required` is true, exactly one field of the oneof must be present. A
    /// validation error is returned if no fields in the oneof are present. The
    /// field itself may still be a default value; further constraints
    /// should be placed on the fields themselves to ensure they are valid values,
    /// such as `min_len` or `gt`.
    ///
    /// ```proto
    /// message MyMessage {
    ///    oneof value {
    ///      // Either `a` or `b` must be set. If `a` is set, it must also be
    ///      // non-empty; whereas if `b` is set, it can still be an empty string.
    ///      option (buf.validate.oneof).required = true;
    ///      string a = 1 \[(buf.validate.field).string.min_len = 1\];
    ///      string b = 2;
    ///    }
    /// }
    /// ```
    #[prost(bool, optional, tag="1")]
    pub required: ::core::option::Option<bool>,
}
/// FieldConstraints encapsulates the rules for each type of field. Depending on
/// the field, the correct set should be used to ensure proper validations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldConstraints {
    /// `cel` is a repeated field used to represent a textual expression
    /// in the Common Expression Language (CEL) syntax. For more information on
    /// CEL, [see our documentation](<https://github.com/bufbuild/protovalidate/blob/main/docs/cel.md>).
    ///
    /// ```proto
    /// message MyMessage {
    ///    // The field `value` must be greater than 42.
    ///    optional int32 value = 1 [(buf.validate.field).cel = {
    ///      id: "my_message.value",
    ///      message: "value must be greater than 42",
    ///      expression: "this > 42",
    ///    }];
    /// }
    /// ```
    #[prost(message, repeated, tag="23")]
    pub cel: ::prost::alloc::vec::Vec<Constraint>,
    /// If `required` is true, the field must be populated. A populated field can be
    /// described as "serialized in the wire format," which includes:
    ///
    /// - the following "nullable" fields must be explicitly set to be considered populated:
    ///    - singular message fields (whose fields may be unpopulated/default values)
    ///    - member fields of a oneof (may be their default value)
    ///    - proto3 optional fields (may be their default value)
    ///    - proto2 scalar fields (both optional and required)
    /// - proto3 scalar fields must be non-zero to be considered populated
    /// - repeated and map fields must be non-empty to be considered populated
    ///
    /// ```proto
    /// message MyMessage {
    ///    // The field `value` must be set to a non-null value.
    ///    optional MyOtherMessage value = 1 \[(buf.validate.field).required = true\];
    /// }
    /// ```
    #[prost(bool, optional, tag="25")]
    pub required: ::core::option::Option<bool>,
    /// Skip validation on the field if its value matches the specified criteria.
    /// See Ignore enum for details.
    ///
    /// ```proto
    /// message UpdateRequest {
    ///    // The uri rule only applies if the field is populated and not an empty
    ///    // string.
    ///    optional string url = 1 [
    ///      (buf.validate.field).ignore = IGNORE_IF_DEFAULT_VALUE,
    ///      (buf.validate.field).string.uri = true,
    ///    ];
    /// }
    /// ```
    #[prost(enumeration="Ignore", optional, tag="27")]
    pub ignore: ::core::option::Option<i32>,
    #[prost(oneof="field_constraints::Type", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22")]
    pub r#type: ::core::option::Option<field_constraints::Type>,
}
/// Nested message and enum types in `FieldConstraints`.
pub mod field_constraints {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Scalar Field Types
        #[prost(message, tag="1")]
        Float(super::FloatRules),
        #[prost(message, tag="2")]
        Double(super::DoubleRules),
        #[prost(message, tag="3")]
        Int32(super::Int32Rules),
        #[prost(message, tag="4")]
        Int64(super::Int64Rules),
        #[prost(message, tag="5")]
        Uint32(super::UInt32Rules),
        #[prost(message, tag="6")]
        Uint64(super::UInt64Rules),
        #[prost(message, tag="7")]
        Sint32(super::SInt32Rules),
        #[prost(message, tag="8")]
        Sint64(super::SInt64Rules),
        #[prost(message, tag="9")]
        Fixed32(super::Fixed32Rules),
        #[prost(message, tag="10")]
        Fixed64(super::Fixed64Rules),
        #[prost(message, tag="11")]
        Sfixed32(super::SFixed32Rules),
        #[prost(message, tag="12")]
        Sfixed64(super::SFixed64Rules),
        #[prost(message, tag="13")]
        Bool(super::BoolRules),
        #[prost(message, tag="14")]
        String(super::StringRules),
        #[prost(message, tag="15")]
        Bytes(super::BytesRules),
        /// Complex Field Types
        #[prost(message, tag="16")]
        Enum(super::EnumRules),
        #[prost(message, tag="18")]
        Repeated(::prost::alloc::boxed::Box<super::RepeatedRules>),
        #[prost(message, tag="19")]
        Map(::prost::alloc::boxed::Box<super::MapRules>),
        /// Well-Known Field Types
        #[prost(message, tag="20")]
        Any(super::AnyRules),
        #[prost(message, tag="21")]
        Duration(super::DurationRules),
        #[prost(message, tag="22")]
        Timestamp(super::TimestampRules),
    }
}
/// PredefinedConstraints are custom constraints that can be re-used with
/// multiple fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredefinedConstraints {
    /// `cel` is a repeated field used to represent a textual expression
    /// in the Common Expression Language (CEL) syntax. For more information on
    /// CEL, [see our documentation](<https://github.com/bufbuild/protovalidate/blob/main/docs/cel.md>).
    ///
    /// ```proto
    /// message MyMessage {
    ///    // The field `value` must be greater than 42.
    ///    optional int32 value = 1 [(buf.validate.predefined).cel = {
    ///      id: "my_message.value",
    ///      message: "value must be greater than 42",
    ///      expression: "this > 42",
    ///    }];
    /// }
    /// ```
    #[prost(message, repeated, tag="1")]
    pub cel: ::prost::alloc::vec::Vec<Constraint>,
}
/// FloatRules describes the constraints applied to `float` values. These
/// rules may also be applied to the `google.protobuf.FloatValue` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatRules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyFloat {
    ///    // value must equal 42.0
    ///    float value = 1 \[(buf.validate.field).float.const = 42.0\];
    /// }
    /// ```
    #[prost(float, optional, tag="1")]
    pub r#const: ::core::option::Option<f32>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message
    /// is generated.
    ///
    /// ```proto
    /// message MyFloat {
    ///    // value must be in list \[1.0, 2.0, 3.0\]
    ///    repeated float value = 1 (buf.validate.field).float = { in: \[1.0, 2.0, 3.0\] };
    /// }
    /// ```
    #[prost(float, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<f32>,
    /// `in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyFloat {
    ///    // value must not be in list \[1.0, 2.0, 3.0\]
    ///    repeated float value = 1 (buf.validate.field).float = { not_in: \[1.0, 2.0, 3.0\] };
    /// }
    /// ```
    #[prost(float, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<f32>,
    /// `finite` requires the field value to be finite. If the field value is
    /// infinite or NaN, an error message is generated.
    #[prost(bool, optional, tag="8")]
    pub finite: ::core::option::Option<bool>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyFloat {
    ///    float value = 1 [
    ///      (buf.validate.field).float.example = 1.0,
    ///      (buf.validate.field).float.example = "Infinity"
    ///    ];
    /// }
    /// ```
    #[prost(float, repeated, packed="false", tag="9")]
    pub example: ::prost::alloc::vec::Vec<f32>,
    #[prost(oneof="float_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<float_rules::LessThan>,
    #[prost(oneof="float_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<float_rules::GreaterThan>,
}
/// Nested message and enum types in `FloatRules`.
pub mod float_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFloat {
        ///    // value must be less than 10.0
        ///    float value = 1 \[(buf.validate.field).float.lt = 10.0\];
        /// }
        /// ```
        #[prost(float, tag="2")]
        Lt(f32),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyFloat {
        ///    // value must be less than or equal to 10.0
        ///    float value = 1 \[(buf.validate.field).float.lte = 10.0\];
        /// }
        /// ```
        #[prost(float, tag="3")]
        Lte(f32),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFloat {
        ///    // value must be greater than 5.0 \[float.gt\]
        ///    float value = 1 \[(buf.validate.field).float.gt = 5.0\];
        ///
        ///    // value must be greater than 5 and less than 10.0 \[float.gt_lt\]
        ///    float other_value = 2 \[(buf.validate.field).float = { gt: 5.0, lt: 10.0 }\];
        ///
        ///    // value must be greater than 10 or less than 5.0 \[float.gt_lt_exclusive\]
        ///    float another_value = 3 \[(buf.validate.field).float = { gt: 10.0, lt: 5.0 }\];
        /// }
        /// ```
        #[prost(float, tag="4")]
        Gt(f32),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFloat {
        ///    // value must be greater than or equal to 5.0 \[float.gte\]
        ///    float value = 1 \[(buf.validate.field).float.gte = 5.0\];
        ///
        ///    // value must be greater than or equal to 5.0 and less than 10.0 \[float.gte_lt\]
        ///    float other_value = 2 \[(buf.validate.field).float = { gte: 5.0, lt: 10.0 }\];
        ///
        ///    // value must be greater than or equal to 10.0 or less than 5.0 \[float.gte_lt_exclusive\]
        ///    float another_value = 3 \[(buf.validate.field).float = { gte: 10.0, lt: 5.0 }\];
        /// }
        /// ```
        #[prost(float, tag="5")]
        Gte(f32),
    }
}
/// DoubleRules describes the constraints applied to `double` values. These
/// rules may also be applied to the `google.protobuf.DoubleValue` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleRules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyDouble {
    ///    // value must equal 42.0
    ///    double value = 1 \[(buf.validate.field).double.const = 42.0\];
    /// }
    /// ```
    #[prost(double, optional, tag="1")]
    pub r#const: ::core::option::Option<f64>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MyDouble {
    ///    // value must be in list \[1.0, 2.0, 3.0\]
    ///    repeated double value = 1 (buf.validate.field).double = { in: \[1.0, 2.0, 3.0\] };
    /// }
    /// ```
    #[prost(double, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<f64>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyDouble {
    ///    // value must not be in list \[1.0, 2.0, 3.0\]
    ///    repeated double value = 1 (buf.validate.field).double = { not_in: \[1.0, 2.0, 3.0\] };
    /// }
    /// ```
    #[prost(double, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<f64>,
    /// `finite` requires the field value to be finite. If the field value is
    /// infinite or NaN, an error message is generated.
    #[prost(bool, optional, tag="8")]
    pub finite: ::core::option::Option<bool>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyDouble {
    ///    double value = 1 [
    ///      (buf.validate.field).double.example = 1.0,
    ///      (buf.validate.field).double.example = "Infinity"
    ///    ];
    /// }
    /// ```
    #[prost(double, repeated, packed="false", tag="9")]
    pub example: ::prost::alloc::vec::Vec<f64>,
    #[prost(oneof="double_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<double_rules::LessThan>,
    #[prost(oneof="double_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<double_rules::GreaterThan>,
}
/// Nested message and enum types in `DoubleRules`.
pub mod double_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyDouble {
        ///    // value must be less than 10.0
        ///    double value = 1 \[(buf.validate.field).double.lt = 10.0\];
        /// }
        /// ```
        #[prost(double, tag="2")]
        Lt(f64),
        /// `lte` requires the field value to be less than or equal to the specified value
        /// (field <= value). If the field value is greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyDouble {
        ///    // value must be less than or equal to 10.0
        ///    double value = 1 \[(buf.validate.field).double.lte = 10.0\];
        /// }
        /// ```
        #[prost(double, tag="3")]
        Lte(f64),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or `lte`,
        /// the range is reversed, and the field value must be outside the specified
        /// range. If the field value doesn't meet the required conditions, an error
        /// message is generated.
        ///
        /// ```proto
        /// message MyDouble {
        ///    // value must be greater than 5.0 \[double.gt\]
        ///    double value = 1 \[(buf.validate.field).double.gt = 5.0\];
        ///
        ///    // value must be greater than 5 and less than 10.0 \[double.gt_lt\]
        ///    double other_value = 2 \[(buf.validate.field).double = { gt: 5.0, lt: 10.0 }\];
        ///
        ///    // value must be greater than 10 or less than 5.0 \[double.gt_lt_exclusive\]
        ///    double another_value = 3 \[(buf.validate.field).double = { gt: 10.0, lt: 5.0 }\];
        /// }
        /// ```
        #[prost(double, tag="4")]
        Gt(f64),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyDouble {
        ///    // value must be greater than or equal to 5.0 \[double.gte\]
        ///    double value = 1 \[(buf.validate.field).double.gte = 5.0\];
        ///
        ///    // value must be greater than or equal to 5.0 and less than 10.0 \[double.gte_lt\]
        ///    double other_value = 2 \[(buf.validate.field).double = { gte: 5.0, lt: 10.0 }\];
        ///
        ///    // value must be greater than or equal to 10.0 or less than 5.0 \[double.gte_lt_exclusive\]
        ///    double another_value = 3 \[(buf.validate.field).double = { gte: 10.0, lt: 5.0 }\];
        /// }
        /// ```
        #[prost(double, tag="5")]
        Gte(f64),
    }
}
/// Int32Rules describes the constraints applied to `int32` values. These
/// rules may also be applied to the `google.protobuf.Int32Value` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyInt32 {
    ///    // value must equal 42
    ///    int32 value = 1 \[(buf.validate.field).int32.const = 42\];
    /// }
    /// ```
    #[prost(int32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MyInt32 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated int32 value = 1 (buf.validate.field).int32 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(int32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error message
    /// is generated.
    ///
    /// ```proto
    /// message MyInt32 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated int32 value = 1 (buf.validate.field).int32 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(int32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyInt32 {
    ///    int32 value = 1 [
    ///      (buf.validate.field).int32.example = 1,
    ///      (buf.validate.field).int32.example = -10
    ///    ];
    /// }
    /// ```
    #[prost(int32, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<i32>,
    #[prost(oneof="int32_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<int32_rules::LessThan>,
    #[prost(oneof="int32_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<int32_rules::GreaterThan>,
}
/// Nested message and enum types in `Int32Rules`.
pub mod int32_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field
        /// < value). If the field value is equal to or greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyInt32 {
        ///    // value must be less than 10
        ///    int32 value = 1 \[(buf.validate.field).int32.lt = 10\];
        /// }
        /// ```
        #[prost(int32, tag="2")]
        Lt(i32),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyInt32 {
        ///    // value must be less than or equal to 10
        ///    int32 value = 1 \[(buf.validate.field).int32.lte = 10\];
        /// }
        /// ```
        #[prost(int32, tag="3")]
        Lte(i32),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyInt32 {
        ///    // value must be greater than 5 \[int32.gt\]
        ///    int32 value = 1 \[(buf.validate.field).int32.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[int32.gt_lt\]
        ///    int32 other_value = 2 \[(buf.validate.field).int32 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[int32.gt_lt_exclusive\]
        ///    int32 another_value = 3 \[(buf.validate.field).int32 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(int32, tag="4")]
        Gt(i32),
        /// `gte` requires the field value to be greater than or equal to the specified value
        /// (exclusive). If the value of `gte` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyInt32 {
        ///    // value must be greater than or equal to 5 \[int32.gte\]
        ///    int32 value = 1 \[(buf.validate.field).int32.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[int32.gte_lt\]
        ///    int32 other_value = 2 \[(buf.validate.field).int32 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[int32.gte_lt_exclusive\]
        ///    int32 another_value = 3 \[(buf.validate.field).int32 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(int32, tag="5")]
        Gte(i32),
    }
}
/// Int64Rules describes the constraints applied to `int64` values. These
/// rules may also be applied to the `google.protobuf.Int64Value` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyInt64 {
    ///    // value must equal 42
    ///    int64 value = 1 \[(buf.validate.field).int64.const = 42\];
    /// }
    /// ```
    #[prost(int64, optional, tag="1")]
    pub r#const: ::core::option::Option<i64>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MyInt64 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated int64 value = 1 (buf.validate.field).int64 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(int64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i64>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyInt64 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated int64 value = 1 (buf.validate.field).int64 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(int64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i64>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyInt64 {
    ///    int64 value = 1 [
    ///      (buf.validate.field).int64.example = 1,
    ///      (buf.validate.field).int64.example = -10
    ///    ];
    /// }
    /// ```
    #[prost(int64, repeated, packed="false", tag="9")]
    pub example: ::prost::alloc::vec::Vec<i64>,
    #[prost(oneof="int64_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<int64_rules::LessThan>,
    #[prost(oneof="int64_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<int64_rules::GreaterThan>,
}
/// Nested message and enum types in `Int64Rules`.
pub mod int64_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyInt64 {
        ///    // value must be less than 10
        ///    int64 value = 1 \[(buf.validate.field).int64.lt = 10\];
        /// }
        /// ```
        #[prost(int64, tag="2")]
        Lt(i64),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyInt64 {
        ///    // value must be less than or equal to 10
        ///    int64 value = 1 \[(buf.validate.field).int64.lte = 10\];
        /// }
        /// ```
        #[prost(int64, tag="3")]
        Lte(i64),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyInt64 {
        ///    // value must be greater than 5 \[int64.gt\]
        ///    int64 value = 1 \[(buf.validate.field).int64.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[int64.gt_lt\]
        ///    int64 other_value = 2 \[(buf.validate.field).int64 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[int64.gt_lt_exclusive\]
        ///    int64 another_value = 3 \[(buf.validate.field).int64 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(int64, tag="4")]
        Gt(i64),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyInt64 {
        ///    // value must be greater than or equal to 5 \[int64.gte\]
        ///    int64 value = 1 \[(buf.validate.field).int64.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[int64.gte_lt\]
        ///    int64 other_value = 2 \[(buf.validate.field).int64 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[int64.gte_lt_exclusive\]
        ///    int64 another_value = 3 \[(buf.validate.field).int64 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(int64, tag="5")]
        Gte(i64),
    }
}
/// UInt32Rules describes the constraints applied to `uint32` values. These
/// rules may also be applied to the `google.protobuf.UInt32Value` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyUInt32 {
    ///    // value must equal 42
    ///    uint32 value = 1 \[(buf.validate.field).uint32.const = 42\];
    /// }
    /// ```
    #[prost(uint32, optional, tag="1")]
    pub r#const: ::core::option::Option<u32>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MyUInt32 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated uint32 value = 1 (buf.validate.field).uint32 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(uint32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u32>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyUInt32 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated uint32 value = 1 (buf.validate.field).uint32 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(uint32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u32>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyUInt32 {
    ///    uint32 value = 1 [
    ///      (buf.validate.field).uint32.example = 1,
    ///      (buf.validate.field).uint32.example = 10
    ///    ];
    /// }
    /// ```
    #[prost(uint32, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<u32>,
    #[prost(oneof="u_int32_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<u_int32_rules::LessThan>,
    #[prost(oneof="u_int32_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<u_int32_rules::GreaterThan>,
}
/// Nested message and enum types in `UInt32Rules`.
pub mod u_int32_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyUInt32 {
        ///    // value must be less than 10
        ///    uint32 value = 1 \[(buf.validate.field).uint32.lt = 10\];
        /// }
        /// ```
        #[prost(uint32, tag="2")]
        Lt(u32),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyUInt32 {
        ///    // value must be less than or equal to 10
        ///    uint32 value = 1 \[(buf.validate.field).uint32.lte = 10\];
        /// }
        /// ```
        #[prost(uint32, tag="3")]
        Lte(u32),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyUInt32 {
        ///    // value must be greater than 5 \[uint32.gt\]
        ///    uint32 value = 1 \[(buf.validate.field).uint32.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[uint32.gt_lt\]
        ///    uint32 other_value = 2 \[(buf.validate.field).uint32 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[uint32.gt_lt_exclusive\]
        ///    uint32 another_value = 3 \[(buf.validate.field).uint32 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(uint32, tag="4")]
        Gt(u32),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyUInt32 {
        ///    // value must be greater than or equal to 5 \[uint32.gte\]
        ///    uint32 value = 1 \[(buf.validate.field).uint32.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[uint32.gte_lt\]
        ///    uint32 other_value = 2 \[(buf.validate.field).uint32 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[uint32.gte_lt_exclusive\]
        ///    uint32 another_value = 3 \[(buf.validate.field).uint32 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(uint32, tag="5")]
        Gte(u32),
    }
}
/// UInt64Rules describes the constraints applied to `uint64` values. These
/// rules may also be applied to the `google.protobuf.UInt64Value` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyUInt64 {
    ///    // value must equal 42
    ///    uint64 value = 1 \[(buf.validate.field).uint64.const = 42\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="1")]
    pub r#const: ::core::option::Option<u64>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MyUInt64 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated uint64 value = 1 (buf.validate.field).uint64 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(uint64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u64>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyUInt64 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated uint64 value = 1 (buf.validate.field).uint64 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(uint64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u64>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyUInt64 {
    ///    uint64 value = 1 [
    ///      (buf.validate.field).uint64.example = 1,
    ///      (buf.validate.field).uint64.example = -10
    ///    ];
    /// }
    /// ```
    #[prost(uint64, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<u64>,
    #[prost(oneof="u_int64_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<u_int64_rules::LessThan>,
    #[prost(oneof="u_int64_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<u_int64_rules::GreaterThan>,
}
/// Nested message and enum types in `UInt64Rules`.
pub mod u_int64_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyUInt64 {
        ///    // value must be less than 10
        ///    uint64 value = 1 \[(buf.validate.field).uint64.lt = 10\];
        /// }
        /// ```
        #[prost(uint64, tag="2")]
        Lt(u64),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyUInt64 {
        ///    // value must be less than or equal to 10
        ///    uint64 value = 1 \[(buf.validate.field).uint64.lte = 10\];
        /// }
        /// ```
        #[prost(uint64, tag="3")]
        Lte(u64),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyUInt64 {
        ///    // value must be greater than 5 \[uint64.gt\]
        ///    uint64 value = 1 \[(buf.validate.field).uint64.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[uint64.gt_lt\]
        ///    uint64 other_value = 2 \[(buf.validate.field).uint64 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[uint64.gt_lt_exclusive\]
        ///    uint64 another_value = 3 \[(buf.validate.field).uint64 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(uint64, tag="4")]
        Gt(u64),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyUInt64 {
        ///    // value must be greater than or equal to 5 \[uint64.gte\]
        ///    uint64 value = 1 \[(buf.validate.field).uint64.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[uint64.gte_lt\]
        ///    uint64 other_value = 2 \[(buf.validate.field).uint64 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[uint64.gte_lt_exclusive\]
        ///    uint64 another_value = 3 \[(buf.validate.field).uint64 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(uint64, tag="5")]
        Gte(u64),
    }
}
/// SInt32Rules describes the constraints applied to `sint32` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SInt32Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MySInt32 {
    ///    // value must equal 42
    ///    sint32 value = 1 \[(buf.validate.field).sint32.const = 42\];
    /// }
    /// ```
    #[prost(sint32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MySInt32 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated sint32 value = 1 (buf.validate.field).sint32 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sint32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MySInt32 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated sint32 value = 1 (buf.validate.field).sint32 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sint32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MySInt32 {
    ///    sint32 value = 1 [
    ///      (buf.validate.field).sint32.example = 1,
    ///      (buf.validate.field).sint32.example = -10
    ///    ];
    /// }
    /// ```
    #[prost(sint32, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<i32>,
    #[prost(oneof="s_int32_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<s_int32_rules::LessThan>,
    #[prost(oneof="s_int32_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<s_int32_rules::GreaterThan>,
}
/// Nested message and enum types in `SInt32Rules`.
pub mod s_int32_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field
        /// < value). If the field value is equal to or greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MySInt32 {
        ///    // value must be less than 10
        ///    sint32 value = 1 \[(buf.validate.field).sint32.lt = 10\];
        /// }
        /// ```
        #[prost(sint32, tag="2")]
        Lt(i32),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MySInt32 {
        ///    // value must be less than or equal to 10
        ///    sint32 value = 1 \[(buf.validate.field).sint32.lte = 10\];
        /// }
        /// ```
        #[prost(sint32, tag="3")]
        Lte(i32),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySInt32 {
        ///    // value must be greater than 5 \[sint32.gt\]
        ///    sint32 value = 1 \[(buf.validate.field).sint32.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[sint32.gt_lt\]
        ///    sint32 other_value = 2 \[(buf.validate.field).sint32 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[sint32.gt_lt_exclusive\]
        ///    sint32 another_value = 3 \[(buf.validate.field).sint32 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sint32, tag="4")]
        Gt(i32),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySInt32 {
        ///   // value must be greater than or equal to 5 \[sint32.gte\]
        ///   sint32 value = 1 \[(buf.validate.field).sint32.gte = 5\];
        ///
        ///   // value must be greater than or equal to 5 and less than 10 \[sint32.gte_lt\]
        ///   sint32 other_value = 2 \[(buf.validate.field).sint32 = { gte: 5, lt: 10 }\];
        ///
        ///   // value must be greater than or equal to 10 or less than 5 \[sint32.gte_lt_exclusive\]
        ///   sint32 another_value = 3 \[(buf.validate.field).sint32 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sint32, tag="5")]
        Gte(i32),
    }
}
/// SInt64Rules describes the constraints applied to `sint64` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SInt64Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MySInt64 {
    ///    // value must equal 42
    ///    sint64 value = 1 \[(buf.validate.field).sint64.const = 42\];
    /// }
    /// ```
    #[prost(sint64, optional, tag="1")]
    pub r#const: ::core::option::Option<i64>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message
    /// is generated.
    ///
    /// ```proto
    /// message MySInt64 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated sint64 value = 1 (buf.validate.field).sint64 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sint64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i64>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MySInt64 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated sint64 value = 1 (buf.validate.field).sint64 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sint64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i64>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MySInt64 {
    ///    sint64 value = 1 [
    ///      (buf.validate.field).sint64.example = 1,
    ///      (buf.validate.field).sint64.example = -10
    ///    ];
    /// }
    /// ```
    #[prost(sint64, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<i64>,
    #[prost(oneof="s_int64_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<s_int64_rules::LessThan>,
    #[prost(oneof="s_int64_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<s_int64_rules::GreaterThan>,
}
/// Nested message and enum types in `SInt64Rules`.
pub mod s_int64_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field
        /// < value). If the field value is equal to or greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MySInt64 {
        ///    // value must be less than 10
        ///    sint64 value = 1 \[(buf.validate.field).sint64.lt = 10\];
        /// }
        /// ```
        #[prost(sint64, tag="2")]
        Lt(i64),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MySInt64 {
        ///    // value must be less than or equal to 10
        ///    sint64 value = 1 \[(buf.validate.field).sint64.lte = 10\];
        /// }
        /// ```
        #[prost(sint64, tag="3")]
        Lte(i64),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySInt64 {
        ///    // value must be greater than 5 \[sint64.gt\]
        ///    sint64 value = 1 \[(buf.validate.field).sint64.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[sint64.gt_lt\]
        ///    sint64 other_value = 2 \[(buf.validate.field).sint64 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[sint64.gt_lt_exclusive\]
        ///    sint64 another_value = 3 \[(buf.validate.field).sint64 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sint64, tag="4")]
        Gt(i64),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySInt64 {
        ///    // value must be greater than or equal to 5 \[sint64.gte\]
        ///    sint64 value = 1 \[(buf.validate.field).sint64.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[sint64.gte_lt\]
        ///    sint64 other_value = 2 \[(buf.validate.field).sint64 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[sint64.gte_lt_exclusive\]
        ///    sint64 another_value = 3 \[(buf.validate.field).sint64 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sint64, tag="5")]
        Gte(i64),
    }
}
/// Fixed32Rules describes the constraints applied to `fixed32` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fixed32Rules {
    /// `const` requires the field value to exactly match the specified value.
    /// If the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyFixed32 {
    ///    // value must equal 42
    ///    fixed32 value = 1 \[(buf.validate.field).fixed32.const = 42\];
    /// }
    /// ```
    #[prost(fixed32, optional, tag="1")]
    pub r#const: ::core::option::Option<u32>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message
    /// is generated.
    ///
    /// ```proto
    /// message MyFixed32 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated fixed32 value = 1 (buf.validate.field).fixed32 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(fixed32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u32>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyFixed32 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated fixed32 value = 1 (buf.validate.field).fixed32 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(fixed32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u32>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyFixed32 {
    ///    fixed32 value = 1 [
    ///      (buf.validate.field).fixed32.example = 1,
    ///      (buf.validate.field).fixed32.example = 2
    ///    ];
    /// }
    /// ```
    #[prost(fixed32, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<u32>,
    #[prost(oneof="fixed32_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<fixed32_rules::LessThan>,
    #[prost(oneof="fixed32_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<fixed32_rules::GreaterThan>,
}
/// Nested message and enum types in `Fixed32Rules`.
pub mod fixed32_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFixed32 {
        ///    // value must be less than 10
        ///    fixed32 value = 1 \[(buf.validate.field).fixed32.lt = 10\];
        /// }
        /// ```
        #[prost(fixed32, tag="2")]
        Lt(u32),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyFixed32 {
        ///    // value must be less than or equal to 10
        ///    fixed32 value = 1 \[(buf.validate.field).fixed32.lte = 10\];
        /// }
        /// ```
        #[prost(fixed32, tag="3")]
        Lte(u32),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFixed32 {
        ///    // value must be greater than 5 \[fixed32.gt\]
        ///    fixed32 value = 1 \[(buf.validate.field).fixed32.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[fixed32.gt_lt\]
        ///    fixed32 other_value = 2 \[(buf.validate.field).fixed32 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[fixed32.gt_lt_exclusive\]
        ///    fixed32 another_value = 3 \[(buf.validate.field).fixed32 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(fixed32, tag="4")]
        Gt(u32),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFixed32 {
        ///    // value must be greater than or equal to 5 \[fixed32.gte\]
        ///    fixed32 value = 1 \[(buf.validate.field).fixed32.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[fixed32.gte_lt\]
        ///    fixed32 other_value = 2 \[(buf.validate.field).fixed32 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[fixed32.gte_lt_exclusive\]
        ///    fixed32 another_value = 3 \[(buf.validate.field).fixed32 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(fixed32, tag="5")]
        Gte(u32),
    }
}
/// Fixed64Rules describes the constraints applied to `fixed64` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fixed64Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyFixed64 {
    ///    // value must equal 42
    ///    fixed64 value = 1 \[(buf.validate.field).fixed64.const = 42\];
    /// }
    /// ```
    #[prost(fixed64, optional, tag="1")]
    pub r#const: ::core::option::Option<u64>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MyFixed64 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated fixed64 value = 1 (buf.validate.field).fixed64 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(fixed64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<u64>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyFixed64 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated fixed64 value = 1 (buf.validate.field).fixed64 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(fixed64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<u64>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyFixed64 {
    ///    fixed64 value = 1 [
    ///      (buf.validate.field).fixed64.example = 1,
    ///      (buf.validate.field).fixed64.example = 2
    ///    ];
    /// }
    /// ```
    #[prost(fixed64, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<u64>,
    #[prost(oneof="fixed64_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<fixed64_rules::LessThan>,
    #[prost(oneof="fixed64_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<fixed64_rules::GreaterThan>,
}
/// Nested message and enum types in `Fixed64Rules`.
pub mod fixed64_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFixed64 {
        ///    // value must be less than 10
        ///    fixed64 value = 1 \[(buf.validate.field).fixed64.lt = 10\];
        /// }
        /// ```
        #[prost(fixed64, tag="2")]
        Lt(u64),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MyFixed64 {
        ///    // value must be less than or equal to 10
        ///    fixed64 value = 1 \[(buf.validate.field).fixed64.lte = 10\];
        /// }
        /// ```
        #[prost(fixed64, tag="3")]
        Lte(u64),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFixed64 {
        ///    // value must be greater than 5 \[fixed64.gt\]
        ///    fixed64 value = 1 \[(buf.validate.field).fixed64.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[fixed64.gt_lt\]
        ///    fixed64 other_value = 2 \[(buf.validate.field).fixed64 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[fixed64.gt_lt_exclusive\]
        ///    fixed64 another_value = 3 \[(buf.validate.field).fixed64 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(fixed64, tag="4")]
        Gt(u64),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyFixed64 {
        ///    // value must be greater than or equal to 5 \[fixed64.gte\]
        ///    fixed64 value = 1 \[(buf.validate.field).fixed64.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[fixed64.gte_lt\]
        ///    fixed64 other_value = 2 \[(buf.validate.field).fixed64 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[fixed64.gte_lt_exclusive\]
        ///    fixed64 another_value = 3 \[(buf.validate.field).fixed64 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(fixed64, tag="5")]
        Gte(u64),
    }
}
/// SFixed32Rules describes the constraints applied to `fixed32` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SFixed32Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MySFixed32 {
    ///    // value must equal 42
    ///    sfixed32 value = 1 \[(buf.validate.field).sfixed32.const = 42\];
    /// }
    /// ```
    #[prost(sfixed32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MySFixed32 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated sfixed32 value = 1 (buf.validate.field).sfixed32 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sfixed32, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MySFixed32 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated sfixed32 value = 1 (buf.validate.field).sfixed32 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sfixed32, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MySFixed32 {
    ///    sfixed32 value = 1 [
    ///      (buf.validate.field).sfixed32.example = 1,
    ///      (buf.validate.field).sfixed32.example = 2
    ///    ];
    /// }
    /// ```
    #[prost(sfixed32, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<i32>,
    #[prost(oneof="s_fixed32_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<s_fixed32_rules::LessThan>,
    #[prost(oneof="s_fixed32_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<s_fixed32_rules::GreaterThan>,
}
/// Nested message and enum types in `SFixed32Rules`.
pub mod s_fixed32_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySFixed32 {
        ///    // value must be less than 10
        ///    sfixed32 value = 1 \[(buf.validate.field).sfixed32.lt = 10\];
        /// }
        /// ```
        #[prost(sfixed32, tag="2")]
        Lt(i32),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MySFixed32 {
        ///    // value must be less than or equal to 10
        ///    sfixed32 value = 1 \[(buf.validate.field).sfixed32.lte = 10\];
        /// }
        /// ```
        #[prost(sfixed32, tag="3")]
        Lte(i32),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySFixed32 {
        ///    // value must be greater than 5 \[sfixed32.gt\]
        ///    sfixed32 value = 1 \[(buf.validate.field).sfixed32.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[sfixed32.gt_lt\]
        ///    sfixed32 other_value = 2 \[(buf.validate.field).sfixed32 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[sfixed32.gt_lt_exclusive\]
        ///    sfixed32 another_value = 3 \[(buf.validate.field).sfixed32 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sfixed32, tag="4")]
        Gt(i32),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySFixed32 {
        ///    // value must be greater than or equal to 5 \[sfixed32.gte\]
        ///    sfixed32 value = 1 \[(buf.validate.field).sfixed32.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[sfixed32.gte_lt\]
        ///    sfixed32 other_value = 2 \[(buf.validate.field).sfixed32 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[sfixed32.gte_lt_exclusive\]
        ///    sfixed32 another_value = 3 \[(buf.validate.field).sfixed32 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sfixed32, tag="5")]
        Gte(i32),
    }
}
/// SFixed64Rules describes the constraints applied to `fixed64` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SFixed64Rules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MySFixed64 {
    ///    // value must equal 42
    ///    sfixed64 value = 1 \[(buf.validate.field).sfixed64.const = 42\];
    /// }
    /// ```
    #[prost(sfixed64, optional, tag="1")]
    pub r#const: ::core::option::Option<i64>,
    /// `in` requires the field value to be equal to one of the specified values.
    /// If the field value isn't one of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MySFixed64 {
    ///    // value must be in list \[1, 2, 3\]
    ///    repeated sfixed64 value = 1 (buf.validate.field).sfixed64 = { in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sfixed64, repeated, packed="false", tag="6")]
    pub r#in: ::prost::alloc::vec::Vec<i64>,
    /// `not_in` requires the field value to not be equal to any of the specified
    /// values. If the field value is one of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MySFixed64 {
    ///    // value must not be in list \[1, 2, 3\]
    ///    repeated sfixed64 value = 1 (buf.validate.field).sfixed64 = { not_in: \[1, 2, 3\] };
    /// }
    /// ```
    #[prost(sfixed64, repeated, packed="false", tag="7")]
    pub not_in: ::prost::alloc::vec::Vec<i64>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MySFixed64 {
    ///    sfixed64 value = 1 [
    ///      (buf.validate.field).sfixed64.example = 1,
    ///      (buf.validate.field).sfixed64.example = 2
    ///    ];
    /// }
    /// ```
    #[prost(sfixed64, repeated, packed="false", tag="8")]
    pub example: ::prost::alloc::vec::Vec<i64>,
    #[prost(oneof="s_fixed64_rules::LessThan", tags="2, 3")]
    pub less_than: ::core::option::Option<s_fixed64_rules::LessThan>,
    #[prost(oneof="s_fixed64_rules::GreaterThan", tags="4, 5")]
    pub greater_than: ::core::option::Option<s_fixed64_rules::GreaterThan>,
}
/// Nested message and enum types in `SFixed64Rules`.
pub mod s_fixed64_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` requires the field value to be less than the specified value (field <
        /// value). If the field value is equal to or greater than the specified value,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySFixed64 {
        ///    // value must be less than 10
        ///    sfixed64 value = 1 \[(buf.validate.field).sfixed64.lt = 10\];
        /// }
        /// ```
        #[prost(sfixed64, tag="2")]
        Lt(i64),
        /// `lte` requires the field value to be less than or equal to the specified
        /// value (field <= value). If the field value is greater than the specified
        /// value, an error message is generated.
        ///
        /// ```proto
        /// message MySFixed64 {
        ///    // value must be less than or equal to 10
        ///    sfixed64 value = 1 \[(buf.validate.field).sfixed64.lte = 10\];
        /// }
        /// ```
        #[prost(sfixed64, tag="3")]
        Lte(i64),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the field value to be greater than the specified value
        /// (exclusive). If the value of `gt` is larger than a specified `lt` or
        /// `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySFixed64 {
        ///    // value must be greater than 5 \[sfixed64.gt\]
        ///    sfixed64 value = 1 \[(buf.validate.field).sfixed64.gt = 5\];
        ///
        ///    // value must be greater than 5 and less than 10 \[sfixed64.gt_lt\]
        ///    sfixed64 other_value = 2 \[(buf.validate.field).sfixed64 = { gt: 5, lt: 10 }\];
        ///
        ///    // value must be greater than 10 or less than 5 \[sfixed64.gt_lt_exclusive\]
        ///    sfixed64 another_value = 3 \[(buf.validate.field).sfixed64 = { gt: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sfixed64, tag="4")]
        Gt(i64),
        /// `gte` requires the field value to be greater than or equal to the specified
        /// value (exclusive). If the value of `gte` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MySFixed64 {
        ///    // value must be greater than or equal to 5 \[sfixed64.gte\]
        ///    sfixed64 value = 1 \[(buf.validate.field).sfixed64.gte = 5\];
        ///
        ///    // value must be greater than or equal to 5 and less than 10 \[sfixed64.gte_lt\]
        ///    sfixed64 other_value = 2 \[(buf.validate.field).sfixed64 = { gte: 5, lt: 10 }\];
        ///
        ///    // value must be greater than or equal to 10 or less than 5 \[sfixed64.gte_lt_exclusive\]
        ///    sfixed64 another_value = 3 \[(buf.validate.field).sfixed64 = { gte: 10, lt: 5 }\];
        /// }
        /// ```
        #[prost(sfixed64, tag="5")]
        Gte(i64),
    }
}
/// BoolRules describes the constraints applied to `bool` values. These rules
/// may also be applied to the `google.protobuf.BoolValue` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolRules {
    /// `const` requires the field value to exactly match the specified boolean value.
    /// If the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyBool {
    ///    // value must equal true
    ///    bool value = 1 \[(buf.validate.field).bool.const = true\];
    /// }
    /// ```
    #[prost(bool, optional, tag="1")]
    pub r#const: ::core::option::Option<bool>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyBool {
    ///    bool value = 1 [
    ///      (buf.validate.field).bool.example = 1,
    ///      (buf.validate.field).bool.example = 2
    ///    ];
    /// }
    /// ```
    #[prost(bool, repeated, packed="false", tag="2")]
    pub example: ::prost::alloc::vec::Vec<bool>,
}
/// StringRules describes the constraints applied to `string` values These
/// rules may also be applied to the `google.protobuf.StringValue` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringRules {
    /// `const` requires the field value to exactly match the specified value. If
    /// the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value must equal `hello`
    ///    string value = 1 \[(buf.validate.field).string.const = "hello"\];
    /// }
    /// ```
    #[prost(string, optional, tag="1")]
    pub r#const: ::core::option::Option<::prost::alloc::string::String>,
    /// `len` dictates that the field value must have the specified
    /// number of characters (Unicode code points), which may differ from the number
    /// of bytes in the string. If the field value does not meet the specified
    /// length, an error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value length must be 5 characters
    ///    string value = 1 \[(buf.validate.field).string.len = 5\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="19")]
    pub len: ::core::option::Option<u64>,
    /// `min_len` specifies that the field value must have at least the specified
    /// number of characters (Unicode code points), which may differ from the number
    /// of bytes in the string. If the field value contains fewer characters, an error
    /// message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value length must be at least 3 characters
    ///    string value = 1 \[(buf.validate.field).string.min_len = 3\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="2")]
    pub min_len: ::core::option::Option<u64>,
    /// `max_len` specifies that the field value must have no more than the specified
    /// number of characters (Unicode code points), which may differ from the
    /// number of bytes in the string. If the field value contains more characters,
    /// an error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value length must be at most 10 characters
    ///    string value = 1 \[(buf.validate.field).string.max_len = 10\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="3")]
    pub max_len: ::core::option::Option<u64>,
    /// `len_bytes` dictates that the field value must have the specified number of
    /// bytes. If the field value does not match the specified length in bytes,
    /// an error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value length must be 6 bytes
    ///    string value = 1 \[(buf.validate.field).string.len_bytes = 6\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="20")]
    pub len_bytes: ::core::option::Option<u64>,
    /// `min_bytes` specifies that the field value must have at least the specified
    /// number of bytes. If the field value contains fewer bytes, an error message
    /// will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value length must be at least 4 bytes
    ///    string value = 1 \[(buf.validate.field).string.min_bytes = 4\];
    /// }
    ///
    /// ```
    #[prost(uint64, optional, tag="4")]
    pub min_bytes: ::core::option::Option<u64>,
    /// `max_bytes` specifies that the field value must have no more than the
    /// specified number of bytes. If the field value contains more bytes, an
    /// error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value length must be at most 8 bytes
    ///    string value = 1 \[(buf.validate.field).string.max_bytes = 8\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="5")]
    pub max_bytes: ::core::option::Option<u64>,
    /// `pattern` specifies that the field value must match the specified
    /// regular expression (RE2 syntax), with the expression provided without any
    /// delimiters. If the field value doesn't match the regular expression, an
    /// error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value does not match regex pattern `^\[a-zA-Z\]//$`
    ///    string value = 1 \[(buf.validate.field).string.pattern = "^[a-zA-Z\]//$"];
    /// }
    /// ```
    #[prost(string, optional, tag="6")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    /// `prefix` specifies that the field value must have the
    /// specified substring at the beginning of the string. If the field value
    /// doesn't start with the specified prefix, an error message will be
    /// generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value does not have prefix `pre`
    ///    string value = 1 \[(buf.validate.field).string.prefix = "pre"\];
    /// }
    /// ```
    #[prost(string, optional, tag="7")]
    pub prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// `suffix` specifies that the field value must have the
    /// specified substring at the end of the string. If the field value doesn't
    /// end with the specified suffix, an error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value does not have suffix `post`
    ///    string value = 1 \[(buf.validate.field).string.suffix = "post"\];
    /// }
    /// ```
    #[prost(string, optional, tag="8")]
    pub suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// `contains` specifies that the field value must have the
    /// specified substring anywhere in the string. If the field value doesn't
    /// contain the specified substring, an error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value does not contain substring `inside`.
    ///    string value = 1 \[(buf.validate.field).string.contains = "inside"\];
    /// }
    /// ```
    #[prost(string, optional, tag="9")]
    pub contains: ::core::option::Option<::prost::alloc::string::String>,
    /// `not_contains` specifies that the field value must not have the
    /// specified substring anywhere in the string. If the field value contains
    /// the specified substring, an error message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value contains substring `inside`.
    ///    string value = 1 \[(buf.validate.field).string.not_contains = "inside"\];
    /// }
    /// ```
    #[prost(string, optional, tag="23")]
    pub not_contains: ::core::option::Option<::prost::alloc::string::String>,
    /// `in` specifies that the field value must be equal to one of the specified
    /// values. If the field value isn't one of the specified values, an error
    /// message will be generated.
    ///
    /// ```proto
    /// message MyString {
    ///    // value must be in list \["apple", "banana"\]
    ///    repeated string value = 1 \[(buf.validate.field).string.in = "apple", (buf.validate.field).string.in = "banana"\];
    /// }
    /// ```
    #[prost(string, repeated, tag="10")]
    pub r#in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `not_in` specifies that the field value cannot be equal to any
    /// of the specified values. If the field value is one of the specified values,
    /// an error message will be generated.
    /// ```proto
    /// message MyString {
    ///    // value must not be in list \["orange", "grape"\]
    ///    repeated string value = 1 \[(buf.validate.field).string.not_in = "orange", (buf.validate.field).string.not_in = "grape"\];
    /// }
    /// ```
    #[prost(string, repeated, tag="11")]
    pub not_in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This applies to regexes `HTTP_HEADER_NAME` and `HTTP_HEADER_VALUE` to
    /// enable strict header validation. By default, this is true, and HTTP header
    /// validations are [RFC-compliant](<https://tools.ietf.org/html/rfc7230#section-3>). Setting to false will enable looser
    /// validations that only disallow `\r\n\0` characters, which can be used to
    /// bypass header matching rules.
    ///
    /// ```proto
    /// message MyString {
    ///    // The field `value` must have be a valid HTTP headers, but not enforced with strict rules.
    ///    string value = 1 \[(buf.validate.field).string.strict = false\];
    /// }
    /// ```
    #[prost(bool, optional, tag="25")]
    pub strict: ::core::option::Option<bool>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyString {
    ///    string value = 1 [
    ///      (buf.validate.field).string.example = "hello",
    ///      (buf.validate.field).string.example = "world"
    ///    ];
    /// }
    /// ```
    #[prost(string, repeated, tag="34")]
    pub example: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `WellKnown` rules provide advanced constraints against common string
    /// patterns
    #[prost(oneof="string_rules::WellKnown", tags="12, 13, 14, 15, 16, 17, 18, 21, 22, 33, 26, 27, 28, 29, 30, 31, 32, 24")]
    pub well_known: ::core::option::Option<string_rules::WellKnown>,
}
/// Nested message and enum types in `StringRules`.
pub mod string_rules {
    /// `WellKnown` rules provide advanced constraints against common string
    /// patterns
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WellKnown {
        /// `email` specifies that the field value must be a valid email address
        /// (addr-spec only) as defined by [RFC 5322](<https://tools.ietf.org/html/rfc5322#section-3.4.1>).
        /// If the field value isn't a valid email address, an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid email address
        ///    string value = 1 \[(buf.validate.field).string.email = true\];
        /// }
        /// ```
        #[prost(bool, tag="12")]
        Email(bool),
        /// `hostname` specifies that the field value must be a valid
        /// hostname as defined by [RFC 1034](<https://tools.ietf.org/html/rfc1034#section-3.5>). This constraint doesn't support
        /// internationalized domain names (IDNs). If the field value isn't a
        /// valid hostname, an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid hostname
        ///    string value = 1 \[(buf.validate.field).string.hostname = true\];
        /// }
        /// ```
        #[prost(bool, tag="13")]
        Hostname(bool),
        /// `ip` specifies that the field value must be a valid IP
        /// (v4 or v6) address, without surrounding square brackets for IPv6 addresses.
        /// If the field value isn't a valid IP address, an error message will be
        /// generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IP address
        ///    string value = 1 \[(buf.validate.field).string.ip = true\];
        /// }
        /// ```
        #[prost(bool, tag="14")]
        Ip(bool),
        /// `ipv4` specifies that the field value must be a valid IPv4
        /// address. If the field value isn't a valid IPv4 address, an error message
        /// will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IPv4 address
        ///    string value = 1 \[(buf.validate.field).string.ipv4 = true\];
        /// }
        /// ```
        #[prost(bool, tag="15")]
        Ipv4(bool),
        /// `ipv6` specifies that the field value must be a valid
        /// IPv6 address, without surrounding square brackets. If the field value is
        /// not a valid IPv6 address, an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IPv6 address
        ///    string value = 1 \[(buf.validate.field).string.ipv6 = true\];
        /// }
        /// ```
        #[prost(bool, tag="16")]
        Ipv6(bool),
        /// `uri` specifies that the field value must be a valid,
        /// absolute URI as defined by [RFC 3986](<https://tools.ietf.org/html/rfc3986#section-3>). If the field value isn't a valid,
        /// absolute URI, an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid URI
        ///    string value = 1 \[(buf.validate.field).string.uri = true\];
        /// }
        /// ```
        #[prost(bool, tag="17")]
        Uri(bool),
        /// `uri_ref` specifies that the field value must be a valid URI
        /// as defined by [RFC 3986](<https://tools.ietf.org/html/rfc3986#section-3>) and may be either relative or absolute. If the
        /// field value isn't a valid URI, an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid URI
        ///    string value = 1 \[(buf.validate.field).string.uri_ref = true\];
        /// }
        /// ```
        #[prost(bool, tag="18")]
        UriRef(bool),
        /// `address` specifies that the field value must be either a valid hostname
        /// as defined by [RFC 1034](<https://tools.ietf.org/html/rfc1034#section-3.5>)
        /// (which doesn't support internationalized domain names or IDNs) or a valid
        /// IP (v4 or v6). If the field value isn't a valid hostname or IP, an error
        /// message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid hostname, or ip address
        ///    string value = 1 \[(buf.validate.field).string.address = true\];
        /// }
        /// ```
        #[prost(bool, tag="21")]
        Address(bool),
        /// `uuid` specifies that the field value must be a valid UUID as defined by
        /// [RFC 4122](<https://tools.ietf.org/html/rfc4122#section-4.1.2>). If the
        /// field value isn't a valid UUID, an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid UUID
        ///    string value = 1 \[(buf.validate.field).string.uuid = true\];
        /// }
        /// ```
        #[prost(bool, tag="22")]
        Uuid(bool),
        /// `tuuid` (trimmed UUID) specifies that the field value must be a valid UUID as
        /// defined by [RFC 4122](<https://tools.ietf.org/html/rfc4122#section-4.1.2>) with all dashes
        /// omitted. If the field value isn't a valid UUID without dashes, an error message
        /// will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid trimmed UUID
        ///    string value = 1 \[(buf.validate.field).string.tuuid = true\];
        /// }
        /// ```
        #[prost(bool, tag="33")]
        Tuuid(bool),
        /// `ip_with_prefixlen` specifies that the field value must be a valid IP (v4 or v6)
        /// address with prefix length. If the field value isn't a valid IP with prefix
        /// length, an error message will be generated.
        ///
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IP with prefix length
        ///     string value = 1 \[(buf.validate.field).string.ip_with_prefixlen = true\];
        /// }
        /// ```
        #[prost(bool, tag="26")]
        IpWithPrefixlen(bool),
        /// `ipv4_with_prefixlen` specifies that the field value must be a valid
        /// IPv4 address with prefix.
        /// If the field value isn't a valid IPv4 address with prefix length,
        /// an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IPv4 address with prefix length
        ///     string value = 1 \[(buf.validate.field).string.ipv4_with_prefixlen = true\];
        /// }
        /// ```
        #[prost(bool, tag="27")]
        Ipv4WithPrefixlen(bool),
        /// `ipv6_with_prefixlen` specifies that the field value must be a valid
        /// IPv6 address with prefix length.
        /// If the field value is not a valid IPv6 address with prefix length,
        /// an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IPv6 address prefix length
        ///     string value = 1 \[(buf.validate.field).string.ipv6_with_prefixlen = true\];
        /// }
        /// ```
        #[prost(bool, tag="28")]
        Ipv6WithPrefixlen(bool),
        /// `ip_prefix` specifies that the field value must be a valid IP (v4 or v6) prefix.
        /// If the field value isn't a valid IP prefix, an error message will be
        /// generated. The prefix must have all zeros for the masked bits of the prefix (e.g.,
        /// `127.0.0.0/16`, not `127.0.0.1/16`).
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IP prefix
        ///     string value = 1 \[(buf.validate.field).string.ip_prefix = true\];
        /// }
        /// ```
        #[prost(bool, tag="29")]
        IpPrefix(bool),
        /// `ipv4_prefix` specifies that the field value must be a valid IPv4
        /// prefix. If the field value isn't a valid IPv4 prefix, an error message
        /// will be generated. The prefix must have all zeros for the masked bits of
        /// the prefix (e.g., `127.0.0.0/16`, not `127.0.0.1/16`).
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IPv4 prefix
        ///     string value = 1 \[(buf.validate.field).string.ipv4_prefix = true\];
        /// }
        /// ```
        #[prost(bool, tag="30")]
        Ipv4Prefix(bool),
        /// `ipv6_prefix` specifies that the field value must be a valid IPv6 prefix.
        /// If the field value is not a valid IPv6 prefix, an error message will be
        /// generated. The prefix must have all zeros for the masked bits of the prefix
        /// (e.g., `2001:db8::/48`, not `2001:db8::1/48`).
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid IPv6 prefix
        ///     string value = 1 \[(buf.validate.field).string.ipv6_prefix = true\];
        /// }
        /// ```
        #[prost(bool, tag="31")]
        Ipv6Prefix(bool),
        /// `host_and_port` specifies the field value must be a valid host and port
        /// pair. The host must be a valid hostname or IP address while the port
        /// must be in the range of 0-65535, inclusive. IPv6 addresses must be delimited
        /// with square brackets (e.g., `\[::1\]:1234`).
        #[prost(bool, tag="32")]
        HostAndPort(bool),
        /// `well_known_regex` specifies a common well-known pattern
        /// defined as a regex. If the field value doesn't match the well-known
        /// regex, an error message will be generated.
        ///
        /// ```proto
        /// message MyString {
        ///    // value must be a valid HTTP header value
        ///    string value = 1 \[(buf.validate.field).string.well_known_regex = KNOWN_REGEX_HTTP_HEADER_VALUE\];
        /// }
        /// ```
        ///
        /// #### KnownRegex
        ///
        /// `well_known_regex` contains some well-known patterns.
        ///
        /// | Name                          | Number | Description                               |
        /// |-------------------------------|--------|-------------------------------------------|
        /// | KNOWN_REGEX_UNSPECIFIED       | 0      |                                           |
        /// | KNOWN_REGEX_HTTP_HEADER_NAME  | 1      | HTTP header name as defined by [RFC 7230](<https://tools.ietf.org/html/rfc7230#section-3.2>)  |
        /// | KNOWN_REGEX_HTTP_HEADER_VALUE | 2      | HTTP header value as defined by [RFC 7230](<https://tools.ietf.org/html/rfc7230#section-3.2.4>) |
        #[prost(enumeration="super::KnownRegex", tag="24")]
        WellKnownRegex(i32),
    }
}
/// BytesRules describe the constraints applied to `bytes` values. These rules
/// may also be applied to the `google.protobuf.BytesValue` Well-Known-Type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesRules {
    /// `const` requires the field value to exactly match the specified bytes
    /// value. If the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value must be "\x01\x02\x03\x04"
    ///    bytes value = 1 \[(buf.validate.field).bytes.const = "\x01\x02\x03\x04"\];
    /// }
    /// ```
    #[prost(bytes="vec", optional, tag="1")]
    pub r#const: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// `len` requires the field value to have the specified length in bytes.
    /// If the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value length must be 4 bytes.
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.len = 4\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="13")]
    pub len: ::core::option::Option<u64>,
    /// `min_len` requires the field value to have at least the specified minimum
    /// length in bytes.
    /// If the field value doesn't meet the requirement, an error message is generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value length must be at least 2 bytes.
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.min_len = 2\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="2")]
    pub min_len: ::core::option::Option<u64>,
    /// `max_len` requires the field value to have at most the specified maximum
    /// length in bytes.
    /// If the field value exceeds the requirement, an error message is generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value must be at most 6 bytes.
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.max_len = 6\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="3")]
    pub max_len: ::core::option::Option<u64>,
    /// `pattern` requires the field value to match the specified regular
    /// expression ([RE2 syntax](<https://github.com/google/re2/wiki/Syntax>)).
    /// The value of the field must be valid UTF-8 or validation will fail with a
    /// runtime error.
    /// If the field value doesn't match the pattern, an error message is generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value must match regex pattern "^\[a-zA-Z0-9\]+$".
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.pattern = "^[a-zA-Z0-9\]+$"];
    /// }
    /// ```
    #[prost(string, optional, tag="4")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    /// `prefix` requires the field value to have the specified bytes at the
    /// beginning of the string.
    /// If the field value doesn't meet the requirement, an error message is generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value does not have prefix \x01\x02
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.prefix = "\x01\x02"\];
    /// }
    /// ```
    #[prost(bytes="vec", optional, tag="5")]
    pub prefix: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// `suffix` requires the field value to have the specified bytes at the end
    /// of the string.
    /// If the field value doesn't meet the requirement, an error message is generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value does not have suffix \x03\x04
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.suffix = "\x03\x04"\];
    /// }
    /// ```
    #[prost(bytes="vec", optional, tag="6")]
    pub suffix: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// `contains` requires the field value to have the specified bytes anywhere in
    /// the string.
    /// If the field value doesn't meet the requirement, an error message is generated.
    ///
    /// ```protobuf
    /// message MyBytes {
    ///    // value does not contain \x02\x03
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.contains = "\x02\x03"\];
    /// }
    /// ```
    #[prost(bytes="vec", optional, tag="7")]
    pub contains: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// `in` requires the field value to be equal to one of the specified
    /// values. If the field value doesn't match any of the specified values, an
    /// error message is generated.
    ///
    /// ```protobuf
    /// message MyBytes {
    ///    // value must in \["\x01\x02", "\x02\x03", "\x03\x04"\]
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.in = {"\x01\x02", "\x02\x03", "\x03\x04"}\];
    /// }
    /// ```
    #[prost(bytes="vec", repeated, tag="8")]
    pub r#in: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// `not_in` requires the field value to be not equal to any of the specified
    /// values.
    /// If the field value matches any of the specified values, an error message is
    /// generated.
    ///
    /// ```proto
    /// message MyBytes {
    ///    // value must not in \["\x01\x02", "\x02\x03", "\x03\x04"\]
    ///    optional bytes value = 1 \[(buf.validate.field).bytes.not_in = {"\x01\x02", "\x02\x03", "\x03\x04"}\];
    /// }
    /// ```
    #[prost(bytes="vec", repeated, tag="9")]
    pub not_in: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyBytes {
    ///    bytes value = 1 [
    ///      (buf.validate.field).bytes.example = "\x01\x02",
    ///      (buf.validate.field).bytes.example = "\x02\x03"
    ///    ];
    /// }
    /// ```
    #[prost(bytes="vec", repeated, tag="14")]
    pub example: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// WellKnown rules provide advanced constraints against common byte
    /// patterns
    #[prost(oneof="bytes_rules::WellKnown", tags="10, 11, 12")]
    pub well_known: ::core::option::Option<bytes_rules::WellKnown>,
}
/// Nested message and enum types in `BytesRules`.
pub mod bytes_rules {
    /// WellKnown rules provide advanced constraints against common byte
    /// patterns
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WellKnown {
        /// `ip` ensures that the field `value` is a valid IP address (v4 or v6) in byte format.
        /// If the field value doesn't meet this constraint, an error message is generated.
        ///
        /// ```proto
        /// message MyBytes {
        ///    // value must be a valid IP address
        ///    optional bytes value = 1 \[(buf.validate.field).bytes.ip = true\];
        /// }
        /// ```
        #[prost(bool, tag="10")]
        Ip(bool),
        /// `ipv4` ensures that the field `value` is a valid IPv4 address in byte format.
        /// If the field value doesn't meet this constraint, an error message is generated.
        ///
        /// ```proto
        /// message MyBytes {
        ///    // value must be a valid IPv4 address
        ///    optional bytes value = 1 \[(buf.validate.field).bytes.ipv4 = true\];
        /// }
        /// ```
        #[prost(bool, tag="11")]
        Ipv4(bool),
        /// `ipv6` ensures that the field `value` is a valid IPv6 address in byte format.
        /// If the field value doesn't meet this constraint, an error message is generated.
        /// ```proto
        /// message MyBytes {
        ///    // value must be a valid IPv6 address
        ///    optional bytes value = 1 \[(buf.validate.field).bytes.ipv6 = true\];
        /// }
        /// ```
        #[prost(bool, tag="12")]
        Ipv6(bool),
    }
}
/// EnumRules describe the constraints applied to `enum` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumRules {
    /// `const` requires the field value to exactly match the specified enum value.
    /// If the field value doesn't match, an error message is generated.
    ///
    /// ```proto
    /// enum MyEnum {
    ///    MY_ENUM_UNSPECIFIED = 0;
    ///    MY_ENUM_VALUE1 = 1;
    ///    MY_ENUM_VALUE2 = 2;
    /// }
    ///
    /// message MyMessage {
    ///    // The field `value` must be exactly MY_ENUM_VALUE1.
    ///    MyEnum value = 1 \[(buf.validate.field).enum.const = 1\];
    /// }
    /// ```
    #[prost(int32, optional, tag="1")]
    pub r#const: ::core::option::Option<i32>,
    /// `defined_only` requires the field value to be one of the defined values for
    /// this enum, failing on any undefined value.
    ///
    /// ```proto
    /// enum MyEnum {
    ///    MY_ENUM_UNSPECIFIED = 0;
    ///    MY_ENUM_VALUE1 = 1;
    ///    MY_ENUM_VALUE2 = 2;
    /// }
    ///
    /// message MyMessage {
    ///    // The field `value` must be a defined value of MyEnum.
    ///    MyEnum value = 1 \[(buf.validate.field).enum.defined_only = true\];
    /// }
    /// ```
    #[prost(bool, optional, tag="2")]
    pub defined_only: ::core::option::Option<bool>,
    /// `in` requires the field value to be equal to one of the
    /// specified enum values. If the field value doesn't match any of the
    /// specified values, an error message is generated.
    ///
    /// ```proto
    /// enum MyEnum {
    ///    MY_ENUM_UNSPECIFIED = 0;
    ///    MY_ENUM_VALUE1 = 1;
    ///    MY_ENUM_VALUE2 = 2;
    /// }
    ///
    /// message MyMessage {
    ///    // The field `value` must be equal to one of the specified values.
    ///    MyEnum value = 1 \[(buf.validate.field).enum = { in: [1, 2\]}];
    /// }
    /// ```
    #[prost(int32, repeated, packed="false", tag="3")]
    pub r#in: ::prost::alloc::vec::Vec<i32>,
    /// `not_in` requires the field value to be not equal to any of the
    /// specified enum values. If the field value matches one of the specified
    /// values, an error message is generated.
    ///
    /// ```proto
    /// enum MyEnum {
    ///    MY_ENUM_UNSPECIFIED = 0;
    ///    MY_ENUM_VALUE1 = 1;
    ///    MY_ENUM_VALUE2 = 2;
    /// }
    ///
    /// message MyMessage {
    ///    // The field `value` must not be equal to any of the specified values.
    ///    MyEnum value = 1 \[(buf.validate.field).enum = { not_in: [1, 2\]}];
    /// }
    /// ```
    #[prost(int32, repeated, packed="false", tag="4")]
    pub not_in: ::prost::alloc::vec::Vec<i32>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// enum MyEnum {
    ///    MY_ENUM_UNSPECIFIED = 0;
    ///    MY_ENUM_VALUE1 = 1;
    ///    MY_ENUM_VALUE2 = 2;
    /// }
    ///
    /// message MyMessage {
    ///      (buf.validate.field).enum.example = 1,
    ///      (buf.validate.field).enum.example = 2
    /// }
    /// ```
    #[prost(int32, repeated, packed="false", tag="5")]
    pub example: ::prost::alloc::vec::Vec<i32>,
}
/// RepeatedRules describe the constraints applied to `repeated` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepeatedRules {
    /// `min_items` requires that this field must contain at least the specified
    /// minimum number of items.
    ///
    /// Note that `min_items = 1` is equivalent to setting a field as `required`.
    ///
    /// ```proto
    /// message MyRepeated {
    ///    // value must contain at least  2 items
    ///    repeated string value = 1 \[(buf.validate.field).repeated.min_items = 2\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="1")]
    pub min_items: ::core::option::Option<u64>,
    /// `max_items` denotes that this field must not exceed a
    /// certain number of items as the upper limit. If the field contains more
    /// items than specified, an error message will be generated, requiring the
    /// field to maintain no more than the specified number of items.
    ///
    /// ```proto
    /// message MyRepeated {
    ///    // value must contain no more than 3 item(s)
    ///    repeated string value = 1 \[(buf.validate.field).repeated.max_items = 3\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="2")]
    pub max_items: ::core::option::Option<u64>,
    /// `unique` indicates that all elements in this field must
    /// be unique. This constraint is strictly applicable to scalar and enum
    /// types, with message types not being supported.
    ///
    /// ```proto
    /// message MyRepeated {
    ///    // repeated value must contain unique items
    ///    repeated string value = 1 \[(buf.validate.field).repeated.unique = true\];
    /// }
    /// ```
    #[prost(bool, optional, tag="3")]
    pub unique: ::core::option::Option<bool>,
    /// `items` details the constraints to be applied to each item
    /// in the field. Even for repeated message fields, validation is executed
    /// against each item unless skip is explicitly specified.
    ///
    /// ```proto
    /// message MyRepeated {
    ///    // The items in the field `value` must follow the specified constraints.
    ///    repeated string value = 1 [(buf.validate.field).repeated.items = {
    ///      string: {
    ///        min_len: 3
    ///        max_len: 10
    ///      }
    ///    }];
    /// }
    /// ```
    #[prost(message, optional, boxed, tag="4")]
    pub items: ::core::option::Option<::prost::alloc::boxed::Box<FieldConstraints>>,
}
/// MapRules describe the constraints applied to `map` values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapRules {
    /// Specifies the minimum number of key-value pairs allowed. If the field has
    /// fewer key-value pairs than specified, an error message is generated.
    ///
    /// ```proto
    /// message MyMap {
    ///    // The field `value` must have at least 2 key-value pairs.
    ///    map<string, string> value = 1 \[(buf.validate.field).map.min_pairs = 2\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="1")]
    pub min_pairs: ::core::option::Option<u64>,
    /// Specifies the maximum number of key-value pairs allowed. If the field has
    /// more key-value pairs than specified, an error message is generated.
    ///
    /// ```proto
    /// message MyMap {
    ///    // The field `value` must have at most 3 key-value pairs.
    ///    map<string, string> value = 1 \[(buf.validate.field).map.max_pairs = 3\];
    /// }
    /// ```
    #[prost(uint64, optional, tag="2")]
    pub max_pairs: ::core::option::Option<u64>,
    /// Specifies the constraints to be applied to each key in the field.
    ///
    /// ```proto
    /// message MyMap {
    ///    // The keys in the field `value` must follow the specified constraints.
    ///    map<string, string> value = 1 [(buf.validate.field).map.keys = {
    ///      string: {
    ///        min_len: 3
    ///        max_len: 10
    ///      }
    ///    }];
    /// }
    /// ```
    #[prost(message, optional, boxed, tag="4")]
    pub keys: ::core::option::Option<::prost::alloc::boxed::Box<FieldConstraints>>,
    /// Specifies the constraints to be applied to the value of each key in the
    /// field. Message values will still have their validations evaluated unless
    /// skip is specified here.
    ///
    /// ```proto
    /// message MyMap {
    ///    // The values in the field `value` must follow the specified constraints.
    ///    map<string, string> value = 1 [(buf.validate.field).map.values = {
    ///      string: {
    ///        min_len: 5
    ///        max_len: 20
    ///      }
    ///    }];
    /// }
    /// ```
    #[prost(message, optional, boxed, tag="5")]
    pub values: ::core::option::Option<::prost::alloc::boxed::Box<FieldConstraints>>,
}
/// AnyRules describe constraints applied exclusively to the `google.protobuf.Any` well-known type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnyRules {
    /// `in` requires the field's `type_url` to be equal to one of the
    /// specified values. If it doesn't match any of the specified values, an error
    /// message is generated.
    ///
    /// ```proto
    /// message MyAny {
    ///    //  The `value` field must have a `type_url` equal to one of the specified values.
    ///    google.protobuf.Any value = 1 \[(buf.validate.field).any.in = ["type.googleapis.com/MyType1", "type.googleapis.com/MyType2"]\];
    /// }
    /// ```
    #[prost(string, repeated, tag="2")]
    pub r#in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// requires the field's type_url to be not equal to any of the specified values. If it matches any of the specified values, an error message is generated.
    ///
    /// ```proto
    /// message MyAny {
    ///    // The field `value` must not have a `type_url` equal to any of the specified values.
    ///    google.protobuf.Any value = 1 \[(buf.validate.field).any.not_in = ["type.googleapis.com/ForbiddenType1", "type.googleapis.com/ForbiddenType2"]\];
    /// }
    /// ```
    #[prost(string, repeated, tag="3")]
    pub not_in: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DurationRules describe the constraints applied exclusively to the `google.protobuf.Duration` well-known type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DurationRules {
    /// `const` dictates that the field must match the specified value of the `google.protobuf.Duration` type exactly.
    /// If the field's value deviates from the specified value, an error message
    /// will be generated.
    ///
    /// ```proto
    /// message MyDuration {
    ///    // value must equal 5s
    ///    google.protobuf.Duration value = 1 \[(buf.validate.field).duration.const = "5s"\];
    /// }
    /// ```
    #[prost(message, optional, tag="2")]
    pub r#const: ::core::option::Option<::pbjson_types::Duration>,
    /// `in` asserts that the field must be equal to one of the specified values of the `google.protobuf.Duration` type.
    /// If the field's value doesn't correspond to any of the specified values,
    /// an error message will be generated.
    ///
    /// ```proto
    /// message MyDuration {
    ///    // value must be in list \[1s, 2s, 3s\]
    ///    google.protobuf.Duration value = 1 \[(buf.validate.field).duration.in = ["1s", "2s", "3s"]\];
    /// }
    /// ```
    #[prost(message, repeated, tag="7")]
    pub r#in: ::prost::alloc::vec::Vec<::pbjson_types::Duration>,
    /// `not_in` denotes that the field must not be equal to
    /// any of the specified values of the `google.protobuf.Duration` type.
    /// If the field's value matches any of these values, an error message will be
    /// generated.
    ///
    /// ```proto
    /// message MyDuration {
    ///    // value must not be in list \[1s, 2s, 3s\]
    ///    google.protobuf.Duration value = 1 \[(buf.validate.field).duration.not_in = ["1s", "2s", "3s"]\];
    /// }
    /// ```
    #[prost(message, repeated, tag="8")]
    pub not_in: ::prost::alloc::vec::Vec<::pbjson_types::Duration>,
    /// `example` specifies values that the field may have. These values SHOULD
    /// conform to other constraints. `example` values will not impact validation
    /// but may be used as helpful guidance on how to populate the given field.
    ///
    /// ```proto
    /// message MyDuration {
    ///    google.protobuf.Duration value = 1 [
    ///      (buf.validate.field).duration.example = { seconds: 1 },
    ///      (buf.validate.field).duration.example = { seconds: 2 },
    ///    ];
    /// }
    /// ```
    #[prost(message, repeated, tag="9")]
    pub example: ::prost::alloc::vec::Vec<::pbjson_types::Duration>,
    #[prost(oneof="duration_rules::LessThan", tags="3, 4")]
    pub less_than: ::core::option::Option<duration_rules::LessThan>,
    #[prost(oneof="duration_rules::GreaterThan", tags="5, 6")]
    pub greater_than: ::core::option::Option<duration_rules::GreaterThan>,
}
/// Nested message and enum types in `DurationRules`.
pub mod duration_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// `lt` stipulates that the field must be less than the specified value of the `google.protobuf.Duration` type,
        /// exclusive. If the field's value is greater than or equal to the specified
        /// value, an error message will be generated.
        ///
        /// ```proto
        /// message MyDuration {
        ///    // value must be less than 5s
        ///    google.protobuf.Duration value = 1 \[(buf.validate.field).duration.lt = "5s"\];
        /// }
        /// ```
        #[prost(message, tag="3")]
        Lt(::pbjson_types::Duration),
        /// `lte` indicates that the field must be less than or equal to the specified
        /// value of the `google.protobuf.Duration` type, inclusive. If the field's value is greater than the specified value,
        /// an error message will be generated.
        ///
        /// ```proto
        /// message MyDuration {
        ///    // value must be less than or equal to 10s
        ///    google.protobuf.Duration value = 1 \[(buf.validate.field).duration.lte = "10s"\];
        /// }
        /// ```
        #[prost(message, tag="4")]
        Lte(::pbjson_types::Duration),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the duration field value to be greater than the specified
        /// value (exclusive). If the value of `gt` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyDuration {
        ///    // duration must be greater than 5s \[duration.gt\]
        ///    google.protobuf.Duration value = 1 \[(buf.validate.field).duration.gt = { seconds: 5 }\];
        ///
        ///    // duration must be greater than 5s and less than 10s \[duration.gt_lt\]
        ///    google.protobuf.Duration another_value = 2 \[(buf.validate.field).duration = { gt: { seconds: 5 }, lt: { seconds: 10 } }\];
        ///
        ///    // duration must be greater than 10s or less than 5s \[duration.gt_lt_exclusive\]
        ///    google.protobuf.Duration other_value = 3 \[(buf.validate.field).duration = { gt: { seconds: 10 }, lt: { seconds: 5 } }\];
        /// }
        /// ```
        #[prost(message, tag="5")]
        Gt(::pbjson_types::Duration),
        /// `gte` requires the duration field value to be greater than or equal to the
        /// specified value (exclusive). If the value of `gte` is larger than a
        /// specified `lt` or `lte`, the range is reversed, and the field value must
        /// be outside the specified range. If the field value doesn't meet the
        /// required conditions, an error message is generated.
        ///
        /// ```proto
        /// message MyDuration {
        ///   // duration must be greater than or equal to 5s \[duration.gte\]
        ///   google.protobuf.Duration value = 1 \[(buf.validate.field).duration.gte = { seconds: 5 }\];
        ///
        ///   // duration must be greater than or equal to 5s and less than 10s \[duration.gte_lt\]
        ///   google.protobuf.Duration another_value = 2 \[(buf.validate.field).duration = { gte: { seconds: 5 }, lt: { seconds: 10 } }\];
        ///
        ///   // duration must be greater than or equal to 10s or less than 5s \[duration.gte_lt_exclusive\]
        ///   google.protobuf.Duration other_value = 3 \[(buf.validate.field).duration = { gte: { seconds: 10 }, lt: { seconds: 5 } }\];
        /// }
        /// ```
        #[prost(message, tag="6")]
        Gte(::pbjson_types::Duration),
    }
}
/// TimestampRules describe the constraints applied exclusively to the `google.protobuf.Timestamp` well-known type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRules {
    /// `const` dictates that this field, of the `google.protobuf.Timestamp` type, must exactly match the specified value. If the field value doesn't correspond to the specified timestamp, an error message will be generated.
    ///
    /// ```proto
    /// message MyTimestamp {
    ///    // value must equal 2023-05-03T10:00:00Z
    ///    google.protobuf.Timestamp created_at = 1 \[(buf.validate.field).timestamp.const = {seconds: 1727998800}\];
    /// }
    /// ```
    #[prost(message, optional, tag="2")]
    pub r#const: ::core::option::Option<::pbjson_types::Timestamp>,
    /// `within` specifies that this field, of the `google.protobuf.Timestamp` type, must be within the specified duration of the current time. If the field value isn't within the duration, an error message is generated.
    ///
    /// ```proto
    /// message MyTimestamp {
    ///    // value must be within 1 hour of now
    ///    google.protobuf.Timestamp created_at = 1 \[(buf.validate.field).timestamp.within = {seconds: 3600}\];
    /// }
    /// ```
    #[prost(message, optional, tag="9")]
    pub within: ::core::option::Option<::pbjson_types::Duration>,
    // `example` specifies values that the field may have. These values SHOULD
    // conform to other constraints. `example` values will not impact validation
    // but may be used as helpful guidance on how to populate the given field.
    //
    // ```proto
    // message MyTimestamp {
    //    google.protobuf.Timestamp value = 1 [
    //      (buf.validate.field).timestamp.example = { seconds: 1672444800 },
    //      (buf.validate.field).timestamp.example = { seconds: 1672531200 },
    //    ];
    // }
    // ```

    #[prost(message, repeated, tag="10")]
    pub example: ::prost::alloc::vec::Vec<::pbjson_types::Timestamp>,
    #[prost(oneof="timestamp_rules::LessThan", tags="3, 4, 7")]
    pub less_than: ::core::option::Option<timestamp_rules::LessThan>,
    #[prost(oneof="timestamp_rules::GreaterThan", tags="5, 6, 8")]
    pub greater_than: ::core::option::Option<timestamp_rules::GreaterThan>,
}
/// Nested message and enum types in `TimestampRules`.
pub mod timestamp_rules {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LessThan {
        /// requires the duration field value to be less than the specified value (field < value). If the field value doesn't meet the required conditions, an error message is generated.
        ///
        /// ```proto
        /// message MyDuration {
        ///    // duration must be less than 'P3D' \[duration.lt\]
        ///    google.protobuf.Duration value = 1 \[(buf.validate.field).duration.lt = { seconds: 259200 }\];
        /// }
        /// ```
        #[prost(message, tag="3")]
        Lt(::pbjson_types::Timestamp),
        /// requires the timestamp field value to be less than or equal to the specified value (field <= value). If the field value doesn't meet the required conditions, an error message is generated.
        ///
        /// ```proto
        /// message MyTimestamp {
        ///    // timestamp must be less than or equal to '2023-05-14T00:00:00Z' \[timestamp.lte\]
        ///    google.protobuf.Timestamp value = 1 \[(buf.validate.field).timestamp.lte = { seconds: 1678867200 }\];
        /// }
        /// ```
        #[prost(message, tag="4")]
        Lte(::pbjson_types::Timestamp),
        /// `lt_now` specifies that this field, of the `google.protobuf.Timestamp` type, must be less than the current time. `lt_now` can only be used with the `within` rule.
        ///
        /// ```proto
        /// message MyTimestamp {
        ///   // value must be less than now
        ///    google.protobuf.Timestamp created_at = 1 \[(buf.validate.field).timestamp.lt_now = true\];
        /// }
        /// ```
        #[prost(bool, tag="7")]
        LtNow(bool),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GreaterThan {
        /// `gt` requires the timestamp field value to be greater than the specified
        /// value (exclusive). If the value of `gt` is larger than a specified `lt`
        /// or `lte`, the range is reversed, and the field value must be outside the
        /// specified range. If the field value doesn't meet the required conditions,
        /// an error message is generated.
        ///
        /// ```proto
        /// message MyTimestamp {
        ///    // timestamp must be greater than '2023-01-01T00:00:00Z' \[timestamp.gt\]
        ///    google.protobuf.Timestamp value = 1 \[(buf.validate.field).timestamp.gt = { seconds: 1672444800 }\];
        ///
        ///    // timestamp must be greater than '2023-01-01T00:00:00Z' and less than '2023-01-02T00:00:00Z' \[timestamp.gt_lt\]
        ///    google.protobuf.Timestamp another_value = 2 \[(buf.validate.field).timestamp = { gt: { seconds: 1672444800 }, lt: { seconds: 1672531200 } }\];
        ///
        ///    // timestamp must be greater than '2023-01-02T00:00:00Z' or less than '2023-01-01T00:00:00Z' \[timestamp.gt_lt_exclusive\]
        ///    google.protobuf.Timestamp other_value = 3 \[(buf.validate.field).timestamp = { gt: { seconds: 1672531200 }, lt: { seconds: 1672444800 } }\];
        /// }
        /// ```
        #[prost(message, tag="5")]
        Gt(::pbjson_types::Timestamp),
        /// `gte` requires the timestamp field value to be greater than or equal to the
        /// specified value (exclusive). If the value of `gte` is larger than a
        /// specified `lt` or `lte`, the range is reversed, and the field value
        /// must be outside the specified range. If the field value doesn't meet
        /// the required conditions, an error message is generated.
        ///
        /// ```proto
        /// message MyTimestamp {
        ///    // timestamp must be greater than or equal to '2023-01-01T00:00:00Z' \[timestamp.gte\]
        ///    google.protobuf.Timestamp value = 1 \[(buf.validate.field).timestamp.gte = { seconds: 1672444800 }\];
        ///
        ///    // timestamp must be greater than or equal to '2023-01-01T00:00:00Z' and less than '2023-01-02T00:00:00Z' \[timestamp.gte_lt\]
        ///    google.protobuf.Timestamp another_value = 2 \[(buf.validate.field).timestamp = { gte: { seconds: 1672444800 }, lt: { seconds: 1672531200 } }\];
        ///
        ///    // timestamp must be greater than or equal to '2023-01-02T00:00:00Z' or less than '2023-01-01T00:00:00Z' \[timestamp.gte_lt_exclusive\]
        ///    google.protobuf.Timestamp other_value = 3 \[(buf.validate.field).timestamp = { gte: { seconds: 1672531200 }, lt: { seconds: 1672444800 } }\];
        /// }
        /// ```
        #[prost(message, tag="6")]
        Gte(::pbjson_types::Timestamp),
        /// `gt_now` specifies that this field, of the `google.protobuf.Timestamp` type, must be greater than the current time. `gt_now` can only be used with the `within` rule.
        ///
        /// ```proto
        /// message MyTimestamp {
        ///    // value must be greater than now
        ///    google.protobuf.Timestamp created_at = 1 \[(buf.validate.field).timestamp.gt_now = true\];
        /// }
        /// ```
        #[prost(bool, tag="8")]
        GtNow(bool),
    }
}
/// `Violations` is a collection of `Violation` messages. This message type is returned by
/// protovalidate when a proto message fails to meet the requirements set by the `Constraint` validation rules.
/// Each individual violation is represented by a `Violation` message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Violations {
    /// `violations` is a repeated field that contains all the `Violation` messages corresponding to the violations detected.
    #[prost(message, repeated, tag="1")]
    pub violations: ::prost::alloc::vec::Vec<Violation>,
}
/// `Violation` represents a single instance where a validation rule, expressed
/// as a `Constraint`, was not met. It provides information about the field that
/// caused the violation, the specific constraint that wasn't fulfilled, and a
/// human-readable error message.
///
/// ```json
/// {
///    "fieldPath": "bar",
///    "constraintId": "foo.bar",
///    "message": "bar must be greater than 0"
/// }
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Violation {
    /// `field` is a machine-readable path to the field that failed validation.
    /// This could be a nested field, in which case the path will include all the parent fields leading to the actual field that caused the violation.
    ///
    /// For example, consider the following message:
    ///
    /// ```proto
    /// message Message {
    ///    bool a = 1 \[(buf.validate.field).required = true\];
    /// }
    /// ```
    ///
    /// It could produce the following violation:
    ///
    /// ```textproto
    /// violation {
    ///    field { element { field_number: 1, field_name: "a", field_type: 8 } }
    ///    ...
    /// }
    /// ```
    #[prost(message, optional, tag="5")]
    pub field: ::core::option::Option<FieldPath>,
    /// `rule` is a machine-readable path that points to the specific constraint rule that failed validation.
    /// This will be a nested field starting from the FieldConstraints of the field that failed validation.
    /// For custom constraints, this will provide the path of the constraint, e.g. `cel\[0\]`.
    ///
    /// For example, consider the following message:
    ///
    /// ```proto
    /// message Message {
    ///    bool a = 1 \[(buf.validate.field).required = true\];
    ///    bool b = 2 [(buf.validate.field).cel = {
    ///      id: "custom_constraint",
    ///      expression: "!this ? 'b must be true': ''"
    ///    }]
    /// }
    /// ```
    ///
    /// It could produce the following violations:
    ///
    /// ```textproto
    /// violation {
    ///    rule { element { field_number: 25, field_name: "required", field_type: 8 } }
    ///    ...
    /// }
    /// violation {
    ///    rule { element { field_number: 23, field_name: "cel", field_type: 11, index: 0 } }
    ///    ...
    /// }
    /// ```
    #[prost(message, optional, tag="6")]
    pub rule: ::core::option::Option<FieldPath>,
    /// `constraint_id` is the unique identifier of the `Constraint` that was not fulfilled.
    /// This is the same `id` that was specified in the `Constraint` message, allowing easy tracing of which rule was violated.
    #[prost(string, optional, tag="2")]
    pub constraint_id: ::core::option::Option<::prost::alloc::string::String>,
    /// `message` is a human-readable error message that describes the nature of the violation.
    /// This can be the default error message from the violated `Constraint`, or it can be a custom message that gives more context about the violation.
    #[prost(string, optional, tag="3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// `for_key` indicates whether the violation was caused by a map key, rather than a value.
    #[prost(bool, optional, tag="4")]
    pub for_key: ::core::option::Option<bool>,
}
/// `FieldPath` provides a path to a nested protobuf field.
///
/// This message provides enough information to render a dotted field path even without protobuf descriptors.
/// It also provides enough information to resolve a nested field through unknown wire data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldPath {
    /// `elements` contains each element of the path, starting from the root and recursing downward.
    #[prost(message, repeated, tag="1")]
    pub elements: ::prost::alloc::vec::Vec<FieldPathElement>,
}
/// `FieldPathElement` provides enough information to nest through a single protobuf field.
///
/// If the selected field is a map or repeated field, the `subscript` value selects a specific element from it.
/// A path that refers to a value nested under a map key or repeated field index will have a `subscript` value.
/// The `field_type` field allows unambiguous resolution of a field even if descriptors are not available.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldPathElement {
    /// `field_number` is the field number this path element refers to.
    #[prost(int32, optional, tag="1")]
    pub field_number: ::core::option::Option<i32>,
    /// `field_name` contains the field name this path element refers to.
    /// This can be used to display a human-readable path even if the field number is unknown.
    #[prost(string, optional, tag="2")]
    pub field_name: ::core::option::Option<::prost::alloc::string::String>,
    /// `field_type` specifies the type of this field. When using reflection, this value is not needed.
    ///
    /// This value is provided to make it possible to traverse unknown fields through wire data.
    /// When traversing wire data, be mindful of both packed\[1\] and delimited\[2\] encoding schemes.
    ///
    /// \[1\]: <https://protobuf.dev/programming-guides/encoding/#packed>
    /// \[2\]: <https://protobuf.dev/programming-guides/encoding/#groups>
    ///
    /// N.B.: Although groups are deprecated, the corresponding delimited encoding scheme is not, and
    /// can be explicitly used in Protocol Buffers 2023 Edition.
    #[prost(enumeration="::pbjson_types::field_descriptor_proto::Type", optional, tag="3")]
    pub field_type: ::core::option::Option<i32>,
    /// `key_type` specifies the map key type of this field. This value is useful when traversing
    /// unknown fields through wire data: specifically, it allows handling the differences between
    /// different integer encodings.
    #[prost(enumeration="::pbjson_types::field_descriptor_proto::Type", optional, tag="4")]
    pub key_type: ::core::option::Option<i32>,
    /// `value_type` specifies map value type of this field. This is useful if you want to display a
    /// value inside unknown fields through wire data.
    #[prost(enumeration="::pbjson_types::field_descriptor_proto::Type", optional, tag="5")]
    pub value_type: ::core::option::Option<i32>,
    /// `subscript` contains a repeated index or map key, if this path element nests into a repeated or map field.
    #[prost(oneof="field_path_element::Subscript", tags="6, 7, 8, 9, 10")]
    pub subscript: ::core::option::Option<field_path_element::Subscript>,
}
/// Nested message and enum types in `FieldPathElement`.
pub mod field_path_element {
    /// `subscript` contains a repeated index or map key, if this path element nests into a repeated or map field.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Subscript {
        /// `index` specifies a 0-based index into a repeated field.
        #[prost(uint64, tag="6")]
        Index(u64),
        /// `bool_key` specifies a map key of type bool.
        #[prost(bool, tag="7")]
        BoolKey(bool),
        /// `int_key` specifies a map key of type int32, int64, sint32, sint64, sfixed32 or sfixed64.
        #[prost(int64, tag="8")]
        IntKey(i64),
        /// `uint_key` specifies a map key of type uint32, uint64, fixed32 or fixed64.
        #[prost(uint64, tag="9")]
        UintKey(u64),
        /// `string_key` specifies a map key of type string.
        #[prost(string, tag="10")]
        StringKey(::prost::alloc::string::String),
    }
}
/// Specifies how FieldConstraints.ignore behaves. See the documentation for
/// FieldConstraints.required for definitions of "populated" and "nullable".
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Ignore {
    /// Validation is only skipped if it's an unpopulated nullable fields.
    ///
    /// ```proto
    /// syntax="proto3";
    ///
    /// message Request {
    ///    // The uri rule applies to any value, including the empty string.
    ///    string foo = 1 [
    ///      (buf.validate.field).string.uri = true
    ///    ];
    ///
    ///    // The uri rule only applies if the field is set, including if it's
    ///    // set to the empty string.
    ///    optional string bar = 2 [
    ///      (buf.validate.field).string.uri = true
    ///    ];
    ///
    ///    // The min_items rule always applies, even if the list is empty.
    ///    repeated string baz = 3 [
    ///      (buf.validate.field).repeated.min_items = 3
    ///    ];
    ///
    ///    // The custom CEL rule applies only if the field is set, including if
    ///    // it's the "zero" value of that message.
    ///    SomeMessage quux = 4 [
    ///      (buf.validate.field).cel = {/* ... */}
    ///    ];
    /// }
    /// ```
    Unspecified = 0,
    /// Validation is skipped if the field is unpopulated. This rule is redundant
    /// if the field is already nullable.
    ///
    /// ```proto
    /// syntax="proto3
    ///
    /// message Request {
    ///    // The uri rule applies only if the value is not the empty string.
    ///    string foo = 1 [
    ///      (buf.validate.field).string.uri = true,
    ///      (buf.validate.field).ignore = IGNORE_IF_UNPOPULATED
    ///    ];
    ///
    ///    // IGNORE_IF_UNPOPULATED is equivalent to IGNORE_UNSPECIFIED in this
    ///    // case: the uri rule only applies if the field is set, including if
    ///    // it's set to the empty string.
    ///    optional string bar = 2 [
    ///      (buf.validate.field).string.uri = true,
    ///      (buf.validate.field).ignore = IGNORE_IF_UNPOPULATED
    ///    ];
    ///
    ///    // The min_items rule only applies if the list has at least one item.
    ///    repeated string baz = 3 [
    ///      (buf.validate.field).repeated.min_items = 3,
    ///      (buf.validate.field).ignore = IGNORE_IF_UNPOPULATED
    ///    ];
    ///
    ///    // IGNORE_IF_UNPOPULATED is equivalent to IGNORE_UNSPECIFIED in this
    ///    // case: the custom CEL rule applies only if the field is set, including
    ///    // if it's the "zero" value of that message.
    ///    SomeMessage quux = 4 [
    ///      (buf.validate.field).cel = {/* ... */},
    ///      (buf.validate.field).ignore = IGNORE_IF_UNPOPULATED
    ///    ];
    /// }
    /// ```
    IfUnpopulated = 1,
    /// Validation is skipped if the field is unpopulated or if it is a nullable
    /// field populated with its default value. This is typically the zero or
    /// empty value, but proto2 scalars support custom defaults. For messages, the
    /// default is a non-null message with all its fields unpopulated.
    ///
    /// ```proto
    /// syntax="proto3
    ///
    /// message Request {
    ///    // IGNORE_IF_DEFAULT_VALUE is equivalent to IGNORE_IF_UNPOPULATED in
    ///    // this case; the uri rule applies only if the value is not the empty
    ///    // string.
    ///    string foo = 1 [
    ///      (buf.validate.field).string.uri = true,
    ///      (buf.validate.field).ignore = IGNORE_IF_DEFAULT_VALUE
    ///    ];
    ///
    ///    // The uri rule only applies if the field is set to a value other than
    ///    // the empty string.
    ///    optional string bar = 2 [
    ///      (buf.validate.field).string.uri = true,
    ///      (buf.validate.field).ignore = IGNORE_IF_DEFAULT_VALUE
    ///    ];
    ///
    ///    // IGNORE_IF_DEFAULT_VALUE is equivalent to IGNORE_IF_UNPOPULATED in
    ///    // this case; the min_items rule only applies if the list has at least
    ///    // one item.
    ///    repeated string baz = 3 [
    ///      (buf.validate.field).repeated.min_items = 3,
    ///      (buf.validate.field).ignore = IGNORE_IF_DEFAULT_VALUE
    ///    ];
    ///
    ///    // The custom CEL rule only applies if the field is set to a value other
    ///    // than an empty message (i.e., fields are unpopulated).
    ///    SomeMessage quux = 4 [
    ///      (buf.validate.field).cel = {/* ... */},
    ///      (buf.validate.field).ignore = IGNORE_IF_DEFAULT_VALUE
    ///    ];
    /// }
    /// ```
    ///
    /// This rule is affected by proto2 custom default values:
    ///
    /// ```proto
    /// syntax="proto2";
    ///
    /// message Request {
    ///    // The gt rule only applies if the field is set and it's value is not
    ///    the default (i.e., not -42). The rule even applies if the field is set
    ///    to zero since the default value differs.
    ///    optional int32 value = 1 [
    ///      default = -42,
    ///      (buf.validate.field).int32.gt = 0,
    ///      (buf.validate.field).ignore = IGNORE_IF_DEFAULT_VALUE
    ///    ];
    /// }
    IfDefaultValue = 2,
    /// The validation rules of this field will be skipped and not evaluated. This
    /// is useful for situations that necessitate turning off the rules of a field
    /// containing a message that may not make sense in the current context, or to
    /// temporarily disable constraints during development.
    ///
    /// ```proto
    /// message MyMessage {
    ///    // The field's rules will always be ignored, including any validation's
    ///    // on value's fields.
    ///    MyOtherMessage value = 1 [
    ///      (buf.validate.field).ignore = IGNORE_ALWAYS];
    /// }
    /// ```
    Always = 3,
}
impl Ignore {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Ignore::Unspecified => "IGNORE_UNSPECIFIED",
            Ignore::IfUnpopulated => "IGNORE_IF_UNPOPULATED",
            Ignore::IfDefaultValue => "IGNORE_IF_DEFAULT_VALUE",
            Ignore::Always => "IGNORE_ALWAYS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IGNORE_UNSPECIFIED" => Some(Self::Unspecified),
            "IGNORE_IF_UNPOPULATED" => Some(Self::IfUnpopulated),
            "IGNORE_IF_DEFAULT_VALUE" => Some(Self::IfDefaultValue),
            "IGNORE_ALWAYS" => Some(Self::Always),
            _ => None,
        }
    }
}
/// WellKnownRegex contain some well-known patterns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KnownRegex {
    Unspecified = 0,
    /// HTTP header name as defined by [RFC 7230](<https://tools.ietf.org/html/rfc7230#section-3.2>).
    HttpHeaderName = 1,
    /// HTTP header value as defined by [RFC 7230](<https://tools.ietf.org/html/rfc7230#section-3.2.4>).
    HttpHeaderValue = 2,
}
impl KnownRegex {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KnownRegex::Unspecified => "KNOWN_REGEX_UNSPECIFIED",
            KnownRegex::HttpHeaderName => "KNOWN_REGEX_HTTP_HEADER_NAME",
            KnownRegex::HttpHeaderValue => "KNOWN_REGEX_HTTP_HEADER_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KNOWN_REGEX_UNSPECIFIED" => Some(Self::Unspecified),
            "KNOWN_REGEX_HTTP_HEADER_NAME" => Some(Self::HttpHeaderName),
            "KNOWN_REGEX_HTTP_HEADER_VALUE" => Some(Self::HttpHeaderValue),
            _ => None,
        }
    }
}
include!("buf.validate.serde.rs");
// @@protoc_insertion_point(module)