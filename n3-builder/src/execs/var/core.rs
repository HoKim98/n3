use crate::ast;
use crate::error::{GraphError, Result};
use crate::graph::{ToValues, Values, Variables};

#[derive(Clone, Default, Debug)]
pub struct Vars {
    inner: Variables,
}

#[derive(Clone)]
pub struct Query<'a> {
    pub name: &'a str,
    pub ty: ast::LetType,
    // order: value -> fn_value
    pub value: Option<String>,
    pub fn_value: Option<fn() -> Option<String>>,
}

impl From<Variables> for Vars {
    fn from(inner: Variables) -> Self {
        Self { inner }
    }
}

impl Vars {
    pub fn from_variables(inner: Variables) -> Self {
        Self::from(inner)
    }

    pub fn load(query: Vec<Query>) -> Result<Self> {
        Ok(Self {
            inner: query
                .into_iter()
                .map(|x| {
                    let name = x.name.to_string();
                    let ty = x.ty;
                    let fn_value = x.fn_value;
                    let value = x
                        .value
                        .or_else(|| fn_value.map(|f| f()).flatten())
                        .map(|v| Self::convert(&name, v, Some(&ty)))
                        .transpose()?;

                    let mut var = ast::Variable::with_name_value(name.clone(), value);
                    var.id = Some(0);
                    var.id_old = Some(0);
                    var.ty = Some(ty);
                    Ok((name, var.into()))
                })
                .collect::<Result<_>>()?,
        })
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

    pub fn set(&self, name: &str, value: &str) -> Result<()> {
        let mut var = self.get(name)?.borrow_mut();
        var.value = Some(Self::convert(name, value.to_string(), var.ty.as_ref())?);
        Ok(())
    }

    pub fn set_as_value(&self, name: &str, value: impl Into<ast::Value>) -> Result<()> {
        let value = value.into();

        let mut var = self.get(name)?.borrow_mut();

        let expected = &var.ty;
        let given = &value.ty();
        if expected != given {
            return GraphError::MismatchedType {
                name: name.to_string(),
                expected: expected.clone().unwrap(),
                given: given.clone(),
            }
            .into();
        }

        var.value = Some(value);
        Ok(())
    }

    fn convert(name: &str, value: String, ty: Option<&ast::LetType>) -> Result<ast::Value> {
        match ty {
            Some(ast::LetType::Bool) => match value.to_lowercase().as_str() {
                "yes" | "true" | "1" => Ok(true.into()),
                "no" | "false" | "0" => Ok(false.into()),
                _ => unparsable_string(name, value, ty),
            },
            Some(ast::LetType::UInt) => match value.parse::<u64>() {
                Ok(value) => Ok(value.into()),
                Err(_) => unparsable_string(name, value, ty),
            },
            Some(ast::LetType::Int) => match value.parse::<i64>() {
                Ok(value) => Ok(value.into()),
                Err(_) => unparsable_string(name, value, ty),
            },
            Some(ast::LetType::Real) => match value.parse::<f64>() {
                Ok(value) => Ok(value.into()),
                Err(_) => unparsable_string(name, value, ty),
            },
            Some(ast::LetType::String) => Ok(ast::Value::String(value)),
            Some(ast::LetType::Node(_)) => Ok(ast::Value::Node(value)),
            _ => todo!(),
        }
    }
}

impl ToValues for Vars {
    fn to_values(&self) -> Values {
        self.inner.to_values()
    }
}

fn unparsable_string<T>(name: &str, value: String, ty: Option<&ast::LetType>) -> Result<T> {
    GraphError::UnparsableString {
        name: name.to_string(),
        value,
        ty: ty.cloned(),
    }
    .into()
}
