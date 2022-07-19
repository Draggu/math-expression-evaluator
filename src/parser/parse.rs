use crate::common::{ast::ASTNode, token::Token};

use super::grammars::expr::expr;

pub fn parse<'a>(tokens: &'a Vec<Token>) -> Option<ASTNode<'a>> {
    expr(&tokens, 0).and_then(|m| {
        if tokens.len() > m.offset {
            None
        } else {
            Some(m.ast)
        }
    })
}
