use n3_core::{ast, Graph};

fn get_simple_graph() -> Graph {
    let mut graph = Graph::new(1);

    let a: ast::RefVariable = ast::Variable::with_name("a".to_string()).into();
    let b: ast::RefVariable =
        ast::Variable::with_name_value("b".to_string(), ast::Value::Int(1)).into();

    // c = a + b - 1
    let c: ast::RefVariable = ast::Variable::with_name_value(
        "c".to_string(),
        ast::Value::Expr {
            op: ast::Operator::Sub,
            lhs: Box::new(ast::Value::Expr {
                op: ast::Operator::Add,
                lhs: Box::new(a.clone().into()),
                rhs: Some(Box::new(b.clone().into())),
            }),
            rhs: Some(Box::new(ast::Value::Int(1))),
        },
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
    let mut graph = get_simple_graph();

    graph.build().unwrap();
}
