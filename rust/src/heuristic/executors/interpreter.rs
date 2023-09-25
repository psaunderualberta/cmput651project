use crate::heuristic::{
    executors::HeuristicExecuter,
    parser::{HeuristicNode, Rule},
    Heuristic,
};

pub struct Interpreter {
    node: HeuristicNode,
}

impl HeuristicExecuter for Interpreter {
    fn create(heuristic: &Heuristic) -> Self {
        Interpreter {
            node: heuristic.root.clone(),
        }
    }

    fn execute(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        let executor = RecursiveExecutor { x1, y1, x2, y2 };
        let val = executor.evaluate_node(&self.node);
        assert!(!val.is_nan());
        val
    }
}

struct RecursiveExecutor {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl RecursiveExecutor {
    fn evaluate_node(&self, node: &HeuristicNode) -> f32 {
        match node {
            HeuristicNode::Terminal(rule) => self.evaluate_terminal(*rule),
            HeuristicNode::Unary(rule, h) => self.evaluate_unary(*rule, h),
            HeuristicNode::Binary(rule, h1, h2) => self.evaluate_binary(*rule, h1, h2),
        }
    }

    fn evaluate_terminal(&self, rule: Rule) -> f32 {
        match rule {
            Rule::x1 => self.x1,
            Rule::y1 => self.y1,
            Rule::x2 => self.x2,
            Rule::y2 => self.y2,
            Rule::deltaX => self.x2 - self.x1,
            Rule::deltaY => self.y2 - self.y1,
            _ => {
                unreachable!("{:?}", rule);
            }
        }
    }

    fn evaluate_unary(&self, rule: Rule, h: &HeuristicNode) -> f32 {
        let result = self.evaluate_node(h);
        match rule {
            Rule::neg => -result,
            Rule::abs => result.abs(),
            Rule::sqrt => result.signum() * result.abs().sqrt(),
            Rule::sqr => result * result,
            _ => {
                unreachable!("{:?}", rule);
            }
        }
    }

    fn evaluate_binary(&self, rule: Rule, h1: &HeuristicNode, h2: &HeuristicNode) -> f32 {
        let result1 = self.evaluate_node(h1);
        let result2 = self.evaluate_node(h2);
        match rule {
            Rule::plus => result1 + result2,
            Rule::minus => result1 - result2,
            Rule::mul => result1 * result2,
            Rule::div => result1 / result2,
            Rule::max => result1.max(result2),
            Rule::min => result1.min(result2),
            _ => {
                unreachable!("{:?}", rule);
            }
        }
    }
}

// pub struct Interpreter {
//     node: HeuristicNode,
// }

// impl HeuristicExecuter for Interpreter {
//     fn create(heuristic: &Heuristic) -> Self {
//         Interpreter {
//             node: heuristic.root.clone(),
//         }
//     }

//     fn execute(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
//         evaluate_node(&self.node, x1, y1, x2, y2)
//     }
// }

// fn evaluate_node(node: &HeuristicNode, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
//     match node {
//         HeuristicNode::Terminal(rule) => evaluate_terminal(*rule, x1, y1, x2, y2),
//         HeuristicNode::Unary(rule, h) => evaluate_unary(*rule, &*h, x1, y1, x2, y2),
//         HeuristicNode::Binary(rule, h1, h2) => evaluate_binary(*rule, &*h1, &*h2, x1, y1, x2, y2),
//     }
// }

// fn evaluate_terminal(rule: Rule, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
//     match rule {
//         Rule::x1 => x1,
//         Rule::y1 => y1,
//         Rule::x2 => x2,
//         Rule::y2 => y2,
//         Rule::deltaX => x2 - x1,
//         Rule::deltaY => y2 - y1,
//         _ => {
//             unreachable!("{:?}", rule);
//         }
//     }
// }

// fn evaluate_unary(rule: Rule, h: &HeuristicNode, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
//     let result = evaluate_node(h, x1, y1, x2, y2);
//     match rule {
//         Rule::neg => -result,
//         Rule::abs => result.abs(),
//         Rule::sqrt => result.signum() * result.abs().sqrt(),
//         Rule::sqr => result * result,
//         _ => {
//             unreachable!("{:?}", rule);
//         }
//     }
// }

// fn evaluate_binary(
//     rule: Rule,
//     h1: &HeuristicNode,
//     h2: &HeuristicNode,
//     x1: f32,
//     y1: f32,
//     x2: f32,
//     y2: f32,
// ) -> f32 {
//     let result1 = evaluate_node(h1, x1, y1, x2, y2);
//     let result2 = evaluate_node(h2, x1, y1, x2, y2);
//     match rule {
//         Rule::plus => result1 + result2,
//         Rule::minus => result1 - result2,
//         Rule::mul => result1 * result2,
//         Rule::div => result1 / result2,
//         Rule::max => result1.max(result2),
//         Rule::min => result1.min(result2),
//         _ => {
//             unreachable!("{:?}", rule);
//         }
//     }
// }
