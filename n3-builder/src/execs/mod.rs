mod dirs;
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
    use maplit::btreemap;

    use super::*;
    use crate::ast;

    #[test]
    fn test_build_ic() {
        fn make_root() -> ExecRoot {
            let envs = GlobalVars::default();
            envs.set("root".to_string(), "tests/data/".to_string())
                .unwrap();
            ExecRoot::try_new(envs).unwrap()
        };

        let root = make_root();

        let args = btreemap! {
            "data".to_string() => ast::Value::from("Mnist".to_string()),
            "model".to_string() => "LeNet5".to_string().into(),
            // "model".to_string() => "LeNet6".to_string().into(),
            "epoch".to_string() => 1i64.into(),
            "batch size".to_string() => 10i64.into(),
        };
        let args = Vars::new(
            args.into_iter()
                .map(|(k, v)| {
                    let name = k.clone();
                    let mut value = ast::Variable::with_name_value(k, Some(v));
                    value.id = Some(0);
                    value.id_old = Some(0);
                    (name, value.into())
                })
                .collect(),
        );

        let program = root.get("DummyImageClassification", args).unwrap();

        // compacting & decompacting
        {
            let mut binary = vec![];
            program.save(&mut binary).unwrap();

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
