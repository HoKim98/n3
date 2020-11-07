use std::collections::BTreeMap;

use crate::ast;

#[derive(Clone, Debug)]
pub struct Table {
    pub id: u64,
    pub variables: Variables,
}

pub trait ToValues {
    fn to_values(&self) -> Values;
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.variables.eq(&other.variables)
    }
}

pub type RawVariables = BTreeMap<String, ast::Variable>;
pub type Variables = BTreeMap<String, ast::RefVariable>;
pub type Values = BTreeMap<String, Option<ast::Value>>;

impl ToValues for Table {
    fn to_values(&self) -> Values {
        self.variables.to_values()
    }
}

impl ToValues for Variables {
    fn to_values(&self) -> Values {
        self.iter()
            .map(|(k, v)| (k.clone(), v.borrow().value.clone()))
            .collect()
    }
}

impl ToValues for Values {
    fn to_values(&self) -> Self {
        self.clone()
    }
}
