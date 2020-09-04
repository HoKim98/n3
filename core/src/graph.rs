use std::collections::HashMap;

use n3_parser::ast;

use super::error::{BuildError, Result};
use super::variable::{CloneValue, Detach};

#[derive(Debug)]
pub struct Graph {
    id: usize,
    shortcuts: Table,
    variables: Table,
}

type Table = HashMap<String, ast::RefVariable>;

impl Graph {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            shortcuts: Table::new(),
            variables: Table::new(),
        }
    }

    pub fn clone_safe(&self, id: usize, variables: &mut Vec<ast::RefVariable>) -> Self {
        // Step 1. get the copies
        let mut self_variables: HashMap<_, _> = self
            .variables
            .iter()
            .map(|(k, v)| (k.clone(), v.detach(id)))
            .collect();
        let self_shortcuts = self_variables
            .values()
            .filter_map(|v| match &v.borrow().shortcut {
                Some(shortcut) => Some((shortcut.clone(), v.clone())),
                None => None,
            })
            .collect();

        for var in self_variables.values_mut() {
            variables.push(var.clone());
            // Step 2. replace the olds into the news
            let mut var = var.borrow_mut();
            var.value = var.value.clone_value(variables);
        }

        // Step 3. store
        Graph {
            id,
            shortcuts: self_shortcuts,
            variables: self_variables,
        }
    }

    pub fn add(&mut self, variable: ast::RefVariable) -> Result<()> {
        let mut var_ref = variable.borrow_mut();
        let name = var_ref.name.clone();

        if self.variables.contains_key(&name) {
            return Err(BuildError::DuplicatedVariable { name }.into());
        }

        var_ref.id = Some(self.id);
        var_ref.id_old = Some(self.id);
        drop(var_ref);

        self.variables.insert(name, variable);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<&ast::RefVariable> {
        match self.variables.get(name) {
            Some(var) => Ok(var),
            None => Err(BuildError::NoSuchVariable {
                name: name.to_string(),
                candidates: self.variables.keys().cloned().collect(),
            }
            .into()),
        }
    }

    pub fn build(&mut self) -> Result<()> {
        let shortcuts_map = self
            .variables
            .iter()
            .filter_map(|(k, v)| match &v.borrow().shortcut {
                Some(shortcut) => Some((shortcut.clone(), k.clone())),
                None => None,
            })
            .collect();

        let variables = self
            .variables
            .iter()
            .map(|(k, v)| {
                let mut names = Vec::new();
                let value = v.replace_to(&mut names, &self.variables, &shortcuts_map)?;
                Ok((k.clone(), value))
            })
            .collect::<Result<_>>()?;
        self.variables = variables;

        Ok(())
    }

    pub fn hint(&self, out: &ast::Out, shape: &ast::Shape) -> Result<ast::Shape> {
        let dims = shape
            .dims
            .iter()
            .map(|d| d.hint(&self.variables, out, d, true))
            .collect::<Result<_>>()?;
        Ok(ast::Shape::with_dims(dims))
    }
}

trait Replace {
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self>
    where
        Self: Sized;
}

impl Replace for ast::RefVariable {
    fn replace_to(
        &self,
        names: &mut Vec<String>,
        variables: &Table,
        shortcuts: &HashMap<String, String>,
    ) -> Result<Self> {
        let raise_cycled_variables = || {
            let names = names.iter().rev().map(|x| x.to_string()).collect();
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

trait Hint {
    fn hint(
        &self,
        shortcuts: &Table,
        out: &ast::Out,
        dim: &ast::Value,
        is_root: bool,
    ) -> Result<Self>
    where
        Self: Sized;
}

impl Hint for ast::RefVariable {
    fn hint(
        &self,
        shortcuts: &Table,
        out: &ast::Out,
        dim: &ast::Value,
        is_root: bool,
    ) -> Result<Self> {
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
                            dim: dim.clone(),
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
    fn hint(
        &self,
        shortcuts: &Table,
        out: &ast::Out,
        dim: &ast::Value,
        is_root: bool,
    ) -> Result<Self> {
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
    fn hint(
        &self,
        shortcuts: &Table,
        out: &ast::Out,
        dim: &ast::Value,
        is_root: bool,
    ) -> Result<Self> {
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
    fn hint(
        &self,
        shortcuts: &Table,
        out: &ast::Out,
        dim: &ast::Value,
        is_root: bool,
    ) -> Result<Self> {
        Ok(Box::new((**self).hint(shortcuts, out, dim, is_root)?))
    }
}
