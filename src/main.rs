use std::collections::HashMap;
use uuid::Uuid;

#[derive(PartialEq, Debug)] // Allows binary comparison such as ==, !=
enum LogType {
    LOG,
    STORE,
    START,
    END,
}

struct Entry {
    entity_name: String,
    log_type: LogType,
    value: Option<String>,
}

#[derive(Debug, Clone)]
struct EntityFlow {
    id: String, //For internal use.
    name: String,
    flow: Vec<String>,
}
fn generate_visual_flow_v2(logs: &Vec<Entry>) -> HashMap<String, EntityFlow> {
    let mut id_flow_map: HashMap<String, EntityFlow> = HashMap::new();
    let mut root_entity: Option<String> = None;
    let mut prev_id: Option<String> = None;
    let mut stack: Vec<String> = Vec::new();

    for log in logs {
        if prev_id.is_none() {
            match log.log_type {
                LogType::START => {}
                _ => panic!(
                    "Starting log has invalid log type {:?}. It needs to be of type START",
                    log.log_type
                ),
            }

            let entity_flow_id: String = Uuid::new_v4().to_string();
            let entity_flow = EntityFlow {
                id: entity_flow_id.clone(),
                name: log.entity_name.clone(),
                flow: Vec::new(),
            };

            id_flow_map.insert(entity_flow_id.clone(), entity_flow);
            prev_id = Some(entity_flow_id.clone());
            root_entity = Some(entity_flow_id);
            continue;
        }

        let prev_entity_id = prev_id.as_ref().unwrap();
        let prev_entity_flow = id_flow_map.get(prev_entity_id).unwrap();

        if prev_entity_flow.name != log.entity_name {
            match log.log_type {
                LogType::START => {
                    stack.push(prev_entity_id.clone());
                    let entity_flow_id: String = Uuid::new_v4().to_string();
                    let entity_flow = EntityFlow {
                        id: entity_flow_id.clone(),
                        name: log.entity_name.clone(),
                        flow: Vec::new(),
                    };
                    id_flow_map.insert(entity_flow_id.clone(), entity_flow);
                    prev_id = Some(entity_flow_id);
                    continue;
                },
                LogType::END => {
                    let current_entity_flow = id_flow_map.get(prev_entity_id).unwrap();
                    prev_id = Some(current_entity_flow.id.clone());
                    continue;
                }
                _ => panic!(
                    "Current log {} & Prev log {} have distinct names BUT has invalid log type {:?}. It needs to be of type START",
                    log.entity_name, prev_entity_flow.name, log.log_type
                ),
            }
        }

        match log.log_type {
            LogType::START => {
                stack.push(prev_entity_id.clone());
                let entity_flow_id: String = Uuid::new_v4().to_string();
                let entity_flow = EntityFlow {
                    id: entity_flow_id.clone(),
                    name: log.entity_name.clone(),
                    flow: Vec::new(),
                };

                id_flow_map.insert(entity_flow_id.clone(), entity_flow);
                prev_id = Some(entity_flow_id);
                continue;
            }
            LogType::END => {
                let prev_flow = id_flow_map.get(prev_entity_id).unwrap();
                if prev_flow.flow.last().unwrap_or(&"".to_string()) == "::STORE::" {
                    panic!(
                        "Latest statement of entity_flow {:?} was STORE but it's ENDING.",
                        prev_flow
                    )
                }
                let current_flow = id_flow_map.get(prev_entity_id).unwrap();

                if let Some(caller_id) = stack.pop() {
                    let caller_entity = id_flow_map.get(&caller_id).unwrap().clone();
                    let mut updated_caller = caller_entity.clone();

                    if caller_entity.flow.is_empty()
                        || caller_entity.flow.last().unwrap() != "::STORE::"
                    {
                        updated_caller
                            .flow
                            .push(format!("::CALL::{}", current_flow.id));
                    } else {
                        updated_caller.flow.pop();
                        updated_caller
                            .flow
                            .push(format!("::CALL_STORE::{}", current_flow.id));
                    }
                    //Since we pop back to caller so it's the latest log entry.
                    id_flow_map.insert(caller_id.clone(), updated_caller);
                    prev_id = Some(caller_id);
                    continue;
                } else {
                    let root_entity_id = root_entity.as_ref().unwrap();
                    if current_flow.id != *root_entity_id {
                        panic!("Popped stack is empty but current entity is not root entity ");
                    }
                }
            }
            LogType::STORE => {
                if let Some(flow) = id_flow_map.get_mut(prev_entity_id) {
                    flow.flow.push("::STORE::".to_string());
                }
            }
            LogType::LOG => {
                if let Some(flow) = id_flow_map.get_mut(prev_entity_id) {
                    if flow.flow.last().unwrap_or(&String::new()) == "::STORE::" {
                        flow.flow.pop();
                        flow.flow.push(format!(
                            "::CALL_STORE::{}",
                            log.value.as_ref().unwrap_or(&"".to_string())
                        ));
                    } else {
                        flow.flow.push(log.value.clone().unwrap_or_default());
                    }
                }
            }
        }
        let current_flow = id_flow_map.get(prev_entity_id).unwrap();
        prev_id = Some(current_flow.id.clone());
    }
    if !stack.is_empty() {
        panic!(
            "Invalid function calls. caller_entity {} never had it's called entity terminated",
            stack.pop().unwrap()
        )
    }
    id_flow_map
}

fn main() -> () {
    //List of fake entries

    let logs = vec![
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("num = 2")),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 4")),
        },
        //INVALID
        // Entry {
        //     entity_name: String::from("add"),
        //     log_type: LogType::STORE,
        //     value: None,
        // },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: Option::from(String::from("4")),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 20")),
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: Option::from(String::from("24+4 = 24")),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("process_data(2) = 24")),
        },
        Entry {
            entity_name: String::from("add2"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add2"),
            log_type: LogType::LOG,
            value: Option::from(String::from("return 3")),
        },
        Entry {
            entity_name: String::from("add2"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];

    let single_fn_call_log = vec![
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Adding 1 and 2")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("1+2 = 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];
    /*
    {
        "110ed917-adf4-4c0a-8fee-55de9db83a8d": EntityFlow {
            name: "main",
            flow: [
                "Adding 1 and 2",
                "CALL::10a8bda9-1a58-4904-9647-39a280c86c56"
            ]
        },
        "10a8bda9-1a58-4904-9647-39a280c86c56": EntityFlow {
            name: "add",
            flow: [
                "1+2 = 3"
            ]
        }
    }
    */
    let single_fn_call_store_log = vec![
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Adding 1 and 2")),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("1+2 = 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];
    /*
    {
        "6a6482b4-75ea-48f3-99b1-aae6902b1bdc": {
            "name": "add",
            "flow": [
                "1+2 = 3"
            ]
        },
        "753a4844-e606-4dd6-95d4-4dfccf713222": {
            "name": "main",
            "flow": [
                "Adding 1 and 2",
                "CALL_RETURN::6a6482b4-75ea-48f3-99b1-aae6902b1bdc"
            ]
        }
    }

     */

    let double_fn_call_log = vec![
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Adding 1 and 2")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("1+2 = 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("1+2 = 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];
    /*
    {
        "fab7e9e5-60ea-4d33-97ce-892b1b733046": EntityFlow {
            name: "add",
            flow: [
                "1+2 = 3"
            ]
        },
        "3f920fb4-0a31-4e37-bae9-11aeb7404094": EntityFlow {
            name: "add",
            flow: [
                "1+2 = 3"
            ]
        },
        "d70ba754-bedf-481e-9b72-fee0dae4b9b7": EntityFlow {
            name: "main",
            flow: [
                "Adding 1 and 2",
                "CALL::fab7e9e5-60ea-4d33-97ce-892b1b733046",
                "CALL::3f920fb4-0a31-4e37-bae9-11aeb7404094"
            ]
        }
    }

    */

    let double_self_fn_call = vec![
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Adding 1 and 2")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("1+2 = 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::STORE,
            value: Option::from(String::from("1+2 = 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("internal 1+2 = 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::STORE,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Storing API call result")),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];

    let super_fake_log = vec![
        // Main function starts
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Initiating main process")),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::STORE,
            value: None,
        },
        // First function call: "add"
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Adding 2 + 3")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::STORE,
            value: Option::from(String::from("5")),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: None,
        },
        // Main function logs result
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Result of add: 5")),
        },
        // Nested function call: "multiply"
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Multiplying 5 * 4")),
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::STORE,
            value: Option::from(String::from("20")),
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::END,
            value: None,
        },
        // Main logs result of multiply
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Result of multiply: 20")),
        },
        // Recursive function call: "factorial"
        Entry {
            entity_name: String::from("factorial"),
            log_type: LogType::START,
            value: None,
        },
        Entry {
            entity_name: String::from("factorial"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Calculating factorial(4)")),
        },
        Entry {
            entity_name: String::from("factorial"),
            log_type: LogType::STORE,
            value: Option::from(String::from("24")),
        },
        Entry {
            entity_name: String::from("factorial"),
            log_type: LogType::END,
            value: None,
        },
        // Main logs result of factorial
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: Option::from(String::from("Result of factorial: 24")),
        },
        // Main ends
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];

    let data = generate_visual_flow_v2(&logs);
    println!("{:?}", data);
}
