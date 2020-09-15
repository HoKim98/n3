use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::ops;
use std::rc::Rc;

use num_traits::Pow;

use super::fmt::FmtGuard;
use super::graph::OutDim;

#[derive(Clone)]
pub struct RefVariable(Rc<RefCell<Variable>>);

impl ops::Deref for RefVariable {
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
    pub id: Option<u64>,
    pub id_old: Option<u64>,

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

    pub fn with_name_value(name: String, value: Option<Value>) -> Self {
        Self {
            name,
            value,
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
    UInt,
    Int,
    Real,
    Node(LetNodeType),
    Dim,
}

impl fmt::Debug for LetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool => write!(f, "bool"),
            Self::UInt | Self::Int => write!(f, "int"),
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

crate::impl_debug_no_guard!(NodeLet);
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
    Dim(OutDim),
    Variable(RefVariable),
    Expr(Box<Expr>),
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
            Self::Expr(expr) => {
                expr.lhs.is_hint() || expr.rhs.as_ref().map(|x| x.is_hint()).unwrap_or_default()
            }
            _ => false,
        }
    }

    pub fn into_uint(self) -> Self {
        match self {
            Self::Bool(value) => Self::UInt(value as u64),
            Self::UInt(value) => Self::UInt(value),
            Self::Int(value) => Self::UInt(value as u64),
            Self::Real(value) => Self::UInt(value as u64),
            _ => error_value_not_built(),
        }
    }

    pub fn into_int(self) -> Self {
        match self {
            Self::Bool(value) => Self::Int(value as i64),
            Self::UInt(value) => Self::Int(value as i64),
            Self::Int(value) => Self::Int(value),
            Self::Real(value) => Self::Int(value as i64),
            _ => error_value_not_built(),
        }
    }

    pub fn into_real(self) -> Self {
        match self {
            Self::Bool(value) => Self::Real(value as u8 as f64),
            Self::UInt(value) => Self::Real(value as f64),
            Self::Int(value) => Self::Real(value as f64),
            Self::Real(value) => Self::Real(value),
            _ => error_value_not_built(),
        }
    }
}

fn error_value_not_built<T>() -> T {
    unreachable!("The value should be built.")
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
            Self::Expr(value) => write!(f, "{:?}", value),
        }
    }
}

#[derive(Clone)]
pub struct Expr {
    pub op: Operator,
    pub lhs: Value,
    pub rhs: Option<Value>,
}

impl Into<Value> for Expr {
    fn into(self) -> Value {
        Value::Expr(Box::new(self))
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.rhs {
            Some(rhs) => write!(f, "({:?} {:?} {:?})", &self.lhs, &self.op, rhs),
            None => write!(f, "{:?}{:?}", &self.op, &self.lhs),
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

impl ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Bool(value) => Self::Int(-(value as i64)),
            Self::UInt(value) => Self::Int(value as i64),
            Self::Int(value) => Self::Int(-value),
            Self::Real(value) => Self::Real(-value),
            _ => error_value_not_built(),
        }
    }
}

macro_rules! impl_binary_op_arith(
    ($trait:ty, $f:ident) => {
        impl $trait for Value {
            type Output = Self;

            fn $f(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Self::Bool(lhs), Self::Bool(rhs)) => Self::Int((lhs as i64).$f(rhs as i64)),
                    (Self::Bool(lhs), Self::UInt(rhs)) => Self::UInt((lhs as u64).$f(rhs)),
                    (Self::Bool(lhs), Self::Int(rhs)) => Self::Int((lhs as i64).$f(rhs)),
                    (Self::Bool(lhs), Self::Real(rhs)) => Self::Real((lhs as u8 as f64).$f(rhs)),
                    (Self::UInt(lhs), Self::Bool(rhs)) => Self::UInt(lhs.$f(rhs as u64)),
                    (Self::UInt(lhs), Self::UInt(rhs)) => Self::UInt(lhs.$f(rhs)),
                    (Self::UInt(lhs), Self::Int(rhs)) => Self::Int((lhs as i64).$f(rhs)),
                    (Self::UInt(lhs), Self::Real(rhs)) => Self::Real((lhs as f64).$f(rhs)),
                    (Self::Int(lhs), Self::Bool(rhs)) => Self::Int(lhs.$f(rhs as i64)),
                    (Self::Int(lhs), Self::UInt(rhs)) => Self::Int(lhs.$f(rhs as i64)),
                    (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs.$f(rhs)),
                    (Self::Int(lhs), Self::Real(rhs)) => Self::Real((lhs as f64).$f(rhs)),
                    (Self::Real(lhs), Self::Bool(rhs)) => Self::Real(lhs.$f(rhs as u8 as f64)),
                    (Self::Real(lhs), Self::UInt(rhs)) => Self::Real(lhs.$f(rhs as f64)),
                    (Self::Real(lhs), Self::Int(rhs)) => Self::Real(lhs.$f(rhs as f64)),
                    (Self::Real(lhs), Self::Real(rhs)) => Self::Real(lhs.$f(rhs)),
                    _ => error_value_not_built(),
                }
            }
        }
    }
);

macro_rules! impl_binary_op_logical(
    ($trait:ty, $f:ident) => {
        impl $trait for Value {
            type Output = Self;

            fn $f(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Self::Bool(lhs), Self::Bool(rhs)) => Self::Bool(lhs.$f(rhs)),
                    (Self::Bool(lhs), Self::UInt(rhs)) => Self::UInt((lhs as u64).$f(rhs)),
                    (Self::Bool(lhs), Self::Int(rhs)) => Self::Int((lhs as i64).$f(rhs)),
                    (Self::Bool(lhs), Self::Real(rhs)) => Self::Int((lhs as i64).$f(rhs as i64)),
                    (Self::UInt(lhs), Self::Bool(rhs)) => Self::UInt(lhs.$f(rhs as u64)),
                    (Self::UInt(lhs), Self::UInt(rhs)) => Self::UInt(lhs.$f(rhs)),
                    (Self::UInt(lhs), Self::Int(rhs)) => Self::Int((lhs as i64).$f(rhs)),
                    (Self::UInt(lhs), Self::Real(rhs)) => Self::Int((lhs as i64).$f(rhs as i64)),
                    (Self::Int(lhs), Self::Bool(rhs)) => Self::Int(lhs.$f(rhs as i64)),
                    (Self::Int(lhs), Self::UInt(rhs)) => Self::Int(lhs.$f(rhs as i64)),
                    (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs.$f(rhs)),
                    (Self::Int(lhs), Self::Real(rhs)) => Self::Int((lhs).$f(rhs as i64)),
                    (Self::Real(lhs), Self::Bool(rhs)) => Self::Int((lhs as i64).$f(rhs as i64)),
                    (Self::Real(lhs), Self::UInt(rhs)) => Self::Int((lhs as i64).$f(rhs as i64)),
                    (Self::Real(lhs), Self::Int(rhs)) => Self::Int((lhs as i64).$f(rhs)),
                    (Self::Real(lhs), Self::Real(rhs)) => Self::Int((lhs as i64).$f(rhs as i64)),
                    _ => error_value_not_built(),
                }
            }
        }
    }
);

impl Pow<Self> for Value {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Bool(lhs), Self::Bool(rhs)) => Self::UInt((lhs as u64).pow(rhs as u32)),
            (Self::Bool(lhs), Self::UInt(rhs)) => Self::UInt((lhs as u64).pow(rhs as u32)),
            (Self::Bool(lhs), Self::Int(rhs)) => Self::Real((lhs as u8 as f64).pow(rhs as i32)),
            (Self::Bool(lhs), Self::Real(rhs)) => Self::Real((lhs as u8 as f64).pow(rhs)),
            (Self::UInt(lhs), Self::Bool(rhs)) => Self::UInt(lhs.pow(rhs as u32)),
            (Self::UInt(lhs), Self::UInt(rhs)) => Self::UInt(lhs.pow(rhs as u32)),
            (Self::UInt(lhs), Self::Int(rhs)) => Self::Real((lhs as f64).pow(rhs as i32)),
            (Self::UInt(lhs), Self::Real(rhs)) => Self::Real((lhs as f64).pow(rhs)),
            (Self::Int(lhs), Self::Bool(rhs)) => Self::Int(lhs.pow(rhs as u32)),
            (Self::Int(lhs), Self::UInt(rhs)) => Self::Int(lhs.pow(rhs as u32)),
            (Self::Int(lhs), Self::Int(rhs)) => Self::Real((lhs as f64).pow(rhs as i32)),
            (Self::Int(lhs), Self::Real(rhs)) => Self::Real((lhs as f64).pow(rhs)),
            (Self::Real(lhs), Self::Bool(rhs)) => Self::Real(lhs.pow(rhs as u8)),
            (Self::Real(lhs), Self::UInt(rhs)) => Self::Real(lhs.pow(rhs as i32)),
            (Self::Real(lhs), Self::Int(rhs)) => Self::Real(lhs.pow(rhs as i32)),
            (Self::Real(lhs), Self::Real(rhs)) => Self::Real(lhs.pow(rhs)),
            _ => error_value_not_built(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
            (Self::Bool(lhs), Self::UInt(rhs)) => *lhs as u64 == *rhs,
            (Self::Bool(lhs), Self::Int(rhs)) => *lhs as i64 == *rhs,
            (Self::Bool(lhs), Self::Real(rhs)) => *lhs as u8 as f64 == *rhs,
            (Self::UInt(lhs), Self::Bool(rhs)) => *lhs == *rhs as u64,
            (Self::UInt(lhs), Self::UInt(rhs)) => lhs == rhs,
            (Self::UInt(lhs), Self::Int(rhs)) => *lhs as i64 == *rhs,
            (Self::UInt(lhs), Self::Real(rhs)) => *lhs as f64 == *rhs,
            (Self::Int(lhs), Self::Bool(rhs)) => *lhs == *rhs as i64,
            (Self::Int(lhs), Self::UInt(rhs)) => *lhs == *rhs as i64,
            (Self::Int(lhs), Self::Int(rhs)) => lhs.eq(rhs),
            (Self::Int(lhs), Self::Real(rhs)) => *lhs as f64 == *rhs,
            (Self::Real(lhs), Self::Bool(rhs)) => *lhs == *rhs as u8 as f64,
            (Self::Real(lhs), Self::UInt(rhs)) => *lhs == *rhs as f64,
            (Self::Real(lhs), Self::Int(rhs)) => *lhs == *rhs as f64,
            (Self::Real(lhs), Self::Real(rhs)) => lhs.eq(rhs),
            _ => error_value_not_built(),
        }
    }
}

impl_binary_op_arith!(ops::Add, add);
impl_binary_op_arith!(ops::Sub, sub);
impl_binary_op_arith!(ops::Mul, mul);
impl_binary_op_arith!(ops::Div, div);
impl_binary_op_arith!(ops::Rem, rem);

impl_binary_op_logical!(ops::BitAnd, bitand);
impl_binary_op_logical!(ops::BitOr, bitor);
impl_binary_op_logical!(ops::BitXor, bitxor);
