use crate::protocol::protocol::*;

use prost::Message as ProtoMessage;

use bytes::BufMut;

use crate::error::VirusError;

#[derive(Debug, Clone)]
pub struct Message<T> {
    metadata: MetaData,
    body: T,
}

impl<T> Message<T> {
    #[inline]
    pub fn new(metadata: MetaData, body: T) -> Self {
        Message {
            metadata: metadata,
            body: body,
        }
    }
}

impl<T> Message<T>
where
    T: ProtoMessage,
{
    pub fn encode(&mut self, dst: &mut bytes::BytesMut) -> Result<(), VirusError> {
        // "virus": 5bytes
        dst.put(&b"virus"[..]);

        // metadata length: 4bytes
        let meta_length = self.metadata.encoded_len();
        dst.put_u32(meta_length as u32);

        // body length: 5bytes
        let body_length = self.body.encoded_len();
        dst.put_u32(body_length as u32);

        let header_split = dst.split_off(13);
        dst.clear();

        self.metadata.encode(dst)?;

        let meta_split = dst.split_off(meta_length);
        dst.clear();

        self.body.encode(dst)?;

        dst.unsplit(header_split);
        dst.unsplit(meta_split);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{
        protocol::{Demo, MetaData},
        Role,
    };

    use bytes::BytesMut;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_message_encode_decode() {
        let _ = BytesMut::with_capacity(256);

        let _ = Demo {
            field1: "David".to_string(),
            field2: 17,
        };

        let _ = MetaData {
            role: Role::Client as i32,
            ..Default::default()
        };
    }
}
