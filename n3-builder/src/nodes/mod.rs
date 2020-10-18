mod builder;
mod code;
mod ir;
mod root;

pub use self::builder::{builtins, ASTBuild};
pub use self::code::NodeCode;
pub use self::ir::NodeIR;
pub use self::root::NodeRoot;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::rc::Rc;

    use maplit::btreemap;

    use super::super::*;
    use super::*;
    use crate::error::TensorNodeError;
    use crate::externs::ExternIR;
    use crate::graph::Graph;
    use crate::tensor::IRData;

    #[test]
    fn test_tensor_graph() {
        fn make_graph((ix, ic): (u64, u64), (ox, oc): (u64, u64)) -> ExternIR {
            let graph = btreemap! {
                "input channels" => (ic.into(), ast::LetType::UInt),
                "output channels" => (oc.into(), ast::LetType::UInt),
                "bias" => (true.into(), ast::LetType::Bool),
            };
            let graph = graph
                .into_iter()
                .map(|(k, (v, ty))| {
                    (
                        k.to_string(),
                        ast::NodeLet {
                            name: k.to_string(),
                            shortcut: None,
                            ty,
                            value: Some(v),
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>();
            let graph = Graph::try_with_variables(1, graph, false).unwrap();
            IRData {
                id: ix,
                name: "Linear".to_string(),
                graph: graph.into(),
                input: btreemap! {
                    "x".to_string() => ast::Out::new(ix, "x".to_string()),
                },
                output: btreemap! {
                    "x".to_string() => ast::Out::new(ox, "x".to_string()),
                },
            }
            .into()
        }
        let graph_1 = make_graph((1, 32), (2, 64));
        let graph_2 = make_graph((2, 64), (3, 10));
        let name = "MyNode".to_string();
        let graph = Rc::new(RefCell::new(Graph::with_id(1)));
        let tensor_graph = vec![graph_1.into(), graph_2.into()].into();
        let ir = NodeIR {
            data: IRData::with_tensor_graph(name, graph, &tensor_graph),
            ty: ast::LetNodeType::Default,
            tensor_graph,
            repeat: None,
        };
        let root = NodeRoot::default();
        ir.build(&root).unwrap();
    }

    #[test]
    fn test_unexpected_extern_node() {
        let root = NodeRoot::default();
        assert_eq!(
            root.get_extern("FOO").err(),
            Some(
                TensorNodeError::NoSuchNode {
                    name: "FOO".to_string(),
                }
                .into()
            )
        );
    }

    #[test]
    fn test_build_process() {
        let model = "
node MyNode:
    let Ic: input dimension = int 32
    let Oc: output dimension = int 10

    node MyRelu:
        let inner = int Ic

        1. Relu

    0. Input = Ic
    1. Linear + MyRelu = 64
    2. Linear + MyRelu = Oc
";
        let root = NodeRoot::default();
        root.add_source("MyNode".to_string(), model.to_string());
        let ir = root.get("MyNode").unwrap();
        ir.build(&root).unwrap();
    }

    #[test]
    fn test_build_lenet5() {
        let root = NodeRoot::default();
        let ir = root.get("LeNet5").unwrap();
        // manually define shapes
        {
            let mut shapes = ir.get_input_shapes().unwrap().0.borrow_mut();
            let shape = shapes.get_mut("x").map(|x| x.as_mut()).flatten().unwrap();
            if let [channel, width, height] = &mut shape.0.as_mut_slice() {
                channel.as_variable().borrow_mut().value = Some(1u64.into());
                width.as_variable().borrow_mut().value = Some(28u64.into());
                height.as_variable().borrow_mut().value = Some(28u64.into());
            }
        }
        ir.build(&root).unwrap();
    }

    #[test]
    fn test_build_concat() {
        let model = fs::read_to_string("tests/data/nodes/__user__/sample/test-cat.n3").unwrap();
        let root = NodeRoot::default();
        root.add_source("TestCat".to_string(), model);
        let ir = root.get("TestCat").unwrap();
        ir.build(&root).unwrap();
    }

    #[test]
    fn test_build_repeat() {
        let model = "
node MyNode:
    let zero = int 0
    let one = int 1
    let two = int 2

    0. Input            = 10
    1. Linear           = 20
    2. Linear * zero    = 20
    3. Linear * one     = 30
    4. Linear * two     = 40
";
        let root = NodeRoot::default();
        root.add_source("MyNode".to_string(), model.to_string());
        let ir = root.get("MyNode").unwrap();
        ir.build(&root).unwrap();
    }
}
