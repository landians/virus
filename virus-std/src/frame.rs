
use std::io::Cursor;

use bytes::{Buf, BufMut, BytesMut};
use tracing::debug;

use crate::error::VirusError;
use crate::protocol::{protocol::*, CompressType, MessageType, RoleType};
use prost::Message as ProtoMessage;
use virus::codec::{Encoder, Decoder};


const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;
const FRAME_HEADER_SIZE: usize = 13;

#[derive(Debug)]
pub struct FrameHead {
    identifier: String,
    metadata_length: u16,
    payload_length: u16,
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
    pub fn new(meta: MetaData) -> Self {
        Frame { metadata: meta, payload: vec![] }
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
}

#[derive(Default)]
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
        dst.put_u16(item.metadata_len() as u16);

        // payload length: 4 bytes
        dst.put_u16(item.payload_len() as u16);

        // metadata: metadata length bytes
        let metadata_buf = Vec::with_capacity(item.metadata_len());
        dst.extend_from_slice(&metadata_buf[..]);

        // payload: payload length bytes
        dst.extend_from_slice(&item.payload[..]);

        debug!("Encode a frame: size:{}, payload size: {}", dst.len(), item.payload_len());

        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;

    type Error = VirusError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() <  FRAME_HEADER_SIZE {
           return Ok(None);
        }

        let head = decode_head(src)?;

        let length: usize = FRAME_HEADER_SIZE + (head.metadata_length + head.payload_length) as usize;

        if src.len() < length {
            // The full string has not yet arrived.
            //
            // We reserve more space in the buffer. This is not strictly
            // necessary, but is a good idea performance-wise.
            src.reserve(length - src.len());

            // We inform the Framed that we need more bytes to form the next
            // frame.
            return Ok(None);
        }

        src.advance(FRAME_HEADER_SIZE + 4 + 4);

        // metadata: metadata length bytes
        let metadata = MetaData::decode(&src[..head.metadata_length as usize])?;

        // payload: payload length bytes
        let payload = src.copy_to_bytes(head.payload_length as usize).to_vec();
        
        let frame = Frame {
            metadata:metadata,
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
    let metadata_len =  cursor.get_u16();
    if metadata_len == 0 {
        return Err("invalid metadata length".into());
    }

    // payload length: 4 bytes
    let payload_len = cursor.get_u16();
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
        payload_length: payload_len
    })
}