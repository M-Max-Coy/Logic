use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Expression {
    expression: Vec<Symbol>,
}

impl Expression {
    pub fn new() -> Self {
        Expression {
            expression: Vec::new(),
        }
    }

    pub fn from_vec(input: Vec<Symbol>) -> Self {
        Expression {
            expression: input,
        }
    }

    pub fn is_wff(&self) -> bool {
        true
    }
}
