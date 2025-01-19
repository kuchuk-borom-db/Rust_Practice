use crate::services::diagram_generator::api::services::DiagramGenerator;
use std::sync::Arc;

//JointJS, Mermaid, GoJS
pub mod api;
mod internal;

#[derive(Clone)]
pub struct AvailableServices {
    pub(crate) mermaid: Arc<dyn DiagramGenerator>,
}
