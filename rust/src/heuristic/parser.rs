use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "heuristic/heuristic.pest"]
struct HeuristicParser;

#[derive(Debug, Clone)]
pub enum Heuristic {
    Terminal(Rule),
    Unary(Rule, Box<Heuristic>),
    Binary(Rule, Box<Heuristic>, Box<Heuristic>)
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
