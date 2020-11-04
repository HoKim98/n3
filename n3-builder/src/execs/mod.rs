pub mod dirs;
mod ir;
mod program;
mod root;
mod var;

pub use self::ir::ExecIR;
pub use self::program::Program;
pub use self::root::ExecRoot;
pub use self::var::{GlobalVars, Vars};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ic() {
        let envs = GlobalVars::default();
        envs.set(dirs::N3_ROOT, "tests/data/").unwrap();
        envs.set(dirs::N3_SOURCE_ROOT, "../n3-torch-ffi-python/n3")
            .unwrap();
        let mut root = ExecRoot::try_new(envs).unwrap();

        let args = root.get("DummyImageClassification").unwrap();
        args.set("data", "Mnist").unwrap();
        args.set("model", "LeNet5").unwrap();
        args.set("epoch", "1").unwrap();
        args.set("batch size", "10").unwrap();

        let program = args.build_uncompacted().unwrap();

        // compacting & decompacting
        {
            let binary = program.save_to_binary().unwrap();
            let program_decompacted = Program::load(&*binary).unwrap();

            // manipulate values to varify RefVariable
            fn manipulate_values(program: &Program) {
                let model = &program.nodes["model"];
                let kernel_size = model.data().graph.variables.get("kernel size").unwrap();
                kernel_size.borrow_mut().value = Some(7u64.into());
            }
            manipulate_values(&program);
            manipulate_values(&program_decompacted);

            assert_eq!(program, program_decompacted);
        }
    }
}
