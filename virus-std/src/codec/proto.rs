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
    T: Message + Default + Sized,
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
