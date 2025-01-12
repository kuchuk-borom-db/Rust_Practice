pub struct MermaidDiagramGenerator;

use crate::services::diagram_generator::api::models::DiagramGeneratorErr;
use crate::services::diagram_generator::api::service::DiagramGeneratorTrait;
use crate::services::graph_generator::api::model::{FlowType, VisEntity};
use std::collections::HashMap;
use uuid::Uuid;

impl DiagramGeneratorTrait for MermaidDiagramGenerator {
    fn generate_diagram(
        &self,
        graph: &HashMap<String, VisEntity>,
    ) -> Result<String, DiagramGeneratorErr> {
        let mut mermaid = String::from("flowchart TB\n");
        //Generate subgraphs and direct arrows
        for (k, v) in graph {
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
                            format!("\t\t{}[/\"{}\"/]", flow.flow_id, called_entity_flow.name)
                        };
                    }
                    FlowType::ExternalCall | FlowType::ExternalCallStore => {
                        to_append = if flow.flow_type == FlowType::ExternalCall {
                            format!("\t\t{}[\\{}/]", flow.flow_id, flow.value.as_ref().unwrap())
                        } else {
                            //TODO Set arrow to self.
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
                //Basic internal flow arrow
                if prev_flow.is_none() {
                    prev_flow = Option::from(flow.flow_id.clone());
                    continue;
                }
                mermaid +=
                    &String::from(format!("\t\t{} ==> {}\n", prev_flow.unwrap(), flow.flow_id));
                prev_flow = Option::from(flow.flow_id.clone());
            }
            mermaid += "\tend\n";
        }

        for (_, v) in graph {
            for flow in &v.flow {
                let mut to_append: String = String::from("");
                match flow.flow_type {
                    FlowType::STORE => {
                        panic!("STORE FlowType should not be present! {:?}", flow)
                    }
                    FlowType::CALL | FlowType::CallStore => {
                        //Get the  called entity_flow and set dotted arrows to the first flow or subgraph itself if flow is empty
                        let called_entity_flow =
                            graph.get(flow.flow_pointer_id.as_ref().unwrap()).unwrap();
                        if let Some(first_flow) = called_entity_flow.flow.first() {
                            to_append = format!("{} ...-o {}", flow.flow_id, first_flow.flow_id);
                        } else {
                            to_append = format!("{} ...-o {}", flow.flow_id, called_entity_flow.id);
                        }
                        //If it's call_store type then we need to have a returning arrow
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
            "\nBEGIN((\"START\")) ==> {}\n",
            starting_flow.flow.first().unwrap().flow_id
        ));
        mermaid += &String::from(format!(
            " {} ==> END((\"END\"))\n",
            starting_flow.flow.last().unwrap().flow_id
        ));

        Ok(mermaid)
    }
}
