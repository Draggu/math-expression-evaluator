use crate::lexer::token_match::TokenMatch;
use lazy_static::lazy_static;
use regex::Regex;

pub fn white_space(representation: &str) -> Option<TokenMatch> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\s+)").unwrap();
    }

    let white_spaces = REGEX.find(representation)?;

    Some(TokenMatch {
        offset: white_spaces.end(),
        matched: None,
    })
}
