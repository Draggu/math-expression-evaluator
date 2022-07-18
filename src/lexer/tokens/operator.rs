use crate::{
    common::token::{InfixOperator, Token},
    lexer::token_match::TokenMatch,
};

pub fn operator(representation: &str) -> Option<TokenMatch> {
    match representation.chars().next()? {
        '+' => Some(InfixOperator::Addition),
        '-' => Some(InfixOperator::Subtraction),
        '*' => Some(InfixOperator::Multiplication),
        '/' => Some(InfixOperator::Division),
        '%' => Some(InfixOperator::Modulo),
        '^' => Some(InfixOperator::Exponentiation),
        '|' => Some(InfixOperator::Pipe),
        _ => None,
    }
    .and_then(|operator| {
        Some(TokenMatch {
            offset: 1,
            matched: Some(Token::InfixOperator(operator)),
        })
    })
}
