use crate::heuristic::parser::Heuristic;
use crate::heuristic::parser::Rule;
use num_complex::{Complex, ComplexFloat};

pub fn evaluate_heuristic(heuristic: &Heuristic, x: i32, y: i32, xg: i32, yg: i32) -> f32 {
    let x_c = Complex::new(x as f32, 0.0);
    let xg_c = Complex::new(xg as f32, 0.0);
    let y_c = Complex::new(y as f32, 0.0);
    let yg_c = Complex::new(yg as f32, 0.0);

    evaluate_heuristic_complex(heuristic, x_c, y_c, xg_c, yg_c).re()
}

fn evaluate_heuristic_complex(
    heuristic: &Heuristic,
    x: Complex<f32>,
    y: Complex<f32>,
    xg: Complex<f32>,
    yg: Complex<f32>,
) -> Complex<f32> {
    match heuristic {
        Heuristic::Terminal(rule) => evaluate_terminal(*rule, x, y, xg, yg),
        Heuristic::Unary(rule, h) => evaluate_unary(*rule, h, x, y, xg, yg),
        Heuristic::Binary(rule, h1, h2) => evaluate_binary(*rule, h1, h2, x, y, xg, yg),
    }
}

// Evaluate terminal nodes
fn evaluate_terminal(
    rule: Rule,
    x: Complex<f32>,
    y: Complex<f32>,
    xg: Complex<f32>,
    yg: Complex<f32>,
) -> Complex<f32> {
    match rule {
        Rule::x1 => x,
        Rule::y1 => y,
        Rule::x2 => xg,
        Rule::y2 => yg,
        Rule::deltaX => Complex::new((x - xg).norm(), 0.0),
        Rule::deltaY => Complex::new((y - yg).norm(), 0.0),
        _ => {
            unreachable!("{:?}", rule);
        }
    }
}

// Evaluate unary operators
fn evaluate_unary(
    rule: Rule,
    h: &Heuristic,
    x: Complex<f32>,
    y: Complex<f32>,
    xg: Complex<f32>,
    yg: Complex<f32>,
) -> Complex<f32> {
    let result = evaluate_heuristic_complex(h, x, y, xg, yg);
    match rule {
        Rule::neg => -result,
        Rule::abs => Complex::new(result.abs(), 0.0),
        Rule::sqrt => result.sqrt(),
        Rule::sqr => result.powi(2),
        _ => {
            unreachable!("{:?}", rule);
        }
    }
}

// Evaluate binary operators
fn evaluate_binary(
    rule: Rule,
    h1: &Heuristic,
    h2: &Heuristic,
    x: Complex<f32>,
    y: Complex<f32>,
    xg: Complex<f32>,
    yg: Complex<f32>,
) -> Complex<f32> {
    let result1 = evaluate_heuristic_complex(h1, x, y, xg, yg);
    let result2 = evaluate_heuristic_complex(h2, x, y, xg, yg);
    match rule {
        Rule::plus => result1 + result2,
        Rule::minus => result1 - result2,
        Rule::mul => result1 * result2,
        Rule::div => result1 / result2,

        // TODO: Correct implementtion
        Rule::max => Complex::new(result1.norm().max(result2.norm()), 0.0),
        Rule::min => Complex::new(result1.norm().max(result2.norm()), 0.0),
        _ => {
            unreachable!("{:?}", rule);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for evaluate_heuristic (terminal)
    #[test]
    fn test_evaluate_x1() {
        let h1 = Heuristic::Terminal(Rule::x1);
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 4), 1.0);
    }

    #[test]
    fn test_evaluate_x2() {
        let h1 = Heuristic::Terminal(Rule::x2);
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 4), 3.0);
    }

    #[test]
    fn test_evaluate_y1() {
        let h1 = Heuristic::Terminal(Rule::y1);
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 4), 2.0);
    }

    #[test]
    fn test_evaluate_y2() {
        let h1 = Heuristic::Terminal(Rule::y2);
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 4), 4.0);
    }

    #[test]
    fn test_evaluate_delta_x() {
        let h1 = Heuristic::Terminal(Rule::deltaX);
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 5), 2.0);
        assert_eq!(evaluate_heuristic(&h1, 3, 2, 1, 5), 2.0);
    }

    #[test]
    fn test_evaluate_delta_y() {
        let h1 = Heuristic::Terminal(Rule::deltaY);
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 5), 3.0);
        assert_eq!(evaluate_heuristic(&h1, 3, 2, 1, 5), 3.0);
    }

    // Tests for evaluate heuristic (unaries)
    #[test]
    fn test_evaluate_neg() {
        let h1 = Heuristic::Unary(Rule::neg, Box::new(Heuristic::Terminal(Rule::x1)));
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 4), -1.0);
    }

    #[test]
    fn test_evaluate_abs() {
        let h1 = Heuristic::Unary(Rule::abs, Box::new(Heuristic::Terminal(Rule::x1)));
        assert_eq!(evaluate_heuristic(&h1, -1, 2, 3, 4), 1.0);
        assert_eq!(evaluate_heuristic(&h1, 1, 2, 3, 4), 1.0);
    }

    // Test some complex number handling
    #[test]
    fn test_sqrt_minus_1() {
        let h1 = Heuristic::Unary(Rule::sqrt, Box::new(Heuristic::Terminal(Rule::x1)));
        assert_eq!(evaluate_heuristic(&h1, -1, 2, 3, 4), 0.0);
    }
}
