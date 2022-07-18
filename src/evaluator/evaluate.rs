use std::collections::HashMap;

use crate::common::{
    ast::ASTNode,
    eval::EvaluateResult,
    function::{Function, FunctionKind},
    token::Associativity,
};

pub fn evaluate(ast: ASTNode, vars: &HashMap<&str, f64>) -> Result<f64, String> {
    match _evaluate(ast, vars)? {
        EvaluateResult::Fn { .. } => Err("cannot return function".to_string()),
        EvaluateResult::Val(result) => Ok(result),
    }
}

fn _evaluate(ast: ASTNode, vars: &HashMap<&str, f64>) -> Result<EvaluateResult, String> {
    match ast {
        ASTNode::Literal(v) => Ok(EvaluateResult::Val(v)),
        ASTNode::Var(var) => vars
            .get(var)
            .map(|v| EvaluateResult::Val(*v))
            .ok_or(format!("variable ( {} ) does not exists!", var)),
        ASTNode::Operation {
            ingredients,
            operator,
        } => {
            let mut iter = ingredients.into_iter();
            let first = iter.next().unwrap();
            let fold = |prev, node| ASTNode::Call {
                func: FunctionKind::FromOperator(operator),
                calls: vec![vec![prev, node]],
            };

            _evaluate(
                if operator.associativity() == Associativity::Right {
                    iter.rfold(first, fold)
                } else {
                    iter.fold(first, fold)
                },
                vars,
            )
        }
        ASTNode::Call { func, calls } => {
            let func = match func {
                FunctionKind::FromOperator(op) => Function::from_operator(&op),
                FunctionKind::Literal(ident) => Function::from_literal(ident)
                    .ok_or(format!("function ( {} ) does not exists!", ident))?,
            };

            let args: Result<Vec<EvaluateResult>, String> = calls
                .into_iter()
                .flatten()
                .map(|node| _evaluate(node, vars))
                .collect();

            match args {
                Ok(args) => {
                    if func.args_num() == args.len() as u8 {
                        func.call(&args).map(|r| EvaluateResult::Val(r))
                    } else {
                        Ok(EvaluateResult::Fn { kind: func, args })
                    }
                }
                Err(err) => Err(err),
            }
        }
    }
}
