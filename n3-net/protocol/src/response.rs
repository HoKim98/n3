use serde::{Deserialize, Serialize};

use n3_machine_ffi::{MachineId, WorkStatus};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Error { message: String },
    Load { num_machines: MachineId },
    Status { status: WorkStatus },
}

impl Response {
    pub fn load(self) -> Result<MachineId, String> {
        match self {
            Self::Load { num_machines } => Ok(num_machines),
            Self::Error { message } => Err(message),
            _ => unreachable!(),
        }
    }

    pub fn status(self) -> Result<WorkStatus, String> {
        match self {
            Self::Status { status } => Ok(status),
            Self::Error { message } => Err(message),
            _ => unreachable!(),
        }
    }
}
