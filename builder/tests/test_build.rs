use maplit::btreemap;

use n3_builder::*;

#[test]
fn test_tensor_graph() {
    fn make_graph((ix, ic): (u64, u64), (ox, oc): (u64, u64)) -> ExternIR {
        ExternIRData {
            id: ix,
            name: "Linear".to_string(),
            kwargs: btreemap! {
                "input channels".to_string() => ic.into(),
                "output channels".to_string() => oc.into(),
                "bias".to_string() => true.into(),
            },
            input: btreemap! {
                "x".to_string() => ast::Out {
                    id: Some(ix),
                    name: Some("x".to_string()),
                },
            },
            output: btreemap! {
                "x".to_string() => ast::Out {
                    id: Some(ox),
                    name: Some("x".to_string()),
                },
            },
        }
        .into()
    }

    let graph_1 = make_graph((1, 32), (2, 64));
    let graph_2 = make_graph((2, 64), (3, 10));

    let ir = NodeIR {
        name: "MyNode".to_string(),
        graph: Graph::new(1),
        tensor_graph: vec![graph_1.into(), graph_2.into()].into(),
        data: Default::default(),
    };

    let root = NodeRoot::new();
    ir.build(&root).unwrap();
}
