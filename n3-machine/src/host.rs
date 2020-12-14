use std::collections::BTreeMap;

pub use n3_machine_ffi::Query;
use n3_machine_ffi::{Machine, MachineId, Program, SignalHandler, WorkId, WorkStatus};

use crate::error::{LoadError, Result, WorkError};

pub type Generator = unsafe fn(&Query) -> Vec<Box<dyn Machine>>;

#[derive(Default)]
pub struct HostMachine {
    pub handler: SignalHandler,
    generators: Vec<(Query, Generator)>,
    works_running: BTreeMap<WorkId, Vec<Box<dyn Machine>>>,
    works_ended: BTreeMap<WorkId, WorkStatus>,
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
        self.works_running.insert(work, machines);
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

    pub fn spawn(&mut self, id_primaries: Vec<MachineId>, mut program: Program) -> WorkStatus {
        program.id.local_signal = self.handler.name().to_string();

        let mut status = Default::default();

        let work = self.works_running.get_mut(&program.id.work).unwrap();
        for (id_local, (id_primary, machine)) in
            id_primaries.into_iter().zip(work.iter_mut()).enumerate()
        {
            program.id.primary = id_primary;
            program.id.local = id_local as MachineId;

            let s = machine.spawn(&mut program, &self.handler);
            if id_primary == 0 {
                status = s;
            }
        }
        status
    }

    pub fn status(&mut self, id: WorkId) -> Result<WorkStatus> {
        match self.works_running.get_mut(&id) {
            Some(work) => {
                let machine = work.get_mut(0).unwrap();
                Ok(machine.status())
            }
            None => match self.works_ended.get(&id) {
                Some(status) => Ok(status.clone()),
                None => WorkError::NoSuchWork { id }.into(),
            },
        }
    }

    pub fn join(&mut self, id: WorkId) -> Result<WorkStatus> {
        if let Some(mut work) = self.works_running.remove(&id) {
            for (id_primary, machine) in work.iter_mut().enumerate() {
                let status = machine.join();

                if id_primary == 0 {
                    self.works_ended.insert(id, status);
                }
            }

            Ok(self.works_ended[&id].clone())
        } else {
            self.status(id)
        }
    }

    pub fn join_all(&mut self) {
        for work in self.works_running.values_mut() {
            for machine in work.iter_mut() {
                machine.join();
            }
        }
        self.works_running.clear();
    }

    pub fn terminate(&mut self, id: WorkId) -> Result<WorkStatus> {
        if let Some(mut work) = self.works_running.remove(&id) {
            for (id_primary, machine) in work.iter_mut().enumerate() {
                let status = machine.terminate();

                if id_primary == 0 {
                    self.works_ended.insert(id, status);
                }
            }

            drop(work);
            Ok(self.works_ended[&id].clone())
        } else {
            self.status(id)
        }
    }
}

impl Drop for HostMachine {
    fn drop(&mut self) {
        for work in self.works_running.values_mut() {
            for machine in work.iter_mut() {
                machine.terminate();
            }
        }
    }
}
