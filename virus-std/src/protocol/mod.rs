pub mod protocol;

// "VIRUS" + META_LEN + MESSAGE_LEN + RESERVED


#[allow(unused)]
pub enum CompressType {
    CompressNone = 0,
    CompressGzip = 1,
}

#[allow(unused)]
pub enum SerializeType {
    SerializeNone = 0,
    SerializeJson = 1,
    SerializeProtobuf = 2,
}

#[allow(unused)]
pub enum MessageType {
    MessageRequest = 0,
    MessageResponse = 1,
    MessageNotify = 2,
    MessageHeartbeat = 3,
    MessageError = 4,
}

#[allow(unused)]
pub enum Role {
    Client = 0,
    Server = 1,
}

#[cfg(test)]
mod tests {
    use super::CompressType;

    #[test]
    fn enum_to_int() {
        println!("{}", CompressType::CompressNone as i32)
    }
}
