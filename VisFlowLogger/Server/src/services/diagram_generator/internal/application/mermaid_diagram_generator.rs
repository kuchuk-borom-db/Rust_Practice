use crate::services::diagram_generator::api::services::DiagramGenerator;
use crate::services::diagram_generator::internal::models::Block;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct MermaidDiagramGenerator;
#[async_trait]
impl DiagramGenerator for MermaidDiagramGenerator {
    fn generate_diagram(graph: HashMap<String, Block>) -> String {

    }
}
