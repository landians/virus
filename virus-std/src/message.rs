use crate::protocol::{protocol::*, CompressType, MessageType, RoleType};

use prost::Message as ProtoMessage;

use bytes::{Buf, BufMut};

use crate::error::VirusError;

const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

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
    #[inline]
    pub fn body_length(&self) -> usize {
        self.body.encoded_len()
    }

    #[inline]
    pub fn meta_length(&self) -> usize {
        self.metadata.encoded_len()
    }

    #[inline]
    pub fn compress_type(&self) -> CompressType {
        self.metadata.compress_type.into()
    }

    #[inline]
    pub fn message_type(&self) -> MessageType {
        self.metadata.message_type.into()
    }

    #[inline]
    pub fn role_type(&self) -> RoleType {
        self.metadata.role.into()
    }

    #[inline]
    pub fn get(&self, k: &String) -> Option<&String> {
        self.metadata.values.get(k)
    }

    #[inline]
    pub fn set(&mut self, k: String, v: String) -> Option<String> {
        self.metadata.values.insert(k, v)
    }
}

pub(crate) fn encode<T>(msg: Message<T>, dst: &mut bytes::BytesMut) -> Result<(), VirusError>
where
    T: ProtoMessage,
{
    // "virus": 5bytes
    dst.put(&b"virus"[..]);

    // metadata length: 4bytes
    let meta_len = msg.metadata.encoded_len();
    dst.put_u32(meta_len as u32);

    // body length: 5bytes
    let body_len = msg.body.encoded_len();
    dst.put_u32(body_len as u32);

    // check body length
    if body_len > MAX_MESSAGE_SIZE {
        return Err("The body length is too long".into());
    }

    msg.metadata.encode(dst)?;

    msg.body.encode(dst)?;

    Ok(())
}

pub(crate) fn decode<T>(src: &mut bytes::BytesMut) -> Result<Option<Message<T>>, VirusError>
where
    T: ProtoMessage + Default,
{
    if Some(&b"virus"[..]) != src.get(0..5) {
        return Err("invalid protocol".into());
    }

    src.advance(5);

    let meta_len = src.get_u32() as usize;
    if meta_len == 0 {
        return Err("invalid metadata length".into());
    }

    let body_len = src.get_u32() as usize;
    if body_len == 0 {
        return Err("invalid body length".into());
    }

    // check body length
    if body_len > MAX_MESSAGE_SIZE {
        return Err("The body length is too long".into());
    }

    // check whether remaining buf is enough
    if src.len() < meta_len + body_len {
        // The full string has not yet arrived.
        //
        // We reserve more space in the buffer. This is not strictly
        // necessary, but is a good idea performance-wise.
        src.reserve(13 + meta_len + body_len - src.len());

        // We inform the Framed that we need more bytes to form the next
        // frame.
        return Ok(None);
    }

    let metadata = MetaData::decode(&src[..meta_len as usize])?;

    let body = T::decode(&src[meta_len as usize..])?;

    let msg = Message::new(metadata, body);

    Ok(Some(msg))
}

#[cfg(test)]
mod tests {
    use crate::protocol::{
        protocol::{Demo, MetaData},
        RoleType,
    };

    use super::Message;
    use crate::message::{decode, encode};
    use bytes::BytesMut;

    #[test]
    fn test_message_encode_decode() {
        let mut buf = BytesMut::with_capacity(256);

        let metadata = MetaData {
            role: RoleType::RoleClient as i32,
            service_name: "hello".to_string(),
            method_name: "ping".to_string(),
            ..Default::default()
        };

        let body = Demo {
            field1: "David".to_string(),
            field2: 17,
        };

        let v1 = Message::new(metadata, body);

        encode(v1, &mut buf).unwrap();

        println!("encode message length: {}", buf.len());

        let v2: Option<Message<Demo>> = decode(&mut buf).unwrap();

        println!("{:?}", v2);
    }

    #[test]
    fn test_message_method() {
        let metadata = MetaData {
            role: RoleType::RoleClient as i32,
            service_name: "hello".to_string(),
            method_name: "ping".to_string(),
            ..Default::default()
        };

        let body = Demo {
            field1: "David".to_string(),
            field2: 17,
        };

        let v1 = Message::new(metadata, body);

        println!("body length: {:?}", v1.body_length());
        println!("meta length: {:?}", v1.meta_length());
    }
}
