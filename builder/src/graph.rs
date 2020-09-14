use std::collections::HashMap;

use n3_parser::ast;

use super::error::{BuildError, Result};
use super::variable::*;

#[derive(Debug)]
pub struct Graph {
    id: usize,
    shortcuts: Table,
    variables: Table,
}

pub(crate) type Table = HashMap<String, ast::RefVariable>;

impl Graph {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            shortcuts: Table::new(),
            variables: Table::new(),
        }
    }

    pub fn try_with_variables<I>(id: usize, variables: I) -> Result<Self>
    where
        I: IntoIterator<Item = (String, ast::NodeLet)>,
    {
        let mut graph = Graph {
            id,
            shortcuts: Table::new(),
            variables: variables
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ast::Variable {
                            name: v.name,
                            shortcut: v.shortcut,
                            ty: Some(v.ty),
                            value: v.value,
                            ..Default::default()
                        }
                        .into(),
                    )
                })
                .collect(),
        };
        graph.build()?;
        Ok(graph)
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
            .enumerate()
            .map(|(dim, v)| v.hint(&self.variables, out, dim, true))
            .collect::<Result<_>>()?;
        Ok(ast::Shape::with_dims(dims))
    }
}

impl Estimable for Graph {
    fn is_estimable(&self) -> bool {
        self.variables.values().all(|x| x.is_estimable())
    }
}
