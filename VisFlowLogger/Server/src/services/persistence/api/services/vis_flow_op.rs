use crate::services::persistence::internal::application::vis_flow_op_impl::VisLogOpImpl;
use async_trait::async_trait;

#[async_trait]
pub trait VisFlowOp: Send + Sync {
    ///If operation doesn't exist creates a new one. If exists, updates the updated timestamp
    async fn upsert(&self, operation_id: &str) -> bool;
}
pub async fn new() -> impl VisFlowOp {
    VisLogOpImpl::new().await
}
