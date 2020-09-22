use crate::nodes::NodeRoot;

pub struct ExecRoot {
    inner: NodeRoot,
}

impl ExecRoot {
    pub fn new() -> Self {
        Self {
            inner: NodeRoot::new(),
        }
    }
}
