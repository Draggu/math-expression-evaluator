use crate::common::{ast::ASTNode, token::Token};

use super::grammars::expr::expr;

pub fn parse<'a>(tokens: &'a Vec<Token>) -> Option<ASTNode<'a>> {
    expr(&tokens, 0).map(|m| m.ast)
}
