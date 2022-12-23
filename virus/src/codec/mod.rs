use bytes::BytesMut;
use std::io;

/// Trait used to encode data bytes into a data frame.
pub trait Encoder {
    /// The type that is encoded.
    type Item;

    /// The type of unrecoverable frame encoding errors.
    type Error: From<io::Error>;

    /// Encodes a message into the buffer.
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error>;
}

/// Trait used to decode frome into data bytes.
pub trait Decoder {
    /// The type that is decoded.
    type Item;

    /// The type of unrecoverable frame decoding errors.
    type Error: From<io::Error>;

    /// Decode a message from the buffer.
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>;
}

/// Trait used to serialize the object to get data bytes
pub trait Serializer {
    /// The type that is serialized.
    type Item;

    /// The type of unrecoverable object seraizlie errors.
    type Error: From<io::Error>;

    /// Serialize a object into data bytes.
    fn serialize(&mut self, item: Self::Item) -> Result<Vec<u8>, Self::Error>;
}

/// Trait used to deserialize data bytes into object.
pub trait Deserializer {
    /// The type that is deserialized.
    type Item;

    /// The type of unrecoverable object deseraizlie errors.
    type Error: From<io::Error>;

    /// Deserialize data bytes into object.
    fn deserialize(&mut self, src: &[u8]) -> Result<Option<Self::Item>, Self::Error>;
}