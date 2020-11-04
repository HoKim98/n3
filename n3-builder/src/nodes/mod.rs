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

            let data = IRData {
                id: ix,
                name: "Linear".to_string(),
                graph: graph.into(),
                input: btreemap! {
                    "x".to_string() => ast::Out::new(ix, "x".to_string()),
                },
                output: btreemap! {
                    "x".to_string() => ast::Out::new(ox, "x".to_string()),
                },
            };
            ExternIR {
                ty: ast::ExternNodeType::Default,
                shapes: (&data).into(),
                data,
            }
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

        let node = ir.build(&root).unwrap();
        let node = node.as_node();

        let node_n0 = node.tensor_graph[0].as_extern();
        assert_eq!(node_n0.data.input["x"].id, Some(1));
        assert_eq!(node_n0.data.output["x"].id, Some(1));

        let node_n1 = node.tensor_graph[1].as_node();
        assert_eq!(node_n1.data.input["x"].id, Some(1));
        assert_eq!(node_n1.data.output["x"].id, Some(2));
        let node_n1_conv = node_n1.tensor_graph[0].as_extern();
        assert_eq!(node_n1_conv.data.input["x"].id, Some(1));
        assert_eq!(node_n1_conv.data.output["x"].id, Some(2));
        let node_n1_relu = node_n1.tensor_graph[1].as_extern();
        assert_eq!(node_n1_relu.data.input["x"].id, Some(2));
        assert_eq!(node_n1_relu.data.output["x"].id, Some(3));

        let node_n2 = node.tensor_graph[2].as_node();
        assert_eq!(node_n2.data.input["x"].id, Some(2));
        assert_eq!(node_n2.data.output["x"].id, Some(3));
        let node_n2_conv = node_n2.tensor_graph[0].as_extern();
        assert_eq!(node_n2_conv.data.input["x"].id, Some(1));
        assert_eq!(node_n2_conv.data.output["x"].id, Some(2));
        let node_n2_relu = node_n2.tensor_graph[1].as_extern();
        assert_eq!(node_n2_relu.data.input["x"].id, Some(2));
        assert_eq!(node_n2_relu.data.output["x"].id, Some(3));

        let node_n3 = node.tensor_graph[3].as_extern();
        assert_eq!(node_n3.data.input["x"].id, Some(3));
        assert_eq!(node_n3.data.output["x"].id, Some(4));

        let node_n4 = node.tensor_graph[4].as_extern();
        assert_eq!(node_n4.data.input["x"].id, Some(4));
        assert_eq!(node_n4.data.output["x"].id, Some(5));
        let node_n5 = node.tensor_graph[5].as_extern();
        assert_eq!(node_n5.data.input["x"].id, Some(5));
        assert_eq!(node_n5.data.output["x"].id, Some(5));
        let node_n6 = node.tensor_graph[6].as_extern();
        assert_eq!(node_n6.data.input["x"].id, Some(5));
        assert_eq!(node_n6.data.output["x"].id, Some(5));

        let node_n7 = node.tensor_graph[7].as_extern();
        assert_eq!(node_n7.data.input["x"].id, Some(5));
        assert_eq!(node_n7.data.output["x"].id, Some(6));
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
