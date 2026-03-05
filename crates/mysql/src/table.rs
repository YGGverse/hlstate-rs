use mysql::prelude::FromRow;

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct Server {
    pub id: u64,
    pub added: u64,
    pub updated: u64,
    pub host: String,
    pub port: u32,
    pub name: String,
    pub description: String,
}

#[derive(Default)]
pub enum Sort {
    Asc,
    #[default]
    Desc,
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "ASC"),
            Self::Desc => write!(f, "DESC"),
        }
    }
}
