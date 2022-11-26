use crate::protocol::{protocol::*, MessageType, CompressType, SerializeType, RoleType};

use prost::Message as ProtoMessage;

use bytes::{Buf, BufMut};

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
    pub fn serialize_type(&self) -> SerializeType {
        self.metadata.serialize_type.into()
    }

    #[inline]
    pub fn message_type(&self) -> MessageType {
        self.metadata.message_type.into()
    }

    #[inline]
    pub fn role_type(&self) -> RoleType {
        self.metadata.role.into()
    }
}

pub(crate) fn encode<T>(msg: Message<T>, dst: &mut bytes::BytesMut) -> Result<(), VirusError>
where
    T: ProtoMessage,
{
    // "virus": 5bytes
    dst.put(&b"virus"[..]);

    // metadata length: 4bytes
    let meta_length = msg.metadata.encoded_len();
    dst.put_u32(meta_length as u32);

    // body length: 5bytes
    let body_length = msg.body.encoded_len();
    dst.put_u32(body_length as u32);

    msg.metadata.encode(dst)?;

    msg.body.encode(dst)?;

    Ok(())
}

pub(crate) fn decode<T>(src: &mut bytes::BytesMut) -> Result<Message<T>, VirusError>
where
    T: ProtoMessage + Default,
{
    if Some(&b"virus"[..]) != src.get(0..5) {
        return Err("invalid protocol".into());
    }

    src.advance(5);

    let meta_len = src.get_u32();
    if meta_len == 0 {
        return Err("invalid metadata length".into());
    }

    let body_len = src.get_u32();
    if body_len == 0 {
        return Err("invalid body length".into());
    }

    let metadata = MetaData::decode(&src[..meta_len as usize])?;

    let body = T::decode(&src[meta_len as usize..])?;

    let msg = Message::new(metadata, body);

    Ok(msg)
}

#[cfg(test)]
mod tests {
    use crate::protocol::{
        protocol::{Demo, MetaData},
        RoleType,
    };

    use super::Message;
    use crate::frame::{decode, encode};
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

        let v2: Message<Demo> = decode(&mut buf).unwrap();

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
