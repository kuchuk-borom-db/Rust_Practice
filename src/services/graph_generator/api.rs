pub mod model {
    pub struct VisFlowLogGraph {}
}
pub mod service {
    use crate::services::graph_generator::api::GraphGenerator;
    use crate::services::graph_generator::internal::model::GGImpl;
    pub fn factory() -> impl GraphGenerator {
        GGImpl {}
    }
}

use crate::services::graph_generator::api::model::VisFlowLogGraph;
use crate::services::graph_generator::repo::api::model::VisFlowLogEntry;
pub trait GraphGenerator {
    fn generate_graph(&self, entries: Vec<VisFlowLogEntry>) -> VisFlowLogGraph;
}
