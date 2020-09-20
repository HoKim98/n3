use std::cell::RefCell;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fmt;
use std::iter::{Product, Sum};
use std::ops;
use std::rc::Rc;

use num_traits::Pow;

use super::fmt::FmtGuard;
use super::graph::OutDim;
use super::node::ExternNodeType;

#[derive(Clone)]
pub struct RefVariable(Rc<RefCell<Variable>>);

impl RefVariable {
    pub fn get_hint(&self) -> Option<Self> {
        if let Some(value) = self.borrow().value.as_ref().map(|x| x.get_hint()).flatten() {
            Some(value)
        } else {
            Some(self.clone())
        }
    }
}

impl ops::Deref for RefVariable {
    type Target = RefCell<Variable>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for RefVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
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

impl Into<Value> for Variable {
    fn into(self) -> Value {
        RefVariable::from(self.into()).into()
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)?;
        if let Some(value) = &self.value {
            write!(f, "={:?}", value)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
pub enum LetNodeType {
    Default,
    Extern(ExternNodeType),
}

impl fmt::Debug for LetNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Extern(ty) => ty.fmt(f),
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

    List(Vec<Self>),
    Map(BTreeMap<String, Option<Self>>),
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

impl Into<Value> for Vec<Value> {
    fn into(self) -> Value {
        Value::List(self)
    }
}

impl Into<Value> for BTreeMap<String, Option<Value>> {
    fn into(self) -> Value {
        Value::Map(self)
    }
}

impl<'a> Sum<&'a Self> for Value {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::UInt(0), |lhs, rhs| {
            Expr {
                op: Operator::Add,
                lhs,
                rhs: Some(rhs.clone()),
            }
            .into()
        })
    }
}

impl<'a> Product<&'a Self> for Value {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::UInt(1), |lhs, rhs| {
            Expr {
                op: Operator::Mul,
                lhs,
                rhs: Some(rhs.clone()),
            }
            .into()
        })
    }
}

impl Value {
    pub fn ty(&self) -> Option<LetType> {
        match self {
            Self::Bool(_) => Some(LetType::Bool),
            Self::UInt(_) => Some(LetType::UInt),
            Self::Int(_) => Some(LetType::Int),
            Self::Real(_) => Some(LetType::Real),
            Self::Variable(var) => var.borrow().ty,
            Self::Node(_) => err_value_not_pruned(),
            // TODO: [proposal] add the other types
            _ => unimplemented!(),
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self {
            Self::Bool(_) | Self::UInt(_) | Self::Int(_) | Self::Real(_) => true,
            _ => false,
        }
    }

    pub fn is_hint(&self) -> bool {
        match self {
            Self::Variable(value) => value.borrow().is_hint(),
            Self::Expr(expr) => {
                expr.lhs.is_hint() || expr.rhs.as_ref().map(|x| x.is_hint()).unwrap_or_default()
            }
            _ => false,
        }
    }

    pub fn get_hint(&self) -> Option<RefVariable> {
        match self {
            Self::Variable(var) => var.get_hint(),
            _ => None,
        }
    }

    pub fn unwrap_uint(&self) -> Option<u64> {
        match self {
            Self::Bool(value) => (*value).try_into().ok(),
            Self::UInt(value) => Some(*value),
            Self::Int(value) => (*value).try_into().ok(),
            Self::Real(value) => Some(*value as u64),
            _ => None,
        }
    }

    pub fn unwrap_int(&self) -> Option<i64> {
        match self {
            Self::Bool(value) => (*value).try_into().ok(),
            Self::UInt(value) => (*value).try_into().ok(),
            Self::Int(value) => Some(*value),
            Self::Real(value) => Some(*value as i64),
            _ => None,
        }
    }

    pub fn unwrap_real(&self) -> Option<f64> {
        match self {
            Self::Bool(value) => Some(*value as u8 as f64),
            Self::UInt(value) => Some(*value as f64),
            Self::Int(value) => Some(*value as f64),
            Self::Real(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_variable(&self) -> &RefVariable {
        match self {
            Self::Variable(var) => &var,
            _ => unreachable!("The value should be variable."),
        }
    }

    pub fn try_as_dim(&self) -> Option<&RefVariable> {
        match self {
            Self::Variable(var) => {
                let dim = var.borrow().ty;
                if dim == Some(LetType::Dim) {
                    Some(var)
                } else {
                    None
                }
            }
            _ => None,
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
            Self::Expr(value) => write!(f, "{:?}", value),
            Self::List(value) => {
                write!(f, "[")?;
                for v in value {
                    write!(f, "{:?}, ", v)?;
                }
                write!(f, "]")
            }
            Self::Map(value) => {
                write!(f, "{{")?;
                for (k, v) in value {
                    write!(f, "{}", k)?;
                    if let Some(v) = v {
                        write!(f, ": {:?}", v)?;
                    }
                    write!(f, ", ")?;
                }
                write!(f, "}}")
            }
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
            _ => Expr {
                op: Operator::Neg,
                lhs: self,
                rhs: None,
            }
            .into(),
        }
    }
}

macro_rules! impl_binary_op_arith(
    ($trait:ty, $f:ident, $op:expr) => {
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
                    (lhs, rhs) => Expr {
                        op: $op,
                        lhs,
                        rhs: Some(rhs),
                    }.into(),
                }
            }
        }
    }
);

macro_rules! impl_binary_op_logical(
    ($trait:ty, $f:ident, $op:expr) => {
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
                    (lhs, rhs) => Expr {
                        op: $op,
                        lhs,
                        rhs: Some(rhs),
                    }.into(),
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
            (lhs, rhs) => Expr {
                op: Operator::Pow,
                lhs,
                rhs: Some(rhs),
            }
            .into(),
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
            _ => {
                // TODO: FUTURE: implement comparing hinted values
                println!("warning: comparing hinted values is not supported yet!");
                true
            }
        }
    }
}

pub fn err_value_not_pruned() -> ! {
    unreachable!("node variable should be pruned.");
}

impl_binary_op_arith!(ops::Add, add, Operator::Add);
impl_binary_op_arith!(ops::Sub, sub, Operator::Sub);
impl_binary_op_arith!(ops::Mul, mul, Operator::Mul);
impl_binary_op_arith!(ops::Div, div, Operator::Div);
impl_binary_op_arith!(ops::Rem, rem, Operator::Mod);

impl_binary_op_logical!(ops::BitAnd, bitand, Operator::And);
impl_binary_op_logical!(ops::BitOr, bitor, Operator::Or);
impl_binary_op_logical!(ops::BitXor, bitxor, Operator::Xor);
