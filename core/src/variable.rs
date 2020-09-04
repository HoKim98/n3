use std::collections::{BTreeMap, HashMap};

use n3_parser::ast;

use super::error::{BuildError, Result};
use super::graph::Table;

pub trait CloneValue {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self;
}

pub trait Detach {
    fn detach(&self, id: usize) -> Self;
}

pub trait Estimable {
    fn is_estimable(&self) -> bool;
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
            Self::Expr { op, lhs, rhs } => Self::Expr {
                op: *op,
                lhs: lhs.clone_value(variables),
                rhs: rhs.clone_value(variables),
            },
            _ => self.clone(),
        }
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

impl<T> CloneValue for Option<T>
where
    T: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        self.as_ref().map(|x| x.clone_value(variables))
    }
}

impl<T> CloneValue for Box<T>
where
    T: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        Box::new((&**self).clone_value(variables))
    }
}

impl Detach for ast::RefVariable {
    fn detach(&self, id: usize) -> Self {
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
            Self::Expr { op: _, lhs, rhs } => lhs.is_estimable() && rhs.is_estimable(),
            _ => true,
        }
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

impl<T> Estimable for Box<T>
where
    T: Estimable,
{
    fn is_estimable(&self) -> bool {
        (&**self).is_estimable()
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
            Err(BuildError::CycledVariables { names }.into())
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
            Self::Expr { op, lhs, rhs } => {
                let lhs = lhs.replace_to(names, variables, shortcuts)?;
                let rhs = rhs.replace_to(names, variables, shortcuts)?;
                Ok(Self::Expr { op: *op, lhs, rhs })
            }
            _ => Ok(self.clone()),
        }
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

impl<T> Replace for Box<T>
where
    T: Replace,
{
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        Ok(Box::new((**self).replace_to(names, variables, shortcuts)?))
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
            None => Err(BuildError::NoSuchVariable {
                name: name.clone(),
                candidates: shortcuts.keys().cloned().collect(),
            }
            .into()),
        }
    }
}

impl Hint for ast::Value {
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        match self {
            Self::Variable(value) => Ok(value.hint(shortcuts, out, dim, is_root)?.into()),
            Self::Expr { op, lhs, rhs } => {
                let lhs = lhs.hint(shortcuts, out, dim, is_root)?;
                let rhs = rhs.hint(shortcuts, out, dim, is_root)?;
                Ok(Self::Expr { op: *op, lhs, rhs })
            }
            _ => Ok(self.clone()),
        }
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

impl<T> Hint for Box<T>
where
    T: Hint,
{
    fn hint(&self, shortcuts: &Table, out: &ast::Out, dim: usize, is_root: bool) -> Result<Self> {
        Ok(Box::new((**self).hint(shortcuts, out, dim, is_root)?))
    }
}
