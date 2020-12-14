mod error;
mod handler;
mod smp;

use std::env;
use std::fmt;
use std::fs::{self, File};
use std::path::PathBuf;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use self::smp::SMPool;

pub use self::error::*;
pub use self::handler::SignalHandler;

type DateTime = chrono::DateTime<Utc>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Program {
    pub id: MachineIdSet,
    pub machine: String,
    pub command: String,
    pub text: ProgramTextVec,
}

#[derive(Clone)]
pub struct WorkHandler {
    signal: SignalHandler,
    status: SMPool<WorkStatus>,
}

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

    pub master_addr: String,
    pub local_signal: String,
}

pub type MachineId = u64;

pub type ProgramTextVec = Vec<u8>;
pub type ProgramText = [u8];

pub trait Machine {
    fn spawn(&mut self, program: &mut Program, handler: &SignalHandler) -> WorkStatus;

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

impl Program {
    pub fn load(id: &MachineIdSet) -> Result<Self> {
        let path = Self::path(id);
        let file = File::open(&path).map_err(NetError::from)?;
        let this = bincode::deserialize_from(file).map_err(NetError::from)?;

        // remove unneeded file
        fs::remove_file(path).map_err(NetError::from)?;
        Ok(this)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path(&self.id);
        let file = File::create(path).map_err(NetError::from)?;
        bincode::serialize_into(file, self).map_err(NetError::from)?;
        Ok(())
    }

    fn path(id: &MachineIdSet) -> PathBuf {
        let mut path = env::temp_dir();
        path.push(format!("n3-program-{}-{}.bin", id.work, id.primary));
        path
    }
}

unsafe impl Send for WorkHandler {}

impl WorkHandler {
    pub fn new(id: &MachineIdSet) -> Result<Self> {
        Ok(Self {
            signal: SignalHandler::load(&id.local_signal),
            status: SMPool::create_or_open(Self::id_to_name(id))?,
        })
    }

    pub fn new_with_signal(id: &MachineIdSet, signal: &SignalHandler) -> Result<Self> {
        Ok(Self {
            signal: signal.clone(),
            status: SMPool::create_or_open(Self::id_to_name(id))?,
        })
    }

    fn id_to_name(id: &MachineIdSet) -> String {
        format!("work-{}", id.work)
    }

    pub fn is_running(&self) -> Result<bool> {
        self.status.with_inner(|x| x.is_running)
    }

    pub fn status(&self) -> Result<WorkStatus> {
        self.status.with_inner(|x| x.clone())
    }

    pub fn update_time(&self, total_secs: i64) -> Result<()> {
        self.status.with_inner(|x| {
            let date_begin = x.date_begin.unwrap();
            let date_end = date_begin + Duration::seconds(total_secs);
            x.date_end = Some(date_end);
        })
    }

    pub fn start(&self) -> Result<()> {
        self.status.with_inner(|x| {
            x.is_running = true;
            x.date_begin = Some(Utc::now());
        })
    }

    pub fn end_ok(&self) -> Result<()> {
        self.status.with_inner(|x| {
            x.is_running = false;
            x.date_end = Some(Utc::now());
        })
    }

    pub fn end_err<S: AsRef<str>>(&self, msg: S) -> Result<()> {
        self.status.with_inner(|x| {
            x.is_running = false;
            x.error_msg = Some(msg.as_ref().to_string());
            x.date_end = Some(Utc::now());
        })
    }
}

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
