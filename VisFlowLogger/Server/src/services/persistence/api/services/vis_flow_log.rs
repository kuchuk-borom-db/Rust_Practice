use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogModel;
use crate::services::persistence::internal::application::vis_flow_log_impl::VisFlowLogImpl;

pub trait VisFlowLog {
    async fn save_log(&self, logs: Vec<VisFlowLogModel>) -> bool;
}
pub async fn new() -> impl VisFlowLog {
    VisFlowLogImpl::new().await
}
