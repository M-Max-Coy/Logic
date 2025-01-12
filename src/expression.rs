use crate::symbol::Symbol::{self,Parameter,Logical};
use crate::logical_symbol::LogicalSymbol::{self,LeftParenthesis,RightParenthesis,Neg,And,Or,IFF,Implies};

#[derive(Debug)]
pub struct Expression<'a> {
    expression: Vec<&'a Symbol>,
}

impl<'a> Expression<'a> {
    pub fn new() -> Self {
        Expression {
            expression: Vec::new(),
        }
    }

    pub fn from_vec(input: Vec<&'a Symbol>) -> Self {
        Expression {
            expression: input,
        }
    }

    pub fn is_wff(&self) -> bool {
        Expression::is_wff_helper(&self.expression)
    }

    fn is_wff_helper(cur: &[&Symbol]) -> bool {
        // An empty expression is not a wff
        if cur.len() == 0 {
            return false;
        }

        
        match &cur[0] {
            // If first symbol is a sentence symbol (parameter) then it is a wff if and only if the expression has length 1
            Parameter(_) => {
                return cur.len() == 1;
            },
            Logical(symbol) => {
                // All wffs of length greater than 1 start with a left parenthesis
                if let LeftParenthesis = symbol {
                    if cur.len() == 1 {
                        return false;
                    }
                    
                    match &cur[1] {
                        // Case where next symbol is logical
                        Logical(symbol) => {
                            match symbol {
                                // Case where the symbol is negation
                                Neg => {
                                    let mut i: usize = 3;

                                    while cur[1..i].iter().fold(0, |acc, x| acc + match x {
                                        Logical(cur_symbol) => match cur_symbol {
                                                LeftParenthesis => 1,
                                                RightParenthesis => -1,
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
                                        Logical(cur_symbol) => match cur_symbol {
                                                LeftParenthesis => 1,
                                                RightParenthesis => -1,
                                                _ => 0,
                                            }
                                        _ => 0,
                                    }) != 0 && i < cur.len() {
                                        i += 1;
                                    }

                                    // Checks if inbetween is binary connective
                                    if let Logical(symbol) = &cur[i] {
                                        if let Neg = symbol {
                                            return false;
                                        }
                                    } else {
                                        return false;
                                    }

                                    // Find index of where first parenthesis balanced expression happens
                                    let mut j: usize = i+2;

                                    while cur[i+1..j].iter().fold(0, |acc, x| acc + match x {
                                        Logical(cur_symbol) => match cur_symbol {
                                                LeftParenthesis => 1,
                                                RightParenthesis => -1,
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
                        Parameter(_) => {
                            // If expression is wff then cur[1] is sentence symbol and cur[2] is binary connective

                            if let Logical(symbol) = &cur[2] {
                                if let Neg = symbol {
                                    return false;
                                }
                            } else {
                                return false;
                            }

                            let mut i: usize = 4;

                            while cur[3..i].iter().fold(0, |acc, x| acc + match x {
                                Logical(cur_symbol) => match cur_symbol {
                                        LeftParenthesis => 1,
                                        RightParenthesis => -1,
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

#[cfg(test)]
mod tests {
    use crate::sentence_symbol::SentenceSymbol;

    use super::*;

    #[test]
    fn is_wff_test() {
        let A1: Symbol = Parameter(SentenceSymbol::new(String::from("A1"), None));
        let A2: Symbol = Parameter(SentenceSymbol::new(String::from("A2"), None));
        let A3: Symbol = Parameter(SentenceSymbol::new(String::from("A3"), None));
        let A4: Symbol = Parameter(SentenceSymbol::new(String::from("A4"), None));
        let lp: Symbol = Logical(LeftParenthesis);
        let rp: Symbol = Logical(RightParenthesis);
        let neg: Symbol = Logical(Neg);
        let and: Symbol = Logical(And);
        let or: Symbol = Logical(Or);
        let implies: Symbol = Logical(Implies);
        let iff: Symbol = Logical(IFF);

        let exp1: Expression = Expression::new();
        assert_eq!(exp1.is_wff(),false);

        let exp2: Expression = Expression::from_vec(vec![&A1]);
        assert_eq!(exp2.is_wff(),true);

        let exp3: Expression = Expression::from_vec(vec![&lp,&neg,&A1,&rp]);
        assert_eq!(exp3.is_wff(),true);

        let exp4: Expression = Expression::from_vec(vec![&lp,&A1,&and,&A2,&rp]);
        assert_eq!(exp4.is_wff(),true);

        let exp5: Expression = Expression::from_vec(vec![&lp,&neg,&A1,&and,&A2,&rp]);
        assert_eq!(exp5.is_wff(),false);

        let exp6: Expression = Expression::from_vec(vec![&lp,&lp,&neg,&A1,&rp,&and,&A2,&rp]);
        assert_eq!(exp6.is_wff(),true);

        let exp7: Expression = Expression::from_vec(vec![&lp,&A1,&and,&lp,&neg,&A2,&rp,&rp]);
        assert_eq!(exp7.is_wff(),true);

        let exp8: Expression = Expression::from_vec(vec![&lp,&lp,&A1,&and,&A2,&rp,&implies,&lp,&lp,&neg,&A3,&rp,&or,&lp,&A4,&iff,&A3,&rp,&rp,&rp]);
        assert_eq!(exp8.is_wff(),true);

    }
}