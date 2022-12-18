use std::sync::{Arc, Mutex};

/// Middleware thread safely stores messages coming both from user and server as vectors of bytes
pub struct Middleware {
    pub incoming_messages: Arc<Mutex<Vec<u8>>>,
    pub outgoing_messages: Arc<Mutex<Vec<u8>>>,
}

impl Middleware {
    pub fn new() -> Self {
        Self {
            incoming_messages: Arc::new(Mutex::new(Vec::new())),
            outgoing_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
