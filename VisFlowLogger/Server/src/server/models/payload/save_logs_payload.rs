use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogModel;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VisFlowLogPayLoad {
    pub operation_id: String,
    //TODO Separate model without operation_id
    pub logs: Vec<VisFlowLogModel>,
}

#[derive(Deserialize)]
pub struct SaveLogsPayload {
    pub operation: Vec<VisFlowLogPayLoad>,
}
