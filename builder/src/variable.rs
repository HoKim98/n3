use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

use num_traits::Pow;

use super::error::{GraphError, LinkError, Result};
use super::graph::Table;
use crate::ast;

pub trait CloneValue {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self;
}

pub trait Detach {
    fn detach(&self, id: u64) -> Self;
}

pub trait Estimable {
    fn is_estimable(&self) -> bool;
}

pub trait BuildValue {
    fn build(&self) -> ast::Value;
}

pub trait Link {
    fn link_to(&self, to: &Self) -> Result<()>;
}

pub(crate) trait Replace {
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self>
    where
        Self: Sized;
}

pub(crate) trait Hint {
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self>
    where
        Self: Sized;
}

impl CloneValue for ast::RefVariable {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        let this = self.borrow();
        for var in variables {
            let var_ref = var.borrow();
            if var_ref.name == this.name && var_ref.id_old == this.id {
                return var.clone();
            }
        }
        self.clone()
    }
}

impl CloneValue for ast::Value {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        match self {
            Self::Variable(value) => Self::Variable(value.clone_value(variables)),
            Self::Expr(value) => Self::Expr(value.clone_value(variables).into()),
            Self::List(value) => Self::List(value.clone_value(variables).into()),
            Self::Map(value) => Self::Map(value.clone_value(variables).into()),
            _ => self.clone(),
        }
    }
}

impl CloneValue for ast::Expr {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        Self {
            op: self.op,
            lhs: self.lhs.clone_value(variables),
            rhs: self.rhs.clone_value(variables),
        }
    }
}

impl CloneValue for ast::Shapes {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        Self(self.0.clone_value(variables))
    }
}

impl CloneValue for ast::Shape {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        Self(self.0.clone_value(variables))
    }
}

impl<K, V> CloneValue for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        self.iter()
            .map(|(k, v)| (k.clone(), v.clone_value(variables)))
            .collect()
    }
}

impl<T> CloneValue for Vec<T>
where
    T: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        self.iter().map(|x| x.clone_value(variables)).collect()
    }
}

impl<T> CloneValue for RefCell<T>
where
    T: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        Self::new(self.borrow().clone_value(variables))
    }
}

impl<T> CloneValue for Option<T>
where
    T: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        self.as_ref().map(|x| x.clone_value(variables))
    }
}

impl Detach for ast::RefVariable {
    fn detach(&self, id: u64) -> Self {
        let this = self.borrow();
        let cloned = ast::Variable {
            id: Some(id),
            id_old: this.id,
            name: this.name.clone(),
            shortcut: this.shortcut.clone(),
            ty: this.ty,
            value: this.value.clone(),
        };
        cloned.into()
    }
}

impl Estimable for ast::RefVariable {
    fn is_estimable(&self) -> bool {
        self.borrow().value.is_estimable()
    }
}

impl Estimable for ast::Value {
    fn is_estimable(&self) -> bool {
        match self {
            Self::Variable(value) => value.is_estimable(),
            Self::Expr(value) => value.is_estimable(),
            Self::List(value) => value.is_estimable(),
            Self::Map(value) => value.is_estimable(),
            _ => true,
        }
    }
}

impl Estimable for ast::Expr {
    fn is_estimable(&self) -> bool {
        self.lhs.is_estimable() && self.rhs.as_ref().map(|x| x.is_estimable()).unwrap_or(true)
    }
}

impl<K, V> Estimable for BTreeMap<K, V>
where
    V: Estimable,
{
    fn is_estimable(&self) -> bool {
        self.values().all(|x| x.is_estimable())
    }
}

impl<T> Estimable for Vec<T>
where
    T: Estimable,
{
    fn is_estimable(&self) -> bool {
        self.iter().all(|x| x.is_estimable())
    }
}

impl<T> Estimable for Option<T>
where
    T: Estimable,
{
    fn is_estimable(&self) -> bool {
        self.as_ref().map(|x| x.is_estimable()).unwrap_or_default()
    }
}

impl Replace for ast::RefVariable {
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        let raise_cycled_variables = || {
            let names = names.iter().cloned().collect();
            GraphError::CycledVariables { names }.into()
        };

        let mut value = self;
        {
            // If a cycle is detected, the same mutable variable cannot be referenced again.
            let mut value_ref = match value.try_borrow_mut() {
                Ok(v) => v,
                Err(_) => return raise_cycled_variables(),
            };
            let name = &mut value_ref.name;

            if let Some(n) = shortcuts.get(name) {
                *name = n.to_string();
            }
            if let Some(var) = variables.get(name) {
                value = var;
            }
        }

        {
            // If a cycle is detected, the same mutable variable cannot be referenced again.
            let mut value_ref = match value.try_borrow_mut() {
                Ok(v) => v,
                Err(_) => return raise_cycled_variables(),
            };
            let name = value_ref.name.clone();

            names.push(name);
            value_ref.value = value_ref.value.replace_to(names, variables, shortcuts)?;
            names.pop();
        }
        Ok(value.clone())
    }
}

impl Replace for ast::Value {
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        match self {
            Self::Variable(value) => Ok(value.replace_to(names, variables, shortcuts)?.into()),
            Self::Expr(value) => Ok(value.replace_to(names, variables, shortcuts)?.into()),
            Self::List(value) => Ok(value.replace_to(names, variables, shortcuts)?.into()),
            Self::Map(value) => Ok(value.replace_to(names, variables, shortcuts)?.into()),
            _ => Ok(self.clone()),
        }
    }
}

impl Replace for ast::Expr {
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        Ok(Self {
            op: self.op,
            lhs: self.lhs.replace_to(names, variables, shortcuts)?,
            rhs: self.rhs.replace_to(names, variables, shortcuts)?,
        })
    }
}

impl<K, V> Replace for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: Replace,
{
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        self.iter()
            .map(|(k, v)| Ok((k.clone(), v.replace_to(names, variables, shortcuts)?)))
            .collect()
    }
}

impl<T> Replace for Vec<T>
where
    T: Replace,
{
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        self.iter()
            .map(|x| x.replace_to(names, variables, shortcuts))
            .collect()
    }
}

impl<T> Replace for Option<T>
where
    T: Replace,
{
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        self.as_ref().map_or(Ok(None), |x| {
            Ok(Some(x.replace_to(names, variables, shortcuts)?))
        })
    }
}

impl Hint for ast::RefVariable {
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        let this = self.borrow();
        let name = &this.name;

        match shortcuts.get(name) {
            // hint in-place
            Some(output) => {
                let mut output_ref = output.borrow_mut();
                if output_ref.ty == Some(ast::LetType::Dim) && is_root {
                    output_ref.value = Some(
                        ast::OutDim {
                            out: out.clone(),
                            dim,
                        }
                        .into(),
                    );
                }
                Ok(output.clone())
            }
            None => GraphError::NoSuchVariable {
                name: name.clone(),
                candidates: shortcuts.keys().cloned().collect(),
            }
            .into(),
        }
    }
}

impl Hint for ast::Value {
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        match self {
            Self::Variable(value) => Ok(value.hint(shortcuts, out, dim, is_root)?.into()),
            Self::Expr(value) => Ok(value.hint(shortcuts, out, dim, is_root)?.into()),
            Self::List(value) => Ok(value.hint(shortcuts, out, dim, is_root)?.into()),
            Self::Map(value) => Ok(value.hint(shortcuts, out, dim, is_root)?.into()),
            _ => Ok(self.clone()),
        }
    }
}

impl Hint for ast::Expr {
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        Ok(ast::Expr {
            op: self.op,
            lhs: self.lhs.hint(shortcuts, out, dim, is_root)?,
            rhs: self.rhs.hint(shortcuts, out, dim, is_root)?,
        }
        .into())
    }
}

impl<K, V> Hint for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: Hint,
{
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        self.iter()
            .map(|(k, v)| Ok((k.clone(), v.hint(shortcuts, out, dim, is_root)?)))
            .collect()
    }
}

impl<T> Hint for Vec<T>
where
    T: Hint,
{
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        self.iter()
            .map(|x| x.hint(shortcuts, out, dim, is_root))
            .collect()
    }
}

impl<T> Hint for Option<T>
where
    T: Hint,
{
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        self.as_ref().map_or(Ok(None), |x| {
            Ok(Some(x.hint(shortcuts, out, dim, is_root)?))
        })
    }
}

impl BuildValue for ast::RefVariable {
    fn build(&self) -> ast::Value {
        match &self.borrow().value {
            Some(value) => value.build(),
            None => ast::Value::Variable(self.clone()),
        }
    }
}

impl BuildValue for ast::Value {
    fn build(&self) -> Self {
        match self {
            Self::Bool(_) | Self::UInt(_) | Self::Int(_) | Self::Real(_) | Self::Dim(_) => {
                self.clone()
            }
            Self::Node(_) => ast::err_value_not_pruned(),
            Self::Variable(value) => value.build(),
            Self::Expr(value) => value.build(),
            Self::List(value) => Self::List(value.iter().map(|x| x.build()).collect()),
            Self::Map(value) => Self::Map(
                value
                    .iter()
                    .map(|(k, v)| (k.clone(), v.as_ref().map(|x| x.build())))
                    .collect(),
            ),
        }
    }
}

impl BuildValue for ast::Expr {
    fn build(&self) -> ast::Value {
        let lhs = self.lhs.build();
        if let Some(rhs) = &self.rhs {
            let rhs = rhs.build();
            match self.op {
                ast::Operator::Add => lhs + rhs,
                ast::Operator::Sub => lhs - rhs,
                ast::Operator::Mul => lhs * rhs,
                ast::Operator::MulInt => {
                    if lhs.is_atomic() && rhs.is_atomic() {
                        (lhs.unwrap_uint().unwrap() * rhs.unwrap_uint().unwrap()).into()
                    } else {
                        ast::Expr {
                            op: self.op,
                            lhs,
                            rhs: Some(rhs),
                        }
                        .into()
                    }
                }
                ast::Operator::Div => lhs / rhs,
                ast::Operator::Mod => lhs % rhs,
                ast::Operator::Pow => lhs.pow(rhs),
                ast::Operator::And => lhs & rhs,
                ast::Operator::Or => lhs | rhs,
                ast::Operator::Xor => lhs ^ rhs,
                _ => unreachable!("expected binary operators"),
            }
        } else {
            match self.op {
                ast::Operator::Pos => lhs,
                ast::Operator::Neg => -lhs,
                _ => unreachable!("expected unary operators"),
            }
        }
    }
}

impl<T> BuildValue for Box<T>
where
    T: BuildValue,
{
    fn build(&self) -> ast::Value {
        (**self).build()
    }
}

impl Link for ast::Shapes {
    fn link_to(&self, to: &Self) -> Result<()> {
        for (name, last_output) in self.0.borrow().iter() {
            if let Some(new_input) = to.0.borrow().get(name) {
                if let Some(new_input) = new_input {
                    if let Some(last_output) = last_output {
                        // test the tensor size
                        let last_output_len = last_output.0.len();
                        let new_input_len = new_input.0.len();

                        if last_output_len != new_input_len {
                            return LinkError::MismatchedShape {
                                expected: new_input.clone(),
                                given: last_output.clone(),
                            }
                            .into();
                        }

                        for (last_dim, new_dim) in last_output.0.iter().zip(new_input.0.iter()) {
                            if !last_dim.is_hint() {
                                // replace
                                if new_dim.is_hint() {
                                    let new_dim = new_dim.get_hint().unwrap();
                                    new_dim.borrow_mut().value = Some(last_dim.clone());
                                }
                                // test value
                                else {
                                    let last_dim = last_dim.build();
                                    let new_dim = new_dim.build();
                                    assert_equal(last_dim, new_dim)?;
                                }
                            }
                            // link
                            else if let Some(new_dim) = new_dim.try_as_dim() {
                                new_dim.borrow_mut().value = Some(last_dim.clone());
                            }
                        }
                    }
                } else {
                    // release borrowing for 'borrow_mut'
                    drop(new_input);

                    // dynamic size
                    *to.0.borrow_mut().get_mut(name).unwrap() = last_output.clone();
                    continue;
                }
            }
        }
        Ok(())
    }
}

impl<'a, T> Link for Option<&'a T>
where
    T: Link,
{
    fn link_to(&self, to: &Self) -> Result<()> {
        match self.zip(to.as_ref()) {
            Some((a, &b)) => a.link_to(b),
            None => Ok(()),
        }
    }
}

pub fn assert_equal(last_dim: ast::Value, new_dim: ast::Value) -> Result<()> {
    if last_dim != new_dim {
        return LinkError::MismatchedDim {
            expected: last_dim,
            given: new_dim,
        }
        .into();
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::graph::Graph;

    fn get_simple_graph() -> Graph {
        let mut graph = Graph::with_id(1);

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

        let parser = crate::Parser::new();
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

        let parser = crate::Parser::new();
        let file = parser.parse_file(SOURCE).unwrap();

        // cycled variable: [a, b, c]
        assert_eq!(
            Graph::try_with_variables(1, file.node.graph).err(),
            Some(
                GraphError::CycledVariables {
                    names: ["a", "b", "c"].iter().map(|x| x.to_string()).collect(),
                }
                .into()
            )
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
}
