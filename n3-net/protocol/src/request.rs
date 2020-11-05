use serde::{Deserialize, Serialize};

use n3_machine_ffi::{JobId, MachineId, ProgramVec};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Load {
        job: JobId,
        query: Vec<String>,
    },
    Spawn {
        job: JobId,
        machines: Vec<MachineId>,
        program: ProgramVec,
        command: String,
    },
    Join {
        job: JobId,
    },
    Terminate {
        job: JobId,
    },
}
