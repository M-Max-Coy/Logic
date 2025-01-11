use crate::symbol::Symbol;
use crate::logical_symbol::LogicalSymbol;

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
        Expression::is_wff_helper(&self.expression)
    }

    fn is_wff_helper(cur: &[Symbol]) -> bool {
        // An empty expression is not a wff
        if cur.len() == 0 {
            return false;
        }

        
        match &cur[0] {
            // If first symbol is a sentence symbol (parameter) then it is a wff if and only if the expression has length 1
            Symbol::Parameter(_) => {
                return cur.len() == 1;
            },
            Symbol::Logical(symbol) => {
                // All wffs of length greater than 1 start with a left parenthesis
                if let LogicalSymbol::LeftParenthesis = symbol {
                    if cur.len() == 1 {
                        return false;
                    }
                    
                    match &cur[1] {
                        // Case where next symbol is logical
                        Symbol::Logical(symbol) => {
                            match symbol {
                                // Case where the symbol is negation
                                LogicalSymbol::Neg => {
                                    let mut i: usize = 3;

                                    while cur[1..i].iter().fold(0, |acc, x| acc + match x {
                                        Symbol::Logical(cur_symbol) => match cur_symbol {
                                                LogicalSymbol::LeftParenthesis => 1,
                                                LogicalSymbol::RightParenthesis => -1,
                                                _ => 0,
                                            }
                                        _ => 0,
                                    }) != 0 && i < cur.len() {
                                        i += 1;
                                    }

                                    return Self::is_wff_helper(&cur[2..i]) && cur.len() == i+1;
                                },
                                _ => {
                                    // Find index of where first parenthesis balanced expression happens
                                    let mut i: usize = 2;

                                    while cur[1..i].iter().fold(0, |acc, x| acc + match x {
                                        Symbol::Logical(cur_symbol) => match cur_symbol {
                                                LogicalSymbol::LeftParenthesis => 1,
                                                LogicalSymbol::RightParenthesis => -1,
                                                _ => 0,
                                            }
                                        _ => 0,
                                    }) != 0 && i < cur.len() {
                                        i += 1;
                                    }

                                    // Checks if inbetween is binary connective
                                    if let Symbol::Logical(symbol) = &cur[i] {
                                        if let LogicalSymbol::Neg = symbol {
                                            return false;
                                        }
                                    } else {
                                        return false;
                                    }

                                    // Find index of where first parenthesis balanced expression happens
                                    let mut j: usize = i+1;

                                    while cur[i+1..j].iter().fold(0, |acc, x| acc + match x {
                                        Symbol::Logical(cur_symbol) => match cur_symbol {
                                                LogicalSymbol::LeftParenthesis => 1,
                                                LogicalSymbol::RightParenthesis => -1,
                                                _ => 0,
                                            }
                                        _ => 0,
                                    }) != 0 && j < cur.len() {
                                        j += 1;
                                    }

                                    return Self::is_wff_helper(&cur[1..i]) && Self::is_wff_helper(&cur[i+1..j]) && cur.len() == j+1;
                                },
                            }
                        },
                        // Case where next symbol is parameter 
                        Symbol::Parameter(_) => {
                            // If expression is wff then cur[1] is sentence symbol and cur[2] is binary connective

                            if let Symbol::Logical(symbol) = &cur[2] {
                                if let LogicalSymbol::Neg = symbol {
                                    return false;
                                }
                            } else {
                                return false;
                            }

                            let mut i: usize = 4;

                            while cur[3..i].iter().fold(0, |acc, x| acc + match x {
                                Symbol::Logical(cur_symbol) => match cur_symbol {
                                        LogicalSymbol::LeftParenthesis => 1,
                                        LogicalSymbol::RightParenthesis => -1,
                                        _ => 0,
                                    }
                                _ => 0,
                            }) != 0 && i < cur.len() {
                                i += 1;
                            }

                            return Self::is_wff_helper(&cur[3..i]) && cur.len() == i+1;
                        },
                    }
                } else {
                    return false;
                }
            }
        }
    }
}
