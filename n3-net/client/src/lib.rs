use std::collections::BTreeMap;
use std::net::ToSocketAddrs;

use simple_socket::SocketClient;

use n3_machine_ffi::{
    Error, MachineId, NetError, Program, Query, QueryError, Result, WorkId, WorkStatus,
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

type NetMachine = SocketClient<Request, Response>;

impl Work {
    pub fn id(&self) -> WorkId {
        self.id
    }

    fn master_machine(&self) -> Option<&NetMachine> {
        self.machines.get(0)
    }

    pub fn spawn<R>(program: &Program, command: &str, query: &[R]) -> Result<Self>
    where
        R: AsRef<str>,
    {
        let id = Self::create_work_id();
        let (machines, num_machines) = Self::load(id, query)?;

        let id_world = num_machines.iter().sum();

        let mut seed = 0;
        for (id_local, (machine, num_machines)) in machines.iter().zip(num_machines).enumerate() {
            let id_end = seed + num_machines;
            let id_machines = (seed..id_end).collect();
            seed = id_end;

            let request = Request::Spawn {
                work: id,
                id_primaries: id_machines,
                id_local: id_local as u64,
                id_world,
                program: program.to_vec(),
                command: command.to_string(),
            };
            machine.request(&request).map_err(NetError::from)?;
        }

        Ok(Self { id, machines })
    }

    pub fn join(&mut self) -> Result<()> {
        for machine in &self.machines {
            let request = Request::Join { work: self.id };
            machine.request(&request).map_err(NetError::from)?;
        }
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<()> {
        for machine in &self.machines {
            let request = Request::Terminate { work: self.id };
            machine.request(&request).map_err(NetError::from)?;
        }
        Ok(())
    }

    pub fn status(&self) -> Result<WorkStatus> {
        let machine = self.master_machine().unwrap();
        let request = Request::Status { work: self.id };
        let response = machine.request(&request).map_err(NetError::from)?;
        response.status().map_err(Error::DeviceError)
    }

    fn load<R>(id: WorkId, query: &[R]) -> Result<(Vec<NetMachine>, Vec<MachineId>)>
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

        let mut machines = vec![];
        let mut num_machines = vec![];
        for (host, query) in hosts {
            if host.provider.is_some() {
                // TODO: to be implemented
                todo!();
            }

            let addr = host.domain.unwrap_or_else(|| "localhost".to_string());
            let addr = format!("{}:{}", addr, PORT)
                .to_socket_addrs()
                .map_err(NetError::from)?
                .into_iter()
                .find(|x| x.is_ipv4()) // TODO: have we support IPv6?
                .ok_or_else(|| Error::from("Failed to parse domain address"))?;

            let socket =
                SocketClient::<Request, Response>::try_new(addr).map_err(NetError::from)?;

            let request = Request::Load { work: id, query };
            let response = socket
                .request(&request)
                .map_err(|x| NetError::from(*x))?
                .load()
                .map_err(Error::DeviceError)?;

            machines.push(socket);
            num_machines.push(response);
        }

        Ok((machines, num_machines))
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
