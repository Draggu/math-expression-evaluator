use std::collections::HashMap;

use crate::common::{
    ast::ASTNode,
    eval::EvaluateResult,
    function::{Function, FunctionKind},
    token::{Associativity, InfixOperator},
};
use std::iter;

pub fn evaluate(ast: &ASTNode, vars: &HashMap<String, f64>) -> Result<f64, String> {
    match _evaluate(ast, vars)? {
        EvaluateResult::Fn { .. } => Err("cannot return function".to_string()),
        EvaluateResult::Val(result) => Ok(result),
    }
}

fn _evaluate(ast: &ASTNode, vars: &HashMap<String, f64>) -> Result<EvaluateResult, String> {
    match ast {
        ASTNode::Literal(v) => Ok(EvaluateResult::Val(*v)),
        ASTNode::Var(var) => vars
            .get(var.to_owned())
            .map(|v| EvaluateResult::Val(*v))
            .ok_or(format!("variable ( {} ) does not exists!", var)),
        ASTNode::Operation { ingredients, first } => {
            let mut prev = _evaluate(first, vars)?;

            let mut it = ingredients.iter().peekable();

            while let Some((next_op, ..)) = it.peek().cloned() {
                let mut values = iter::from_fn(|| it.next_if(|(op, ..)| op == next_op))
                    .map(|(op, node)| _evaluate(node, vars).map(|res| (op, res)))
                    .collect::<Result<Vec<(&InfixOperator, EvaluateResult)>, String>>()?;

                let call = |res: EvaluateResult,
                            (op, node): (&InfixOperator, EvaluateResult)|
                 -> Result<EvaluateResult, String> {
                    Function::from_operator(&op)
                        .call(&vec![res, node])
                        .map(EvaluateResult::Val)
                };

                prev = if next_op.associativity() == Associativity::Right {
                    let (mut last_op, last) = values.pop().unwrap();

                    let values = values
                        .into_iter()
                        .rev()
                        .map(|(op, value)| {
                            let operator = last_op;
                            last_op = op;
                            (operator, value)
                        })
                        .collect::<Vec<(&InfixOperator, EvaluateResult)>>();

                    iter::once((last_op, prev))
                        .chain(values)
                        .try_fold(last, call)?
                } else {
                    values.into_iter().try_fold(prev, call)?
                }
            }

            Ok(prev)
        }
        ASTNode::Call { func, calls } => {
            let func = match func {
                FunctionKind::FromOperator(op) => Function::from_operator(&op),
                FunctionKind::Literal(ident) => Function::from_literal(ident)
                    .ok_or(format!("function ( {} ) does not exists!", ident))?,
            };

            let args = calls
                .into_iter()
                .flatten()
                .map(|node| _evaluate(node, vars))
                .collect::<Result<Vec<EvaluateResult>, String>>()?;

            if func.args_num() == args.len() as u8 {
                func.call(&args).map(EvaluateResult::Val)
            } else {
                Ok(EvaluateResult::Fn { func, args })
            }
        }
    }
}
