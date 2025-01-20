use crate::services::diagram_generator::api::models::block::DGBlock;
use crate::services::diagram_generator::internal::application::mermaid_diagram_generator::MermaidDiagramGenerator;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait DiagramGenerator: Send + Sync {
    fn generate_diagram(&self, graph: HashMap<String, DGBlock>) -> Result<String, String>;
}
pub enum DiagramType {
    Mermaid,
}
pub fn new(diagram_type: DiagramType) -> impl DiagramGenerator {
    match diagram_type {
        DiagramType::Mermaid => MermaidDiagramGenerator {},
    }
}
