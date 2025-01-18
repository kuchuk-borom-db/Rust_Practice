use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogModel;
use crate::services::persistence::internal::application::vis_flow_log_impl::VisFlowLogImpl;
use async_trait::async_trait;

#[async_trait]
pub trait VisFlowLog: Send + Sync {
    ///Save log to database
    async fn save_log(&self, logs: &Vec<VisFlowLogModel>) -> bool;
    async fn get_logs_by_operation_id(
        &self,
        operation_id: String,
    ) -> Result<Vec<VisFlowLogModel>, String>;
}
pub async fn new() -> impl VisFlowLog {
    VisFlowLogImpl::new().await
}

/*
Why we need Box:

First, trait objects (dyn VisFlowLog) are "dynamically sized types" (DSTs)
Rust needs to know the size of all types at compile time to allocate memory on the stack
With trait objects, the concrete type implementing the trait could be different sizes
Box solves this by putting the trait object on the heap, where size can be dynamic
The Box itself has a fixed size (a pointer) that Rust can work with on the stack

*/
