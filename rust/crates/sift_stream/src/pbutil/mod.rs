use prost::Message;
use sift_error::prelude::*;
use std::{io::BufRead, mem};

#[cfg(test)]
mod test;

/// Serialize protobuf message to length-prefixed write format.
pub fn encode_message_length_prefixed<M: Message>(message: &M) -> Vec<u8> {
    let encoded = message.encode_to_vec();
    let length = (encoded.len() as u32).to_le_bytes(); // 4 bytes to store the length
    let mut wire_format = Vec::with_capacity(encoded.len() + length.len());
    wire_format.extend_from_slice(&length);
    wire_format.extend_from_slice(&encoded);
    wire_format
}

/// Deserialize protobuf messages from a reader.
pub fn decode_messages_length_prefixed<R, M>(mut reader: R) -> Result<Vec<M>>
where
    R: BufRead,
    M: Message + Default,
{
    let mut messages = Vec::new();
    loop {
        let mut length_buf = [0; mem::size_of::<u32>()];
        if reader.read_exact(&mut length_buf).is_err() {
            break; // EOF
        }

        let length = u32::from_le_bytes(length_buf) as usize;
        let mut buffer = vec![0; length];
        reader.read_exact(&mut buffer)?;

        let message = <M as Message>::decode(&buffer[..])
            .map_err(|e| Error::new(ErrorKind::ProtobufError, e))
            .context("failed to decode decode protobuf message")?;

        messages.push(message);
    }
    Ok(messages)
}
