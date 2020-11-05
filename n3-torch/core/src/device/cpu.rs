use n3_machine::Query;

use super::base::CandidatesMachine;

pub struct CpuMachine;

impl CandidatesMachine for CpuMachine {
    fn get_candidates() -> Vec<Query> {
        vec![Query {
            device: Some("cpu".to_string()),
            id: Some("0".to_string()),
            ..Default::default()
        }]
    }
}
