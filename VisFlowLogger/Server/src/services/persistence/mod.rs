use crate::services::persistence::api::services::vis_flow_log::VisFlowLog;
use crate::services::persistence::api::services::vis_flow_op::VisFlowOp;
use std::sync::Arc;

pub mod api;
mod internal;
#[derive(Clone)]
pub struct AvailableServices {
    pub vis_flow_log: Arc<Box<dyn VisFlowLog>>,
    pub vis_flow_op: Arc<Box<dyn VisFlowOp>>,
}
/*
Why we need Arc:

In Actix, each request is handled in a separate thread
When a request comes in, the server needs to clone the app state for that thread
Box cannot be cloned (it's single-owner)
Arc (Atomic Reference Counting) allows multiple threads to share the same data safely
Arc keeps track of how many references exist and only drops the data when all references are gone
*/
