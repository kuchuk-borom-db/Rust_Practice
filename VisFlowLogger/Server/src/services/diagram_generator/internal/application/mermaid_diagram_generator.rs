use crate::services::diagram_generator::api::models::block::{DGBlock, DGBlockFlowType};
use crate::services::diagram_generator::api::services::DiagramGenerator;
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

pub struct MermaidDiagramGenerator;
impl MermaidDiagramGenerator {
    fn generate_color() -> String {
        let r = rand::random::<u8>();
        let g = rand::random::<u8>();
        let b = rand::random::<u8>();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}
#[async_trait]
impl DiagramGenerator for MermaidDiagramGenerator {
    fn generate_diagram(&self, graph: HashMap<String, DGBlock>) -> Result<String, String> {
        let mut syntax = String::from("flowchart TB\n");
        let mut subgraph_colors = HashMap::new();
        let start_color = "#FF5733";

        for (block_id, block) in &graph {
            let color = subgraph_colors
                .entry(block_id.clone())
                .or_insert_with(MermaidDiagramGenerator::generate_color);
            syntax += &String::from(format!("\tsubgraph {}[\"{}\"]\n", block_id, block.name));
            let mut prev_flow: Option<String> = None;
            for flow in &block.flow {
                let mut to_append: String = String::from("");
                match flow.flow_type {
                    DGBlockFlowType::Log => {
                        to_append = format!(
                            "\t\t{}([\"{}\"])",
                            flow.flow_id,
                            flow.value.as_ref().unwrap()
                        )
                    }
                    DGBlockFlowType::Call | DGBlockFlowType::CallStore => {
                        let called_entity_flow =
                            graph.get(flow.flow_pointer_id.as_ref().unwrap()).unwrap();
                        to_append = if flow.flow_type == DGBlockFlowType::Call {
                            format!("\t\t{}[\"{}\"]", flow.flow_id, called_entity_flow.name)
                        } else {
                            format!(
                                "\t\t{}[/\"{}\"/]",
                                flow.flow_id,
                                flow.value.as_ref().unwrap()
                            )
                        };
                    }
                    DGBlockFlowType::ExternalCall | DGBlockFlowType::ExternalCallStore => {
                        to_append = if flow.flow_type == DGBlockFlowType::ExternalCall {
                            format!("\t\t{}[\\{}/]", flow.flow_id, flow.value.as_ref().unwrap())
                        } else {
                            format!(
                                "\t\t{}[/\"{}\"\\]",
                                flow.flow_id,
                                flow.value.as_ref().unwrap()
                            )
                        };
                    }
                }
                syntax += &to_append;
                syntax += "\n";
                // Basic internal flow arrow
                if prev_flow.is_none() {
                    prev_flow = Option::from(flow.flow_id.clone());
                    continue;
                }
                syntax +=
                    &String::from(format!("\t\t{} ==> {}\n", prev_flow.unwrap(), flow.flow_id));
                prev_flow = Option::from(flow.flow_id.clone());
            }
            syntax += "\tend\n";
            // Add style for the subgraph
            syntax += &format!("style {} fill:{}\n", block_id, color);
        }

        for (_, v) in &graph {
            for flow in &v.flow {
                let mut to_append: String = String::from("");
                match flow.flow_type {
                    DGBlockFlowType::Call | DGBlockFlowType::CallStore => {
                        let called_entity_flow =
                            graph.get(flow.flow_pointer_id.as_ref().unwrap()).unwrap();
                        if let Some(caller_first_flow) = called_entity_flow.flow.first() {
                            to_append =
                                format!("{} ...-o {}", flow.flow_id, caller_first_flow.flow_id);
                        } else {
                            to_append = format!(
                                "{} ...-o {}",
                                flow.flow_id,
                                flow.flow_pointer_id.as_ref().unwrap()
                            );
                        }
                        if flow.flow_type == DGBlockFlowType::CallStore {
                            to_append += &String::from("\n");
                            if let Some(last_flow) = called_entity_flow.flow.last() {
                                to_append += &String::from(format!(
                                    "{} ---> {}",
                                    last_flow.flow_id, flow.flow_id
                                ));
                            } else {
                                to_append = format!(
                                    "{} ---> {}",
                                    flow.flow_pointer_id.as_ref().unwrap(),
                                    flow.flow_id
                                );
                            }
                        }
                    }
                    DGBlockFlowType::ExternalCallStore => {
                        to_append = format!("{} .-x {}", flow.flow_id, flow.flow_id);
                    }
                    DGBlockFlowType::ExternalCall => {
                        to_append += &String::from(format!(
                            "{} ..-x {}([\"{}\"])",
                            flow.flow_id,
                            Uuid::new_v4().to_string(),
                            "External Call"
                        ));
                    }
                    DGBlockFlowType::Log => {}
                }
                syntax += &to_append;
                syntax += "\n";
            }
        }

        let starting_flow = graph.get("START").unwrap();
        syntax += &String::from(format!(
            "\nBEGIN((\"START\")):::starting ==> {}\n",
            starting_flow.flow.first().unwrap().flow_id
        ));
        syntax += &String::from(format!(
            " {} ==> END((\"END\")):::starting\n",
            starting_flow.flow.last().unwrap().flow_id
        ));
        syntax += &format!("classDef starting fill:{};\n", start_color);

        Ok(syntax)
    }
}
