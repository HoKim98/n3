mod handle;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use simple_socket::{PostServing, SocketServer};

use n3_torch_core::HostMachine;
use n3_torch_core_protocol::request::Request;
use n3_torch_core_protocol::response::Response;

use crate::handle::Handle;

const PORT: u16 = 40960;

fn run_server(ip: IpAddr) {
    let socket = SocketAddr::new(ip, PORT);

    let mut host = HostMachine::try_new().unwrap();

    let backlog = Default::default();
    let server = SocketServer::<Request, Response>::try_new(socket, backlog).unwrap();
    server
        .run(|x| Request::handle(x, &mut host), |_| PostServing::Yield)
        .unwrap()
}

fn main() {
    const IP_V4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const IP: IpAddr = IpAddr::V4(IP_V4);

    run_server(IP)
}
