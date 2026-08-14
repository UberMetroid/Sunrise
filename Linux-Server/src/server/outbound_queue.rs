// File: Linux-Server/src/server/outbound_queue.rs
// Title: Per-Connection Outbound Frame Queue
// Plain English: Buffers unsolicited BAP frames for a single client; drained between reads.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::protocol::bap_frame::BapFrame;

#[derive(Debug, Clone, Default)]
pub struct OutboundQueue {
    inner: Arc<Mutex<VecDeque<BapFrame>>>,
}

impl OutboundQueue {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(VecDeque::new())) }
    }

    pub fn push(&self, frame: BapFrame) {
        let mut q = self.inner.lock().expect("OutboundQueue mutex poisoned");
        q.push_back(frame);
    }

    pub fn try_drain_next(&self) -> Option<BapFrame> {
        let mut q = self.inner.lock().expect("OutboundQueue mutex poisoned");
        q.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        let q = self.inner.lock().expect("OutboundQueue mutex poisoned");
        q.is_empty()
    }

    pub fn len(&self) -> usize {
        let q = self.inner.lock().expect("OutboundQueue mutex poisoned");
        q.len()
    }
}