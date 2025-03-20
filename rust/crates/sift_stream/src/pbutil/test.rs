use super::{encode_message_length_prefixed, ProtobufDecoder};
use sift_rs::runs::v2::Run;
use std::io::{BufReader, BufWriter, Write};

#[test]
fn test_write_format_encode_decode() {
    let runs = vec![
        Run {
            name: String::from("foo"),
            ..Default::default()
        },
        Run {
            name: String::from("bar"),
            ..Default::default()
        },
        Run {
            name: String::from("baz"),
            ..Default::default()
        },
    ];

    let mut encoded_messages = Vec::<u8>::new();
    {
        let mut writer = BufWriter::new(&mut encoded_messages);

        for run in &runs {
            let wire_format = encode_message_length_prefixed(run);
            writer
                .write_all(&wire_format)
                .expect("failed to write to buffer");
        }
    }

    let encoded = encoded_messages.clone();
    let reader = BufReader::new(encoded.as_slice());

    let decoded_messages = ProtobufDecoder::new(reader);

    for (lhs, rhs) in runs.into_iter().zip(decoded_messages) {
        assert_eq!(
            lhs, rhs,
            "expected encoded and decoded messages to be identical"
        );
    }
}
