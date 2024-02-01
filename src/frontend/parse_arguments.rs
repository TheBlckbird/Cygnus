use super::{
    ast::{Arguments, Position},
    Argument,
};
use crate::frontend::Rule;
use pest::iterators::Pair;

pub fn parse_arguments(call_part: Pair<Rule>) -> Arguments {
    let mut arguments = Arguments::new(Position::from(call_part.line_col()));

    for argument in call_part.into_inner() {
        match argument.as_rule() {
            Rule::string => {
                let position = Position::from(argument.line_col());

                arguments.arguments.push(Argument::String(
                    argument.into_inner().next().unwrap().as_str().to_owned(),
                    position,
                ));
            }
            _ => unreachable!(),
        }
    }

    arguments
}
