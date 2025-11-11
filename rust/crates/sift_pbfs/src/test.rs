use crate::{BackupsDecoder, chunk::PbfsChunk};
use sift_error::prelude::*;
use sift_rs::ingest::v1::{
    IngestWithConfigDataChannelValue, IngestWithConfigDataStreamRequest,
    ingest_with_config_data_channel_value::Type,
};
use std::{
    fs,
    io::{Seek, Write},
};
use uuid::Uuid;

#[test]
fn test_chunk_encoding_decoding() {
    let num_messages = 100;
    let mut messages = Vec::with_capacity(num_messages);
    for i in 0..num_messages {
        messages.push(IngestWithConfigDataStreamRequest {
            ingestion_config_id: "some-ingestion-config-id".into(),
            run_id: "some-run-id".into(),
            channel_values: vec![IngestWithConfigDataChannelValue {
                r#type: Some(Type::Int32(i.try_into().unwrap())),
            }],
            ..Default::default()
        })
    }

    // This encodes the chunks to a Vec<u8>
    let pbfs_chunk = PbfsChunk::new(&messages).expect("failed to encode chunk");

    // This will decode them back.
    let decoded_messages = pbfs_chunk
        .into_iter()
        .collect::<Vec<Result<IngestWithConfigDataStreamRequest>>>();

    assert_eq!(
        messages.len(),
        decoded_messages.len(),
        "expected amount of messages to be preserved"
    );

    // Ensure that decoding messages produces the same result.
    for (lhs, rhs) in messages.into_iter().zip(decoded_messages) {
        assert_eq!(lhs, rhs.expect("encountered a decoding error"));
    }
}

#[test]
fn test_writing_and_reading_from_disk_chunks() {
    let dir = Uuid::new_v4().to_string();

    let tempdir = tempdir::TempDir::new(&dir).expect("failed to create tmpdir");
    let file_path = tempdir
        .path()
        .join("test_writing_and_reading_from_disk_chunks");

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .expect("failed to create file");

    let num_messages = 100;
    let num_chunks = 2;

    let mut chunks = vec![];

    for _ in 0..num_chunks {
        let mut messages = Vec::with_capacity(num_messages);

        for i in 0..num_messages {
            messages.push(IngestWithConfigDataStreamRequest {
                ingestion_config_id: "some-ingestion-config-id".into(),
                run_id: "some-run-id".into(),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(i.try_into().unwrap())),
                }],
                ..Default::default()
            })
        }

        chunks.push(messages);
    }

    let mut pbfs_chunks = vec![];

    for chunk in &chunks {
        let pbfs_chunk = PbfsChunk::new(&chunk).expect("failed to create pbfs chunk");

        file.write_all(&pbfs_chunk)
            .expect("failed to write to file");
        file.flush().expect("failed to flush");
        file.sync_all().expect("failed to sync");

        pbfs_chunks.push(pbfs_chunk);
    }
    file.rewind().expect("failed to rewind");

    let expected_bytes_written = pbfs_chunks.iter().map(|c| c.len()).sum::<usize>();
    let file_md = fs::metadata(&file_path).expect("failed to get file metadata");
    assert_eq!(
        expected_bytes_written,
        file_md.len() as usize,
        "expected file to have same amount of bytes as chunk in-memory"
    );

    let decoder = BackupsDecoder::<IngestWithConfigDataStreamRequest, fs::File>::new(file);

    let decoded_messages = decoder.collect::<Vec<Result<IngestWithConfigDataStreamRequest>>>();

    assert_eq!(
        num_messages * num_chunks,
        decoded_messages.len(),
        "unexpected amount of chunks"
    );

    let original_messages = chunks.into_iter().flatten().collect::<Vec<_>>();

    for (decoded_pb_message, original) in decoded_messages.into_iter().zip(original_messages) {
        let decoded_pb_message = decoded_pb_message.expect("encountered pb message decode error");
        assert_eq!(
            original, decoded_pb_message,
            "expected original and decoded message to match"
        );
    }
}

#[test]
fn test_data_integrity() {
    let dir = Uuid::new_v4().to_string();

    let tempdir = tempdir::TempDir::new(&dir).expect("failed to create tmpdir");
    let file_path = tempdir.path().join("test_data_integrity");

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .expect("failed to create file");

    let num_messages = 100;
    let num_chunks = 2;

    let mut chunks = vec![];

    for _ in 0..num_chunks {
        let mut messages = Vec::with_capacity(num_messages);

        for i in 0..num_messages {
            messages.push(IngestWithConfigDataStreamRequest {
                ingestion_config_id: "some-ingestion-config-id".into(),
                run_id: "some-run-id".into(),
                channel_values: vec![IngestWithConfigDataChannelValue {
                    r#type: Some(Type::Int32(i.try_into().unwrap())),
                }],
                ..Default::default()
            })
        }

        chunks.push(messages);
    }

    let mut pbfs_chunks = vec![];

    for chunk in &chunks {
        let pbfs_chunk = PbfsChunk::new(&chunk).expect("failed to create pbfs chunk");

        file.write_all(&pbfs_chunk)
            .expect("failed to write to file");
        file.flush().expect("failed to flush");
        file.sync_all().expect("failed to sync");

        pbfs_chunks.push(pbfs_chunk);
    }

    // Corrupt the file by modifying a byte
    file.seek_relative(-5)
        .expect("failed to offset file cursor backwards");
    file.write_all(&[u8::MAX]).expect("failed to write byte");
    file.sync_all().expect("failed to sync after corruption");
    file.rewind().expect("failed to rewind after corruption");

    let decoder = BackupsDecoder::<IngestWithConfigDataStreamRequest, fs::File>::new(file);

    let decoded_chunks = decoder.collect::<Vec<Result<IngestWithConfigDataStreamRequest>>>();
    assert_eq!(
        num_messages + 1,
        decoded_chunks.len(),
        "first chunk had valid data, but second chunk is corrupt"
    );

    let first_half_decoded = &decoded_chunks[0..num_messages];

    let mut loop_entered = false;
    for (decoded, original) in first_half_decoded.iter().zip(&chunks[0]) {
        loop_entered = true;
        let decoded = decoded
            .as_ref()
            .expect("first chunk should have valid data");
        assert_eq!(original, decoded)
    }
    assert!(loop_entered, "one of the iterators were empty");

    let last_item = decoded_chunks.last().unwrap().as_ref();
    assert!(
        last_item.is_err_and(|e| e.kind() == ErrorKind::BackupIntegrityError),
        "expected backup integrity error"
    )
}
