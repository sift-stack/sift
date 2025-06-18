use prost::Message;
use sift_error::prelude::*;
use std::{marker::PhantomData, ops::Deref};

/// Length of the checksum byte-header length.
pub const CHECKSUM_HEADER_LEN: usize = std::mem::size_of::<u32>();

/// Length of the header that indicates the total byte-length of all protobuf messages.
pub const BATCH_SIZE_LEN: usize = std::mem::size_of::<u64>();

/// Length of the length prefix of the individual protobuf message.
pub const MESSAGE_LENGTH_PREFIX_LEN: usize = std::mem::size_of::<u32>();

/// Represents a chunk of protobuf messages that is written to and read from disk.
///
/// The byte layout of this chunk is:
///
/// ```text
/// ┌───────────┬───────────┬───────────────────────────────┐
/// │   x (4B)  │   y (8B)  │   z (4B)  | pb_message (z B)  │ * n
/// └───────────┴───────────┴───────────────────────────────┘
/// ```
///
/// - **x**: Checksum (4 bytes, little-endian)
/// - **y**: Total byte-length of all protobuf messages (8 bytes, little-endian)
/// - **z**: Length of a single protobuf message (4 bytes, little-endian)
/// - **n**: number of protobuf messages
///
/// Each protobuf message is prefixed by its length (`z`).
#[derive(Debug)]
pub struct PbfsChunk<M>
where
    M: Message + Default + 'static,
{
    data: Vec<u8>,
    message_type: PhantomData<M>,
}

pub struct PbfsMessageIter<M>
where
    M: Message + Default + 'static,
{
    data: Vec<u8>,
    offset: usize,
    encountered_error: bool,
    message_type: PhantomData<M>,
}

impl<M> PbfsChunk<M>
where
    M: Message + Default + 'static,
{
    /// Encodes `messages` and returns a [PbfsChunk] which wraps around the encoded messages.
    pub fn new(messages: &[M]) -> Result<Self> {
        let serialized_messages_length_delimited = messages
            .iter()
            .flat_map(Self::encode_message_length_prefixed)
            .collect::<Vec<u8>>();

        let messages_len = u64::try_from(serialized_messages_length_delimited.len())
            .map(|num| num.to_le_bytes())
            .map_err(|e| Error::new(ErrorKind::NumberConversionError, e))
            .context("size of messages exceeds u64 max")
            .help("this is a bug - please contact Sift")?;

        let mut data = Vec::with_capacity(
            CHECKSUM_HEADER_LEN + messages_len.len() + serialized_messages_length_delimited.len(),
        );
        data.extend_from_slice(&[0; CHECKSUM_HEADER_LEN]);
        data.extend_from_slice(&messages_len);
        data.extend_from_slice(&serialized_messages_length_delimited);

        let checksum = Self::compute_checksum(&data).to_le_bytes();

        data[0..CHECKSUM_HEADER_LEN].copy_from_slice(&checksum);

        Ok(Self {
            data,
            message_type: PhantomData,
        })
    }

    /// Serialize a protobuf message to its length-prefixed wire format. The length is a `u32` encoded
    /// as little-endian bytes.
    pub fn encode_message_length_prefixed(message: &M) -> Vec<u8> {
        let encoded = message.encode_to_vec();
        let length = (encoded.len() as u32).to_le_bytes(); // 4 bytes to store the length
        let mut wire_format = Vec::with_capacity(encoded.len() + length.len());
        wire_format.extend_from_slice(&length);
        wire_format.extend_from_slice(&encoded);
        wire_format
    }

    /// Computes the checksum from all bytes following the checksum header.
    fn compute_checksum(bytes: &[u8]) -> u32 {
        crc32fast::hash(&bytes[CHECKSUM_HEADER_LEN..])
    }

    // Retrieves the checksum value from the byte headers of the chunk.
    fn checksum_from_header(bytes: &[u8]) -> u32 {
        let mut checksum_le = [0_u8; CHECKSUM_HEADER_LEN];
        checksum_le.copy_from_slice(&bytes[0..CHECKSUM_HEADER_LEN]);
        u32::from_le_bytes(checksum_le)
    }

    /// Returns the byte length of all length-prefixed protobuf messages from the byte headers of
    /// the chunk.
    #[allow(dead_code)]
    pub fn messages_len_from_header(bytes: &[u8]) -> u64 {
        let mut messages_len_le = [0_u8; BATCH_SIZE_LEN];
        messages_len_le
            .copy_from_slice(&bytes[CHECKSUM_HEADER_LEN..CHECKSUM_HEADER_LEN + BATCH_SIZE_LEN]);
        u64::from_le_bytes(messages_len_le)
    }
}

impl<M> IntoIterator for PbfsChunk<M>
where
    M: Message + Default + 'static,
{
    type Item = Result<M>;
    type IntoIter = PbfsMessageIter<M>;

    fn into_iter(self) -> Self::IntoIter {
        PbfsMessageIter::<M>::new(self.data)
    }
}

impl<M> PbfsMessageIter<M>
where
    M: Message + Default + 'static,
{
    fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            // We'll skip over the headers since we only want the messages
            offset: CHECKSUM_HEADER_LEN + BATCH_SIZE_LEN,
            message_type: PhantomData,
            encountered_error: false,
        }
    }
}

/// Ensures that the checksum found in the byte-vector matches the computed checksum.
impl<M> TryFrom<Vec<u8>> for PbfsChunk<M>
where
    M: Message + Default + 'static,
{
    type Error = Error;

    fn try_from(data: Vec<u8>) -> Result<Self> {
        let checksum_from_header = Self::checksum_from_header(&data);
        let computed_checksum = Self::compute_checksum(&data);

        if checksum_from_header != computed_checksum {
            return Err(Error::new_msg(
                ErrorKind::BackupIntegrityError,
                "encountered backup chunk with mismatched checksums",
            ));
        }

        Ok(Self {
            data,
            message_type: PhantomData,
        })
    }
}

/// Iterates through protobuf messages, returning `Some` `Result::Err` if an error is encountered.
/// If an error is encountered, a subsequent call to `next` will end the iterator.
impl<M> Iterator for PbfsMessageIter<M>
where
    M: Message + Default + 'static,
{
    type Item = Result<M>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.data.len() || self.encountered_error {
            return None;
        }

        let mut offset = self.offset;
        let mut message_len_le = [0_u8; MESSAGE_LENGTH_PREFIX_LEN];
        message_len_le.copy_from_slice(&self.data[offset..offset + MESSAGE_LENGTH_PREFIX_LEN]);

        offset += MESSAGE_LENGTH_PREFIX_LEN;

        let message_len_result: Result<usize> = u32::from_le_bytes(message_len_le)
            .try_into()
            .map_err(|e| Error::new(ErrorKind::NumberConversionError, e))
            .context("message length prefix failed u32 -> usize")
            .help("this is a bug - please contact Sift");

        let message_len = match message_len_result {
            Ok(len) => len,
            Err(err) => {
                self.encountered_error = true;
                return Some(Err(err));
            }
        };

        let message_result = <M as Message>::decode(&self.data[offset..offset + message_len])
            .map_err(|e| Error::new(ErrorKind::ProtobufDecodeError, e))
            .context("failed to decode protobuf message")
            .help("please notify Sift");

        offset += message_len;

        match message_result {
            Ok(message) => {
                self.offset = offset;
                Some(Ok(message))
            }
            Err(err) => {
                self.encountered_error = true;
                Some(Err(err))
            }
        }
    }
}

impl<M> Deref for PbfsChunk<M>
where
    M: Message + Default + 'static,
{
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
