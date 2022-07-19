use std::collections::HashMap;

pub mod common;
mod evaluator;
mod lexer;
mod parser;

pub fn calculate(input_str: &str) -> Result<f64, String> {
    let tokens = lexer::tokenize(input_str)?;

    //TODO parsing erros
    let ast = parser::parse(&tokens).ok_or("parsing error")?;

    evaluator::evaluate(&ast, &HashMap::new())
}

fn main() -> Result<(), String> {
    println!("{}", calculate("23* 2 *2|(+) 1--2)".as_ref())?);

    Ok(())
}
