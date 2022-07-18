use crate::{
    common::{
        ast::ASTNode,
        token::{Bracket, Token},
    },
    parser::node_match::NodeMatch,
};

use super::{
    expr::expr,
    function::function_call,
    terminals::{bracket, identifier, literal},
};

pub fn value<'a>(tokens: &'a Vec<Token>, offset: usize) -> Option<NodeMatch<'a>> {
    literal(tokens, offset)
        .map(|l| NodeMatch {
            offset: 1,
            ast: ASTNode::Literal(l),
        })
        .or_else(|| function_call(tokens, offset))
        .or_else(|| {
            identifier(tokens, offset).map(|ident| NodeMatch {
                offset: 1,
                ast: ASTNode::Var(ident),
            })
        })
        .or_else(|| {
            bracket(tokens, offset, Bracket::Open)?;

            let expr = expr(tokens, offset + 1)?;

            bracket(tokens, offset + 1 + expr.offset, Bracket::Close)?;

            Some(NodeMatch {
                offset: expr.offset + 2,
                ast: expr.ast,
            })
        })
}
