pub mod proto;

pub mod bincode;

#[cfg(test)]
mod tests {
    use virus::codec::{Decoder, Encoder};

    use crate::protocol::protocol::{self, MetaKeyValue};

    use crate::codec::proto::{ProtoDecoder, ProtoEncoder};

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
        assert_eq!(
            v2.value,
            Some(protocol::meta_key_value::Value::String("value".to_string()))
        );
    }

    use serde::{Deserialize, Serialize};

    use super::bincode::{BincodeDecoder, BincodeEncoder};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_bincode_encode_decode() {
        let v1 = Person {
            name: "David".to_string(),
            age: 17,
        };

        let mut encoder = BincodeEncoder::default();

        let mut bytes_value = bytes::BytesMut::with_capacity(256);

        encoder.encode(v1, &mut bytes_value).unwrap();

        let mut decoder = BincodeDecoder::default();

        let v2: Person = decoder.decode(&mut bytes_value).unwrap().unwrap();

        assert_eq!(v2.name, "David".to_string());

        assert_eq!(v2.age, 17);
    }
}
