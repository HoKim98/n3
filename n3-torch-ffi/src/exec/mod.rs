mod trainer;

pub use self::trainer::Trainer;

#[cfg(test)]
mod test {
    use n3_builder::*;
    use n3_torch_machine::HostMachine;

    #[test]
    fn test_exec_ic() -> n3_builder::Result<()> {
        let envs = GlobalVars::default();
        envs.set("root", "../n3-builder/tests/data/")?;
        let mut root = ExecRoot::try_new(envs)?;

        let args = root.get("DummyImageClassification")?;
        args.set("data", "Mnist")?;
        args.set("model", "LeNet5")?;
        args.set_as_value("epoch", 1)?;
        args.set_as_value("batch size", 10)?;

        let program = args.build()?;

        // load a machine
        let mut host = HostMachine::try_new().unwrap();
        host.load(&["cuda:0"]).unwrap();

        // spawn a process
        host.spawn(&program, "train").unwrap();

        // wait the process
        host.join().unwrap();
        Ok(())
    }
}
