use crate::{
    common::{
        ast::ASTNode,
        token::{InfixOperator, Token},
    },
    parser::{
        grammars::{function::fn_call_or_fn, terminals::operator},
        node_match::NodeMatch,
    },
};

use std::iter;

use super::unary::unary;

macro_rules! operation {
    ($name:ident, $first:expr, $next:expr, $($operator:path)+) => {
        pub fn $name<'a>(tokens: &'a Vec<Token>, base_offset: usize) -> Option<NodeMatch<'a>> {
            let NodeMatch { mut offset, ast } = $first(tokens, base_offset)?;
            let mut last_offset = 0;

            let mut occurences = iter::from_fn(|| {
                let op = operator(tokens, base_offset + offset)?;
                let m = $next(tokens, base_offset + offset + 1)?;

                last_offset = m.offset + 1;
                offset += last_offset;

                Some((op, m.ast))
            });

            match occurences.next() {
                None => Some(NodeMatch { offset, ast }),
                Some((op, value)) => {
                    let  is_in_op = |op: &InfixOperator| false $(
                        || &$operator == op
                    )+;
                    let mut is_ended_earlier = true;

                    let ast = if is_in_op(&op) {
                        let mut ingredients = vec![(op, value)];

                        ingredients.extend(occurences.take_while(|(op, ..)| {
                            is_ended_earlier = is_in_op(op);
                            is_ended_earlier
                        }));

                        if is_ended_earlier {
                            // in this case there should be no - last_offset later
                            // so add it to remove later
                            offset += last_offset;
                        }

                        ASTNode::Operation {
                            first: Box::new(ast),
                            ingredients,
                        }
                    } else {
                        ast
                    };

                    Some(NodeMatch {
                        ast,
                        offset: offset - last_offset,
                    })
                }
            }
        }
    };
}

operation!(exponentiation, unary, unary, InfixOperator::Exponentiation);
operation!(
    multiplication,
    exponentiation,
    exponentiation,
    InfixOperator::Multiplication
    InfixOperator::Division
);
operation!(addition, multiplication, multiplication, InfixOperator::Addition InfixOperator::Subtraction);
operation!(pipe, addition, fn_call_or_fn, InfixOperator::Pipe);
