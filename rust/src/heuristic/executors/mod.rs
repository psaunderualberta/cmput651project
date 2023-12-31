pub mod interpreter;
pub mod jit;

use crate::heuristic::Heuristic;

pub trait HeuristicExecuter {
    fn create(heuristic: &Heuristic) -> Self;

    fn execute(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32;
}

#[cfg(test)]
mod tests {
    use super::interpreter::Interpreter;
    use super::jit::Jit;
    use super::HeuristicExecuter;
    use crate::heuristic::parser::{HeuristicNode, Rule};
    use crate::heuristic::Heuristic;

    use test_case::test_case;

    #[test_case ( HeuristicNode::Number(1), (1.0, 2.0, 3.0, 4.0), 1.0)]
    #[test_case ( HeuristicNode::Number(9), (1.0, 2.0, 3.0, 4.0), 9.0)]
    #[test_case( HeuristicNode::Terminal(Rule::x1), (1.0, 2.0, 3.0, 4.0), 1.0)]
    #[test_case( HeuristicNode::Terminal(Rule::x2), (1.0, 2.0, 3.0, 4.0), 3.0)]
    #[test_case( HeuristicNode::Terminal(Rule::y1), (1.0, 2.0, 3.0, 4.0), 2.0)]
    #[test_case( HeuristicNode::Terminal(Rule::y2), (1.0, 2.0, 3.0, 4.0), 4.0)]
    #[test_case( HeuristicNode::Terminal(Rule::deltaX), (1.0, 2.0, 3.0, 5.0), 2.0)]
    #[test_case( HeuristicNode::Terminal(Rule::deltaX), (3.0, 2.0, 1.0, 5.0), 2.0)]
    #[test_case( HeuristicNode::Terminal(Rule::deltaY), (1.0, 2.0, 3.0, 5.0), 3.0)]
    #[test_case( HeuristicNode::Terminal(Rule::deltaY), (3.0, 2.0, 1.0, 5.0), 3.0)]
    #[test_case( HeuristicNode::Unary(Rule::neg, Box::new(HeuristicNode::Terminal(Rule::x1))), (1.0, 2.0, 3.0, 4.0), -1.0)]
    #[test_case( HeuristicNode::Unary(Rule::abs, Box::new(HeuristicNode::Terminal(Rule::x1))), (1.0, 2.0, 3.0, 4.0), 1.0)]
    #[test_case( HeuristicNode::Unary(Rule::sqrt, Box::new(HeuristicNode::Terminal(Rule::x1))), (1.0, 2.0, 3.0, 4.0), 1.0)]
    #[test_case( HeuristicNode::Unary(Rule::sqrt, Box::new(HeuristicNode::Unary(Rule::neg, Box::new(HeuristicNode::Terminal(Rule::y2))))), (1.0, 2.0, 3.0, 4.0), -2.0)]
    #[test_case( HeuristicNode::Binary(Rule::plus, Box::new(HeuristicNode::Terminal(Rule::x1)), Box::new(HeuristicNode::Terminal(Rule::x2))), (1.0, 2.0, 3.0, 4.0), 4.0)]
    #[test_case( HeuristicNode::Binary(Rule::minus, Box::new(HeuristicNode::Terminal(Rule::x1)), Box::new(HeuristicNode::Terminal(Rule::x2))), (1.0, 2.0, 3.0, 4.0), -2.0)]
    #[test_case( HeuristicNode::Binary(Rule::mul, Box::new(HeuristicNode::Terminal(Rule::x1)), Box::new(HeuristicNode::Terminal(Rule::y1))), (1.0, 2.0, 3.0, 4.0), 2.0)]
    #[test_case( HeuristicNode::Binary(Rule::div, Box::new(HeuristicNode::Terminal(Rule::x1)), Box::new(HeuristicNode::Terminal(Rule::y1))), (1.0, 2.0, 3.0, 4.0), 0.5)]
    #[test_case( HeuristicNode::Binary(Rule::max, Box::new(HeuristicNode::Terminal(Rule::x1)), Box::new(HeuristicNode::Terminal(Rule::y2))), (1.0, 2.0, 3.0, 4.0), 4.0)]
    #[test_case( HeuristicNode::Binary(Rule::min, Box::new(HeuristicNode::Terminal(Rule::x1)), Box::new(HeuristicNode::Terminal(Rule::y2))), (1.0, 2.0, 3.0, 4.0), 1.0)]
    fn test_evaluate_heuristic(
        heuristic: HeuristicNode,
        (x1, y1, x2, y2): (f32, f32, f32, f32),
        expected: f32,
    ) {
        let heuristic = Heuristic::new(heuristic);
        {
            let interpreter = Interpreter::create(&heuristic);
            let result = interpreter.execute(x1, y1, x2, y2);
            assert_eq!(result, expected);
        }

        {
            let context = inkwell::context::Context::create();
            let jit = Jit::create(&heuristic, &context);
            let result = jit.execute(x1, y1, x2, y2);
            assert_eq!(result, expected);
        }
    }
}
