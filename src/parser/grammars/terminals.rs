use crate::common::token::{Bracket, InfixOperator, Token};

pub fn bracket(tokens: &Vec<Token>, offset: usize, kind: Bracket) -> Option<()> {
    match tokens.get(offset) {
        Some(Token::Bracket(bracket)) if bracket == &kind => Some(()),
        _ => None,
    }
}

pub fn comma(tokens: &Vec<Token>, offset: usize) -> Option<()> {
    match tokens.get(offset) {
        Some(Token::Comma) => Some(()),
        _ => None,
    }
}

pub fn identifier<'a>(tokens: &'a Vec<Token>, offset: usize) -> Option<&'a str> {
    if let Some(Token::Identificator(identificator)) = tokens.get(offset) {
        Some(identificator)
    } else {
        None
    }
}

pub fn literal<'a>(tokens: &'a Vec<Token>, offset: usize) -> Option<f64> {
    if let Some(Token::Literal(value)) = tokens.get(offset) {
        Some(value.to_owned())
    } else {
        None
    }
}

pub fn operator(tokens: &Vec<Token>, offset: usize) -> Option<InfixOperator> {
    if let Some(Token::InfixOperator(operator)) = tokens.get(offset) {
        Some(operator.to_owned())
    } else {
        None
    }
}
