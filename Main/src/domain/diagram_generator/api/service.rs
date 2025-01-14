use crate::domain::diagram_generator::api::models::DiagramGeneratorErr;
use crate::domain::diagram_generator::internal::MermaidDiagramGenerator;
use crate::domain::graph_generator::api::model::VisEntity;
use std::collections::HashMap;

pub trait DiagramGeneratorTrait {
    fn generate_diagram(
        &self,
        graph: &HashMap<String, VisEntity>,
    ) -> Result<String, DiagramGeneratorErr>;
}

pub struct Factory;
impl Factory {
    pub fn factory() -> impl DiagramGeneratorTrait {
        MermaidDiagramGenerator {}
    }
}
