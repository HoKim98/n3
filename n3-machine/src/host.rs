use std::collections::BTreeMap;

pub use n3_machine_ffi::Query;
use n3_machine_ffi::{Machine, MachineId, MachineIdSet, Program, SignalHandler, WorkId};

use crate::error::{LoadError, Result};

pub type Generator = unsafe fn(&Query) -> Vec<Box<dyn Machine>>;

#[derive(Default)]
pub struct HostMachine {
    pub handler: SignalHandler,
    generators: Vec<(Query, Generator)>,
    works: BTreeMap<WorkId, Vec<Box<dyn Machine>>>,
}

impl HostMachine {
    pub fn add_generator<R>(&mut self, query: R, f: Generator) -> Result<()>
    where
        R: AsRef<str>,
    {
        let query = Query::parse(query)?;
        self.generators.push((query, f));
        Ok(())
    }

    pub fn load(&mut self, work: WorkId, query: Vec<Query>) -> Result<u64> {
        let mut machines = vec![];
        for query in query {
            if let Some(mut machine) = self.get_machines(&query) {
                machines.append(&mut machine);
            } else {
                return LoadError::NoSuchMachine { query }.into();
            }
        }

        let num_machines = machines.len();
        self.works.insert(work, machines);
        Ok(num_machines as u64)
    }

    fn get_machines(&self, query: &Query) -> Option<Vec<Box<dyn Machine>>> {
        for (pattern, generator) in &self.generators {
            if pattern.eq_weakly(query) {
                let machines = unsafe { generator(query) };
                if !machines.is_empty() {
                    return Some(machines);
                }
            }
        }
        None
    }

    pub fn spawn(
        &mut self,
        work: WorkId,
        id_primaries: Vec<MachineId>,
        id_local: MachineId,
        id_world: MachineId,
        program: &Program,
        command: &str,
    ) -> Result<()> {
        let work = self.works.get_mut(&work).unwrap();
        for (id, machine) in id_primaries.into_iter().zip(work.iter_mut()) {
            let id = MachineIdSet {
                primary: id,
                local: id_local,
                world: id_world,
            };
            machine.spawn(id, program, command, self.handler.clone())?;
        }
        Ok(())
    }

    pub fn join(&mut self, work: WorkId) -> Result<()> {
        let work = self.works.remove(&work).unwrap();
        for mut machine in work {
            machine.join()?;
        }
        Ok(())
    }

    pub fn terminate(&mut self, work: WorkId) -> Result<()> {
        let work = self.works.remove(&work).unwrap();
        for mut machine in work {
            machine.terminate()?;
            drop(machine);
        }
        Ok(())
    }
}

impl Drop for HostMachine {
    fn drop(&mut self) {
        for work in self.works.values_mut() {
            for machine in work.iter_mut() {
                machine.terminate().unwrap();
            }
        }
    }
}
