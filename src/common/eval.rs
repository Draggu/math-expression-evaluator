use super::function::Function;

#[derive(Clone, Debug)]
pub enum EvaluateResult {
    Val(f64),
    Fn {
        kind: Function,
        args: Vec<EvaluateResult>,
    },
}
