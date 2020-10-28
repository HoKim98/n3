use n3_builder::Program;

use crate::error::{ParseError, Result};

pub struct Monitor {}

impl Monitor {
    pub fn wait(self) -> Result<()> {
        todo!()
    }
}

pub struct HostMachine {}

impl HostMachine {
    pub fn load(query: &[&str]) -> Result<Self> {
        let query: Vec<_> = query
            .iter()
            .map(|x| Query::parse(x))
            .collect::<Result<_>>()?;

        dbg!(&query);
        todo!()
    }

    pub fn spawn(&self, program: &Program) -> Result<Monitor> {
        todo!()
    }
}

#[derive(Debug)]
struct Query {
    provider: Option<String>,
    domain: Option<String>,
    device: Option<String>,
    id: Option<String>,
}

impl Query {
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
