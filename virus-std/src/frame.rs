#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::Cursor;

use bytes::{Buf, BufMut, BytesMut};
use tracing::debug;

use crate::error::VirusError;
use crate::protocol::{protocol::*, CompressType, MessageType, RoleType};
use prost::Message as ProtoMessage;
use virus::codec::{Decoder, Encoder};

const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;
const FRAME_HEADER_SIZE: usize = 13;

#[derive(Debug)]
pub struct FrameHead {
    identifier: String,
    metadata_length: u32,
    payload_length: u32,
}

/// Frame layout
/// virus(string) + meta length(u16) + payload length(u16) + metadata([u8; meta length]) + payload([u8; meta length])
///
/// request layout
/// virus(string) + meta length(u16) + payload length(u16) + metadata([u8; meta length]) + payload([u8; meta length])
///
/// response layout
/// virus(string) + meta length(u16) + payload length(u16) + metadata([u8; meta length]) + payload([u8; meta length])
#[derive(Debug)]
pub struct Frame {
    // frame matadata, it contains role, service name, method name and so on.
    metadata: MetaData,
    // frame payload
    payload: Vec<u8>,
}

impl Frame {
    pub fn new(meta: MetaData, payload: Vec<u8>) -> Self {
        Frame {
            metadata: meta,
            payload: payload,
        }
    }

    #[inline]
    pub fn metadata_len(&self) -> usize {
        self.metadata.encoded_len()
    }

    #[inline]
    pub fn payload_len(&self) -> usize {
        self.payload.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        5 + 4 + 4 + self.metadata.encoded_len() + self.payload.len()
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
}

#[derive(Default, Debug)]
pub struct FrameCodec {}

impl Encoder for FrameCodec {
    type Item = Frame;

    type Error = VirusError;

    fn encode(&mut self, item: Self::Item, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        if item.len() > MAX_FRAME_SIZE {
            return Err("Frame size is too long".into());
        }

        // "virus": 5 bytes
        dst.put(&b"virus"[..]);

        // metadata length: 4 bytes
        dst.put_u32(item.metadata_len() as u32);

        // payload length: 4 bytes
        dst.put_u32(item.payload_len() as u32);

        // metadata: metadata length bytes
        item.metadata.encode(dst)?;

        // payload: payload length bytes
        dst.extend_from_slice(&item.payload[..]);

        debug!(
            "Encode a frame: size:{}, header size: 13, metadata size: {}, payload size: {}",
            dst.len(),
            item.metadata_len(),
            item.payload_len()
        );

        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;

    type Error = VirusError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < FRAME_HEADER_SIZE {
            return Ok(None);
        }

        let head = decode_head(src)?;

        debug!("Got head: {:?}", head);

        let length: usize =
            FRAME_HEADER_SIZE + (head.metadata_length + head.payload_length) as usize;

        debug!("Frame length: {}, buffer length: {}", length, src.len());

        if src.len() < length {
            // The full string has not yet arrived.
            //
            // We reserve more space in the buffer. This is not strictly
            // necessary, but is a good idea performance-wise.
            src.reserve(length - src.len());

            debug!("Trigger buffer reserve.");

            // We inform the Framed that we need more bytes to form the next
            // frame.
            return Ok(None);
        }

        src.advance(FRAME_HEADER_SIZE);

        debug!("buffer data: {:?}", String::from_utf8_lossy(&src.to_vec()));

        // metadata: metadata length bytes
        let metadata = MetaData::decode(&src[..head.metadata_length as usize])?;

        debug!("Get metadata: {:?}", metadata);

        // skip decoded metadata length
        src.advance(head.metadata_length as usize);

        // payload: payload length bytes
        let payload = src.copy_to_bytes(head.payload_length as usize).to_vec();

        debug!("Get payload: {:?}", String::from_utf8_lossy(&payload));

        let frame = Frame {
            metadata: metadata,
            payload: payload,
        };

        Ok(Some(frame))
    }
}

fn decode_head(src: &mut BytesMut) -> Result<FrameHead, VirusError> {
    let mut cursor = Cursor::new(&src);

    let pos = cursor.position() as u64;

    // "virus": 5 bytes
    let identifier = cursor.copy_to_bytes(5).to_vec();
    if "virus".to_string() != String::from_utf8_lossy(&identifier) {
        return Err("invalid protocol".into());
    }

    // metadata length: 4 bytes
    let metadata_len = cursor.get_u32();
    if metadata_len == 0 {
        return Err("invalid metadata length".into());
    }

    // payload length: 4 bytes
    let payload_len = cursor.get_u32();
    if payload_len == 0 {
        return Err("invalid payload length".into());
    }

    let length: usize = FRAME_HEADER_SIZE + (metadata_len + payload_len) as usize;
    if length > MAX_FRAME_SIZE {
        return Err("Frame size is too long".into());
    }

    cursor.set_position(pos);

    Ok(FrameHead {
        identifier: "virus".to_string(),
        metadata_length: metadata_len,
        payload_length: payload_len,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bytes::BytesMut;
    use virus::codec::{Decoder, Encoder};

    use crate::protocol::protocol::MetaData;

    use super::{Frame, FrameCodec};

    #[test]
    fn test_frame_encode_decode() {
//        let subscriber = tracing_subscriber::FmtSubscriber::new();
//        // use that subscriber to process traces emitted after this point
//        tracing::subscriber::set_global_default(subscriber).unwrap();

        let metadata = MetaData {
            role: 1,
            service_name: "demo".to_string(),
            method_name: "greeter".to_string(),
            seq_id: 1,
            virus_version: "0.0.1".to_string(),
            compress_type: 0,
            message_type: 0,
            values: HashMap::new(),
        };

        let value = "Hello World".as_bytes();
        let mut payload = Vec::with_capacity(value.len());
        payload.extend_from_slice(&value);

        let mut codec = FrameCodec::default();

        let frame = Frame::new(metadata, payload);

        let mut buf = BytesMut::with_capacity(128);

        codec.encode(frame, &mut buf).unwrap();

        // case one: not enough data to decode
        println!(" ===================> case one <======================");

        let mut case_one = BytesMut::with_capacity(12);
        match codec.decode(&mut case_one) {
            Ok(v) => println!("Got {:?}", v), // should match this case
            Err(e) => println!("unexpected err: {:?}", e.to_string()),
        };

        // case two: not enough space to decode, then it will trigger reserve, and return None.
        println!(" ===================> case two <======================");

        let mut case_two = BytesMut::with_capacity(25);
        case_two.extend_from_slice(&buf[..20]);

        match codec.decode(&mut case_two) {
            Ok(v) => println!("Ok Got {:?}", v), // should match this case, and get none.
            Err(e) => println!("unexpected err: {:?}", e.to_string()),
        };
        case_two.extend_from_slice(&buf[20..]);
        match codec.decode(&mut case_two) {
            Ok(v) => println!("Ok Got {:?}", v), // should match this case, and get data
            Err(e) => println!("unexpected err: {:?}", e.to_string()),
        };

        // case three: normal decode
        println!(" ===================> case three <======================");
        let decode_frame = codec.decode(&mut buf).unwrap();

        println!("decode frame: {:?}", decode_frame);
    }
}
