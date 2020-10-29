pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub type Program = [u8];

pub trait Machine {
    fn spawn(&mut self, id: usize, program: &Program) -> Result<()>;

    fn join(&mut self) -> Result<()>;
    fn terminate(&mut self) -> Result<()>;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Query {
    pub provider: Option<String>,
    pub domain: Option<String>,
    pub device: Option<String>,
    pub id: Option<String>,
}
