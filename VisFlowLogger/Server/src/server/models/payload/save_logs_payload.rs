use serde::Deserialize;
use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogEntry;

#[derive(Deserialize)]
pub struct VisFlowLogPayLoad {
    pub operation_id: String,
    //TODO Separate model without operation_id
    pub logs: Vec<VisFlowLogEntry>,
}

#[derive(Deserialize)]
pub struct SaveLogsPayload {
    pub operation: Vec<VisFlowLogPayLoad>,
}
