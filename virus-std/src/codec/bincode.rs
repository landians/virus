use std::marker::PhantomData;

use serde::{Serialize, Deserialize};

use virus::codec::{Serializer, Deserializer};

use crate::error::VirusError;

#[derive(Debug, Clone)]
pub struct BincodeCodec<T>(PhantomData<T>);

impl<T> Serializer for BincodeCodec<T> 
where
    T: Serialize
{
    type Item = T;

    type Error = VirusError;

    fn serialize(&mut self, item: Self::Item) -> Result<Vec<u8>, Self::Error> {
        let v = bincode::serialize(&item)?;
        Ok(v)
    }
}


impl<T> Deserializer for BincodeCodec<T>
where
    T: for<'de> Deserialize<'de>
{
    type Item = T;

    type Error = VirusError;

    fn deserialize(&mut self, src: &[u8]) -> Result<Option<Self::Item>, Self::Error> {
        let v = bincode::deserialize(src)?;
        Ok(Some(v))
    }
}

impl<T> Default for BincodeCodec<T> {
    fn default() -> Self {
        BincodeCodec(PhantomData)
    }
}
