use crate::services::graph_generator::api::models::vis_flow_log_entry::{
    VisFlowLogEntry, VisFlowLogEntryLogType,
};
use crate::services::graph_generator::api::services::graph_generator::GraphGenerator;
use crate::services::graph_generator::internal::models::vis_flow::{
    BlockFlow, BlockFlowType, VisFlowBlock,
};

use std::collections::HashMap;
use uuid::Uuid;

pub struct GraphGeneratorImpl;

impl GraphGenerator for GraphGeneratorImpl {
    fn generate_graph(
        &self,
        entries: Vec<VisFlowLogEntry>,
    ) -> Result<HashMap<String, VisFlowBlock>, String> {
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
        let mut previous_block_id = None;
        let mut caller_stack = Vec::new();
        //First entry operations
        {
            let first_entry = entries.first().as_ref().unwrap();
            let block = create_block(**first_entry.block_name, None);
            previous_block_id = Some("START".to_string());
            graph.insert(String::from("START"), block);
        }
        //Other Entries
        for entry in entries.iter().skip(1) {
            let prev_block = graph.get_mut(&previous_block_id.as_ref().unwrap()).unwrap();
            match entry.log_type {
                //If it's a log, it is to be directly added to the flow of the previous block.
                VisFlowLogEntryLogType::Log => {
                    let block = graph.get_mut(previous_block_id.as_ref().unwrap()).unwrap();
                    block.flow.push(BlockFlow {
                        flow_id: Uuid::new_v4().to_string(),
                        flow_type: BlockFlowType::Log,
                        value: Some(**entry.log_value),
                        flow_pointer_id: None,
                    })
                }
                //Is a start of a new block. Called by previous block
                VisFlowLogEntryLogType::Start => {
                    let block_id = Uuid::new_v4().to_string();
                    let block = create_block(**entry.block_name, *previous_block_id.as_ref());
                    caller_stack.push(previous_block_id.as_ref().unwrap());
                    graph.insert(block_id, block);
                }
                //End of current block. Added to the caller's flow.
                VisFlowLogEntryLogType::End => {
                    //Validations
                    {
                        //Previous block's name and current entry's name needs to match
                        if prev_block.name != entry.block_name {
                            return Err(
                                "Starting and Ending block's name are not the same".to_string()
                            );
                        }
                    }
                    let caller_id = caller_stack.pop().unwrap();
                    let caller_block = graph.get_mut(&caller_id).unwrap();
                    caller_block.flow.push(BlockFlow {
                        flow_id: Uuid::new_v4().to_string(),
                        flow_type: BlockFlowType::Call,
                        value: None,
                        //Points to the ID of the called block
                        flow_pointer_id: Some(***previous_block_id.as_ref().unwrap()),
                    });
                    //Caller was popped and is now the latest interacted block.
                    previous_block_id = Some((**caller_id).parse().unwrap());
                }
                //Previous flow was a block call and it's return value is stored
                VisFlowLogEntryLogType::Store => {
                    //Validations
                    {
                        //Latest flow must be of type call
                        if prev_block.flow.is_empty()
                            || prev_block.flow.last().unwrap().flow_type != BlockFlowType::Call
                        {
                            return Err("Store entry type while the previous block flow was either empty or not a block call".to_string());
                        }
                    }
                    let last_flow = prev_block.flow.last_mut().unwrap();
                    last_flow.flow_type = BlockFlowType::CallStore;
                    last_flow.value = Some(entry.log_value.clone());
                }
                //External block call or block store call.
                VisFlowLogEntryLogType::ExternalCall
                | VisFlowLogEntryLogType::ExternalCallStore => {
                    let block = BlockFlow {
                        value: Some(entry.log_value.clone()),
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
                    prev_block.flow.push(block);
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
fn create_block(block_name: String, caller: Option<String>) -> VisFlowBlock {
    VisFlowBlock {
        name: block_name,
        flow: vec![],
        caller,
    }
}
