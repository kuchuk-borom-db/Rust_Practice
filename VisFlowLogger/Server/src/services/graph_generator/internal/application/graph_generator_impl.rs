use crate::services::graph_generator::api::models::vis_flow_log_entry::{
    VisFlowLogEntry, VisFlowLogEntryLogType,
};
use crate::services::graph_generator::api::services::graph_generator::GraphGenerator;
use crate::services::graph_generator::internal::models::vis_flow::{
    Block, BlockFlow, BlockFlowType,
};

use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

pub struct GraphGeneratorImpl;

#[async_trait]
impl GraphGenerator for GraphGeneratorImpl {
    fn generate_graph(
        &self,
        entries: Vec<VisFlowLogEntry>,
    ) -> Result<HashMap<String, Block>, String> {
        // Initial entries validation
        {
            if entries.len() < 2 {
                return Err("GraphGenerator needs at least two entries".to_string());
            }
            if entries.first().unwrap().block_name != entries.last().unwrap().block_name {
                return Err("Starting and Ending block's name are not the same".to_string());
            }
        }
        let mut graph = HashMap::new();
        let mut current_block_id: Option<String> = None;
        let mut caller_stack: Vec<String> = Vec::new();
        //First entry operations
        {
            let first_entry = entries.first().unwrap();
            let block = create_block(&first_entry.block_name, None);
            current_block_id = Some("START".to_string());
            graph.insert("START".to_string(), block);
        }
        //Other Entries
        for entry in entries.iter().skip(1) {
            let current_block: &mut Block =
                graph.get_mut(&current_block_id.clone().unwrap()).unwrap();
            match entry.log_type {
                //If it's a log, it is to be directly added to the flow of the previous block.
                VisFlowLogEntryLogType::Log => current_block.flow.push(BlockFlow {
                    flow_id: Uuid::new_v4().to_string(),
                    flow_type: BlockFlowType::Log,
                    value: entry.log_value.clone(),
                    flow_pointer_id: None,
                    //current_block_id is still the same
                }),
                //Is a start of a new block. Called by previous block
                VisFlowLogEntryLogType::Start => {
                    let block_id = Uuid::new_v4().to_string();
                    let block = create_block(&entry.block_name, current_block_id.clone());
                    caller_stack.push(current_block_id.clone().unwrap());
                    graph.insert(block_id.clone(), block);
                    current_block_id = Some(block_id); //start of a new block
                }
                //End of current block. Added to the caller's flow.
                VisFlowLogEntryLogType::End => {
                    //Validations
                    {
                        //Previous block's name and current entry's name needs to match
                        if current_block.name != entry.block_name {
                            return Err(
                                "Starting and Ending block's name are not the same".to_string()
                            );
                        }
                    }
                    let caller_id_popped: Option<String> = caller_stack.pop();
                    //If current block has no caller then it MUST be the starting block
                    if caller_id_popped.is_none() {
                        let root_block = graph.get("START").unwrap();
                        if root_block.name != entry.block_name {
                            return Err("End of a block but it has no caller. Only starting block can have no caller".to_string());
                        }
                        break;
                    }
                    let caller_id: String = caller_id_popped.unwrap();
                    let caller_block: &mut Block = graph.get_mut(&caller_id).unwrap();
                    caller_block.flow.push(BlockFlow {
                        flow_id: Uuid::new_v4().to_string(),
                        flow_type: BlockFlowType::Call,
                        value: None,
                        //Points to the ID of the called block
                        flow_pointer_id: Some(current_block_id.unwrap()),
                    });
                    //Caller was popped and is now the latest interacted block.
                    current_block_id = Some(caller_id);
                }
                //Previous flow was a block call, and it's return value is stored
                VisFlowLogEntryLogType::Store => {
                    //Validations
                    {
                        //Latest flow must be of type call
                        if current_block.flow.is_empty()
                            || current_block.flow.last().unwrap().flow_type != BlockFlowType::Call
                        {
                            return Err("Store entry type while the previous block flow was either empty or not a block call".to_string());
                        }
                    }
                    let last_flow = current_block.flow.last_mut().unwrap();
                    last_flow.flow_type = BlockFlowType::CallStore;
                    last_flow.value = Some(entry.log_value.clone().unwrap());
                }
                //External block call or block store call.
                VisFlowLogEntryLogType::ExternalCall
                | VisFlowLogEntryLogType::ExternalCallStore => {
                    let block = BlockFlow {
                        value: Some(entry.log_value.clone().unwrap()),
                        flow_type: match entry.log_type {
                            VisFlowLogEntryLogType::ExternalCallStore => {
                                BlockFlowType::ExternalCallStore
                            }
                            VisFlowLogEntryLogType::ExternalCall => BlockFlowType::ExternalCall,
                            _ => {
                                return Err("Expected entry log's type to be ExternalCallStore or ExternalCallStore".to_string());
                            }
                        },
                        flow_id: Uuid::new_v4().to_string(),
                        flow_pointer_id: None,
                    };
                    current_block.flow.push(block);
                }
            }
        }
        //Post graph gen validation
        {
            if !caller_stack.is_empty() {
                return Err(
                    "caller stack is NOT empty after generating graph. Please check your log"
                        .to_string(),
                );
            }
        }
        Ok(graph)
    }
}
fn create_block(block_name: &String, caller: Option<String>) -> Block {
    Block {
        name: block_name.clone(),
        flow: vec![],
        caller,
    }
}
