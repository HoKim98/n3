use serde::{Deserialize, Serialize};

use n3_machine_ffi::{MachineId, Program, Query, WorkId};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Load {
        work: WorkId,
        query: Vec<Query>,
    },
    Spawn {
        id_primaries: Vec<MachineId>,
        program: Program,
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
