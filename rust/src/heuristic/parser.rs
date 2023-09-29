use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Parser)]
#[grammar = "heuristic/grammar/heuristic.pest"]
struct HeuristicParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeuristicNode {
    Number(i32),
    Terminal(Rule),
    Unary(Rule, Box<HeuristicNode>),
    Binary(Rule, Box<HeuristicNode>, Box<HeuristicNode>),
}

// Pretty printing for heuristics
impl Display for HeuristicNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HeuristicNode::Number(num) => write!(f, "{:?}", num),
            HeuristicNode::Terminal(rule) => write!(f, "{:?}", rule),
            HeuristicNode::Unary(rule, h) => write!(f, "({:?} {})", rule, h),
            HeuristicNode::Binary(rule, h1, h2) => write!(f, "({:?} {} {})", rule, h1, h2),
        }
    }
}

impl Hash for HeuristicNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

pub fn parse_heuristic(input: &str) -> HeuristicNode {
    let result = HeuristicParser::parse(Rule::heuristic, input).unwrap_or_else(|e| panic!("{}", e));
    pairs2struct(result)
}

fn pairs2struct(result: Pairs<Rule>) -> HeuristicNode {
    let mut pairs = result.peek().unwrap().into_inner();
    let operator = pairs.next().unwrap();

    match operator.as_rule() {
        Rule::binary => HeuristicNode::Binary(
            operator.into_inner().next().unwrap().as_rule(),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap()))),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap()))),
        ),
        Rule::unary => HeuristicNode::Unary(
            operator.into_inner().next().unwrap().as_rule(),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap()))),
        ),
        Rule::terminal => HeuristicNode::Terminal(operator.into_inner().next().unwrap().as_rule()),
        Rule::number => HeuristicNode::Number(operator.as_str().parse::<i32>().unwrap()),
        other => {
            unreachable!("{:?}", other)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success_1() {
        let h1 = parse_heuristic("(+ deltaX deltaY)");
        assert_eq!(
            h1,
            HeuristicNode::Binary(
                Rule::plus,
                Box::new(HeuristicNode::Terminal(Rule::deltaX)),
                Box::new(HeuristicNode::Terminal(Rule::deltaY))
            )
        );
    }

    #[test]
    fn test_parse_success_2() {
        let h2 = parse_heuristic("(/ (max deltaX deltaY) (abs x1))");
        assert_eq!(
            h2,
            HeuristicNode::Binary(
                Rule::div,
                Box::new(HeuristicNode::Binary(
                    Rule::max,
                    Box::new(HeuristicNode::Terminal(Rule::deltaX)),
                    Box::new(HeuristicNode::Terminal(Rule::deltaY))
                )),
                Box::new(HeuristicNode::Unary(
                    Rule::abs,
                    Box::new(HeuristicNode::Terminal(Rule::x1))
                ))
            )
        );
    }

    #[test]
    fn test_parse_success_3() {
        let h3 = parse_heuristic("x1");
        assert_eq!(
            h3,
            HeuristicNode::Terminal(Rule::x1)
        );
    }
    
    #[test]
    fn test_parse_success_4() {
        let h4 = parse_heuristic("(+ 1 3)");
        assert_eq!(
            h4,
            HeuristicNode::Binary(
                Rule::plus,
                Box::new(HeuristicNode::Number(1)),
                Box::new(HeuristicNode::Number(3))
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
