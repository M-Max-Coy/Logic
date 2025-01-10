#[derive(Debug)]
pub struct SentenceSymbol {
    name: String,
    truth_value: Option<bool>,
}

impl SentenceSymbol {
    pub fn new(nam: String, val: Option<bool>) -> Self {
        SentenceSymbol {
            name: nam,
            truth_value: val,
        }
    }
}