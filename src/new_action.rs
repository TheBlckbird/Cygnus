use crate::{Parameters, WorkflowAction};

pub enum ActionType {
    ShowResult { text: String },
}

pub fn new_action(action_type: ActionType) -> WorkflowAction {
    match action_type {
        ActionType::ShowResult { text } => WorkflowAction {
            w_f_workflow_action_identifier: "is.workflow.actions.showresult".to_owned(),
            w_f_workflow_action_parameters: Parameters { text: Some(text) },
        },
    }
}
