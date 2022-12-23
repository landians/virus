mod proto;
mod bincode;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::{BytesMut, Buf};
    use serde::{Serialize, Deserialize};

    use crate::protocol::protocol::Demo;

    use super::{bincode::BincodeCodec, proto::ProtoCodec};

    use virus::codec::{Serializer, Deserializer};

    #[derive(Debug, Serialize, Deserialize)]
    struct Animal {
        name: String,
        age: u16,
    }

    #[test]
    fn test_bincode_serialize_deserialize() {
        let v1 = Animal {
            name: "Cat".to_string(),
            age: 10,
        };

        let mut codec = BincodeCodec::default();

        let data = codec.serialize(v1).unwrap();

        let v2 = codec.deserialize(&data).unwrap();

        println!("{:?}", v2);

    }


    #[test]
    fn test_proto_serialize_deserialize() {
        let v1 = Demo {
            field1: "Cat".to_string(),
            field2: 10,
        };

        let mut codec = ProtoCodec::default();

        let data = codec.serialize(v1).unwrap();

        let v2 = codec.deserialize(&data).unwrap();

        println!("{:?}", v2);
    }

    #[test]
    fn test_cursor_usage() {
        let value = "Hello World".as_bytes();

        let mut buf = BytesMut::with_capacity(20);

        buf.extend_from_slice(&value);

        let mut cursor = Cursor::new(&buf);

        let pos = cursor.position() as u64;

        println!("start cursor postion: {}", pos);

        let d = cursor.copy_to_bytes(5).to_vec();

        let s = String::from_utf8_lossy(&d);

        assert_eq!(s, "Hello".to_string());

        println!("after copy, cursor postion: {}", cursor.position());

        let v1 = cursor.get_u32();

        println!("v1: {}", v1);

        println!("after get u32, cursor postion: {}", cursor.position());

        cursor.set_position(pos);

        println!("after reset, cursor postion: {}", cursor.position());

        let v2 = cursor.get_u32();

        println!("v2: {}", v2);
    }
}