use bytes::BytesMut;
use std::io;

/// Trait used to encode object as bytes.
pub trait Encoder {
    /// The type that is encoded.
    type Item;

    /// The type of unrecoverable frame encoding errors.
    type Error: From<io::Error>;

    /// Encodes a message into the buffer.
    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error>;
}

/// Trait used to decode object from bytes.
pub trait Decoder {
    /// The type that is decoded.
    type Item;

    /// The type of unrecoverable frame decoding errors.
    type Error: From<io::Error>;

    /// Decode a message from the buffer.
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>;
}
