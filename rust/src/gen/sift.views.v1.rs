// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    #[prost(string, tag="1")]
    pub view_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub axis_groups: ::core::option::Option<view::AxisGroups>,
    #[prost(message, repeated, tag="4")]
    pub channels: ::prost::alloc::vec::Vec<view::Channel>,
    #[prost(message, optional, tag="5")]
    pub created_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub modified_date: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(string, tag="7")]
    pub created_by_user_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub modified_by_user_id: ::prost::alloc::string::String,
    #[prost(bool, tag="10")]
    pub is_pinned: bool,
}
/// Nested message and enum types in `View`.
pub mod view {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AxisGroups {
        #[prost(string, repeated, tag="1")]
        pub left: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag="2")]
        pub right: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Channel {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub component: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub data_type: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub axis_group: ::prost::alloc::string::String,
        #[prost(string, repeated, tag="5")]
        pub bit_field_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, optional, tag="6")]
        pub calculated_channel_config: ::core::option::Option<channel::CalculatedChannelConfig>,
    }
    /// Nested message and enum types in `Channel`.
    pub mod channel {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CalculatedChannelConfig {
            #[prost(string, tag="1")]
            pub channel_key: ::prost::alloc::string::String,
            #[prost(map="string, message", tag="2")]
            pub channel_references: ::std::collections::HashMap<::prost::alloc::string::String, calculated_channel_config::ChannelReference>,
            #[prost(string, tag="3")]
            pub expression: ::prost::alloc::string::String,
            #[prost(string, tag="4")]
            pub unit: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `CalculatedChannelConfig`.
        pub mod calculated_channel_config {
            #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ChannelReference {
                #[prost(string, tag="1")]
                pub name: ::prost::alloc::string::String,
                #[prost(string, tag="2")]
                pub component: ::prost::alloc::string::String,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewRequest {
    #[prost(string, tag="1")]
    pub view_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetViewResponse {
    #[prost(message, optional, tag="1")]
    pub view: ::core::option::Option<View>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateViewRequest {
    #[prost(message, optional, tag="1")]
    pub view: ::core::option::Option<View>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateViewResponse {
    #[prost(message, optional, tag="1")]
    pub view: ::core::option::Option<View>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateViewRequest {
    #[prost(message, optional, tag="1")]
    pub view: ::core::option::Option<View>,
    /// The list of fields to update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::pbjson_types::FieldMask>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateViewResponse {
    #[prost(message, optional, tag="1")]
    pub view: ::core::option::Option<View>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsRequest {
    /// The maximum number of views to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 views will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListViews` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListViews` must match
    /// the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// A [Common Expression Language (CEL)](<https://github.com/google/cel-spec>) filter string
    /// Available fields to filter by are 'name', 'createdDate', and 'modifiedDate'.
    /// For further information about how to use CELs, please refer to [this guide](<https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions>).
    /// For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#view). Optional.
    #[prost(string, tag="3")]
    pub filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListViewsResponse {
    #[prost(message, repeated, tag="1")]
    pub views: ::prost::alloc::vec::Vec<View>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicableViewsRequest {
    /// The maximum number of views to return.
    /// The service may return fewer than this value.
    /// If unspecified, at most 50 views will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(uint32, tag="1")]
    pub page_size: u32,
    /// A page token, received from a previous `ListApplicableViews` call.
    /// Provide this to retrieve the subsequent page.
    /// When paginating, all other parameters provided to `ListApplicableViews` must match
    /// the call that provided the page token.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub asset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub run_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicableViewsResponse {
    #[prost(message, repeated, tag="1")]
    pub views: ::prost::alloc::vec::Vec<View>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteViewRequest {
    #[prost(string, tag="1")]
    pub view_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteViewResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinViewRequest {
    #[prost(string, tag="1")]
    pub view_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinViewResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpinViewRequest {
    #[prost(string, tag="1")]
    pub view_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnpinViewResponse {
}
include!("sift.views.v1.tonic.rs");
include!("sift.views.v1.serde.rs");
// @@protoc_insertion_point(module)