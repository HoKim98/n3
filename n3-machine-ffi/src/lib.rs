mod error;
mod handler;

use std::fmt;

use serde::{Deserialize, Serialize};

pub use self::error::*;
pub use self::handler::SignalHandler;

pub type WorkId = u128;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MachineIdSet {
    pub primary: MachineId,
    pub local: MachineId,
    pub world: MachineId,
}

pub type MachineId = u64;

pub type ProgramVec = Vec<u8>;
pub type Program = [u8];

pub trait Machine {
    fn spawn(
        &mut self,
        id: MachineIdSet,
        program: &Program,
        command: &str,
        handler: SignalHandler,
    ) -> Result<()>;

    fn join(&mut self) -> Result<()>;
    fn terminate(&mut self) -> Result<()>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Query {
    pub provider: Option<String>,
    pub domain: Option<String>,
    pub device: Option<String>,
    pub id: Option<String>,
}

pub struct LocalQuery<'a>(pub &'a Query);

impl Query {
    pub fn parse<R>(query: R) -> std::result::Result<Self, ParseError>
    where
        R: AsRef<str>,
    {
        let query = query.as_ref();

        let mut tokens =
            query
                .split(':')
                .map(|x| x.to_string())
                .map(|x| if x.is_empty() { None } else { Some(x) });

        let mut provider = tokens.next();
        let mut domain = tokens.next();
        let mut device = tokens.next();
        let mut id = tokens.next();

        if tokens.next().is_some() {
            return Err(ParseError::UnexpectedTokens {
                query: query.to_string(),
            });
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
            provider: provider.flatten(),
            domain: domain.flatten(),
            device: device.flatten(),
            id: id.flatten(),
        })
    }

    pub fn eq_weakly(&self, target: &Self) -> bool {
        let test_provider = || eq_field(&self.provider, &target.provider, || true);
        let test_domain = || eq_field(&self.domain, &target.domain, test_provider);
        let test_device = || eq_field(&self.device, &target.device, test_domain);
        let test_id = || eq_field(&self.id, &target.id, test_device);
        test_id()
    }

    pub fn eq_device(&self, target: &Self) -> bool {
        let test_device = || eq_field(&self.device, &target.device, || true);
        let test_id = || eq_field(&self.id, &target.id, test_device);
        test_id()
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut write_colon = false;

        for (prefix, field) in &[
            (true, &self.provider),
            (true, &self.domain),
            (true, &self.device),
            (false, &self.id),
        ] {
            if write_colon && (*prefix || field.is_some()) {
                write!(f, ":")?;
            }
            if let Some(field) = field {
                write_colon = true;
                write!(f, "{}", field)?;
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Display for LocalQuery<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(device) = &self.0.device {
            write!(f, "{}", device)?;
        }
        if let Some(id) = &self.0.id {
            if id != "0" {
                write!(f, ":{}", id)?;
            }
        }
        Ok(())
    }
}

fn eq_field<T, F>(a: &Option<T>, b: &Option<T>, additional: F) -> bool
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
