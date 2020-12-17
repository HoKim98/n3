use std::env;

use n3_machine_ffi::{MachineId, MachineIdSet, WorkId};

pub fn parse_ids() -> Option<MachineIdSet> {
    let mut args = env::args().skip(1);

    let work: WorkId = args.next()?.parse().unwrap();
    let primary: MachineId = args.next()?.parse().unwrap();

    Some(MachineIdSet {
        work,
        primary,
        ..Default::default()
    })
}

pub fn parse_python_path() -> String {
    // TODO: better ideas?
    which::which("python").unwrap().display().to_string()
}
