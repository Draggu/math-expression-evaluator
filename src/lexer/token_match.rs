use crate::common::token::Token;

#[derive(Debug)]
pub struct TokenMatch<'a> {
    pub matched: Option<Token<'a>>,
    pub offset: usize,
}
