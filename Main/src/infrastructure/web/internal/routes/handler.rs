use crate::domain::diagram_generator::api::models::DiagramGeneratorErr;
use crate::domain::diagram_generator::api::service::DiagramGeneratorTrait;
use crate::domain::graph_generator::api::service::GraphGeneratorTrait;
use crate::domain::graph_generator::repo::api::model::{LogType, VisLog};
use crate::domain::{diagram_generator, graph_generator};
use crate::infrastructure::web::internal::payloads::VisLogEntry;
use uuid::Uuid;

pub fn generate_diagram_from_vis_log_entries(
    entries: Vec<VisLogEntry>,
) -> Result<String, DiagramGeneratorErr> {
    let graph_generation = graph_generator::api::service::Factory::factory();
    let diagram_generator = diagram_generator::api::service::Factory::factory();

    // Map VisLogEntry to VisLog, matching log_type using &str for pattern matching
    let vis_log_entries = entries
        .iter()
        .map(|vis_log_entry| {
            VisLog {
                log_type: match vis_log_entry.log_type.as_str() {
                    // Using `as_str()` to get the &str reference
                    "START" => LogType::START,
                    "END" => LogType::END,
                    "STORE" => LogType::STORE,
                    "LOG" => LogType::LOG,
                    "EXTERNAL_CALL" => LogType::ExternalCall,
                    "EXTERNAL_CALL_STORE" => LogType::ExternalCallStore,
                    _ => panic!("Unknown vis log type {}", vis_log_entry.log_type),
                },
                value: vis_log_entry.value.clone(), // Assuming value is an Option<T>
                name: vis_log_entry.name.clone(),
                operation_id: vis_log_entry.operation_id.clone(),
                id: Uuid::new_v4().to_string(),
            }
        })
        .collect::<Vec<VisLog>>();

    // Generate the graph and handle potential errors
    let generated_graph = graph_generation.generate_graph(vis_log_entries);
    // Generate diagram and return it as a String
    let mermaid_diagram = diagram_generator.generate_diagram(&generated_graph.unwrap());
    Ok(mermaid_diagram?)
}
