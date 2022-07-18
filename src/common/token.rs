#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Bracket {
    Open,
    Close,
}

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub enum Token<'a> {
    InfixOperator(InfixOperator),
    Bracket(Bracket),
    Identificator(&'a str),
    Literal(f64),
    Comma,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InfixOperator {
    Pipe,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
}

impl InfixOperator {
    pub fn associativity(&self) -> Associativity {
        match self {
            InfixOperator::Pipe => Associativity::Left,
            InfixOperator::Addition => Associativity::Left,
            InfixOperator::Subtraction => Associativity::Left,
            InfixOperator::Multiplication => Associativity::Left,
            InfixOperator::Division => Associativity::Left,
            InfixOperator::Modulo => Associativity::Left,
            InfixOperator::Exponentiation => Associativity::Right,
        }
    }
}
