use crate::{
    common::token::{Bracket, Token},
    lexer::token_match::TokenMatch,
};

pub fn bracket(representation: &str) -> Option<TokenMatch> {
    match representation.chars().next()? {
        '(' => Some(Bracket::Open),
        ')' => Some(Bracket::Close),
        _ => None,
    }
    .and_then(|bracket| {
        Some(TokenMatch {
            offset: 1,
            matched: Some(Token::Bracket(bracket)),
        })
    })
}
