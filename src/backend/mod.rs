use crate::{error::print_error, frontend::ast::*};
use plist::{Dictionary, Value};
use serde::{Deserialize, Serialize};

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
                print_error("Functions are currently not available".to_owned());
            }
            Statement::ActionCall(action_call) => {
                let mut action_definition: Option<&ActionDefinition> = None;

                for (index, action_) in ast.actions.iter().enumerate() {
                    if action_.action_identifier == action_call.action_name {
                        action_definition = Some(action_);
                        break;
                    }

                    if index == ast.actions.len() - 1 {
                        print_error(format!("Unknown action \"{}\"", action_call.action_name));
                    }
                }

                let mut action = Action::new(&action_definition.unwrap().action_id);

                for (index, argument) in action_call.arguments.iter().enumerate() {
                    match argument {
                        Argument::String(string, _) => {
                            let (expected_argument_identifier, expected_type, _) =
                                &action_definition.unwrap().action_arguments.entries[index];

                            if argument.as_type() == *expected_type {
                                action.parameters.insert(
                                    expected_argument_identifier.to_string(),
                                    Value::String(string.to_string()),
                                );
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
