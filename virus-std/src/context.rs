use std::any::Any;

use crate::{
    client::Client, error::VirusError, frame::Message, handler::HandlerFunc, protocol::MessageType,
};

use prost::Message as ProtoMessage;

pub struct Context<T> {
    client: Client,
    message: Message<T>,
    index: usize,
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
            index: 0,
            handles: handlers,
        }
    }

    #[inline]
    pub fn abort(&mut self) {
        self.index = usize::MAX
    }
    
    #[inline]
    pub fn next(&mut self) {
        let i = self.index;
        if i < self.handles.len()  {
            self.index += 1;
            self.handles[i](self)
        }
    }
}
