use clap::Parser;
use std::collections::HashMap;

pub mod common;
mod evaluator;
mod lexer;
mod parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    expr: String,

    /// variables in format name=value
    #[clap(short, long, value_parser)]
    var: Vec<String>,
}

fn main() -> Result<(), String> {
    let Args { expr, var } = Args::parse();

    let vars = var
        .iter()
        .map(|v| {
            let mut chars = v.chars();
            let key: String = chars.by_ref().take_while(|c| c != &'=').collect();
            let value: String = chars.collect();
            let value: f64 = value.parse().map_err(|err| format!("{}", err))?;

            Ok((key, value))
        })
        .collect::<Result<HashMap<String, f64>, String>>()?;

    let tokens = lexer::tokenize(expr.as_str())?;

    let ast = parser::parse(&tokens).ok_or("err")?;

    let result = evaluator::evaluate(&ast, &vars)?;

    println!("{}", result);

    Ok(())
}
