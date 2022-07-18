use crate::{common::token::Token, lexer::token_match::TokenMatch};
use lazy_static::lazy_static;
use regex::Regex;

pub fn literal(representation: &str) -> Option<TokenMatch> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([0-9]+(?:\.[0-9]+)?)").unwrap();
    }

    let identificator_match = REGEX.find(representation)?;

    Some(TokenMatch {
        offset: identificator_match.end(),
        matched: Some(Token::Literal(identificator_match.as_str().parse().ok()?)),
    })
}
