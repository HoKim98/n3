mod error;
mod handler;

use std::fmt;

use serde::{Deserialize, Serialize};

pub use self::error::*;
pub use self::handler::SignalHandler;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorkStatus {
    pub is_running: bool,
    pub error_msg: Option<String>,
    pub date_begin: Option<DateTime>,
    pub date_end: Option<DateTime>,
}

pub type WorkId = u128;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MachineIdSet {
    pub work: WorkId,
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
    ) -> WorkStatus;

    fn status(&mut self) -> WorkStatus;

    fn join(&mut self) -> WorkStatus;
    fn terminate(&mut self) -> WorkStatus;
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
    pub fn parse<R>(query: R) -> std::result::Result<Self, QueryError>
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
            return Err(QueryError::UnexpectedTokens {
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

        for (postfix, field) in &[
            (false, &self.provider),
            (false, &self.domain),
            (false, &self.device),
            (true, &self.id),
        ] {
            if write_colon && (*postfix || field.is_some()) {
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
            write!(f, ":{}", id)?;
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

#[cfg(test)]
mod test {
    use super::Query;

    fn test_parse_with(machine_struct: Query, query: &str) {
        let machine_query = Query::parse(query).unwrap();
        let query_recon = machine_query.to_string();
        let machine_query_recon = Query::parse(&query_recon).unwrap();

        assert_eq!(&machine_struct, &machine_query);
        assert_eq!(&machine_struct, &machine_query_recon);
    }

    #[test]
    fn test_parse_with_local() {
        test_parse_with(Default::default(), "");
        test_parse_with(
            Query {
                device: Some("cpu".to_string()),
                ..Default::default()
            },
            "cpu",
        );
        test_parse_with(
            Query {
                device: Some("cpu".to_string()),
                id: Some("0".to_string()),
                ..Default::default()
            },
            "cpu:0",
        );
    }

    #[test]
    fn test_parse_with_domain() {
        test_parse_with(
            Query {
                domain: Some("localhost".to_string()),
                device: Some("cpu".to_string()),
                ..Default::default()
            },
            "localhost:cpu:",
        );
        test_parse_with(
            Query {
                domain: Some("localhost".to_string()),
                device: Some("cpu".to_string()),
                id: Some("0".to_string()),
                ..Default::default()
            },
            "localhost:cpu:0",
        );
    }
}
