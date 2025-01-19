use crate::services::graph_generator::api::models::vis_flow::VisFlowBlock;
use crate::services::graph_generator::api::models::vis_flow_log_entry::VisFlowLogEntry;
use crate::services::graph_generator::internal::application::graph_generator_impl::GraphGeneratorImpl;
use std::collections::HashMap;

pub trait GraphGenerator {
    /**
    Generates graph based on VisFlowLogEntry. Vector needs to be sorted. <br>
    Returns back a HashMap where key is the ID of the Block and Value is the block itself.
    */
    /*
    Rules :-
    1. When a block starts it must end.
    2. The starting and ending block needs to be the same
    3. After a block's end. If the caller's next log was STORE. It needs to be stored.
    4. Store can't be present without previous log being of type CALL
    5. Starting block will always have key "START"
     */
    fn generate_graph(
        &self,
        entries: Vec<VisFlowLogEntry>,
    ) -> Result<HashMap<String, VisFlowBlock>, String>;
}

pub fn new() -> impl GraphGenerator {
    GraphGeneratorImpl {}
}
