use std::collections::BTreeMap;

pub use n3_machine_ffi::Query;
use n3_machine_ffi::{JobId, Machine, MachineId, Program};

use crate::error::{LoadError, ParseError, Result};

pub type Generator = unsafe fn(&Query) -> Vec<Box<dyn Machine>>;

#[derive(Default)]
pub struct HostMachine {
    generators: Vec<(Query, Generator)>,
    jobs: BTreeMap<JobId, Vec<Box<dyn Machine>>>,
}

impl HostMachine {
    pub fn add_generator(&mut self, query: &str, f: Generator) -> Result<()> {
        let query = Query::parse(query)?;
        self.generators.push((query, f));
        Ok(())
    }

    pub fn load(&mut self, job: JobId, query: &[String]) -> Result<u64> {
        let queries: Vec<_> = query
            .iter()
            .map(|x| Query::parse(x))
            .collect::<Result<_>>()?;

        let mut machines = vec![];
        for query in queries {
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
            if pattern.cmp_weakly(query) {
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
        machines: Vec<MachineId>,
        program: &Program,
        command: &str,
    ) -> Result<()> {
        let job = self.jobs.get_mut(&job).unwrap();
        for (id, machine) in machines.into_iter().zip(job.iter_mut()) {
            machine.spawn(id, program, command)?;
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

trait QueryImpl {
    fn parse(query: &str) -> Result<Self>
    where
        Self: Sized;

    fn cmp_weakly(&self, target: &Self) -> bool;
}

impl QueryImpl for Query {
    fn parse(query: &str) -> Result<Self> {
        let mut tokens = query.split(':').map(|x| x.to_string());

        let mut provider = tokens.next();
        let mut domain = tokens.next();
        let mut device = tokens.next();
        let mut id = tokens.next();

        if tokens.next().is_some() {
            return ParseError::UnexpectedTokens {
                query: query.to_string(),
            }
            .into();
        }

        if domain.is_none() {
            domain = provider.take();
        }
        if device.is_none() {
            device = domain.take();
            domain = provider.take();
        }
        if id.is_none() && domain.is_some() {
            id = device.take();
            device = domain.take();
            domain = provider.take();
        }

        Ok(Query {
            provider,
            domain,
            device,
            id,
        })
    }

    fn cmp_weakly(&self, target: &Self) -> bool {
        fn cmp_field<T, F>(a: &Option<T>, b: &Option<T>, additional: F) -> bool
        where
            T: PartialEq + Eq,
            F: FnOnce() -> bool,
        {
            if a.is_none() || a == b {
                additional()
            } else {
                false
            }
        }

        let test_provider = || cmp_field(&self.provider, &target.provider, || true);
        let test_domain = || cmp_field(&self.domain, &target.domain, test_provider);
        let test_device = || cmp_field(&self.device, &target.device, test_domain);
        let test_id = || cmp_field(&self.id, &target.id, test_device);
        test_id()
    }
}
