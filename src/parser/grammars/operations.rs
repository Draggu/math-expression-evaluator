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

struct OperationPart<'a> {
    op: InfixOperator,
    value: ASTNode<'a>,
}

//TODO fix same precedense operators are missed ie. 2*2/2 results in 2*2 where /2 is lost
macro_rules! operation {
    ($name:ident, $first:expr, $next:expr, $($operator:path)+) => {
        pub fn $name<'a>(tokens: &'a Vec<Token>, base_offset: usize) -> Option<NodeMatch<'a>> {
            let NodeMatch { mut offset, ast } = $first(tokens, base_offset)?;
            let mut last_offset = 0;

            let mut occurences = iter::from_fn(|| -> Option<OperationPart> {
                let op = operator(tokens, base_offset + offset)?;
                let m = $next(tokens, base_offset + offset + 1)?;

                last_offset = m.offset + 1;
                offset += last_offset;

                Some(OperationPart { op, value: m.ast })
            });

            match occurences.next() {
                None => Some(NodeMatch { offset, ast }),
                Some(OperationPart { op, value }) => {
                    let is_in_op = false $(
                        || $operator == op
                    )+;

                    let ast = if is_in_op {
                        let mut ingredients = vec![ast, value];
                        let mut is_ended_earlier = true;

                        ingredients.extend(
                            occurences
                                .take_while(|m| {
                                    is_ended_earlier = m.op == op;
                                    is_ended_earlier
                                })
                                .map(|m| m.value),
                        );

                        if is_ended_earlier {
                            // in this case there should be no - last_offset later
                            // so add it to remove later
                            offset += last_offset;
                        }

                        ASTNode::Operation {
                            ingredients,
                            operator: op,
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
