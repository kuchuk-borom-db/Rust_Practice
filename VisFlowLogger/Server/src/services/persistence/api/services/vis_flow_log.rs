use crate::services::persistence::api::model::vis_flow_log_model::{
    VisFlowLogEntity, VisFlowLogEntry,
};
use crate::services::persistence::internal::application::vis_flow_log_impl::VisFlowLogImpl;
use async_trait::async_trait;

#[async_trait]
pub trait VisFlowLog: Send + Sync {
    ///Save log to database
    async fn save_log(&self, logs: &Vec<&VisFlowLogEntry>) -> bool;
    async fn get_logs_by_operation_id(
        &self,
        operation_id: String,
    ) -> Result<Vec<VisFlowLogEntity>, String>;
}
pub async fn new() -> impl VisFlowLog {
    VisFlowLogImpl::new().await
}
