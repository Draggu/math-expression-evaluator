use std::collections::HashMap;

pub mod common;
mod evaluator;
mod lexer;
mod parser;

fn main() -> Result<(), String> {
    let tokens = lexer::tokenize("2*4/2".as_ref())?;

    //TODO parsing erros
    let ast = parser::parse(&tokens).ok_or("err")?;
    println!("ast {:?}", ast);

    let result = evaluator::evaluate(ast, &HashMap::new())?;

    println!("{}", result);

    Ok(())
}
