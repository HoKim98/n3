use std::collections::BTreeMap;
use std::net::{SocketAddr, ToSocketAddrs};

use simple_socket::SocketClient;

use n3_machine_ffi::{
    Error, MachineId, MachineIdSet, NetError, Program, ProgramText, Query, QueryError, Result,
    WorkId, WorkStatus,
};
use n3_net_protocol::{Request, Response, PORT};

pub struct Work {
    id: WorkId,
    machines: Vec<NetMachine>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NetHost {
    provider: Option<String>,
    domain: Option<String>,
}

struct LoadInfo {
    machines: Vec<NetMachine>,
    num_machines: Vec<MachineId>,
    master_addr: String,
}

type NetMachine = SocketClient<Request, Response>;

impl Work {
    pub fn id(&self) -> WorkId {
        self.id
    }

    fn master_machine(&self) -> Option<&NetMachine> {
        self.machines.get(0)
    }

    pub fn spawn<R>(program: &ProgramText, command: &str, query: &[R]) -> Result<Self>
    where
        R: AsRef<str>,
    {
        let id = Self::create_work_id();
        let LoadInfo {
            machines,
            num_machines,
            master_addr,
        } = Self::load(id, query)?;

        let id_world = num_machines.iter().sum();

        let mut seed = 0;
        for (machine, num_machines) in machines.iter().zip(num_machines) {
            let id_end = seed + num_machines;
            let id_machines = (seed..id_end).collect();
            seed = id_end;

            let request = Request::Spawn {
                id_primaries: id_machines,
                program: Program {
                    id: MachineIdSet {
                        work: id,
                        world: id_world,
                        master_addr: master_addr.to_string(),
                        ..Default::default()
                    },
                    command: command.to_string(),
                    text: program.to_vec(),
                    ..Default::default()
                },
            };
            machine.request(&request).map_err(|x| NetError(x))?;
        }

        Ok(Self { id, machines })
    }

    pub fn join(&mut self) -> Result<()> {
        for machine in &self.machines {
            let request = Request::Join { work: self.id };
            machine.request(&request).map_err(|x| NetError(x))?;
        }
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<()> {
        for machine in &self.machines {
            let request = Request::Terminate { work: self.id };
            machine.request(&request).map_err(|x| NetError(x))?;
        }
        Ok(())
    }

    pub fn status(&self) -> Result<WorkStatus> {
        let machine = self.master_machine().unwrap();
        let request = Request::Status { work: self.id };
        let response = machine.request(&request).map_err(|x| NetError(x))?;
        response.status().map_err(Error::DeviceError)
    }

    fn load<R>(id: WorkId, query: &[R]) -> Result<LoadInfo>
    where
        R: AsRef<str>,
    {
        if query.is_empty() {
            return QueryError::EmptyMachines.into();
        }

        let query: Vec<_> = query
            .iter()
            .map(|x| Query::parse(x).map_err(|x| x.into()))
            .collect::<Result<_>>()?;

        let mut hosts = BTreeMap::new();
        for query in query {
            let host = NetHost {
                provider: query.provider.clone(),
                domain: query.domain.clone(),
            };
            let entry = hosts.entry(host).or_insert_with(Vec::new);
            entry.push(query);
        }
        let num_hosts = hosts.len();

        let mut machines = vec![];
        let mut num_machines = vec![];
        let mut master_addr = None;
        for (host, query) in hosts {
            if host.provider.is_some() {
                // TODO: to be implemented
                todo!();
            }

            let addr = match host.domain {
                Some(addr) => addr,
                None => {
                    let is_distributed = num_hosts > 1;
                    if is_distributed {
                        get_public_ip()?
                    } else {
                        get_local_ip()
                    }
                }
            };
            let addr = get_ipv4(format!("{}:{}", addr, PORT))?
                .ok_or_else(|| Error::from("Failed to parse domain address"))?;

            let socket =
                SocketClient::<Request, Response>::try_new(addr).map_err(NetError::from)?;

            let request = Request::Load { work: id, query };
            let response = socket
                .request(&request)
                .map_err(|x| NetError(x))?
                .load()
                .map_err(Error::DeviceError)?;

            machines.push(socket);
            num_machines.push(response);
            if master_addr.is_none() {
                master_addr = Some(addr.ip().to_string());
            }
        }

        Ok(LoadInfo {
            machines,
            num_machines,
            master_addr: master_addr.unwrap(),
        })
    }

    fn create_work_id() -> WorkId {
        use std::time::SystemTime;

        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}

impl Drop for Work {
    fn drop(&mut self) {
        self.terminate().unwrap()
    }
}

fn get_public_ip() -> Result<String> {
    Ok(get_if_addrs::get_if_addrs()
        .map_err(NetError::from)?
        .into_iter()
        .filter(|x| x.name != "lo")
        .map(|x| x.addr.ip())
        .find(|x| x.is_ipv4()) // TODO: have we support IPv6?
        .map(|x| x.to_string())
        .unwrap_or_else(get_local_ip))
}

fn get_local_ip() -> String {
    "localhost".to_string()
}

fn get_ipv4<T>(addr: T) -> Result<Option<SocketAddr>>
where
    T: ToSocketAddrs,
{
    Ok(
        addr.to_socket_addrs()
            .map_err(NetError::from)?
            .into_iter()
            .find(|x| x.is_ipv4()), // TODO: have we support IPv6?
    )
}
