use prost::Message;
use sift_error::prelude::*;
use std::{
    io::{ErrorKind as IoErrorKind, Read},
    marker::PhantomData,
};

pub mod chunk;
pub use chunk::{
    BATCH_SIZE_LEN, CHECKSUM_HEADER_LEN, MESSAGE_LENGTH_PREFIX_LEN, PbfsChunk, PbfsMessageIter,
};

#[cfg(test)]
mod test;

/// Takes a `reader` to the backup file containing the backed up protobuf messages and offers
/// functionality to iterate over all protobuf messages in the file. Each chunk of protobuf
/// messages will be validated by having its checksum computed and compared against the checksum
/// that stored in its byte-header.
pub struct BackupsDecoder<M, R>
where
    M: Message + Default + 'static,
    R: Read,
{
    reader: R,
    encountered_error: bool,
    current_chunk_iter: Option<PbfsMessageIter<M>>,
    message_type: PhantomData<M>,
}

impl<M, R> BackupsDecoder<M, R>
where
    M: Message + Default + 'static,
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_chunk_iter: None,
            encountered_error: false,
            message_type: PhantomData,
        }
    }

    /// This can return a [ErrorKind::BackupIntegrityError] if the checksum in the chunk's header
    /// doesn't match the computed checksum.
    fn decode_chunk(&mut self) -> Result<Option<PbfsChunk<M>>> {
        let mut headers = [0_u8; CHECKSUM_HEADER_LEN + BATCH_SIZE_LEN];

        match self.reader.read_exact(&mut headers) {
            Ok(_) => (),
            Err(err) if err.kind() == IoErrorKind::UnexpectedEof => {
                return Ok(None);
            }
            Err(err) => {
                return Err(Error::from(err))
                    .context("something unexpected occurred while decoding backup headers");
            }
        }

        let messages_len_le: [u8; 8] = headers
            [CHECKSUM_HEADER_LEN..CHECKSUM_HEADER_LEN + BATCH_SIZE_LEN]
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

        chunk[offset..CHECKSUM_HEADER_LEN].copy_from_slice(&headers[offset..CHECKSUM_HEADER_LEN]);
        offset += CHECKSUM_HEADER_LEN;

        chunk[offset..offset + BATCH_SIZE_LEN]
            .copy_from_slice(&headers[offset..offset + BATCH_SIZE_LEN]);
        offset += BATCH_SIZE_LEN;

        self.reader
            .read_exact(&mut chunk[offset..offset + messages_len])
            .map_err(Error::from)
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
    type Item = Result<M>;

    /// This will return `Some(Result::Err)` where the underlying error kind is [ErrorKind::BackupIntegrityError]
    /// if the checksum computed for a chunk doesn't match the checksum that's stored in the
    /// byte-header; a subsequent call to `next` will return `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.encountered_error {
            return None;
        }
        match self.current_chunk_iter.as_mut() {
            Some(iter) => match iter.next() {
                Some(message) => Some(message),
                None => {
                    self.current_chunk_iter = None;
                    self.next()
                }
            },
            None => match self.decode_chunk() {
                Ok(Some(chunk)) => {
                    self.current_chunk_iter = Some(chunk.into_iter());
                    self.next()
                }
                Ok(None) => None,
                Err(err) => {
                    self.encountered_error = true;
                    Some(Err(err))
                }
            },
        }
    }
}
