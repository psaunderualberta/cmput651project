
use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/heuristic.pest"]
struct HeuristicParser;

#[derive(Debug)]
pub enum Heuristic<'a> {
    Terminal(&'a str),
    Unary(&'a str, Box<Heuristic<'a>>),
    Binary(&'a str, Box<Heuristic<'a>>, Box<Heuristic<'a>>)
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
            operator.as_str(),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap()))),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap())))
        ),
        Rule::unary => Heuristic::Unary(
            operator.as_str(),
            Box::new(pairs2struct(Pairs::single(pairs.next().unwrap())))
        ),
        Rule::terminal => Heuristic::Terminal(operator.as_str()),
        other => { unreachable!("{:?}", other) }
    }
}
