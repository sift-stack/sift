use prost::Message;
use sift_error::prelude::*;
use std::{io::{BufRead, Read, ErrorKind as IoErrorKind}, marker::PhantomData, mem};

mod chunk;
use chunk::{PbfsChunk, CHECKSUM_HEADER_LEN, MESSAGES_LEN_HEADER_LEN};

#[cfg(test)]
mod test;

pub struct BackupsDecoder<M, R>
where
    M: Message + Default + 'static,
    R: Read,
{
    reader: R,
    encountered_error: bool,
    message_type: PhantomData<M>,
}

impl<M, R> BackupsDecoder<M, R>
where
    M: Message + Default + 'static,
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self { reader, encountered_error: false, message_type: PhantomData }
    }

    /// This can return a [ErrorKind::BackupIntegrityError] if the checksum in the chunk's header
    /// doesn't match the computed checksum.
    pub fn decode_chunk(&mut self) -> Result<Option<PbfsChunk<M>>> {
        let mut headers = [0_u8; CHECKSUM_HEADER_LEN + MESSAGES_LEN_HEADER_LEN];

        match self.reader.read_exact(&mut headers) {
            Ok(_) => (),
            Err(err) if err.kind() == IoErrorKind::UnexpectedEof => {
                return Ok(None);
            },
            Err(err) => {
                return Err(Error::from(err)).context("something unexpected occurred while decoding backup headers")
            }
        }

        let messages_len_le: [u8; 8] = headers[
            CHECKSUM_HEADER_LEN..CHECKSUM_HEADER_LEN+MESSAGES_LEN_HEADER_LEN
        ]
        .try_into()
        .map_err(|e| Error::new(ErrorKind::ProtobufDecodeError, e))
        .context("failed to create buffer of length of all messages")
        .help("please contact Sift")?;

        let messages_len: usize = u64::from_le_bytes(messages_len_le)
            .try_into()
            .map_err(|e| Error::new(ErrorKind::ProtobufDecodeError, e))
            .context("failed to decode length of all messages")
            .help("this is a bug - please contact Sift")?;

        let mut offset = 0;
        let mut chunk = vec![0_u8; headers.len() + messages_len];

        chunk[offset..CHECKSUM_HEADER_LEN].copy_from_slice(
            &headers[offset..CHECKSUM_HEADER_LEN]
        );
        offset += CHECKSUM_HEADER_LEN;

        chunk[offset..offset+MESSAGES_LEN_HEADER_LEN].copy_from_slice(
            &headers[offset..offset+MESSAGES_LEN_HEADER_LEN]
        );
        offset += MESSAGES_LEN_HEADER_LEN;

        self.reader.read_exact(&mut chunk[offset..offset+messages_len])
            .map_err(|e| Error::from(e))
            .context("unexpected EOF while reading messages")
            .help("this is a bug - please contact Sift")?;

        let verified_chunk = PbfsChunk::<M>::try_from(chunk)?;

        Ok(Some(verified_chunk))
    }
}

impl<M, R> Iterator for BackupsDecoder<M, R>
where
    M: Message + Default + 'static,
    R: Read,
{
    type Item = Result<PbfsChunk<M>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.encountered_error {
            return None;
        }
        match self.decode_chunk() {
            Ok(Some(chunk)) => Some(Ok(chunk)),
            Ok(None) => None,
            Err(err) => {
                self.encountered_error = true;
                Some(Err(err))
            }
        }
    }
}


/// Serialize a protobuf message to its length-prefixed write format. The length is a `u32` encoded
/// as little-endian bytes.
pub fn encode_message_length_prefixed<M: Message>(message: &M) -> Vec<u8> {
    let encoded = message.encode_to_vec();
    let length = (encoded.len() as u32).to_le_bytes(); // 4 bytes to store the length
    let mut wire_format = Vec::with_capacity(encoded.len() + length.len());
    wire_format.extend_from_slice(&length);
    wire_format.extend_from_slice(&encoded);
    wire_format
}

/// Constructed from a [BufRead] that is expected to contain protobuf message(s) written as
/// length-prefixed wire format. Iterating over [ProtobufDecoder] will decode and yield each
/// message.
pub struct ProtobufDecoder<R, M> {
    reader: R,
    item: PhantomData<M>,
}

impl<R, M> ProtobufDecoder<R, M>
where
    R: BufRead,
    M: Message + Default,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            item: PhantomData,
        }
    }
}

impl<R, M> Iterator for ProtobufDecoder<R, M>
where
    R: BufRead,
    M: Message + Default,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut length_buf = [0; mem::size_of::<u32>()];

            if self.reader.read_exact(&mut length_buf).is_err() {
                return None;
            }

            let length = u32::from_le_bytes(length_buf) as usize;
            let mut buffer = vec![0; length];

            if self.reader.read_exact(&mut buffer).is_err() {
                continue;
            }
            let Ok(message) = <M as Message>::decode(&buffer[..]) else {
                continue;
            };

            return Some(message);
        }
    }
}
