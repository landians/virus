use crate::{
    client::Client, error::VirusError, frame::Message, handler::HandlerFunc
};

use prost::Message as ProtoMessage;

pub struct Context<T> {
    client: Client,
    message: Message<T>,
    index: i8,
    handles: Vec<HandlerFunc<T>>,
}

impl<T> Context<T>
where
    T: ProtoMessage,
{
    pub fn new(client: Client, message: Message<T>, handlers: Vec<HandlerFunc<T>>) -> Context<T> {
        Context {
            client: client,
            message: message,
            index: -1,
            handles: handlers,
        }
    }

    #[inline]
    pub fn abort(&mut self) {
        self.index = i8::MAX
    }

    #[inline]
    pub fn next(&mut self) {
        self.index += 1;
        while (self.index as usize) < self.handles.len() {
            self.index += 1;
            self.handles[self.index as usize](self)
        }
    }

    #[inline]
    pub fn get(&self, k: &String) -> Option<&String> {
        self.message.get(k)
    }

    #[inline]
    pub fn set(&mut self, k: String, v: String) -> Option<String> {
        self.message.set(k, v)
    }

    #[inline]
    pub fn write(&self) -> Result<u32, VirusError> {
        todo!()
    }
}
