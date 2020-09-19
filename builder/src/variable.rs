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
            _ => true,
        }
    }
}

impl Estimable for ast::Expr {
    fn is_estimable(&self) -> bool {
        self.lhs.is_estimable() && self.rhs.as_ref().map(|x| x.is_estimable()).unwrap_or(true)
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
        match self {
            Some(value) => Ok(Some(value.replace_to(names, variables, shortcuts)?)),
            None => Ok(None),
        }
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

impl<T> Hint for Option<T>
where
    T: Hint,
{
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        match self {
            Some(value) => Ok(Some(value.hint(shortcuts, out, dim, is_root)?)),
            None => Ok(None),
        }
    }
}

impl BuildValue for ast::RefVariable {
    fn build(&self) -> ast::Value {
        self.borrow().value.build()
    }
}

impl BuildValue for ast::Value {
    fn build(&self) -> Self {
        match self {
            Self::Bool(_) | Self::UInt(_) | Self::Int(_) | Self::Real(_) => self.clone(),
            Self::Node(_) => unreachable!("node variable should be pruned."),
            Self::Dim(_) => unreachable!("The value should have already been checked."),
            Self::Variable(value) => value.build(),
            Self::Expr(value) => value.build(),
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
                ast::Operator::MulInt => lhs.into_uint() * rhs.into_uint(),
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

impl<T> BuildValue for Option<T>
where
    T: BuildValue,
{
    fn build(&self) -> ast::Value {
        match self {
            Some(value) => value.build(),
            None => unreachable!("The value should have already been checked."),
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
        let mut to_borrowed = to.0.borrow_mut();

        for (name, last_output) in self.0.borrow().iter() {
            if let Some(new_input) = to_borrowed.get_mut(name) {
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

                        for (last_dim, new_dim) in last_output.0.iter().zip(new_input.0.iter_mut())
                        {
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
                                    if last_dim != new_dim {
                                        return LinkError::MismatchedDim {
                                            expected: last_dim,
                                            given: new_dim,
                                        }
                                        .into();
                                    }
                                }
                            }
                            // link
                            else if let Some(new_dim) = new_dim.try_as_dim() {
                                new_dim.borrow_mut().value = Some(last_dim.clone());
                            }
                        }
                    }
                } else {
                    // dynamic size
                    *new_input = last_output.clone();
                    continue;
                }
            }
        }
        Ok(())
    }
}
