use std::collections::BTreeMap;
use std::fmt;

use super::fmt::FmtGuard;
use super::variable::{Keywords, Value};

pub type Outs = BTreeMap<String, Out>;

#[derive(Clone, Debug, PartialEq)]
pub struct OutDim {
    pub out: Out,
    pub dim: usize,
}

impl Into<Value> for OutDim {
    fn into(self) -> Value {
        Value::Dim(self)
    }
}

#[derive(Clone, PartialEq)]
pub struct Out {
    pub id: Option<u64>,
    pub name: Option<String>,
}

impl Out {
    pub fn with_name(name: String) -> Self {
        Self {
            id: None,
            name: Some(name),
        }
    }
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

#[derive(Clone)]
pub struct Shape {
    pub dims: Vec<Value>,
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(Clone)]
pub enum Shapes {
    Dict(ShapesDict),
    Single(Shape),
}

#[derive(Clone)]
pub struct ShapesDict(pub BTreeMap<String, Option<Shape>>);

impl<'a> fmt::Debug for FmtGuard<'a, Shapes> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &**self {
            Shapes::Dict(dict) => self.sibling(dict).fmt(f),
            Shapes::Single(shape) => write!(f, " = {:?}\n", shape),
        }
    }
}

impl<'a> fmt::Debug for FmtGuard<'a, ShapesDict> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, ":\n")?;

        for (name, shape) in &self.0 {
            write!(f, "{}{} = ", &indent, name)?;
            match shape {
                Some(shape) => shape.fmt(f)?,
                None => write!(f, "...")?, // TODO: ambiguous shapes?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub enum GraphInputs {
    Dict(Outs),
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
    pub args: Option<Keywords>,
    pub repeat: Option<Value>,
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
        if let Some(repeat) = &self.repeat {
            write!(f, " * {:?}", repeat)?;
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
            self.child(shapes).fmt(f)
        } else {
            write!(f, "\n")
        }
    }
}
