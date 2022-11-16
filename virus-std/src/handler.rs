use crate::context::Context;

pub type HandlerFunc<T> = fn (&mut Context<T>);