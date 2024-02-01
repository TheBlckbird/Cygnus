use std::cmp::Ordering;

use crate::frontend::ast::{Arguments, AsType, DictionaryType};

use super::TypeCheckerError;

pub fn arguments_match_definition(
    call_arguments: &Arguments,
    definition_parameters: &DictionaryType,
) -> Result<(), Vec<Box<TypeCheckerError>>> {
    let mut errors = vec![];

    match call_arguments.len().cmp(&definition_parameters.len()) {
        Ordering::Greater =>
        {
            #[allow(clippy::needless_range_loop)]
            for i in definition_parameters.len()..call_arguments.len() {
                errors.push(Box::new(TypeCheckerError::UnnecessaryArgument {
                    position: *call_arguments.arguments[i].position(),
                }));
            }
        }
        Ordering::Less =>
        {
            #[allow(clippy::needless_range_loop)]
            for i in call_arguments.len()..definition_parameters.len() {
                let position = match call_arguments.arguments.last() {
                    Some(item) => *item.position(),
                    None => call_arguments.position,
                };

                errors.push(Box::new(TypeCheckerError::MissingArgument {
                    missing_argument: (
                        definition_parameters.entries[i].0.to_owned(),
                        definition_parameters.entries[i].1.clone(),
                    ),
                    position,
                }));
            }
        }
        _ => {}
    }

    for (index, argument) in call_arguments.iter().enumerate() {
        if definition_parameters.len() == index {
            break;
        }

        if argument.as_type() != definition_parameters.entries[index].1 {
            errors.push(Box::new(TypeCheckerError::UnexpectedType {
                given_type: argument.as_type(),
                expected_type: definition_parameters.entries[index].1.clone(),
                position: *argument.position(),
            }))
        }
    }

    if errors.is_empty() {
        return Ok(());
    }

    Err(errors)
}
