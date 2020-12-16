mod device;
mod host;
mod process;
mod python;

use std::time::Duration;

use self::device::CandidatesMachine;
use self::host::HostMachine;

/// Define built-in machine generators here.
pub(crate) const BUILTIN_MACHINES: &[(&str, n3_machine::Generator)] = &[
    ("cpu", self::device::CpuMachine::try_new),
    ("cuda", self::device::CudaMachine::try_new),
];

use std::net::{IpAddr, Ipv4Addr};

use n3_net_server::{PostServing, SocketServer};

pub fn run_server(post: impl Fn(&mut SocketServer) -> PostServing) {
    const IP_V4: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
    const IP: IpAddr = IpAddr::V4(IP_V4);

    let host = HostMachine::try_new().unwrap();
    n3_net_server::run_server(host, IP, post);
}

fn main() {
    run_server(|_| PostServing::Wait(Duration::from_millis(10)));
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use std::thread;

    use n3_builder::{dirs, ExecRoot, GlobalVars};
    use n3_machine_ffi::ProgramTextVec;
    use n3_net_client::Work;

    use super::*;

    #[derive(Clone)]
    struct BooleanFlag(Arc<AtomicBool>);

    impl BooleanFlag {
        fn new(value: bool) -> Self {
            Self(Arc::new(AtomicBool::new(value)))
        }

        fn get(&self) -> bool {
            self.0.load(Ordering::SeqCst)
        }

        fn set(&self, value: bool) {
            self.0.store(value, Ordering::SeqCst)
        }
    }

    fn get_dummy_program() -> (ExecRoot, ProgramTextVec) {
        let envs = GlobalVars::default();
        envs.set(dirs::N3_ROOT, "../../n3-builder/tests/data/")
            .unwrap();
        envs.set(dirs::N3_SOURCE_ROOT, "../ffi/python/n3").unwrap();
        let mut root = ExecRoot::try_new(envs).unwrap();

        let args = root.get("DummyImageClassification").unwrap();
        args.set("data", "Mnist").unwrap();
        args.set("model", "LeNet5").unwrap();
        args.set("epoch", "1").unwrap();
        args.set("batch size", "10").unwrap();

        let program = args.build_with_env().unwrap();
        (root, program)
    }

    #[test]
    fn test_simple() {
        let alive_client = BooleanFlag::new(true);
        let alive_server = BooleanFlag::new(false);

        // spawn a server
        let alive_client_t = alive_client.clone();
        let alive_server_t = alive_server.clone();
        let server = thread::spawn(move || {
            run_server(|_| {
                alive_server_t.set(true);
                if alive_client_t.get() {
                    PostServing::Yield
                } else {
                    PostServing::Stop
                }
            });

            alive_server_t.set(false);
        });
        while !alive_server.get() {
            thread::yield_now();
        }

        // spawn a work
        let (root, program) = get_dummy_program();
        let command = "train";
        let machines = &["cpu"];

        let mut work = Work::spawn(&program, command, machines).unwrap();

        // wait the work
        {
            let status = work.status().unwrap();
            assert_eq!(status.is_running, true);

            work.join().unwrap();

            let status = work.status().unwrap();
            assert_eq!(status.is_running, false);
            assert_eq!(status.error_msg, None);
            assert_eq!(status.date_end.is_some(), true);
        }
        drop(work);
        alive_client.set(false);

        // stop the server
        // note: the order is important to finalize Python interpreter safely.
        // order: server (host) -> root (Python)
        server.join().unwrap();
        drop(root);
    }
}
