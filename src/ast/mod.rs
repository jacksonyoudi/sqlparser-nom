use std::fmt::Formatter;

pub mod expr;

pub mod set_expr;

pub mod statement;

pub mod table_ref;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub value: String,
}


impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.value)
    }
}


