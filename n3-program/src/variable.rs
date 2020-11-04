use num_traits::Pow;

use crate::ast;

pub trait BuildValue {
    fn build(&self) -> ast::Value;
}

impl BuildValue for ast::RefVariable {
    fn build(&self) -> ast::Value {
        match &self.borrow().value {
            Some(value) => value.build(),
            None => ast::Value::Variable(self.clone()),
        }
    }
}

impl BuildValue for ast::Value {
    fn build(&self) -> Self {
        match self {
            Self::Bool(_)
            | Self::UInt(_)
            | Self::Int(_)
            | Self::Real(_)
            | Self::Dim(_)
            | Self::String(_) => self.clone(),
            Self::Node(_) => node_variable_should_be_pruned(),
            Self::Variable(value) => value.build(),
            Self::Expr(value) => value.build(),
            Self::List(value) => Self::List(value.iter().map(|x| x.build()).collect()),
            Self::Map(value) => Self::Map(
                value
                    .iter()
                    .map(|(k, v)| (k.clone(), v.as_ref().map(|x| x.build())))
                    .collect(),
            ),
        }
    }
}

impl BuildValue for ast::Expr {
    fn build(&self) -> ast::Value {
        let lhs = self.lhs.build();
        if let Some(rhs) = &self.rhs {
            let rhs = rhs.build();
            match self.op {
                ast::Operator::Add => lhs + rhs,
                ast::Operator::Sub => lhs - rhs,
                ast::Operator::Mul => lhs * rhs,
                ast::Operator::MulInt => {
                    if lhs.is_atomic() && rhs.is_atomic() {
                        (lhs.unwrap_uint().unwrap() * rhs.unwrap_uint().unwrap()).into()
                    } else {
                        ast::Expr {
                            op: self.op,
                            lhs,
                            rhs: Some(rhs),
                        }
                        .into()
                    }
                }
                ast::Operator::Div => lhs / rhs,
                ast::Operator::Mod => lhs % rhs,
                ast::Operator::Pow => lhs.pow(rhs),
                ast::Operator::And => lhs & rhs,
                ast::Operator::Or => lhs | rhs,
                ast::Operator::Xor => lhs ^ rhs,
                _ => unreachable!("expected binary operators"),
            }
        } else {
            match self.op {
                ast::Operator::Pos => lhs,
                ast::Operator::Neg => -lhs,
                _ => unreachable!("expected unary operators"),
            }
        }
    }
}

impl<T> BuildValue for Box<T>
where
    T: BuildValue,
{
    fn build(&self) -> ast::Value {
        (**self).build()
    }
}

pub(crate) fn node_variable_should_be_pruned() -> ! {
    unreachable!("node variable should be pruned.")
}
