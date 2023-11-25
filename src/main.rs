use new_action::{new_action, ActionType};
use plist::{from_file, to_file_binary, Dictionary, Integer, Value};
use serde::{Deserialize, Serialize};

mod new_action;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
struct Workflow {
    w_f_workflow_minimum_client_version_string: String,
    w_f_workflow_minimum_client_version: Integer,
    w_f_workflow_icon: Dictionary,
    w_f_workflow_client_version: String,
    w_f_workflow_output_content_item_classes: Vec<Value>,
    w_f_workflow_has_output_fallback: bool,
    w_f_workflow_actions: Vec<WorkflowAction>,
    w_f_workflow_input_content_item_classes: Vec<String>,
    w_f_workflow_import_questions: Vec<Value>,
    w_f_workflow_types: Vec<Value>,
    w_f_quick_action_surfaces: Vec<Value>,
    w_f_workflow_has_shortcut_input_variables: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
struct WorkflowAction {
    w_f_workflow_action_identifier: String,
    w_f_workflow_action_parameters: Parameters,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
struct Parameters {
    text: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut plist: Workflow = from_file("dev_assets/modified.shortcut")
        .expect("The file you provided is not a valid shortcut");

    for i in 1..6 {
        plist
            .w_f_workflow_actions
            .push(new_action(ActionType::ShowResult {
                text: format!("Hello {}!", i),
            }));
    }

    to_file_binary::<_, Workflow>("dev_assets/new.shortcut", &plist)?;

    Ok(())
}
