use std::marker::PhantomData;

use virus::codec::{Decoder, Encoder};

use crate::error::VirusError;

#[derive(Debug, Clone)]
pub struct BincodeEncoder<T>(PhantomData<T>);

impl<T> Encoder for BincodeEncoder<T>
where
    T: serde::Serialize + Sized,
{
    type Item = T;
    type Error = VirusError;

    fn encode(&mut self, item: Self::Item, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let v = bincode::serialize(&item)?;
        dst.extend_from_slice(&v[..]);
        Ok(())
    }
}

impl<T> Default for BincodeEncoder<T> {
    fn default() -> Self {
        BincodeEncoder(PhantomData)
    }
}

#[derive(Debug, Clone)]
pub struct BincodeDecoder<T>(PhantomData<T>);

impl<T> Decoder for BincodeDecoder<T>
where
    T: for<'de> serde::de::Deserialize<'de>,
{
    type Item = T;
    type Error = VirusError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let v = bincode::deserialize(&src.to_vec()[..])?;
        Ok(Some(v))
    }
}

impl<T> Default for BincodeDecoder<T> {
    fn default() -> Self {
        BincodeDecoder(PhantomData)
    }
}
