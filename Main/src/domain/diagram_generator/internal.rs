pub struct MermaidDiagramGenerator;

use crate::domain::diagram_generator::api::models::DiagramGeneratorErr;
use crate::domain::diagram_generator::api::service::DiagramGeneratorTrait;
use crate::domain::graph_generator::api::model::{FlowType, VisEntity};
use std::collections::HashMap;
use uuid::Uuid;

impl MermaidDiagramGenerator {
    // Function to generate a random color in hex format
    fn generate_color() -> String {
        let r = rand::random::<u8>();
        let g = rand::random::<u8>();
        let b = rand::random::<u8>();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl DiagramGeneratorTrait for MermaidDiagramGenerator {
    fn generate_diagram(
        &self,
        graph: &HashMap<String, VisEntity>,
    ) -> Result<String, DiagramGeneratorErr> {
        let mut mermaid = String::from("flowchart TB\n");
        let mut subgraph_colors = HashMap::new();
        let starting_color = "#FF5733"; // Consistent color for the START module

        // Generate subgraphs and direct arrows
        for (k, v) in graph {
            // Assign a unique color for the subgraph if not already assigned
            let color = subgraph_colors
                .entry(k.clone())
                .or_insert_with(MermaidDiagramGenerator::generate_color);

            mermaid += &String::from(format!("\tsubgraph {}[\"{}\"]\n", k, v.name));
            let mut prev_flow: Option<String> = None;
            for flow in &v.flow {
                let mut to_append: String = String::from("");
                match flow.flow_type {
                    FlowType::LOG => {
                        to_append = format!(
                            "\t\t{}([\"{}\"])",
                            flow.flow_id,
                            flow.value.as_ref().unwrap()
                        );
                    }
                    FlowType::CALL | FlowType::CallStore => {
                        let called_entity_flow =
                            graph.get(flow.flow_pointer_id.as_ref().unwrap()).unwrap();
                        to_append = if flow.flow_type == FlowType::CALL {
                            format!("\t\t{}[\"{}\"]", flow.flow_id, called_entity_flow.name)
                        } else {
                            format!(
                                "\t\t{}[/\"{}\"/]",
                                flow.flow_id,
                                flow.value.as_ref().unwrap()
                            )
                        };
                    }
                    FlowType::ExternalCall | FlowType::ExternalCallStore => {
                        to_append = if flow.flow_type == FlowType::ExternalCall {
                            format!("\t\t{}[\\{}/]", flow.flow_id, flow.value.as_ref().unwrap())
                        } else {
                            format!(
                                "\t\t{}[/\"{}\"\\]",
                                flow.flow_id,
                                flow.value.as_ref().unwrap()
                            )
                        };
                    }
                    FlowType::STORE => {
                        panic!("STORE FlowType should not be present! {:?}", flow)
                    }
                }
                mermaid += &to_append;
                mermaid += "\n";
                // Basic internal flow arrow
                if prev_flow.is_none() {
                    prev_flow = Option::from(flow.flow_id.clone());
                    continue;
                }
                mermaid +=
                    &String::from(format!("\t\t{} ==> {}\n", prev_flow.unwrap(), flow.flow_id));
                prev_flow = Option::from(flow.flow_id.clone());
            }
            mermaid += "\tend\n";
            // Add style for the subgraph
            mermaid += &format!("style {} fill:{}\n", k, color);
        }

        for (_, v) in graph {
            for flow in &v.flow {
                let mut to_append: String = String::from("");
                match flow.flow_type {
                    FlowType::STORE => {
                        panic!("STORE FlowType should not be present! {:?}", flow)
                    }
                    FlowType::CALL | FlowType::CallStore => {
                        let called_entity_flow =
                            graph.get(flow.flow_pointer_id.as_ref().unwrap()).unwrap();
                        if let Some(first_flow) = called_entity_flow.flow.first() {
                            to_append = format!("{} ...-o {}", flow.flow_id, first_flow.flow_id);
                        } else {
                            to_append = format!("{} ...-o {}", flow.flow_id, called_entity_flow.id);
                        }
                        if flow.flow_type == FlowType::CallStore {
                            to_append += &String::from("\n");
                            if let Some(last_flow) = called_entity_flow.flow.last() {
                                to_append += &String::from(format!(
                                    "{} ---> {}",
                                    last_flow.flow_id, flow.flow_id
                                ));
                            } else {
                                to_append =
                                    format!("{} ---> {}", called_entity_flow.id, flow.flow_id);
                            }
                        }
                    }
                    FlowType::ExternalCallStore => {
                        to_append = format!("{} .-x {}", flow.flow_id, flow.flow_id);
                    }
                    FlowType::ExternalCall => {
                        to_append += &String::from(format!(
                            "{} ..-x {}([\"{}\"])",
                            flow.flow_id,
                            Uuid::new_v4().to_string(),
                            "External Call"
                        ));
                    }
                    FlowType::LOG => {}
                }
                mermaid += &to_append;
                mermaid += "\n";
            }
        }

        let starting_flow = graph.get("START").unwrap();
        mermaid += &String::from(format!(
            "\nBEGIN((\"START\")):::starting ==> {}\n",
            starting_flow.flow.first().unwrap().flow_id
        ));
        mermaid += &String::from(format!(
            " {} ==> END((\"END\")):::starting\n",
            starting_flow.flow.last().unwrap().flow_id
        ));
        mermaid += &format!("classDef starting fill:{};\n", starting_color);

        Ok(mermaid)
    }
}
