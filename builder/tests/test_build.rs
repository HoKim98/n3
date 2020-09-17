use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use maplit::btreemap;

use n3_builder::*;

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
        let graph = Graph::try_with_variables(1, graph).unwrap();

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
    let graph = Rc::new(RefCell::new(Graph::new(1)));

    let input = ast::Shapes(btreemap! {
        "x".to_string() => Some(ast::Shape(vec![32u64.into()])),
    });
    let output = ast::Shapes(btreemap! {
        "x".to_string() => Some(ast::Shape(vec![10u64.into()])),
    });

    let ir = NodeIR {
        data: IRData::new(name, graph, Some(&input), Some(&output)),
        tensor_graph: vec![graph_1.into(), graph_2.into()].into(),
        repeat: None,
    };

    let root = NodeRoot::new();
    ir.build(&root).unwrap();
}

#[test]
fn test_unexpected_extern_node() {
    let root = NodeRoot::new();

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

    let root = NodeRoot::new();
    root.add_source("MyNode".to_string(), model.to_string());

    let ir = root.get("MyNode").unwrap();
    ir.build(&root).unwrap();
}
