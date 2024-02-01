use pest::iterators::Pair;

use crate::error::print_error;

use super::{
    ast::{
        ActionCall, ActionDefinition, Arguments, DictionaryType, FunctionCall, Identifier, Position,
    },
    parse_arguments::parse_arguments,
    parse_types::parse_dictionary_type,
    Rule,
};

pub fn parse_function_call(statement_part: Pair<Rule>) -> FunctionCall {
    let mut function_call = FunctionCall {
        function_name: Identifier::new(),
        arguments: Arguments::new_blank(),
        position: Position::from(statement_part.line_col()),
    };

    for function_call_part in statement_part.into_inner() {
        match function_call_part.as_rule() {
            Rule::identifier => {
                function_call.function_name = function_call_part.as_str().to_owned();
            }
            Rule::arguments => {
                function_call.arguments = parse_arguments(function_call_part);
            }
            _ => unreachable!(),
        }
    }

    function_call
}

pub fn parse_action_call(statement_part: Pair<Rule>) -> ActionCall {
    let mut action_call = ActionCall {
        action_name: Identifier::new(),
        arguments: Arguments::new_blank(),
        position: Position::from(statement_part.line_col()),
    };

    for action_call_part in statement_part.into_inner() {
        match action_call_part.as_rule() {
            Rule::identifier => {
                action_call.action_name = action_call_part.as_str().to_owned();
            }
            Rule::arguments => {
                action_call.arguments = parse_arguments(action_call_part);
            }
            _ => unreachable!(),
        }
    }

    action_call
}

pub fn parse_action_definition(
    statement_part: Pair<Rule>,
    action_definitions: &[ActionDefinition],
) -> ActionDefinition {
    let mut action_definition = ActionDefinition {
        action_identifier: Identifier::new(),
        action_id: Identifier::new(),
        action_arguments: DictionaryType::new(Position::new(0, 0)),
        position: Position::from(statement_part.line_col()),
    };

    for action_definition_part in statement_part.into_inner() {
        match action_definition_part.as_rule() {
            Rule::identifier => {
                if action_definitions
                    .iter()
                    .any(|m| m.action_identifier == action_definition_part.as_str())
                {
                    // TODO:TODO: Implement proper error handling!
                    print_error(format!(
                        "Action \"{}\" is defined multiple times!",
                        action_definition_part.as_str()
                    ));
                }

                action_definition.action_identifier = action_definition_part.as_str().to_owned();
            }
            Rule::action_body => {
                for action_body_part in action_definition_part.into_inner() {
                    match action_body_part.as_rule() {
                        Rule::action_id => {
                            action_definition.action_id = action_body_part
                                .into_inner()
                                .next()
                                .unwrap()
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str()
                                .to_owned();
                        }
                        Rule::action_parameters => {
                            let entries = action_body_part.into_inner().next().unwrap();

                            action_definition.action_arguments = parse_dictionary_type(entries);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    action_definition
}
