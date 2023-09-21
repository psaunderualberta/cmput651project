use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;
use std::fmt::Display;

#[derive(Parser)]
#[grammar = "heuristic/grammar/heuristic.pest"]
struct HeuristicParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Heuristic {
    Terminal(Rule),
    Unary(Rule, Box<Heuristic>),
    Binary(Rule, Box<Heuristic>, Box<Heuristic>)
}

// Pretty printing for heuristics
impl Display for Heuristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heuristic::Terminal(rule) => write!(f, "{:?}", rule),
            Heuristic::Unary(rule, h) => write!(f, "({:?} {})", rule, h),
            Heuristic::Binary(rule, h1, h2) => write!(f, "({:?} {} {})", rule, h1, h2)
        }
    }
}

pub fn parse_heuristic(input: &str) -> Heuristic {
    let result = HeuristicParser::parse(Rule::heuristic, input).unwrap_or_else(|e| panic!("{}", e));
    pairs2struct(result)
}

fn pairs2struct(result: Pairs<Rule>) -> Heuristic {
    let mut pairs = result.peek().unwrap().into_inner();
    let operator = pairs.next().unwrap();

    match operator.as_rule() {
        Rule::binary => Heuristic::Binary(
            operator.into_inner().next().unwrap().as_rule(),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap()))),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap())))
        ),
        Rule::unary => Heuristic::Unary(
            operator.into_inner().next().unwrap().as_rule(),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap())))
        ),
        Rule::terminal => Heuristic::Terminal(operator.into_inner().next().unwrap().as_rule()),
        other => { unreachable!("{:?}", other) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success_1() {
        let h1 = parse_heuristic("(+ deltaX deltaY)");
        assert_eq!(h1,
            Heuristic::Binary(
                Rule::plus, 
                Box::new(
                    Heuristic::Terminal(Rule::deltaX)
                ),
                Box::new(
                    Heuristic::Terminal(Rule::deltaY)
                )
            )
        );
    }

    #[test]
    fn test_parse_success_2() {
        let h2 = parse_heuristic("(/ (max deltaX deltaY) (abs x1))");
        assert_eq!(h2,
            Heuristic::Binary(
                Rule::div, 
                Box::new(
                    Heuristic::Binary(
                        Rule::max, 
                        Box::new(
                            Heuristic::Terminal(Rule::deltaX)
                        ),
                        Box::new(
                            Heuristic::Terminal(Rule::deltaY)
                        )
                    )
                ),
                Box::new(
                    Heuristic::Unary(
                        Rule::abs, 
                        Box::new(
                            Heuristic::Terminal(Rule::x1)
                        )
                    )
                )
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_failure_1() {
        parse_heuristic("(+ deltaX)");
    }

    #[test]
    #[should_panic]
    fn test_parse_failure_2() {
        parse_heuristic("(/ (max deltaX deltaY) (abs x1 y2))");
    }

    #[test]
    #[should_panic]
    fn test_parse_failure_3() {
        parse_heuristic("(/ (max deltaX deltaY) ())");
    }
}
