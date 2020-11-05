use std::collections::BTreeMap;

pub use n3_machine_ffi::Query;
use n3_machine_ffi::{JobId, Machine, MachineId, MachineIdSet, Program, SignalHandler};

use crate::error::{LoadError, Result};

pub type Generator = unsafe fn(&Query) -> Vec<Box<dyn Machine>>;

#[derive(Default)]
pub struct HostMachine {
    pub handler: SignalHandler,
    generators: Vec<(Query, Generator)>,
    jobs: BTreeMap<JobId, Vec<Box<dyn Machine>>>,
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

    pub fn load(&mut self, job: JobId, query: Vec<Query>) -> Result<u64> {
        let mut machines = vec![];
        for query in query {
            if let Some(mut machine) = self.get_machines(&query) {
                machines.append(&mut machine);
            } else {
                return LoadError::NoSuchMachine { query }.into();
            }
        }

        let num_machines = machines.len();
        self.jobs.insert(job, machines);
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
        job: JobId,
        id_primaries: Vec<MachineId>,
        id_local: MachineId,
        id_world: MachineId,
        program: &Program,
        command: &str,
    ) -> Result<()> {
        let job = self.jobs.get_mut(&job).unwrap();
        for (id, machine) in id_primaries.into_iter().zip(job.iter_mut()) {
            let id = MachineIdSet {
                primary: id,
                local: id_local,
                world: id_world,
            };
            machine.spawn(id, program, command, self.handler.clone())?;
        }
        Ok(())
    }

    pub fn join(&mut self, job: JobId) -> Result<()> {
        let job = self.jobs.remove(&job).unwrap();
        for mut machine in job {
            machine.join()?;
        }
        Ok(())
    }

    pub fn terminate(&mut self, job: JobId) -> Result<()> {
        let job = self.jobs.remove(&job).unwrap();
        for mut machine in job {
            machine.terminate()?;
            drop(machine);
        }
        Ok(())
    }
}

impl Drop for HostMachine {
    fn drop(&mut self) {
        for job in self.jobs.values_mut() {
            for machine in job.iter_mut() {
                machine.terminate().unwrap();
            }
        }
    }
}
