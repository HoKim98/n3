use serde::{Deserialize, Serialize};

use n3_machine_ffi::MachineId;

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Awk,
    Error { message: String },
    Load { num_machines: MachineId },
}

impl Response {
    pub fn load(self) -> MachineId {
        match self {
            Self::Load { num_machines } => num_machines,
            Self::Error { message } => panic!(message),
            _ => unreachable!(),
        }
    }
}
