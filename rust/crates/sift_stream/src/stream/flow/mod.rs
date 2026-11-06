use std::collections::HashMap;
use std::hash::Hash;

use sift_error::prelude::*;
use sift_rs::common::r#type::v1::{ChannelBitFieldElement, ChannelDataType, ChannelEnumType};
use sift_rs::ingest::v1::IngestWithConfigDataChannelValue;
use sift_rs::ingest::v1::{
    IngestWithConfigDataStreamRequest, ingest_with_config_data_channel_value::Type,
};
use sift_rs::ingestion_configs::v2::{ChannelConfig, FlowConfig};
use sift_rs::wrappers::ingestion_configs::{
    IngestionConfigServiceWrapper, new_ingestion_config_service,
};
use sift_rs::{RateLimitPolicy, SiftChannel};
use tokio::task::JoinError;

use crate::{TimeValue, Value, stream::retry::control_plane_retry};

/// Represents the index of a channel in a flow.
///
/// This provides a convenient and performant way to access the value at the given channel index
/// when building a new flow.
///
/// This type is only returned by the [`FlowDescriptor`] when adding a new channel to the
/// flow ensuring that the index is safe to use.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ChannelIndex(usize);

#[cfg(test)]
impl ChannelIndex {
    pub(crate) fn as_usize(self) -> usize {
        self.0
    }
}

/// Describes the schema of a flow, providing a convenient, performant, and correct way to
/// build the flow being described.
///
/// The descriptor itself is immutable, to ensure that the flow is constructed correctly
/// since successful ingestion requires Sift and the client to agree on the schema of the flow.
///
/// While the key `K` can be arbitrary, it is recommended to use a trivial key that avoids
/// allocations, such as a `usize` or `u32`, though for convenience, a string (the channel
/// name) could also be used and will still help minimize additional string allocations.
///
/// # Example
///
/// ```rust
/// use sift_stream::{FlowDescriptor, FlowDescriptorBuilder, FlowBuilder, ChannelDataType};
///
/// let mut flow_descriptor_builder = FlowDescriptorBuilder::new("ingestion_config_id", "my_flow_name");
/// let my_channel_idx = flow_descriptor_builder.add("my_channel_key", ChannelDataType::String);
/// let my_other_channel_idx = flow_descriptor_builder.add("my_other_channel_key", ChannelDataType::Uint64);
///
/// let flow_descriptor = flow_descriptor_builder.build();
///
/// let mut flow_builder = FlowBuilder::new(&flow_descriptor);
/// flow_builder.set(my_channel_idx, "my_value".to_string());
/// flow_builder.set_with_key("my_other_channel_key", 123_u64);
/// ```
#[derive(Clone)]
pub struct FlowDescriptor<K> {
    /// The name of the flow.
    name: String,

    /// The ID of the ingestion config that this flow belongs to.
    ingestion_config_id: String,

    /// The data types of the channels in the flow which will be used
    /// to validate the values when building a new flow.
    field_types: Vec<ChannelDataType>,

    /// A mapping of arbitrary keys to the index of the channel in the flow.
    ///
    /// Ideally the key should be a trivial key that avoids allocations, though
    /// for convenience, a string (the channel name) could also be used.
    index_map: HashMap<K, ChannelIndex>,
}

impl<K> FlowDescriptor<K>
where
    K: Eq + Hash,
{
    /// Initializes a new flow descriptor with the provided ingestion config ID and flow name.
    fn new(ingestion_config_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            ingestion_config_id: ingestion_config_id.into(),
            name: name.into(),
            field_types: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    /// Gets the type of the channel with the given key.
    pub fn get<Q>(&self, key: &Q) -> Option<ChannelDataType>
    where
        K: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        let index = self.index_map.get(key)?.0;
        Some(self.field_types[index])
    }

    /// Gets the mapping of keys to channel indices.
    pub fn mapping(&self) -> &HashMap<K, ChannelIndex> {
        &self.index_map
    }
}

/// Builds a [`FlowDescriptor`], which defines the schema of a flow.
///
/// The builder is mutable, to allow for the addition of channels to the flow descriptor
/// while the descriptor itself is immuatble, ensuring that the described flow will be
/// constructed correctly.
pub struct FlowDescriptorBuilder<K> {
    flow_descriptor: FlowDescriptor<K>,
}

impl<K> FlowDescriptorBuilder<K>
where
    K: Eq + Hash,
{
    /// Initializes a new [`FlowDescriptorBuilder`] with the provided ingestion config ID and flow name.
    pub fn new(ingestion_config_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            flow_descriptor: FlowDescriptor::new(ingestion_config_id, name),
        }
    }

    /// Adds a new channel to the flow.
    ///
    /// This returns the index of the channel in the flow. This index can then be used to
    /// access the value at the given channel index when building a new flow.
    pub fn add(&mut self, key: K, field_type: ChannelDataType) -> ChannelIndex {
        let index = self.flow_descriptor.field_types.len();
        self.flow_descriptor.field_types.push(field_type);

        self.flow_descriptor
            .index_map
            .insert(key, ChannelIndex(index));

        ChannelIndex(index)
    }

    /// Builds the [`FlowDescriptor`] from the builder.
    pub fn build(self) -> FlowDescriptor<K> {
        self.flow_descriptor
    }
}

impl<S> TryFrom<(S, &'_ FlowConfig)> for FlowDescriptor<String>
where
    S: ToString,
{
    type Error = Error;

    fn try_from((ingestion_config_id, flow_config): (S, &'_ FlowConfig)) -> Result<Self> {
        let mut builder =
            FlowDescriptorBuilder::new(ingestion_config_id.to_string(), flow_config.name.clone());
        for channel in flow_config.channels.iter() {
            let data_type = ChannelDataType::try_from(channel.data_type).map_err(|_| {
                Error::new_msg(
                    ErrorKind::ArgumentValidationError,
                    format!(
                        "invalid data type {:?} for channel {}",
                        channel.data_type, channel.name
                    ),
                )
            })?;

            builder.add(channel.name.clone(), data_type);
        }
        Ok(builder.build())
    }
}

impl<S> TryFrom<(S, FlowConfig)> for FlowDescriptor<String>
where
    S: ToString,
{
    type Error = Error;

    fn try_from((ingestion_config_id, flow_config): (S, FlowConfig)) -> Result<Self> {
        let mut builder =
            FlowDescriptorBuilder::new(ingestion_config_id.to_string(), flow_config.name);
        for channel in flow_config.channels {
            let data_type = ChannelDataType::try_from(channel.data_type).map_err(|_| {
                Error::new_msg(
                    ErrorKind::ArgumentValidationError,
                    format!(
                        "invalid data type {:?} for channel {}",
                        channel.data_type, channel.name
                    ),
                )
            })?;

            builder.add(channel.name, data_type);
        }
        Ok(builder.build())
    }
}

/// Builder to assist in constructing a flow, utilizing the flow descriptor
/// to ensure that the flow is constructed correctly (i.e. value in the
/// correct order and the correct data type).
///
/// By using the builder and the flow descriptor, the channel names are not
/// necessary, which helps improve performance.
pub struct FlowBuilder<'a, K> {
    /// The flow descriptor which defines the value schema of the flow.
    flow_descriptor: &'a FlowDescriptor<K>,

    /// The values of the flow, where the index of the value corresponds to
    /// the index of the channel in the [`FlowDescriptor`].
    values: Vec<IngestWithConfigDataChannelValue>,

    /// The optional run ID of the flow.
    run_id: String,
}

impl<K> FlowBuilder<'_, K> {
    /// Builds an [IngestWithConfigDataStreamRequest], consuming the builder.
    pub fn request(self, now: TimeValue) -> IngestWithConfigDataStreamRequest {
        IngestWithConfigDataStreamRequest {
            ingestion_config_id: self.flow_descriptor.ingestion_config_id.clone(),
            flow: self.flow_descriptor.name.clone(),
            timestamp: Some(now.0),
            channel_values: self.values,
            run_id: self.run_id,
            ..Default::default()
        }
    }
}

impl<'a, K> FlowBuilder<'a, K>
where
    K: Eq + Hash,
{
    /// Initializes a new flow builder with the provided flow descriptor.
    pub fn new(flow_descriptor: &'a FlowDescriptor<K>) -> Self {
        let values = vec![
            IngestWithConfigDataChannelValue {
                r#type: Some(Type::Empty(pbjson_types::Empty {}))
            };
            flow_descriptor.field_types.len()
        ];
        Self {
            flow_descriptor,
            values,
            run_id: String::new(),
        }
    }

    /// Attaches a run ID to the flow.
    pub fn attach_run_id(&mut self, run_id: impl Into<String>) {
        self.run_id = run_id.into();
    }

    /// Sets the value of the channel with the given key.
    pub fn set<V>(&mut self, index: ChannelIndex, value: V) -> Result<()>
    where
        V: Into<Value>,
    {
        let value = value.into();
        let pb_value = value.pb_value();

        // If the value is not empty, validate that the value has the correct data type.
        if !matches!(value, Value::Empty) {
            let pb_data_type = value.pb_data_type();

            // Since the [ChannelIndex] is only created by the [FlowDescriptor], we can safely
            // assume that the index is valid and index directly into the `field_types` vector.
            let expected_data_type = self.flow_descriptor.field_types[index.0];

            // Validate that the value has the correct data type.
            if expected_data_type != pb_data_type {
                return Err(Error::new_msg(
                    ErrorKind::ArgumentValidationError,
                    format!(
                        "value has incorrect data type, expected {expected_data_type:?}, got {pb_data_type:?}"
                    ),
                ));
            }
        }

        // Update the value.
        self.values[index.0].r#type = Some(pb_value);

        Ok(())
    }

    /// Sets the value of the channel with the given key.
    pub fn set_with_key<Q, V>(&mut self, key: &Q, value: V) -> Result<()>
    where
        K: core::borrow::Borrow<Q>,
        Q: Eq + Hash + ?Sized,
        V: Into<Value>,
    {
        // Get the index of the channel with the given key.
        let Some(index) = self.flow_descriptor.index_map.get(key) else {
            return Err(Error::new_msg(
                ErrorKind::NotFoundError,
                "provided key was not found in flow descriptor",
            ));
        };

        self.set(*index, value)
    }
}

#[cfg(test)]
mod test;

/// Returns `true` if two flow or channel names refer to the same Sift entity. Sift treats names
/// that differ only by case as the same name.
pub(crate) fn names_match(a: &str, b: &str) -> bool {
    a == b || a.to_lowercase() == b.to_lowercase()
}

/// Describes the first difference that makes `sift` incompatible with `local`, or returns `None`
/// if Sift's flow is equivalent for ingestion.
///
/// Sift accepts some variation in how a flow is declared and normalizes it, so a strict `==`
/// against the stored flow can report an unchanged flow as modified. This comparison accepts the
/// same variation:
///
/// - flow and channel name casing
/// - the order of enum types and bit field elements
/// - channel descriptions
///
/// Channel order, data types, units, and the sets of enum types and bit field elements must still
/// match, because ingestion is positional.
pub(crate) fn flow_difference(local: &FlowConfig, sift: &FlowConfig) -> Option<String> {
    if !names_match(&local.name, &sift.name) {
        return Some(format!(
            "flow name is '{}' locally but '{}' in Sift",
            local.name, sift.name
        ));
    }
    if local.channels.len() != sift.channels.len() {
        return Some(format!(
            "flow has {} channels locally but {} in Sift",
            local.channels.len(),
            sift.channels.len()
        ));
    }
    local
        .channels
        .iter()
        .zip(&sift.channels)
        .enumerate()
        .find_map(|(i, (l, s))| {
            channel_difference(l, s).map(|d| format!("channel {i} ('{}'): {d}", l.name))
        })
}

fn channel_difference(local: &ChannelConfig, sift: &ChannelConfig) -> Option<String> {
    if !names_match(&local.name, &sift.name) {
        return Some(format!(
            "name is '{}' locally but '{}' in Sift",
            local.name, sift.name
        ));
    }
    if local.data_type != sift.data_type {
        let name = |t: i32| {
            ChannelDataType::try_from(t)
                .map(|t| t.as_str_name().to_string())
                .unwrap_or_else(|_| t.to_string())
        };
        return Some(format!(
            "data type is {} locally but {} in Sift",
            name(local.data_type),
            name(sift.data_type)
        ));
    }
    if local.unit != sift.unit {
        return Some(format!(
            "unit is '{}' locally but '{}' in Sift",
            local.unit, sift.unit
        ));
    }
    if sorted_enum_types(local) != sorted_enum_types(sift) {
        return Some("enum types differ".into());
    }
    if sorted_bit_field_elements(local) != sorted_bit_field_elements(sift) {
        return Some("bit field elements differ".into());
    }
    None
}

fn sorted_enum_types(channel: &ChannelConfig) -> Vec<&ChannelEnumType> {
    let mut enum_types: Vec<_> = channel.enum_types.iter().collect();
    enum_types.sort_by(|a, b| (a.key, &a.name, a.is_signed).cmp(&(b.key, &b.name, b.is_signed)));
    enum_types
}

fn sorted_bit_field_elements(channel: &ChannelConfig) -> Vec<&ChannelBitFieldElement> {
    let mut elements: Vec<_> = channel.bit_field_elements.iter().collect();
    elements.sort_by(|a, b| (a.index, a.bit_count, &a.name).cmp(&(b.index, b.bit_count, &b.name)));
    elements
}

/// Returns the flow in `sift_flows` that is equivalent to `local`, if any.
pub(crate) fn find_equivalent_flow<'a>(
    local: &FlowConfig,
    sift_flows: &'a [FlowConfig],
) -> Option<&'a FlowConfig> {
    sift_flows
        .iter()
        .find(|f| flow_difference(local, f).is_none())
}

/// Compares two sets of flows and ensures that all flows in `user_specified` has a corresponding
/// equivalent flow in `sift_flows`, as defined by [`flow_difference`]. If there is no
/// corresponding flow then it either doesn't exist in Sift (this would be a bug) or a user made a
/// backwards incompatible change to their ingestion config.
pub(crate) fn validate_flows(
    user_specified: &[FlowConfig],
    sift_flows: &[FlowConfig],
) -> Result<()> {
    for user_flow in user_specified {
        let same_name: Vec<&FlowConfig> = sift_flows
            .iter()
            .filter(|f| names_match(&user_flow.name, &f.name))
            .collect();

        if same_name.is_empty() {
            return Err(Error::new_msg(ErrorKind::IncompatibleIngestionConfigChange, "incompatible change to ingestion config"))
                .with_context(|| format!("flow(s) with name '{}' not found in Sift", user_flow.name))
                .help("try creating a new ingestion config by providing a new 'client_key' to `sift_stream::IngestionConfigForm` and notify Sift");
        }

        if same_name
            .iter()
            .any(|f| flow_difference(user_flow, f).is_none())
        {
            continue;
        }

        if let Some(difference) = flow_difference(user_flow, same_name[0]) {
            return Err(Error::new_msg(ErrorKind::IncompatibleIngestionConfigChange, "incompatible change to ingestion config"))
                .with_context(|| format!("flow(s) with name '{}' exist but their channel configs do not match what the user specified: {difference}", user_flow.name))
                .help("Did you modify an existing flow? Try updating the the flow's name or the 'client_key' of `sift_stream::IngestionConfigForm`");
        }
    }
    Ok(())
}

/// Concurrently creates flows for the given ingestion config.
///
/// Takes ownership of `flow_configs` so each can be moved directly into its spawned task
/// without an intermediate clone. The per-retry clone inside each task is unavoidable.
/// Errors are returned per-flow so callers can handle them individually, including treating
/// [`ErrorKind::AlreadyExistsError`] as a success when a concurrent instance races to create
/// the same flow.
pub(crate) async fn add_new_flows(
    grpc_channel: SiftChannel,
    ingestion_config_id: &str,
    flow_configs: Vec<FlowConfig>,
    rate_limit_policy: RateLimitPolicy,
) -> Vec<std::result::Result<Result<()>, JoinError>> {
    #[cfg(feature = "tracing")]
    tracing::info!(
        ingestion_config_id =? ingestion_config_id,
        new_flows = flow_configs
            .iter()
            .map(|f| f.name.as_str())
            .collect::<Vec<&str>>()
            .join(","),
        "adding new flows to ingestion config"
    );

    let mut calls = Vec::with_capacity(flow_configs.len());

    for flow_config in flow_configs {
        let channel = grpc_channel.clone();
        let config_id = ingestion_config_id.to_string();

        calls.push(tokio::spawn(async move {
            let wrapper = new_ingestion_config_service(channel);
            let retrying = control_plane_retry(wrapper, rate_limit_policy);
            retrying
                .call(|mut w| {
                    let config_id = config_id.clone();
                    let flow_config = flow_config.clone();
                    async move { w.try_create_flows(&config_id, vec![flow_config]).await }
                })
                .await
                .context("SiftStream::add_new_flows")
        }));
    }

    futures::future::join_all(calls).await
}
