use super::{
    expr::expr,
    terminals::{bracket, comma, identifier, operator},
};
use crate::{
    common::{
        ast::ASTNode,
        function::FunctionKind,
        token::{Bracket, InfixOperator, Token},
    },
    parser::node_match::NodeMatch,
};
use std::iter;

#[derive(Debug)]
pub struct FnMatch<'a> {
    func: FunctionKind<'a>,
    offset: usize,
}

enum Func<'a> {
    Op(InfixOperator),
    Nested(FnMatch<'a>),
}

pub fn fn_call_or_fn<'a>(tokens: &'a Vec<Token>, base_offset: usize) -> Option<NodeMatch<'a>> {
    function_call(tokens, base_offset).or_else(|| {
        function(tokens, base_offset).map(|FnMatch { offset, func }| NodeMatch {
            offset,
            ast: ASTNode::Call {
                func,
                calls: Vec::new(),
            },
        })
    })
}

pub fn function<'a>(tokens: &'a Vec<Token>, offset: usize) -> Option<FnMatch<'a>> {
    identifier(tokens, offset)
        .map(|name| FnMatch {
            offset: 1,
            func: FunctionKind::Literal(name),
        })
        .or_else(|| {
            bracket(tokens, offset, Bracket::Open)?;

            let op = operator(tokens, offset + 1)
                .map(Func::Op)
                .or_else(|| function(tokens, offset + 1).map(Func::Nested))?;

            let bracket_offset = 1 + match op {
                Func::Op(_) => 1,
                Func::Nested(ref fn_match) => fn_match.offset,
            };

            bracket(tokens, offset + bracket_offset, Bracket::Close)?;

            Some(match op {
                Func::Op(op) => FnMatch {
                    offset: 3,
                    func: FunctionKind::FromOperator(op),
                },
                Func::Nested(mut fn_match) => {
                    fn_match.offset += 2;
                    fn_match
                }
            })
        })
}

pub fn function_call<'a>(tokens: &'a Vec<Token>, base_offset: usize) -> Option<NodeMatch<'a>> {
    let FnMatch { mut offset, func } = function(tokens, base_offset)?;

    let calls: Vec<Vec<ASTNode<'a>>> = iter::from_fn(|| {
        bracket(tokens, base_offset + offset, Bracket::Open)?;

        let mut args = Vec::<ASTNode<'a>>::new();

        let mut local_offset = offset + 1;

        while let Some(expr) = expr(tokens, base_offset + local_offset)
            .filter(|expr| comma(tokens, expr.offset + base_offset + local_offset).is_some())
        {
            local_offset += expr.offset + 1;

            args.push(expr.ast);
        }

        if let Some(expr) = expr(tokens, base_offset + local_offset) {
            local_offset += expr.offset;

            args.push(expr.ast);
        }

        bracket(tokens, base_offset + local_offset, Bracket::Close)?;

        offset = local_offset + 1;

        Some(args)
    })
    .collect();

    if calls.is_empty() {
        None
    } else {
        Some(NodeMatch {
            offset,
            ast: ASTNode::Call { func, calls },
        })
    }
}
