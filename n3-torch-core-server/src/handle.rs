use n3_torch_core::HostMachine;
use n3_torch_core_protocol::request::Request;
use n3_torch_core_protocol::response::Response;

pub trait Handle {
    type Response;

    fn handle(self, host: &mut HostMachine) -> Self::Response;
}

impl Handle for Request {
    type Response = Response;

    fn handle(self, host: &mut HostMachine) -> Self::Response {
        match self {
            Self::Load { job, query } => Self::Response::Load {
                num_machines: host.load(job, &query).unwrap(),
            },
            Self::Spawn {
                job,
                machines,
                program,
                command,
            } => {
                host.spawn(job, machines, &program, &command).unwrap();
                Self::Response::Awk
            }
            Self::Join { job } => {
                host.join(job).unwrap();
                Self::Response::Awk
            }
            Self::Terminate { job } => {
                host.terminate(job).unwrap();
                Self::Response::Awk
            }
        }
    }
}
