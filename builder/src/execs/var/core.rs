use crate::ast;
use crate::error::{GraphError, Result};
use crate::graph::Table;

#[derive(Default, Debug)]
pub struct Vars {
    inner: Table,
}

#[derive(Clone)]
pub struct Query<'a> {
    pub name: &'a str,
    pub ty: ast::LetType,
    // order: value -> fn_value
    pub value: Option<String>,
    pub fn_value: Option<fn() -> Option<String>>,
}

impl From<Table> for Vars {
    fn from(inner: Table) -> Self {
        Self { inner }
    }
}

impl Vars {
    pub fn new(inner: Table) -> Self {
        Self::from(inner)
    }

    pub fn load(query: Vec<Query>) -> Self {
        Self {
            inner: query
                .into_iter()
                .map(|x| {
                    let name = x.name.to_string();
                    let ty = x.ty;
                    let value = x
                        .value
                        .or(x.fn_value.map(|f| f()).flatten())
                        .map(|v| Self::convert(v, Some(&ty)));

                    let mut var = ast::Variable::with_name_value(name.clone(), value);
                    var.ty = Some(ty);
                    (name, var.into())
                })
                .collect(),
        }
    }

    pub fn get(&self, name: &str) -> Result<&ast::RefVariable> {
        self.inner.get(name).ok_or_else(|| {
            GraphError::NoSuchVariable {
                name: name.to_string(),
                candidates: self.inner.keys().cloned().collect(),
            }
            .into()
        })
    }

    pub fn try_get_checked(
        &self,
        name: &str,
        expected: ast::LetType,
    ) -> Result<Option<&ast::RefVariable>> {
        match self.inner.get(name) {
            Some(var) => {
                let var_ref = var.borrow();

                if let Some(value) = var_ref.value.as_ref() {
                    let value_ty = value.ty();
                    if value_ty.as_ref() == Some(&expected) {
                        Ok(Some(var))
                    } else {
                        GraphError::MismatchedType {
                            name: name.to_string(),
                            expected,
                            given: value_ty,
                        }
                        .into()
                    }
                } else {
                    GraphError::EmptyValue {
                        name: name.to_string(),
                        expected,
                    }
                    .into()
                }
            }
            None => Ok(None),
        }
    }

    pub fn get_node_name(&self, name: &str, ty: ast::LetNodeType) -> Result<String> {
        self.get_and_cast(name, ast::LetType::Node(Some(ty)), |x| {
            x.unwrap_node_name().map(|x| x.to_string())
        })
    }

    pub fn get_string(&self, name: &str) -> Result<String> {
        self.get_and_cast(name, ast::LetType::String, |x| {
            x.unwrap_string().map(|x| x.to_string())
        })
    }

    fn get_and_cast<T>(
        &self,
        name: &str,
        expected: ast::LetType,
        f: impl Fn(&ast::Value) -> Option<T>,
    ) -> Result<T> {
        match self.get(name)?.borrow().value.as_ref() {
            Some(value) => f(value).ok_or_else(|| {
                GraphError::MismatchedType {
                    name: name.to_string(),
                    expected,
                    given: value.ty(),
                }
                .into()
            }),
            None => GraphError::EmptyValue {
                name: name.to_string(),
                expected,
            }
            .into(),
        }
    }

    pub fn set(&self, name: String, value: String) -> Result<()> {
        let mut var = self.get(&name)?.borrow_mut();
        var.value = Some(Self::convert(value, var.ty.as_ref()));
        Ok(())
    }

    fn convert(value: String, ty: Option<&ast::LetType>) -> ast::Value {
        match ty {
            Some(ast::LetType::String) => ast::Value::String(value),
            _ => todo!(),
        }
    }
}
