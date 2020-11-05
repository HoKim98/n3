use std::net::{IpAddr, SocketAddr};
use std::ops::{Deref, DerefMut};

pub use simple_socket::PostServing;

use n3_machine::HostMachine;
use n3_net_protocol::{Request, Response, PORT};

pub type SocketServer = simple_socket::SocketServer<Request, Response>;

pub(crate) trait Handle<H>
where
    H: Deref<Target = HostMachine> + DerefMut,
{
    fn handle(self, host: &mut H) -> Response;
}

impl<H> Handle<H> for Request
where
    H: Deref<Target = HostMachine> + DerefMut,
{
    fn handle(self, host: &mut H) -> Response {
        match self {
            Self::Load { job, query } => Response::Load {
                num_machines: host.load(job, query).unwrap(),
            },
            Self::Spawn {
                job,
                machines,
                program,
                command,
            } => {
                host.spawn(job, machines, &program, &command).unwrap();
                Response::Awk
            }
            Self::Join { job } => {
                host.join(job).unwrap();
                Response::Awk
            }
            Self::Terminate { job } => {
                host.terminate(job).unwrap();
                Response::Awk
            }
        }
    }
}

pub fn run_server<H, P>(mut host: H, ip: IpAddr, post: P)
where
    H: Deref<Target = HostMachine> + DerefMut,
    P: Fn(&mut SocketServer) -> PostServing,
{
    let socket = SocketAddr::new(ip, PORT);

    let backlog = Default::default();
    let server = SocketServer::try_new(socket, backlog).unwrap();

    server
        .run(|x| Handle::<H>::handle(x, &mut host), post)
        .unwrap()
}
