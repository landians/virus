use std::marker::PhantomData;

use prost::Message;

use virus::codec::{Serializer, Deserializer};

use crate::error::VirusError;

#[derive(Debug, Clone)]
pub struct ProtoCodec<T>(PhantomData<T>);

impl<T> Serializer for ProtoCodec<T>
where
    T: Message + Default,
{
    type Item = T;

    type Error = VirusError;

    fn serialize(&mut self, item: Self::Item) -> Result<Vec<u8>, Self::Error> {
        let v = item.encode_to_vec();
        Ok(v)
    }
}


impl<T> Deserializer for ProtoCodec<T>
where
    T: Message + Default,
{
    type Item = T;

    type Error = VirusError;

    fn deserialize(&mut self, src: &[u8]) -> Result<Option<Self::Item>, Self::Error> {
        let v = Message::decode(src)?;
        Ok(Some(v))
    }
}

impl<T> Default for ProtoCodec<T> {
    fn default() -> Self {
        ProtoCodec(PhantomData)
    }
}