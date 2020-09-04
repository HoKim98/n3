use std::collections::BTreeMap;
use std::fmt;

use super::fmt::FmtGuard;
use super::variable::Value;

#[derive(Clone, Debug)]
pub struct OutDim {
    pub out: Out,
    pub dim: Value,
}

impl Into<Value> for OutDim {
    fn into(self) -> Value {
        Value::Dim(Box::new(self))
    }
}

#[derive(Clone)]
pub struct Out {
    pub id: Option<u64>,
    pub name: Option<String>,
}

impl fmt::Debug for Out {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
        }
        write!(f, "$")?;
        if let Some(id) = &self.id {
            write!(f, "{}", id)?;
        }
        Ok(())
    }
}

pub struct Shape {
    pub dims: Vec<Value>,
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " = ")?;
        for dim in &self.dims {
            write!(f, "{:?}, ", dim)?;
        }
        Ok(())
    }
}

impl Shape {
    pub fn with_dims(dims: Vec<Value>) -> Self {
        Self { dims }
    }
}

pub enum Shapes {
    Dict(BTreeMap<String, Shape>),
    Single(Shape),
}

impl<'a> fmt::Debug for FmtGuard<'a, Shapes> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &**self {
            Shapes::Dict(dict) => {
                let indent = self.indent();
                write!(f, ":\n")?;

                for (name, shape) in dict {
                    write!(f, "{}{}{:?}\n", &indent, name, shape)?;
                }
                Ok(())
            }
            Shapes::Single(shape) => write!(f, "{:?}\n", shape),
        }
    }
}

pub enum GraphInputs {
    Dict(BTreeMap<String, Out>),
    List(Vec<Out>),
}

impl fmt::Debug for GraphInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dict(dict) => {
                write!(f, "{{")?;
                for (k, x) in dict {
                    write!(f, "{}={:?}, ", k, x)?;
                }
                write!(f, "}}")
            }
            Self::List(list) => {
                write!(f, "[")?;
                for x in list {
                    write!(f, "{:?}, ", x)?;
                }
                write!(f, "]")
            }
        }
    }
}

pub struct GraphCall {
    pub name: String,
    pub inputs: Option<GraphInputs>,
    pub args: Option<BTreeMap<String, Value>>,
}

impl fmt::Debug for GraphCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)?;

        if let Some(inputs) = &self.inputs {
            inputs.fmt(f)?;
        }
        if let Some(args) = &self.args {
            write!(f, "(")?;
            for (name, value) in args {
                write!(f, "{}={:?}, ", name, value)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

pub struct GraphNode {
    pub id: u64,
    pub calls: Vec<GraphCall>,
    pub shapes: Option<Shapes>,
}

impl<'a> fmt::Debug for FmtGuard<'a, GraphNode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}{}. ", &indent, self.id)?;

        for value in &self.calls {
            write!(f, "{:?} + ", value)?;
        }

        if let Some(shapes) = &self.shapes {
            self.resolve(shapes).fmt(f)
        } else {
            write!(f, "\n")
        }
    }
}
