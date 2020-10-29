use std::fmt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub type Program = [u8];

pub trait Machine {
    fn spawn(&mut self, id: usize, program: &Program, command: &str) -> Result<()>;

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

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut write_colon = false;

        for (prefix, field) in &[
            (true, &self.provider),
            (true, &self.domain),
            (true, &self.device),
            (false, &self.id),
        ] {
            if write_colon && (*prefix || field.is_some()) {
                write!(f, ":")?;
            }
            if let Some(field) = field {
                write_colon = true;
                write!(f, "{}", field)?;
            }
        }
        Ok(())
    }
}
