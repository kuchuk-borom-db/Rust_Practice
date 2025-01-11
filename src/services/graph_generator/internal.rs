pub(crate) mod model {
    use crate::services::graph_generator::api::model::VisFlowLogGraph;
    use crate::services::graph_generator::api::service::GraphGenerator;
    use crate::services::graph_generator::repo::api::model::VisFlowLogEntry;

    pub(crate) struct GGImpl;
    impl GraphGenerator for GGImpl {
        fn generate_graph(&self, entries: Vec<VisFlowLogEntry>) -> VisFlowLogGraph {
            VisFlowLogGraph {}
        }
    }
}
