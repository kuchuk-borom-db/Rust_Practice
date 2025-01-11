mod services;

use std::collections::HashMap;
use uuid::Uuid;

#[derive(PartialEq, Debug, Clone)] // Allows binary comparison such as ==, !=
enum LogType {
    LOG,
    STORE,
    START,
    END,
    ExternalCall,
    ExternalCallStore,
}

struct Entry {
    uuid: String,
    entity_name: String,
    log_type: LogType,
    value: Option<String>,
}

#[derive(PartialEq, Debug, Clone)]
enum FlowType {
    STORE,
    CALL,
    CALL_STORE,
    LOG,
    EXTERNAL_CALL,
    EXTERNAL_CALL_STORE,
}

#[derive(Debug, Clone)]
struct Flow {
    flow_pointer_id: Option<String>,
    flow_id: String,
    flow_type: FlowType,
    value: Option<String>,
}
#[derive(Debug, Clone)]
struct EntityFlow {
    id: String, //For internal use.
    caller: Option<String>,
    name: String,
    flow: Vec<Flow>,
}

fn create_entity_flow(
    id_flow_map: &mut HashMap<String, EntityFlow>,
    entity_name: String,
    caller: Option<String>,
    id: Option<String>,
) -> String {
    let entity_flow_id: String = if id.is_some() {
        id.unwrap()
    } else {
        Uuid::new_v4().to_string()
    };
    let entity_flow = EntityFlow {
        id: entity_flow_id.clone(),
        caller,
        name: entity_name,
        flow: Vec::new(),
    };

    id_flow_map.insert(entity_flow_id.clone(), entity_flow);
    entity_flow_id
}

fn generate_visual_flow_v2(logs: &Vec<Entry>) -> HashMap<String, EntityFlow> {
    //To store entity's flow mapped to a unique ID
    let mut id_flow_map: HashMap<String, EntityFlow> = HashMap::new();
    //The starting entity_flow
    let mut root_entity: Option<String> = None;
    //Previously iterated entity_flow
    let mut prev_id: Option<String> = None;
    //Stores caller_entity_flow. i.e entity_flow that "called" another entity_flow.
    // Example:- foo(){ bar(); }    "foo" is pushed into stack and popped when bar function is complete, thus maintaining the link between caller and the called "function"
    let mut stack: Vec<String> = Vec::new();

    // Handle first log entry separately
    if let Some(first_log) = logs.first() {
        if first_log.log_type != LogType::START {
            panic!(
                "Starting log has invalid log type {:?}. It needs to be of type START",
                first_log.log_type
            );
        }
        let entity_id = create_entity_flow(
            &mut id_flow_map,
            first_log.entity_name.clone(),
            None,
            Some("START".parse().unwrap()),
        );
        prev_id = Some(entity_id.clone());
        root_entity = Some(entity_id);
    } else {
        return HashMap::new();
    }

    for log in logs.iter().skip(1) {
        let prev_entity_id = prev_id.as_ref().unwrap();
        match log.log_type {
            //Start of a new function call.
            LogType::START => {
                stack.push(prev_entity_id.clone());
                let entity_id =
                    create_entity_flow(&mut id_flow_map, log.entity_name.clone(), prev_id, None);
                prev_id = Some(entity_id);
                continue;
            }
            //End of a function call. To link it to its caller we pop the stack and Link it based on entity_flow ids.
            LogType::END => {
                //prev_flow will ALWAYS match current flow as for a function to END it at least NEEDS to start previously.
                let current_entity_flow = id_flow_map.get(prev_entity_id).unwrap();
                //Having store and then ending a function is NOT allowed. STORE needs to be accompanied by another function call
                if let Some(last_flow) = current_entity_flow.flow.last() {
                    if (last_flow.flow_type == FlowType::STORE) {
                        panic!(
                            "Latest statement of entity_flow {:?} was STORE but it's ENDING.",
                            current_entity_flow
                        )
                    }
                }

                if let Some(caller_entity_id) = stack.pop() {
                    let mut updated_caller = id_flow_map.get(&caller_entity_id).unwrap().clone();
                    //If caller's last flow was not a store then it is not storing and thus this function call can be linked directly
                    if updated_caller.flow.is_empty()
                        || updated_caller.flow.last().unwrap().flow_type != FlowType::STORE
                    {
                        updated_caller.flow.push(Flow {
                            flow_id: log.uuid.clone(),
                            flow_type: FlowType::CALL,
                            flow_pointer_id: Some(current_entity_flow.id.clone()),
                            value: None,
                        });
                    } else {
                        //If caller's last flow was store then it needs to be popped and replaced with CALL_STORE flow
                        updated_caller.flow.pop();
                        updated_caller.flow.push(Flow {
                            flow_id: log.uuid.clone(),
                            flow_type: FlowType::CALL_STORE,
                            flow_pointer_id: Some(current_entity_flow.id.clone()),
                            value: None,
                        });
                    }
                    //Replace existing caller_entity_flow with the updated one.
                    id_flow_map.insert(caller_entity_id.clone(), updated_caller);
                    //Since we popped back to the caller, it is the latest entity_flow we interacted with and thus, needs to be set as prev_id.
                    prev_id = Some(caller_entity_id);
                    continue;
                } else {
                    //If popped stack is empty it must be the end of the root entity_flow
                    let root_entity_id = root_entity.as_ref().unwrap();
                    if current_entity_flow.id != *root_entity_id {
                        panic!("Popped stack is empty but current entity is not root entity ");
                    }
                }
            }
            LogType::STORE => {
                if let Some(prev_entity_flow) = id_flow_map.get_mut(prev_entity_id) {
                    prev_entity_flow.flow.push(Flow {
                        flow_id: log.uuid.clone(),
                        flow_type: FlowType::STORE,
                        value: None,
                        flow_pointer_id: None,
                    });
                }
            }
            LogType::ExternalCall | LogType::ExternalCallStore | LogType::LOG => {
                if let Some(prev_entity_flow) = id_flow_map.get_mut(prev_entity_id) {
                    //If previous flow was ::STORE:: it is NOT valid. STORE is only used to represent that the return value of a function call is stored
                    if prev_entity_flow.flow.last().is_some()
                        && prev_entity_flow.flow.last().unwrap().flow_type == FlowType::STORE
                    {
                        panic!("Previous flow log was of type STORE but now it's LOG which is NOT valid for entity_flow {:?}", prev_entity_flow)
                    }
                    prev_entity_flow
                        .flow
                        .push(if log.log_type == LogType::ExternalCall {
                            Flow {
                                flow_id: log.uuid.clone(),
                                flow_type: FlowType::EXTERNAL_CALL,
                                value: Some(log.value.clone().unwrap_or("_".to_string())),
                                flow_pointer_id: None,
                            }
                        } else if log.log_type == LogType::ExternalCallStore {
                            Flow {
                                flow_id: log.uuid.clone(),
                                flow_type: FlowType::EXTERNAL_CALL_STORE,
                                value: Some(log.value.clone().unwrap_or("_".to_string())),
                                flow_pointer_id: None,
                            }
                        } else {
                            Flow {
                                flow_id: log.uuid.clone(),
                                flow_type: FlowType::LOG,
                                value: Option::from(log.value.clone().unwrap_or("".to_string())),
                                flow_pointer_id: None,
                            }
                        });
                }
            }
        }

        //Since current_flow and prev flow is the same we can get the ID by using prev_entity_id
        let current_flow = id_flow_map.get(prev_entity_id).unwrap();
        prev_id = Some(current_flow.id.clone());
    }

    //If stack is NOT empty after iteration then some function call was left dangling and is not valid.
    if !stack.is_empty() {
        panic!(
            "Invalid function calls. caller_entity {} never had it's called entity terminated",
            stack.pop().unwrap()
        )
    }
    //Replace the root_entity_id key with "START" to define the Starting entity_flow
    let root_entity_flow = id_flow_map.remove(&root_entity.unwrap()).unwrap();
    id_flow_map.insert(String::from("START"), root_entity_flow);
    id_flow_map
}

fn generate_mermaid_diagram_from_visual_flow_log_map(map: &HashMap<String, EntityFlow>) -> String {
    let mut mermaid = String::from("flowchart TB\n");
    //Generate subgraphs and direct arrows
    for (k, v) in map {
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
                FlowType::CALL | FlowType::CALL_STORE => {
                    let called_entity_flow =
                        map.get(flow.flow_pointer_id.as_ref().unwrap()).unwrap();
                    to_append = if flow.flow_type == FlowType::CALL {
                        format!("\t\t{}[\"{}\"]", flow.flow_id, called_entity_flow.name)
                    } else {
                        format!("\t\t{}[/\"{}\"/]", flow.flow_id, called_entity_flow.name)
                    };
                }
                FlowType::EXTERNAL_CALL | FlowType::EXTERNAL_CALL_STORE => {
                    to_append = if flow.flow_type == FlowType::EXTERNAL_CALL {
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
            mermaid += &String::from(format!("\t\t{} ==> {}\n", prev_flow.unwrap(), flow.flow_id));
            prev_flow = Option::from(flow.flow_id.clone());
        }
        mermaid += "\tend\n";
    }

    for (k, v) in map {
        for flow in &v.flow {
            let mut to_append: String = String::from("");
            match flow.flow_type {
                FlowType::STORE => {
                    panic!("STORE FlowType should not be present! {:?}", flow)
                }
                FlowType::CALL | FlowType::CALL_STORE => {
                    //Get the  called entity_flow and set dotted arrows to the first flow or subgraph itself if flow is empty
                    let called_entity_flow =
                        map.get(flow.flow_pointer_id.as_ref().unwrap()).unwrap();
                    if let Some(first_flow) = called_entity_flow.flow.first() {
                        to_append = format!("{} ..-o {}", flow.flow_id, first_flow.flow_id);
                    } else {
                        to_append = format!("{} ..-o {}", flow.flow_id, called_entity_flow.id);
                    }
                    //If it's call_store type then we need to have a returning arrow
                    if flow.flow_type == FlowType::CALL_STORE {
                        to_append += &String::from("\n");
                        if let Some(last_flow) = called_entity_flow.flow.last() {
                            to_append += &String::from(format!(
                                "{} ---> {}",
                                last_flow.flow_id, flow.flow_id
                            ));
                        } else {
                            to_append = format!("{} ---> {}", called_entity_flow.id, flow.flow_id);
                        }
                    }
                }
                FlowType::EXTERNAL_CALL_STORE => {
                    to_append = format!("{} .-x {}", flow.flow_id, flow.flow_id);
                }
                FlowType::EXTERNAL_CALL => {
                    to_append += &String::from(format!(
                        "{} ..-x {}([\"{}\"])",
                        flow.flow_id,
                        Uuid::new_v4().to_string(),
                        "External Call"
                    ));
                }
                FlowType::LOG => {}
                _ => {}
            }
            mermaid += &to_append;
            mermaid += "\n";
        }
    }

    let starting_flow = map.get("START").unwrap();
    mermaid += &String::from(format!(
        "\nBEGIN((\"START\")) ==> {}\n",
        starting_flow.flow.first().unwrap().flow_id
    ));
    mermaid += &String::from(format!(
        " {} ==> END((\"END\"))\n",
        starting_flow.flow.last().unwrap().flow_id
    ));

    mermaid
}

fn main() -> () {
    //List of fake entries

    let logs = vec![
        Entry {
            uuid: String::from("a"),
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("b"),
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("num = 2")),
        },
        Entry {
            uuid: String::from("c"),
            entity_name: String::from("main"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            uuid: String::from("d"),
            entity_name: String::from("processData"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("e"),
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            uuid: String::from("f"),
            entity_name: String::from("processData"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("g"),
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            uuid: String::from("h"),
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("i"),
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 0")),
        },
        Entry {
            uuid: String::from("j"),
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            uuid: String::from("k"),
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: Option::from(String::from("0")),
        },
        Entry {
            uuid: String::from("l"),
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            uuid: String::from("m"),
            entity_name: String::from("multiply"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("n"),
            entity_name: String::from("multiply"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 0")),
        },
        Entry {
            uuid: String::from("o"),
            entity_name: String::from("multiply"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            uuid: String::from("p"),
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: Option::from(String::from("0+0 = 0")),
        },
        Entry {
            uuid: String::from("q"),
            entity_name: String::from("processData"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            uuid: String::from("r"),
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            uuid: String::from("s"),
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("t"),
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 4")),
        },
        Entry {
            uuid: String::from("u"),
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            uuid: String::from("v"),
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: Option::from(String::from("4")),
        },
        Entry {
            uuid: String::from("w"),
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            uuid: String::from("x"),
            entity_name: String::from("multiply"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("y"),
            entity_name: String::from("multiply"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 20")),
        },
        Entry {
            uuid: String::from("z"),
            entity_name: String::from("multiply"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            uuid: String::from("1"),
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: Option::from(String::from("24+4 = 24")),
        },
        Entry {
            uuid: String::from("2"),
            entity_name: String::from("processData"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            uuid: String::from("3"),
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("process_data(2) = 24")),
        },
        Entry {
            uuid: String::from("4"),
            entity_name: String::from("add2"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            uuid: String::from("5"),
            entity_name: String::from("add2"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 3")),
        },
        Entry {
            uuid: String::from("6"),
            entity_name: String::from("add2"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            uuid: String::from("7"),
            entity_name: String::from("main"),
            log_type: LogType::ExternalCallStore,
            value: Option::from(String::from("Db repo")),
        },
        Entry {
            uuid: String::from("8"),
            entity_name: String::from("main"),
            log_type: LogType::ExternalCall,
            value: Option::from(String::from("External API Call")),
        },
        Entry {
            uuid: String::from("9"),
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];

    let data = generate_visual_flow_v2(&logs);
    let mermaid = generate_mermaid_diagram_from_visual_flow_log_map(&data);
    println!("{:?}", data);
    println!("{}", mermaid);

    use crate::services::graph_generator::api::service::factory;
    let gg = factory();
}
