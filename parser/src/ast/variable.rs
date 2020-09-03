use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::fmt::FmtGuard;

pub type RefVariable = Rc<RefCell<Variable>>;

#[derive(Default)]
pub struct Variable {
    pub id: Option<usize>,
    pub id_old: Option<usize>,

    pub name: String,
    pub shortcut: Option<String>,

    pub ty: Option<LetType>,
    pub value: Option<Value>,
}

struct FmtVariable<'a>(&'a RefVariable);

impl Variable {
    pub fn with_name(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl Into<RefVariable> for Variable {
    fn into(self) -> RefVariable {
        Rc::new(RefCell::new(self))
    }
}

impl<'a> fmt::Debug for FmtVariable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0.borrow();
        write!(f, "{}", &inner.name)?;
        if let Some(value) = &inner.value {
            write!(f, "={:?}", value)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
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

#[derive(PartialEq, Eq)]
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

pub enum Value {
    Bool(bool),
    Int(i64),
    Real(f64),
    Node(String),
    Variable(RefVariable),
    Expr {
        op: Operator,
        lhs: Box<Value>,
        rhs: Option<Box<Value>>,
    },
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
            Self::Int(value) => write!(f, "{}", value),
            Self::Real(value) => write!(f, "{}", value),
            Self::Node(value) => write!(f, "{}", value),
            Self::Variable(value) => write!(f, "{:?}", FmtVariable(value)),
            Self::Expr { op, lhs, rhs } => match rhs {
                Some(rhs) => write!(f, "({:?} {:?} {:?})", lhs, op, rhs),
                None => write!(f, "{:?}{:?}", op, lhs),
            },
        }
    }
}

pub enum Operator {
    // unary
    Pos,
    Neg,
    // binary
    Add,
    Sub,
    Mul,
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
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Pow => write!(f, "**"),
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
        }
    }
}
