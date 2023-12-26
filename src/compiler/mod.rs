mod arguments_match_definition;

use std::borrow::Borrow;

use crate::{error::error, parser::ast::*};
use clap::builder::Str;
use plist::{Dictionary, Value};
use serde::{Deserialize, Serialize};

use self::arguments_match_definition::arguments_match_definition;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
pub struct Workflow {
    pub w_f_workflow_minimum_client_version_string: String,
    pub w_f_workflow_minimum_client_version: i16,
    pub w_f_workflow_icon: Icon,
    pub w_f_workflow_client_version: String,
    pub w_f_workflow_output_content_item_classes: Vec<Value>,
    pub w_f_workflow_has_output_fallback: bool,
    pub w_f_workflow_actions: Vec<WorkflowAction>,
    pub w_f_workflow_input_content_item_classes: Vec<String>,
    pub w_f_workflow_import_questions: Vec<Value>,
    pub w_f_workflow_types: Vec<Value>,
    pub w_f_quick_action_surfaces: Vec<Value>,
    pub w_f_workflow_has_shortcut_input_variables: bool,
}
//////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
pub struct WorkflowAction {
    pub w_f_workflow_action_identifier: String,
    pub w_f_workflow_action_parameters: Dictionary,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Action {
    pub identifier: String,
    pub parameters: Dictionary,
}

impl Action {
    pub fn new(identifier: &str) -> Self {
        Action {
            identifier: identifier.to_owned(),
            parameters: Dictionary::new(),
        }
    }

    pub fn convert(self) -> WorkflowAction {
        WorkflowAction {
            w_f_workflow_action_identifier: self.identifier,
            w_f_workflow_action_parameters: self.parameters,
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub enum Parameter {
    Text(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
pub struct Icon {
    w_f_workflow_icon_start_color: i64,
    w_f_workflow_glyph_number: i64,
}

pub fn compiler(ast: Ast) -> Workflow {
    let mut workflow = Workflow {
        w_f_workflow_minimum_client_version_string: "900".to_owned(),
        w_f_workflow_minimum_client_version: 900,
        w_f_workflow_icon: Icon {
            w_f_workflow_icon_start_color: 946986751,
            w_f_workflow_glyph_number: 61440,
        },
        w_f_workflow_client_version: "2196.0.3".to_owned(),
        w_f_workflow_output_content_item_classes: Vec::new(),
        w_f_workflow_has_output_fallback: false,
        w_f_workflow_actions: Vec::new(),
        w_f_workflow_input_content_item_classes: Vec::new(),
        w_f_workflow_import_questions: Vec::new(),
        w_f_workflow_types: Vec::new(),
        w_f_quick_action_surfaces: Vec::new(),
        w_f_workflow_has_shortcut_input_variables: false,
    };

    for node in &ast.nodes {
        match node {
            Statement::FunctionCall(function_call) => {
                error("Functions are currently not available".to_owned());
            }
            Statement::MacroCall(macro_call) => {
                let mut macro_definition: Option<&MacroDefinition> = None;

                for (index, macro_) in ast.macros.iter().enumerate() {
                    if macro_.macro_identifier == macro_call.macro_name {
                        macro_definition = Some(macro_);
                        break;
                    }

                    if index == ast.macros.len() - 1 {
                        error(format!("Unknown macro \"{}\"", macro_call.macro_name));
                    }
                }

                if !arguments_match_definition(
                    &macro_call.arguments,
                    &macro_definition.unwrap().action_arguments,
                ) {
                    error(format!(
                        "Macro \"{}\" has wrong arguments",
                        macro_call.macro_name
                    ));
                }

                let mut action = Action::new(&macro_definition.unwrap().action_id);

                for (index, argument) in macro_call.arguments.iter().enumerate() {
                    match argument {
                        Argument::String(string) => {
                            let (expected_argument_identifier, expected_type) =
                                &macro_definition.unwrap().action_arguments.entries[index];

                            if argument.as_type() == *expected_type {
                                action.parameters.insert(
                                    expected_argument_identifier.clone(),
                                    Value::String(string.clone()),
                                );
                            } else {
                                // TODO: Move this into the type checker
                                error(format!(
                                    "Wrong argument type, expected type {expected_type}, got {}",
                                    argument.as_type()
                                ));
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                workflow.w_f_workflow_actions.push(action.convert());
            }
        }
    }

    workflow
}
