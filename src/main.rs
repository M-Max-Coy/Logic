mod expression;
mod wff;
mod symbol;
mod sentence_symbol;
mod logical_symbol;

use expression::Expression;
use wff::WFF;
use symbol::Symbol;
use sentence_symbol::SentenceSymbol;

fn main() {
    let ex1: Expression = Expression::from_vec(vec![Symbol::Parameter(SentenceSymbol::new(String::from("A1"),None))]);
    let wff1: Result<WFF, ()> = WFF::from_expression(ex1);
    match wff1 {
        Ok(wff) => {
            println!("{:?}", wff);
        },
        Err(err) => {

        }
    }
}
