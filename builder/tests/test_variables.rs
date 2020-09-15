use n3_builder::*;

pub fn get_simple_graph() -> Graph {
    let mut graph = Graph::new(1);

    let a: ast::RefVariable = ast::Variable::with_name("a".to_string()).into();
    let b: ast::RefVariable =
        ast::Variable::with_name_value("b".to_string(), Some(ast::Value::Int(3))).into();

    // c = a + b - 1
    let c: ast::RefVariable = ast::Variable::with_name_value(
        "c".to_string(),
        Some(
            ast::Expr {
                op: ast::Operator::Sub,
                lhs: ast::Expr {
                    op: ast::Operator::Add,
                    lhs: a.clone().into(),
                    rhs: Some(b.clone().into()),
                }
                .into(),
                rhs: Some(ast::Value::Int(1)),
            }
            .into(),
        ),
    )
    .into();

    a.borrow_mut().ty = Some(ast::LetType::Int);
    b.borrow_mut().ty = Some(ast::LetType::Int);
    c.borrow_mut().ty = Some(ast::LetType::Int);

    graph.add(a).unwrap();
    graph.add(b).unwrap();
    graph.add(c).unwrap();
    graph
}

#[test]
fn test_simple() {
    let graph = get_simple_graph();

    let a = graph.get("a").unwrap();

    // unestimable variable: a
    assert_eq!(graph.is_estimable(), false);

    // hinting
    a.borrow_mut().value = Some(
        ast::OutDim {
            out: ast::Out::with_name("x".to_string()),
            dim: 0,
        }
        .into(),
    );
    assert_eq!(graph.is_estimable(), true);
}

#[test]
fn test_node_root() {
    const SOURCE: &'static str = "
node MyNode:
    let c = int a + b - 1
    let a = int 4
    let b = int 3
    let d = int c
";

    let parser = n3_builder::Parser::new();
    let file = parser.parse_file(SOURCE).unwrap();

    let graph = Graph::try_with_variables(1, file.node.graph).unwrap();
    assert_eq!(graph.is_estimable(), true);
}

#[test]
fn test_cycle() {
    const SOURCE: &'static str = "
node MyNode:
    let a = int b + 1
    let b = int c + 2
    let c = int a + 3
";

    let parser = n3_builder::Parser::new();
    let file = parser.parse_file(SOURCE).unwrap();

    // cycled variable: [a, b, c]
    assert_eq!(
        Graph::try_with_variables(1, file.node.graph).err(),
        Some(Error::BuildError(BuildError::CycledVariables {
            names: ["a", "b", "c"].iter().map(|x| x.to_string()).collect(),
        }))
    );
}

#[test]
fn test_build() {
    let graph = get_simple_graph();

    let a = graph.get("a").unwrap();
    a.borrow_mut().value = Some(4u64.into());

    let c = graph.get("c").unwrap();
    assert_eq!(c.build(), 6u64.into());
}
