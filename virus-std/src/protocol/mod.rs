pub mod protocol;

// "VIRUS" + META_LEN + MESSAGE_LEN + RESERVED

#[allow(unused)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum CompressType {
    CompressNone = 0,
    CompressGzip = 1,
}

#[allow(unused)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum SerializeType {
    SerializeNone = 0,
    SerializeJson = 1,
    SerializeProtobuf = 2,
}

#[allow(unused)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum MessageType {
    MessageNone = 0,
    MessageRequest = 1,
    MessageResponse = 2,
    MessageNotify = 3,
    MessageHeartbeat = 4,
    MessageError = 5,
}

#[allow(unused)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum RoleType {
    RoleNone = 0,
    RoleClient = 1,
    RoleServer = 2,
}

impl From<i32> for CompressType {
    fn from(value: i32) -> Self {
        match value {
            1 => CompressType::CompressGzip,
            _ => CompressType::CompressNone,
        }
    }
}

impl From<i32> for SerializeType {
    fn from(value: i32) -> Self {
        match value {
            1 => SerializeType::SerializeJson,
            2 => SerializeType::SerializeProtobuf,
            _ => SerializeType::SerializeNone,
        }
    }
}

impl From<i32> for MessageType {
    fn from(value: i32) -> Self {
        match value {
            1 => MessageType::MessageRequest,
            2 => MessageType::MessageResponse,
            3 => MessageType::MessageNotify,
            4 => MessageType::MessageHeartbeat,
            5 => MessageType::MessageError,
            _ => MessageType::MessageNone,
        }
    }
}

impl From<i32> for RoleType {
    fn from(value: i32) -> Self {
        match value {
            1 => RoleType::RoleClient,
            2 => RoleType::RoleServer,
            _ => RoleType::RoleNone,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{CompressType, MessageType};

    #[test]
    fn enum_to_i32() {
        println!("{}", CompressType::CompressNone as i32)
    }

    #[test]
    fn i32_into_enum() {
        assert_eq!(MessageType::MessageNotify, 3.into())
    }
}
