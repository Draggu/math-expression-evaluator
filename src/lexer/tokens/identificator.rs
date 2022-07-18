use crate::{common::token::Token, lexer::token_match::TokenMatch};
use lazy_static::lazy_static;
use regex::Regex;

pub fn identificator(representation: &str) -> Option<TokenMatch> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9]*)").unwrap();
    }

    let identificator_match = REGEX.find(representation)?;

    Some(TokenMatch {
        offset: identificator_match.end(),
        matched: Some(Token::Identificator(identificator_match.as_str())),
    })
}
