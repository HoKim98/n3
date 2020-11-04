use std::fmt;

pub enum UseBy {
    Local,
    Repo { author: String },
    Web { source: String },
}

impl fmt::Debug for UseBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => Ok(()),
            Self::Repo { author } => write!(f, " by {}", author),
            Self::Web { source } => write!(f, " by \"{}\"", source),
        }
    }
}

pub struct Use {
    pub name: String,
    pub by: UseBy,
}

impl fmt::Debug for Use {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "use {}{:?}", &self.name, &self.by)
    }
}
