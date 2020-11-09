use std::process::Command;
use std::sync::atomic::{AtomicI64, Ordering};

use n3_machine::Query;

use super::base::CandidatesMachine;

pub struct CudaMachine;

impl CandidatesMachine for CudaMachine {
    fn get_candidates() -> Vec<Query> {
        let num_devices = NUM_DEVICES.get();
        (0..num_devices)
            .map(|id| Query {
                device: Some("cuda".to_string()),
                id: Some(id.to_string()),
                ..Default::default()
            })
            .collect()
    }
}

static NUM_DEVICES: NumDevices = NumDevices::new();

struct NumDevices(AtomicI64);

impl NumDevices {
    const fn new() -> Self {
        Self(AtomicI64::new(-1))
    }

    fn get(&self) -> u32 {
        let value = self.0.load(Ordering::SeqCst);
        if value >= 0 {
            return value as u32;
        }

        let value = {
            let nvidia_smi = Command::new("nvidia-smi")
                .args(&["--query-gpu=name", "--format=csv,noheader"])
                .output()
                .unwrap();

            String::from_utf8(nvidia_smi.stdout)
                .unwrap()
                .split('\n')
                .count() as i64
                - 1
        };

        self.0.store(value, Ordering::SeqCst);
        value as u32
    }
}
