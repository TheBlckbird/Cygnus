use super::Argument;
use crate::parser::Rule;
use pest::iterators::Pair;

pub fn parse_arguments(call_part: Pair<Rule>) -> Vec<Argument> {
    let mut arguments = Vec::new();

    for argument in call_part.into_inner() {
        match argument.as_rule() {
            Rule::string => {
                arguments.push(Argument::String(
                    argument.into_inner().next().unwrap().as_str().to_owned(),
                ));
            }
            _ => unreachable!(),
        }
    }

    arguments
}
