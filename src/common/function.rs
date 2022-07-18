use super::{eval::EvaluateResult, token::InfixOperator};

#[derive(Debug)]
pub enum FunctionKind<'a> {
    Literal(&'a str),
    FromOperator(InfixOperator),
}

#[derive(Clone, Copy, Debug)]
pub enum Function {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Pipe,
}

impl Function {
    #[inline]
    fn too_less_args_message(name: &str, args_num: u8, args_got: u8) -> String {
        format!(
            "function {} expectes {} args, got {}",
            name, args_num, args_got
        )
    }
    #[inline]
    fn mismatched_type_message(name: &str, index: u8, received: &str, expected: &str) -> String {
        format!(
            "function {} received {} as argument at index {}, expected {}",
            name, received, index, expected
        )
    }

    #[inline]
    fn binary_op(
        args: &Vec<EvaluateResult>,
        name: &str,
        operation: fn(f64, f64) -> f64,
    ) -> Result<f64, String> {
        match args.get(0).ok_or(Self::too_less_args_message(name, 2, 0))? {
            EvaluateResult::Fn { .. } => {
                Err(Self::mismatched_type_message(name, 0, "function", "number"))
            }
            EvaluateResult::Val(first) => {
                match args.get(1).ok_or(Self::too_less_args_message(name, 2, 1))? {
                    EvaluateResult::Fn { .. } => {
                        Err(Self::mismatched_type_message(name, 1, "function", "number"))
                    }
                    EvaluateResult::Val(second) => Ok(operation(*first, *second)),
                }
            }
        }
    }

    pub fn call(&self, args: &Vec<EvaluateResult>) -> Result<f64, String> {
        //TODO check for too many args
        match self {
            Function::Add => Self::binary_op(args, Function::Add.get_name(), |a, b| a + b),
            Function::Sub => Self::binary_op(args, Function::Sub.get_name(), |a, b| a - b),
            Function::Mul => Self::binary_op(args, Function::Mul.get_name(), |a, b| a * b),
            Function::Div => Self::binary_op(args, Function::Div.get_name(), |a, b| a / b),
            Function::Mod => Self::binary_op(args, Function::Mod.get_name(), |a, b| a % b),
            Function::Pow => Self::binary_op(args, Function::Pow.get_name(), |a, b| a.powf(b)),
            Function::Pipe => match args.get(0).ok_or(Self::too_less_args_message(
                Function::Pipe.get_name(),
                2,
                0,
            ))? {
                EvaluateResult::Fn { .. } => Err(Self::mismatched_type_message(
                    Function::Pipe.get_name(),
                    0,
                    "function",
                    "number",
                )),
                EvaluateResult::Val(first) => {
                    match args.get(1).ok_or(Self::too_less_args_message(
                        Function::Pipe.get_name(),
                        2,
                        1,
                    ))? {
                        EvaluateResult::Val(_) => Err(Self::mismatched_type_message(
                            Function::Pipe.get_name(),
                            1,
                            "number",
                            "function",
                        )),
                        EvaluateResult::Fn { ref args, kind } => {
                            let mut args = args.to_owned();

                            args.push(EvaluateResult::Val(*first));

                            kind.call(&args)
                        }
                    }
                }
            },
        }
    }

    pub fn from_operator(operator: &InfixOperator) -> Function {
        match operator {
            InfixOperator::Addition => Function::Add,
            InfixOperator::Subtraction => Function::Sub,
            InfixOperator::Multiplication => Function::Mul,
            InfixOperator::Division => Function::Div,
            InfixOperator::Modulo => Function::Mod,
            InfixOperator::Exponentiation => Function::Pow,
            InfixOperator::Pipe => Function::Pipe,
        }
    }

    pub fn from_literal(str: &str) -> Option<Function> {
        match str {
            "add" => Some(Function::Add),
            "sub" => Some(Function::Sub),
            "mul" => Some(Function::Mul),
            "div" => Some(Function::Div),
            "mod" => Some(Function::Mod),
            "pow" => Some(Function::Pow),
            "pipe" => Some(Function::Pipe),
            _ => None,
        }
    }
    pub fn get_name(&self) -> &str {
        match self {
            Function::Add => "add",
            Function::Sub => "sub",
            Function::Mul => "mul",
            Function::Div => "div",
            Function::Mod => "mod",
            Function::Pow => "pow",
            Function::Pipe => "pipe",
        }
    }

    pub fn args_num(&self) -> u8 {
        // currently all functions have 2 args
        2
    }
}
