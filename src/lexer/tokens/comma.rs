use crate::{common::token::Token, lexer::token_match::TokenMatch};

pub fn comma(representation: &str) -> Option<TokenMatch> {
    if representation.starts_with(',') {
        Some(TokenMatch {
            offset: 1,
            matched: Some(Token::Comma),
        })
    } else {
        None
    }
}
