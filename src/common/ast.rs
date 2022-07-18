use super::function::FunctionKind;
use super::token::InfixOperator;

#[derive(Debug)]
pub enum ASTNode<'a> {
    Literal(f64),
    Var(&'a str),
    Call {
        func: FunctionKind<'a>,
        calls: Vec<Vec<ASTNode<'a>>>,
    },
    Operation {
        ingredients: Vec<ASTNode<'a>>,
        operator: InfixOperator,
    },
}
