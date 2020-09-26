mod dirs;
mod ir;
mod program;
mod root;
mod var;

pub use self::ir::ExecIR;
pub use self::program::{Nodes, Program};
pub use self::root::ExecRoot;
pub use self::var::{GlobalVars, Vars};

#[cfg(test)]
mod tests {
    use maplit::btreemap;

    use super::*;
    use crate::ast;

    #[test]
    fn test_build_ic() {
        let envs = GlobalVars::default();
        envs.set("root".to_string(), "tests/data/".to_string().into())
            .unwrap();

        let root = ExecRoot::try_new(envs).unwrap();

        let args = btreemap! {
            "data".to_string() => ast::Value::from("Mnist".to_string()),
            "model".to_string() => "LeNet6".to_string().into(),
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

        // serialization & deserialization
        {
            let mut binary = vec![];
            root.compact_into(&mut binary, &program).unwrap();

            let program_decompacted = root.decompact_from(binary.as_slice()).unwrap();
        }
    }
}
