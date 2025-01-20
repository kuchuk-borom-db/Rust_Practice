use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogEntry;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VisFlowLogPayLoad {
    pub operation_id: String,
    pub operation_name: String,
    //TODO Separate model without operation_id
    pub logs: Vec<VisFlowLogEntry>,
}

#[derive(Deserialize)]
pub struct SaveLogsPayload {
    pub operation: Vec<VisFlowLogPayLoad>,
}
