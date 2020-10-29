#[cfg(test)]
mod test {
    use n3_builder::*;

    use crate::machine::HostMachine;

    #[test]
    fn test_exec_ic() -> Result<()> {
        let envs = GlobalVars::default();
        envs.set("root", "../n3-builder/tests/data/")?;
        let mut root = ExecRoot::try_new(envs)?;

        let args = root.get("DummyImageClassification")?;
        args.set("data", "Mnist")?;
        args.set("model", "LeNet5")?;
        args.set_as_value("epoch", 1)?;
        args.set_as_value("batch size", 10)?;

        let program = args.build()?;

        // load machines
        let mut host = HostMachine::default();
        host.load(&["cuda:0"]).unwrap();

        // spawn processes
        host.spawn(&program).unwrap();

        // wait processes
        host.join().unwrap();
        Ok(())
    }
}
