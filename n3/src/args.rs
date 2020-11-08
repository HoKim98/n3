use n3_builder::{Args, GlobalVars};

pub struct Command<'a> {
    pub command: &'a str,
    pub env: &'a GlobalVars,
    pub args: Option<Args<'a>>,
}
