use std::collections::HashMap;

use crate::handler::HandlerFunc;

pub struct Service<T> {
    name: String,
    methods: HashMap<String, HandlerFunc<T>>,
}

impl<T> Service<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            methods: HashMap::new(),
        }
    }

    #[inline]
    pub fn service_name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn add_method(&mut self, method_name: &str, handler: HandlerFunc<T>) {
        let _ = self.methods.insert(method_name.to_string(), handler);
        ()
    }

    #[inline]
    pub fn method(&self, method_name: &String) -> Option<&HandlerFunc<T>> {
        self.methods.get(method_name)
    }
}
