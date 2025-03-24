use prost::Message;
use std::{io::BufRead, marker::PhantomData, mem};

#[cfg(test)]
mod test;

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
