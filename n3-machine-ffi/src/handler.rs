use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SignalHandler {
    running: Arc<AtomicBool>,
}

impl Default for SignalHandler {
    fn default() -> Self {
        SignalHandler {
            running: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl SignalHandler {
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn set(&self, running: bool) {
        self.running.store(running, Ordering::SeqCst)
    }
}
