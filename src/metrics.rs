use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct MessageStats {
    pub total_messages: AtomicUsize,
    pub total_bytes: AtomicUsize,
}

impl MessageStats {
    pub fn new() -> Self {
        Self {
            total_messages: AtomicUsize::new(0),
            total_bytes: AtomicUsize::new(0),
        }
    }

    pub fn record_message(&self, bytes: usize) {
        self.total_messages.fetch_add(1, Ordering::Relaxed);
        self.total_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn get_total_messages(&self) -> usize {
        self.total_messages.load(Ordering::Relaxed)
    }

    pub fn get_total_bytes(&self) -> usize {
        self.total_bytes.load(Ordering::Relaxed)
    }
}

impl Default for MessageStats {
    fn default() -> Self {
        Self::new()
    }
}
