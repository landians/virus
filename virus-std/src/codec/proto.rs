use std::marker::PhantomData;

use prost::Message;

use virus::codec::{Decoder, Encoder};

use crate::error::VirusError;

#[derive(Debug, Clone)]
pub struct ProtoEncoder<T>(PhantomData<T>);

impl<T> Encoder for ProtoEncoder<T>
where
    T: Message + Default,
{
    type Item = T;
    type Error = VirusError;

    fn encode(&mut self, item: Self::Item, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        item.encode(dst)?;
        Ok(())
    }
}

impl<T> Default for ProtoEncoder<T> {
    fn default() -> Self {
        ProtoEncoder(PhantomData)
    }
}

#[derive(Debug, Clone)]
pub struct ProtoDecoder<T>(PhantomData<T>);

impl<T> Decoder for ProtoDecoder<T>
where
    T: Message + Default,
{
    type Item = T;
    type Error = VirusError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let v = Message::decode(src)?;
        Ok(Some(v))
    }
}

impl<T> Default for ProtoDecoder<T> {
    fn default() -> Self {
        ProtoDecoder(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use virus::codec::{Encoder, Decoder};

    use crate::protocol::protocol::{self, MetaKeyValue};

    use super::{ProtoEncoder, ProtoDecoder};

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
