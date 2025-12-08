use std::collections::HashMap;
use std::hash::Hash;

use sift_error::prelude::*;
use sift_rs::common::r#type::v1::ChannelDataType;
use sift_rs::ingest::v1::IngestWithConfigDataChannelValue;
use sift_rs::ingest::v1::{
    IngestWithConfigDataStreamRequest, ingest_with_config_data_channel_value::Type,
};
use sift_rs::ingestion_configs::v2::FlowConfig;

use crate::{TimeValue, Value};

/// Represents the index of a channel in a flow.
///
/// This provides a convenient and performant way to access the value at the given channel index
/// when building a new flow.
///
/// This type is only returned by the [`FlowDescriptor`] when adding a new channel to the
/// flow ensuring that the index is safe to use.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ChannelIndex(usize);

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
        let pb_data_type = value.pb_data_type();
        let pb_value = value.pb_value();

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

/// Compares two sets of flows and ensures that all flows in `user_specified` has a corresponding
/// equivalent flow in `sift_flows`. If there is no corresponding flow then it either doesn't exist
/// in Sift (this would be a bug) or a user made a backwards incompatible change to their ingestion
/// config.
pub(crate) fn validate_flows(
    user_specified: &[FlowConfig],
    sift_flows: &[FlowConfig],
) -> Result<()> {
    for user_flow in user_specified {
        let num_matches_by_name = sift_flows
            .iter()
            .filter(|f| user_flow.name == f.name)
            .count();
        let num_exact_matches = sift_flows.iter().filter(|f| &user_flow == f).count();

        if num_matches_by_name > 0 && num_exact_matches == 0 {
            return Err(Error::new_msg(ErrorKind::IncompatibleIngestionConfigChange, "incompatible change to ingestion config"))
                .with_context(|| format!("flow(s) with name '{}' exist but their channel configs do not match what the user specified", user_flow.name))
                .help("Did you modify an existing flow? Try updating the the flow's name or the 'client_key' of `sift_stream::IngestionConfigForm`");
        } else if num_exact_matches == 0 {
            return Err(Error::new_msg(ErrorKind::IncompatibleIngestionConfigChange, "incompatible change to ingestion config"))
                .with_context(|| format!("flow(s) with name '{}' not found in Sift", user_flow.name))
                .help("try creating a new ingestion config by providing a new 'client_key' to `sift_stream::IngestionConfigForm` and notify Sift");
        }
    }
    Ok(())
}
