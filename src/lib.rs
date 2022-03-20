#[derive(Clone, Debug)]
pub struct Variable {
    key: String,
    type_of: TokenKind,
    value: String,
}

#[derive(Clone, Debug)]
pub enum Tokens {
    Var(Variable)
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    Variable
}

impl Variable {
    pub fn new(key: String, value: String) -> Variable {
        Variable {
            key,
            type_of: TokenKind::Variable,
            value,
        }
    }
}
