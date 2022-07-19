use crate::{
    common::{
        ast::ASTNode,
        token::{InfixOperator, Token},
    },
    parser::{
        grammars::{terminals::operator, value::value},
        node_match::NodeMatch,
    },
};
use std::iter;

pub fn unary<'a>(tokens: &'a Vec<Token>, offset: usize) -> Option<NodeMatch<'a>> {
    let mut index = 0;

    let negates = iter::from_fn(|| {
        let op = operator(tokens, offset + index)
            .filter(|op| op == &InfixOperator::Addition || op == &InfixOperator::Subtraction);

        index += 1;

        op
    })
    .filter(|op| op == &InfixOperator::Subtraction)
    .count();

    let v = value(tokens, offset + index - 1)?;

    Some(NodeMatch {
        offset: v.offset + index - 1,
        ast: if negates % 2 == 0 {
            v.ast
        } else {
            ASTNode::Operation {
                first: Box::new(v.ast),
                ingredients: vec![(InfixOperator::Multiplication, ASTNode::Literal(-1.0))],
            }
        },
    })
}
