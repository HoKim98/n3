use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::fmt::FmtGuard;
use super::graph::OutDim;

#[derive(Clone)]
pub struct RefVariable(Rc<RefCell<Variable>>);

impl Deref for RefVariable {
    type Target = RefCell<Variable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for RefVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0.borrow();
        write!(f, "{}", &inner.name)?;
        if let Some(value) = &inner.value {
            write!(f, "={:?}", value)?;
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct Variable {
    pub id: Option<usize>,
    pub id_old: Option<usize>,

    pub name: String,
    pub shortcut: Option<String>,

    pub ty: Option<LetType>,
    pub value: Option<Value>,
}

impl Variable {
    pub fn with_name(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn with_name_value(name: String, value: Value) -> Self {
        Self {
            name,
            value: Some(value),
            ..Default::default()
        }
    }

    pub fn is_hint(&self) -> bool {
        self.ty.map(|x| x == LetType::Dim).unwrap_or_default()
            || self.value.as_ref().map(|x| x.is_hint()).unwrap_or_default()
    }
}

impl Into<RefVariable> for Variable {
    fn into(self) -> RefVariable {
        RefVariable(Rc::new(RefCell::new(self)))
    }
}

impl Into<Value> for RefVariable {
    fn into(self) -> Value {
        Value::Variable(self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LetType {
    Bool,
    Int,
    Real,
    Node(LetNodeType),
    Dim,
}

impl fmt::Debug for LetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Real => write!(f, "real"),
            Self::Node(ty) => write!(f, "{:?}node", &ty),
            Self::Dim => write!(f, "dim"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LetNodeType {
    Default,
    Data,
    Optim,
}

impl fmt::Debug for LetNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Data => write!(f, "data "),
            Self::Optim => write!(f, "optim "),
        }
    }
}

pub struct NodeLet {
    pub name: String,
    pub shortcut: Option<String>,
    pub ty: LetType,
    pub value: Option<Value>,
}

impl<'a> fmt::Debug for FmtGuard<'a, NodeLet> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}let ", indent)?;

        if let Some(shortcut) = &self.shortcut {
            write!(f, "{}: ", shortcut)?;
        }
        write!(f, "{} = {:?}", &self.name, &self.ty)?;

        if self.ty != LetType::Dim {
            match &self.value {
                Some(value) => write!(f, " {:?}", value)?,
                None => write!(f, " *")?,
            }
        }
        write!(f, "\n")
    }
}

pub type Keywords = BTreeMap<String, Value>;

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    UInt(u64),
    Int(i64),
    Real(f64),
    Node(String),
    Dim(Box<OutDim>),
    Variable(RefVariable),
    Expr {
        op: Operator,
        lhs: Box<Value>,
        rhs: Option<Box<Value>>,
    },
}

impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Bool(self)
    }
}

impl Into<Value> for u64 {
    fn into(self) -> Value {
        Value::UInt(self)
    }
}

impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::Int(self)
    }
}

impl Into<Value> for f64 {
    fn into(self) -> Value {
        Value::Real(self)
    }
}

impl Value {
    pub fn is_hint(&self) -> bool {
        match self {
            Self::Variable(value) => value.borrow().is_hint(),
            Self::Expr { op: _, lhs, rhs } => {
                lhs.is_hint() || rhs.as_ref().map(|x| x.is_hint()).unwrap_or_default()
            }
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => {
                if *value {
                    write!(f, "yes")
                } else {
                    write!(f, "no")
                }
            }
            Self::UInt(value) => write!(f, "{}", value),
            Self::Int(value) => write!(f, "{}", value),
            Self::Real(value) => write!(f, "{}", value),
            Self::Node(value) => write!(f, "{}", value),
            Self::Dim(value) => write!(f, "{:?}", value),
            Self::Variable(value) => write!(f, "{:?}", value),
            Self::Expr { op, lhs, rhs } => match rhs {
                Some(rhs) => write!(f, "({:?} {:?} {:?})", lhs, op, rhs),
                None => write!(f, "{:?}{:?}", op, lhs),
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    // unary
    Pos,
    Neg,
    // binary
    Add,
    Sub,
    Mul,
    MulInt,
    Div,
    Mod,
    Pow,
    // logical
    And,
    Or,
    Xor,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pos | Self::Add => write!(f, "+"),
            Self::Neg | Self::Sub => write!(f, "-"),
            Self::Mul | Self::MulInt => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Pow => write!(f, "**"),
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
        }
    }
}
