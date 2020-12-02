use serde::{Deserialize, Serialize};

use n3_machine_ffi::{MachineId, ProgramVec, Query, WorkId};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Load {
        work: WorkId,
        query: Vec<Query>,
    },
    Spawn {
        work: WorkId,
        id_primaries: Vec<MachineId>,
        id_local: MachineId,
        id_world: MachineId,
        program: ProgramVec,
        command: String,
    },
    Status {
        work: WorkId,
    },
    Join {
        work: WorkId,
    },
    Terminate {
        work: WorkId,
    },
}
