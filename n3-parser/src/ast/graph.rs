use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::fmt::FmtGuard;
use super::variable::{Keywords, Value};

pub type Outs = BTreeMap<String, Out>;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OutDim {
    pub out: Out,
    pub dim: usize,
}

impl From<OutDim> for Value {
    fn from(value: OutDim) -> Self {
        Self::Dim(value)
    }
}

impl fmt::Debug for OutDim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}[{}]", &self.out, self.dim)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Out {
    pub id: Option<u64>,
    pub name: String,
}

impl Out {
    pub fn with_name(name: String) -> Self {
        Self { id: None, name }
    }

    pub fn new(id: u64, name: String) -> Self {
        Self { id: Some(id), name }
    }
}

impl fmt::Debug for Out {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)?;
        write!(f, "$")?;
        if let Some(id) = &self.id {
            write!(f, "{}", id)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct Shape(pub Vec<Value>);

impl Shape {
    pub fn sum(&self) -> Value {
        self.0.iter().sum()
    }

    pub fn product(&self) -> Value {
        self.0.iter().product()
    }
}

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for dim in &self.0 {
            write!(f, "{:?}, ", dim)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct Shapes(pub RefCell<ShapesInner>);

type ShapesInner = BTreeMap<String, Option<Shape>>;

impl Shapes {
    pub fn new(shapes: ShapesInner) -> Self {
        Self(RefCell::new(shapes))
    }

    pub fn to_outs(&self, id: u64) -> Outs {
        self.0
            .borrow()
            .keys()
            .map(|n| (n.clone(), Out::new(id, n.clone())))
            .collect()
    }
}

crate::impl_debug_no_guard!(Shapes);
impl<'a> fmt::Debug for FmtGuard<'a, Shapes> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let borrowed = self.0.borrow();

        if borrowed.len() == 1 {
            if let Some(shape) = borrowed.get("x") {
                if let Some(shape) = shape {
                    return writeln!(f, " = {:?}", shape);
                }
            }
        }

        let indent = self.indent();
        writeln!(f, ":")?;

        for (name, shape) in borrowed.iter() {
            write!(f, "{}{}", &indent, name)?;
            match shape {
                Some(shape) => writeln!(f, " = {:?}", shape)?,
                None => writeln!(f)?,
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub enum GraphInputs {
    Dict(Outs),
    List(Vec<Out>),
}

impl Default for GraphInputs {
    fn default() -> Self {
        Self::Dict(Outs::default())
    }
}

impl GraphInputs {
    pub fn ty(&self) -> GraphInputsType {
        match self {
            Self::Dict(_) => GraphInputsType::Dict,
            Self::List(_) => GraphInputsType::List,
        }
    }

    pub fn unwrap_dict(self) -> Option<Outs> {
        match self {
            Self::Dict(outs) => Some(outs),
            _ => None,
        }
    }

    pub fn unwrap_list(self) -> Option<Vec<Out>> {
        match self {
            Self::List(outs) => Some(outs),
            _ => None,
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

#[derive(Clone)]
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

#[derive(Clone)]
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
            writeln!(f)
        }
    }
}
