use crate::expression::Expression;

#[derive(Debug)]
pub struct WFF {
    wff: Expression,
}

impl WFF {
    pub fn from_expression(exp: Expression) -> Result<Self, ()> {
        if Expression::is_wff(&exp) {
            Ok(WFF {
                wff: exp,
            })
        } else {
            Err(())
        }
    }
}
