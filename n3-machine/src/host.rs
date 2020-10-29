pub use n3_machine_ffi::Query;
use n3_machine_ffi::{Machine, Program};

use crate::error::{LoadError, ParseError, Result};

pub type Generator = unsafe fn(&Query) -> Option<Box<dyn Machine>>;

#[derive(Default)]
pub struct HostMachine {
    generators: Vec<(Query, Generator)>,
    machines: Vec<Box<dyn Machine>>,
}

impl HostMachine {
    pub fn add_generator(&mut self, query: &str, f: Generator) -> Result<()> {
        let query = Query::parse(query)?;
        self.generators.push((query, f));
        Ok(())
    }

    pub fn load(&mut self, query: &[&str]) -> Result<()> {
        let queries: Vec<_> = query
            .iter()
            .map(|x| Query::parse(x))
            .collect::<Result<_>>()?;

        for query in queries {
            if let Some(machine) = self.get_machine(&query) {
                self.machines.push(machine);
            } else {
                return LoadError::NoSuchMachine { query }.into();
            }
        }
        Ok(())
    }

    fn get_machine(&self, query: &Query) -> Option<Box<dyn Machine>> {
        for (pattern, generator) in &self.generators {
            if pattern.cmp_weakly(query) {
                if let Some(machine) = unsafe { generator(query) } {
                    return Some(machine);
                }
            }
        }
        None
    }

    pub fn spawn(&mut self, program: &Program) -> Result<()> {
        for (id, machine) in self.machines.iter_mut().enumerate() {
            machine.spawn(id, program)?;
        }
        Ok(())
    }

    pub fn join(&mut self) -> Result<()> {
        for machine in &mut self.machines {
            machine.join()?;
        }
        Ok(())
    }

    pub fn terminate(self) -> Result<()> {
        for mut machine in self.machines {
            machine.terminate()?;
            drop(machine);
        }
        Ok(())
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
