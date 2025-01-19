use crate::services::graph_generator::api::services::graph_generator::GraphGenerator;
use std::sync::Arc;

pub mod api;
mod internal;

#[derive(Clone)]
pub struct AvailableServices {
    //Arc = multi owner just like ReferenceCount but thread safe
    pub graph_generator: Arc<dyn GraphGenerator>,
}
