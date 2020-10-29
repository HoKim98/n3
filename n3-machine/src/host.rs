use std::collections::BTreeMap;

pub use n3_machine_ffi::Query;
use n3_machine_ffi::{Machine, Program};

use crate::error::{LoadError, ParseError, Result};

pub type Generator = Box<dyn Fn(Query) -> Box<dyn Machine>>;

#[derive(Default)]
pub struct HostMachine {
    generators: BTreeMap<Query, Generator>,
    machines: Vec<Box<dyn Machine>>,
}

impl HostMachine {
    pub fn add_generator(&mut self, query: &str, f: Generator) -> Result<()> {
        let query = Query::parse(query)?;
        self.generators.insert(query, f);
        Ok(())
    }

    pub fn load(&mut self, query: &[&str]) -> Result<()> {
        let queries: Vec<_> = query
            .iter()
            .map(|x| Query::parse(x))
            .collect::<Result<_>>()?;

        for query in queries {
            if let Some(f) = self.get_generator(&query) {
                let machine = f.call((query,));
                self.machines.push(machine);
            } else {
                return LoadError::NoSuchMachine { query }.into();
            }
        }

        dbg!(&self.machines);
        todo!()
    }

    fn get_generator(&self, query: &Query) -> Option<&Generator> {
        dbg!("todo");
        None
    }

    pub fn spawn(&mut self, program: &Program) -> Result<()> {
        todo!()
    }

    pub fn join(&mut self) -> Result<()> {
        todo!()
    }

    pub fn terminate(self) -> Result<()> {
        for mut machine in self.machines {
            machine.terminate()?;
            drop(machine);
        }
        Ok(())
    }
}

trait QueryParse {
    fn parse(query: &str) -> Result<Self>
    where
        Self: Sized;
}

impl QueryParse for Query {
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
}
