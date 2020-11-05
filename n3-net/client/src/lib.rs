use std::collections::BTreeMap;
use std::net::ToSocketAddrs;

use simple_socket::SocketClient;

use n3_machine_ffi::{ExternalError as Error, JobId, MachineId, Program, Query, Result};
use n3_net_protocol::{Request, Response, PORT};

pub struct Job {
    id: JobId,
    machines: Vec<SocketClient<Request, Response>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetHost {
    provider: Option<String>,
    domain: Option<String>,
}

type NetMachine = SocketClient<Request, Response>;

impl Job {
    pub fn spawn<R>(id: JobId, query: &[R], program: &Program, command: &str) -> Result<Self>
    where
        R: AsRef<str>,
    {
        let (machines, num_machines) = Self::load(id, query)?;

        let mut seed = 0;
        for (machine, num_machines) in machines.iter().zip(num_machines) {
            let id_end = seed + num_machines;
            let id_machines: Vec<_> = (seed..id_end).collect();
            seed = id_end;

            let request = Request::Spawn {
                job: id,
                machines: id_machines,
                program: program.to_vec(),
                command: command.to_string(),
            };
            machine.request(&request).map_err(Error::from)?;
        }

        Ok(Self { id, machines })
    }

    pub fn join(&mut self) -> Result<()> {
        for machine in &self.machines {
            let request = Request::Join { job: self.id };
            machine.request(&request).map_err(Error::from)?;
        }
        self.machines.clear();
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<()> {
        for machine in &self.machines {
            let request = Request::Terminate { job: self.id };
            machine.request(&request).map_err(Error::from)?;
        }
        self.machines.clear();
        Ok(())
    }

    fn load<R>(id: JobId, query: &[R]) -> Result<(Vec<NetMachine>, Vec<MachineId>)>
    where
        R: AsRef<str>,
    {
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
                .map_err(Error::from)?
                .into_iter()
                .next()
                .ok_or_else(|| Error::from("Failed to parse domain address"))?;

            let socket = SocketClient::<Request, Response>::try_new(addr).map_err(Error::from)?;

            let request = Request::Load { job: id, query };
            let response = socket
                .request(&request)
                .map_err(|x| Error::from(*x))?
                .load();

            machines.push(socket);
            num_machines.push(response);
        }

        Ok((machines, num_machines))
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        self.terminate().unwrap()
    }
}
