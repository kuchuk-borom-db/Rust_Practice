use crate::services::diagram_generator::internal::application::mermaid_diagram_generator::MermaidDiagramGenerator;
use crate::services::diagram_generator::internal::models::Block;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait DiagramGenerator: Send + Sync {
    fn generate_diagram(graph: HashMap<String, Block>) -> String;
}
pub enum DiagramType {
    Mermaid,
}
pub fn new(diagram_type: DiagramType) -> impl DiagramGenerator {
    match diagram_type {
        DiagramType::Mermaid => {
            MermaidDiagramGenerator {}
        }
    }
}
