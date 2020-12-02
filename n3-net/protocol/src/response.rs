use serde::{Deserialize, Serialize};

use n3_machine_ffi::{MachineId, WorkStatus};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Awk,
    Error { message: String },
    Load { num_machines: MachineId },
    Status { status: WorkStatus },
}

impl Response {
    pub fn load(self) -> MachineId {
        match self {
            Self::Load { num_machines } => num_machines,
            Self::Error { message } => panic!(message),
            _ => unreachable!(),
        }
    }

    pub fn status(self) -> WorkStatus {
        match self {
            Self::Status { status } => status,
            Self::Error { message } => panic!(message),
            _ => unreachable!(),
        }
    }
}
