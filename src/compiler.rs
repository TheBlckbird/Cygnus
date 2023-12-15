use crate::{error::error, parser::ast::*};
use plist::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
pub struct Workflow {
    pub w_f_workflow_minimum_client_version_string: String,
    pub w_f_workflow_minimum_client_version: i16,
    pub w_f_workflow_icon: WorkflowIcon,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
pub struct WorkflowAction {
    pub w_f_workflow_action_identifier: String,
    pub w_f_workflow_action_parameters: Parameters,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
pub struct Parameters {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
pub struct WorkflowIcon {
    w_f_workflow_icon_start_color: i64,
    w_f_workflow_glyph_number: i64,
}

pub fn compiler(ast: Ast) -> Workflow {
    let mut workflow = Workflow {
        w_f_workflow_minimum_client_version_string: "900".to_owned(),
        w_f_workflow_minimum_client_version: 900,
        w_f_workflow_icon: WorkflowIcon {
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

    for node in ast.nodes {
        match node {
            Statement::FunctionCall(function_call) => match function_call.function_name.as_str() {
                "print" => match &function_call.arguments[0] {
                    Argument::String(string) => {
                        workflow.w_f_workflow_actions.push(WorkflowAction {
                            w_f_workflow_action_identifier: "is.workflow.actions.showresult"
                                .to_owned(),
                            w_f_workflow_action_parameters: Parameters {
                                text: Some(string.clone()),
                            },
                        })
                    }
                    _ => unreachable!(),
                },
                _ => error(format!(
                    "Unknown function \"{}\"",
                    function_call.function_name
                )),
            },
            Statement::MacroCall(macro_call) => {
                // println!("{macro_call:#?}");
            }
            Statement::MacroDefinition(macro_definition) => {}
            _ => {}
        }
    }

    workflow
}
