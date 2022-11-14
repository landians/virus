
use crate::{protocol::protocol::*};

#[derive(Debug, Clone)]
pub struct Message<T> {
    header: Vec<u8>,
    body: T,
    metadata: MetaData,
}

impl<T> Message<T> {
    #[inline]
    pub fn new(header: Vec<u8>, body: T, metadata: MetaData) -> Self {
        Message {
            header: header,
            body: body,
            metadata: metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{protocol::{protocol::MetaData, Role}, codec::bincode::BincodeEncoder};

    use super::Message;
    use bytes::{BytesMut, BufMut};
    use prost::Message;
    use serde::{Deserialize, Serialize};
    use virus::codec::Encoder;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_message_encode_decode() {
        let mut header = Vec::with_capacity(13);
        let mut buf = BytesMut::with_capacity(256);

        header.put(&b"virus"[..]);

        let body = Person {
            name: "David".to_string(),
            age: 17,
        };

        let meta = MetaData {
            role: Role::Client as i32,
            ..Default::default()
        };

        meta.encode(&mut buf).unwrap();

        header.put_u32(buf.len() as u32); 

        let mut bin_encoder = BincodeEncoder::default();

        bin_encoder.encode(body, &mut buf).unwrap(); 
    }
}
