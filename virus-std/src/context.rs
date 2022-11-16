use crate::{client::Client, frame::Message, handler::HandlerFunc};

pub struct Context<T> {
    client: Client,
    message: Message<T>,
    index: u32,
    handles: Vec<HandlerFunc<T>>,
}
