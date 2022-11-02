pub mod proto;

#[cfg(test)]
mod tests {
    use virus::codec::{Encoder, Decoder};

    use crate::protocol::protocol::{self, MetaKeyValue};

    use crate::codec::proto::{ProtoEncoder, ProtoDecoder};

    #[test]
    fn test_proto_encode_decode() {
        let v1 = protocol::MetaKeyValue {
            key: "key".to_string(),
            value: Some(protocol::meta_key_value::Value::String("value".to_string())),
        };

        let mut encoder = ProtoEncoder::default();

        let mut bytes_value = bytes::BytesMut::with_capacity(256);
        
        encoder.encode(v1, &mut bytes_value).unwrap();

        let mut decoder = ProtoDecoder::default();

        let value = decoder.decode(&mut bytes_value).unwrap();

        let v2: MetaKeyValue = value.unwrap();

        assert_eq!(v2.key, "key".to_string());
        assert_eq!(v2.value, Some(protocol::meta_key_value::Value::String("value".to_string())));
    }
}