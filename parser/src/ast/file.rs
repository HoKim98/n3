use std::collections::BTreeMap;
use std::fmt;

use super::fmt::FmtGuard;
use super::node::Node;
use super::uses::Use;

pub struct File {
    pub uses: BTreeMap<String, Use>,
    pub node: Node,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for u in self.uses.values() {
            u.fmt(f)?;
        }
        FmtGuard::new(&self.node).fmt(f)
    }
}
