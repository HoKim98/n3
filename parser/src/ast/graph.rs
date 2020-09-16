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

    pub fn new(id: u64, name: String) -> Self {
        Self {
            id: Some(id),
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
pub struct Shapes(pub BTreeMap<String, Option<Shape>>);

crate::impl_debug_no_guard!(Shapes);
impl<'a> fmt::Debug for FmtGuard<'a, Shapes> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 1 {
            if let Some(shape) = self.0.get("x") {
                return write!(f, " = {:?}\n", shape);
            }
        }

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

impl GraphInputs {
    pub fn ty(&self) -> GraphInputsType {
        match self {
            Self::Dict(_) => GraphInputsType::Dict,
            Self::List(_) => GraphInputsType::List,
        }
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GraphInputsType {
    UseLast,
    Dict,
    List,
}

pub struct GraphCall {
    pub name: String,
    pub inputs: Option<GraphInputs>,
    pub args: Option<Keywords>,
    pub repeat: Option<Value>,
}

impl GraphCall {
    pub fn get_inputs_ty(&self) -> GraphInputsType {
        match &self.inputs {
            Some(inputs) => inputs.ty(),
            None => GraphInputsType::UseLast,
        }
    }
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

crate::impl_debug_no_guard!(GraphNode);
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
