use crate::expression::Expression;

#[derive(Debug)]
pub struct WFF<'a> {
    wff: Expression<'a>,
}

impl<'a> WFF<'a> {
    pub fn from_expression(exp: Expression<'a>) -> Result<Self, ()> {
        if Expression::is_wff(&exp) {
            Ok(WFF {
                wff: exp,
            })
        } else {
            Err(())
        }
    }
}
