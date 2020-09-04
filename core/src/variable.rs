use std::collections::BTreeMap;

use n3_parser::ast;

pub trait CloneValue {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self;
}

pub trait Detach {
    fn detach(&self, id: usize) -> Self;
}

impl CloneValue for ast::RefVariable {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        let this = self.borrow();
        for var in variables {
            let var_ref = var.borrow();
            if var_ref.name == this.name && var_ref.id_old == this.id {
                return var.clone();
            }
        }
        self.clone()
    }
}

impl Detach for ast::RefVariable {
    fn detach(&self, id: usize) -> Self {
        let this = self.borrow();
        let cloned = ast::Variable {
            id: Some(id),
            id_old: this.id,
            name: this.name.clone(),
            shortcut: this.shortcut.clone(),
            ty: this.ty,
            value: this.value.clone(),
        };
        cloned.into()
    }
}

impl CloneValue for ast::Value {
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        match self {
            Self::Variable(value) => Self::Variable(value.clone_value(variables)),
            Self::Expr { op, lhs, rhs } => Self::Expr {
                op: *op,
                lhs: Box::new(lhs.clone_value(variables)),
                rhs: rhs.as_ref().map(|x| Box::new(x.clone_value(variables))),
            },
            _ => self.clone(),
        }
    }
}

impl<K, V> CloneValue for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        self.iter()
            .map(|(k, v)| (k.clone(), v.clone_value(variables)))
            .collect()
    }
}

impl<T> CloneValue for Vec<T>
where
    T: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        self.iter().map(|x| x.clone_value(variables)).collect()
    }
}

impl<T> CloneValue for Option<T>
where
    T: CloneValue,
{
    fn clone_value(&self, variables: &[ast::RefVariable]) -> Self {
        self.as_ref().map(|x| x.clone_value(variables))
    }
}
