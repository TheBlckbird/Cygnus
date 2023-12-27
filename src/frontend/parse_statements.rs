use pest::iterators::Pair;

use crate::error::error;

use super::{
    ast::{DictionaryType, FunctionCall, Identifier, MacroCall, MacroDefinition},
    parse_arguments::parse_arguments,
    parse_types::parse_dictionary_type,
    Rule,
};

pub fn parse_function_call(statement_part: Pair<Rule>) -> FunctionCall {
    let mut function_call = FunctionCall {
        function_name: String::new(),
        arguments: Vec::new(),
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

pub fn parse_macro_call(statement_part: Pair<Rule>) -> MacroCall {
    let mut macro_call = MacroCall {
        macro_name: String::new(),
        arguments: Vec::new(),
    };

    for macro_call_part in statement_part.into_inner() {
        match macro_call_part.as_rule() {
            Rule::identifier => {
                macro_call.macro_name = macro_call_part.as_str().to_owned();
            }
            Rule::arguments => {
                macro_call.arguments = parse_arguments(macro_call_part);
            }
            _ => unreachable!(),
        }
    }

    macro_call
}

pub fn parse_macro_definition(
    statement_part: Pair<Rule>,
    macro_definitions: &[MacroDefinition],
) -> MacroDefinition {
    let mut macro_definition = MacroDefinition {
        macro_identifier: Identifier::new(),
        action_id: String::new(),
        action_arguments: DictionaryType::new(),
    };

    for macro_definition_part in statement_part.into_inner() {
        match macro_definition_part.as_rule() {
            Rule::identifier => {
                if macro_definitions
                    .iter()
                    .any(|m| m.macro_identifier == macro_definition_part.as_str())
                {
                    // TODO:TODO: Implement proper error handling!
                    error(format!(
                        "Macro \"{}\" is defined multiple times!",
                        macro_definition_part.as_str()
                    ));
                }

                macro_definition.macro_identifier = macro_definition_part.as_str().to_owned();
            }
            Rule::macro_body => {
                for macro_body_part in macro_definition_part.into_inner() {
                    match macro_body_part.as_rule() {
                        Rule::macro_action_id => {
                            macro_definition.action_id = macro_body_part
                                .into_inner()
                                .next()
                                .unwrap()
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str()
                                .to_owned();
                        }
                        Rule::macro_action_parameters => {
                            let entries = macro_body_part.into_inner().next().unwrap();

                            macro_definition.action_arguments = parse_dictionary_type(entries);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    macro_definition
}
