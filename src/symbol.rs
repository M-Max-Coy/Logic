use crate::logical_symbol::LogicalSymbol;
use crate::sentence_symbol::SentenceSymbol;

#[derive(Debug)]
pub enum Symbol {
    Logical(LogicalSymbol),
    Parameter(SentenceSymbol),
}