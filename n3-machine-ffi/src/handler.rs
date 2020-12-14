use serde::{Deserialize, Serialize};

use crate::smp::SMPool;
use chrono::Utc;

#[repr(C)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignalData {
    running: bool,
}

impl Default for SignalData {
    fn default() -> Self {
        Self { running: true }
    }
}

#[derive(Clone)]
pub struct SignalHandler {
    pool: SMPool<SignalData>,
}

unsafe impl Send for SignalHandler {}

impl Default for SignalHandler {
    fn default() -> Self {
        let time = Utc::now();
        let name = time.timestamp_nanos().to_string();

        Self {
            pool: SMPool::create(name).unwrap(),
        }
    }
}

impl SignalHandler {
    pub fn load(id: &str) -> Self {
        Self {
            pool: SMPool::open(id).unwrap(),
        }
    }

    pub fn name(&self) -> &str {
        self.pool.name()
    }

    pub fn get_running(&self) -> bool {
        self.pool.with_inner(|x| x.running).unwrap()
    }

    pub fn set_running(&self, running: bool) {
        self.pool.with_inner(|x| x.running = running).unwrap()
    }
}
