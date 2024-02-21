mod arguments_match_definition;

use super::ast::*;
use crate::error::print_error;
use arguments_match_definition::arguments_match_definition;
use thiserror::Error;

pub fn type_checker(ast: &Ast) -> Result<(), Vec<Box<TypeCheckerError>>> {
    let mut errors = vec![];

    for node in &ast.nodes {
        match node {
            Statement::FunctionCall(_) => {
                print_error("Functions are currently not available".to_owned());
            }
            Statement::ActionCall(action_call) => {
                let action_definition_index = ast
                    .actions
                    .iter()
                    .position(|item| item.action_identifier == action_call.action_name);

                let action_definition = match action_definition_index {
                    Some(index) => &ast.actions[index],

                    None => {
                        errors.push(Box::new(TypeCheckerError::UnknownAction {
                            identifier: action_call.action_name.to_owned(),
                            position: action_call.position,
                        }));
                        continue;
                    }
                };

                let arguments_result = arguments_match_definition(
                    &action_call.arguments,
                    &action_call.position,
                    &action_definition.action_arguments,
                );

                if let Err(argument_errors) = arguments_result {
                    errors.extend(argument_errors);
                }
            }
        }
    }

    if errors.is_empty() {
        return Ok(());
    }

    Err(errors)
}

#[derive(Error, Debug)]
pub enum TypeCheckerError {
    #[error("Unexpected type {given_type}, expected type {expected_type} at position {position}")]
    UnexpectedType {
        given_type: Type,
        expected_type: Type,
        position: Position,
    },

    #[error("Missing argument \"{}\" with type {} at position {position}", missing_argument.0, missing_argument.1)]
    MissingArgument {
        missing_argument: (String, Type),
        position: Position,
    },

    #[error("Unnecessary argument at position {position}")]
    UnnecessaryArgument { position: Position },

    #[error("Unknown action \"{identifier}\" at position {position}")]
    UnknownAction {
        identifier: String,
        position: Position,
    },
}
