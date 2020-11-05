use std::net::{IpAddr, Ipv4Addr};

use n3_net_server::{PostServing, SocketServer};
use n3_torch_core::HostMachine;

pub fn run_server(post: impl Fn(&mut SocketServer) -> PostServing) {
    const IP_V4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const IP: IpAddr = IpAddr::V4(IP_V4);

    let host = HostMachine::try_new().unwrap();
    n3_net_server::run_server(host, IP, post);
}

fn main() {
    run_server(|_| PostServing::Continue);
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use std::thread;

    use pyo3::{PyResult, Python};

    use n3_builder::{dirs, ExecRoot, GlobalVars};
    use n3_machine_ffi::ProgramVec;
    use n3_net_client::Job;
    use n3_torch_core::pyo3;

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

    fn load_n3(py: Python) -> PyResult<()> {
        let sys = py.import("sys")?;
        sys.get("path")?
            .call_method1("insert", (0, "../ffi/python"))?;
        Ok(())
    }

    fn get_dummy_program() -> ProgramVec {
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

        args.build_with_env().unwrap()
    }

    #[test]
    fn test_simple() {
        let alive_client = BooleanFlag::new(true);
        let alive_server = BooleanFlag::new(false);

        // load n3
        Python::with_gil(load_n3).unwrap();

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

        // spawn a job
        let program = get_dummy_program();
        let command = "train";
        let machines = &["cuda:0"];

        let job = Job::spawn(&program, command, machines).unwrap();

        // wait the job
        drop(job);
        alive_client.set(false);

        // stop the server
        server.join().unwrap();
    }
}