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

#[derive(Debug)]
struct EntityFlow {
    name: String,
    flow: Vec<String>,
}

fn generate_visual_flow(logs: &Vec<Entry>) {
    let mut prev: Option<&Entry> = None;
    let mut root_entity: Option<String> = None;
    let mut entity_flow_id_map: HashMap<String, String> = HashMap::new();
    let mut flow_map: HashMap<String, EntityFlow> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();

    for log in logs {
        if prev.is_none() {
            // Root entity needs to be None too
            if root_entity.is_some() {
                panic!("Root entity is valid when prev is NOT valid.");
            }
            // The log type needs to be start
            if log.log_type != LogType::START {
                panic!("Invalid starting log_type {:?}", log.log_type);
            }
            // entity_name_map should not contain the log's entity_name as key as this is the first iteration.
            if entity_flow_id_map.contains_key(&log.entity_name) {
                panic!(
                    "First Entry but somehow entity_name_map already contains {} key",
                    log.entity_name
                );
            }
            // Set entity_name_map for the entity.
            let entity_flow_id: String = Uuid::new_v4().to_string();
            entity_flow_id_map.insert(log.entity_name.clone(), entity_flow_id.clone());
            let entity_flow: EntityFlow = EntityFlow {
                name: log.entity_name.clone(),
                flow: Vec::new(),
            };
            flow_map.insert(entity_flow_id.clone(), entity_flow);
            root_entity = Some(entity_flow_id);
            prev = Some(log);
            continue;
        }

        if prev.unwrap().entity_name == log.entity_name {
            let entity_id_option: Option<&String> =
                entity_flow_id_map.get(&prev.unwrap().entity_name);
            if entity_id_option.is_none() {
                panic!(
                    "entity_name_map doesn't exist for entity_name {}",
                    prev.unwrap().entity_name
                );
            }
            let entity_id: &String = entity_id_option.unwrap();

            if !flow_map.contains_key(entity_id) {
                panic!("entity_flow doesn't exist {}", entity_id);
            }

            let log_type: &LogType = &log.log_type;
            match log_type {
                LogType::STORE => {
                    flow_map
                        .get_mut(entity_id)
                        .unwrap()
                        .flow
                        .push("STORE::".to_string());
                }
                LogType::LOG => {
                    let log_val = log.value.as_ref().unwrap().clone();
                    flow_map.get_mut(entity_id).unwrap().flow.push(log_val);
                }
                LogType::END => {
                    let parent_entity_id_option: Option<String> = stack.pop();
                    if parent_entity_id_option.is_none() {
                        println!("parent_entity_id is NONE for entity {}", log.entity_name);
                        continue;
                    }
                    let parent_entity_id = parent_entity_id_option.unwrap();
                    let parent_entity_flow_option = flow_map.get_mut(&parent_entity_id);
                    let parent_entity_flow = parent_entity_flow_option.unwrap();
                    let mut parent_flow: &mut Vec<String> = &mut parent_entity_flow.flow;

                    if parent_flow.is_empty() {
                        parent_flow.push(format!(
                            "CALL::{}",
                            entity_flow_id_map.get(&log.entity_name).unwrap()
                        ));
                    } else {
                        let mut last_flow = parent_flow.last_mut().unwrap();
                        if last_flow == "STORE::" {
                            *last_flow = format!(
                                "CALL_RETURN::{}",
                                entity_flow_id_map.get(&log.entity_name).unwrap()
                            );
                        } else {
                            parent_flow.push(format!(
                                "CALL::{}",
                                entity_flow_id_map.get(&log.entity_name).unwrap()
                            ));
                        }
                    }
                }
                LogType::START => {
                    //Starting another function call to same entity type
                    let prev_entity_name =
                        entity_flow_id_map.get(&prev.unwrap().entity_name).unwrap();
                    stack.push(prev_entity_name.clone());
                    let entity_flow_id = Uuid::new_v4().to_string();
                    entity_flow_id_map.insert(log.entity_name.clone(), entity_flow_id.clone());

                    let entity_flow = EntityFlow {
                        flow: Vec::new(),
                        name: log.entity_name.clone(),
                    };
                    flow_map.insert(entity_flow_id, entity_flow);
                }
                _ => panic!(
                    "Invalid log_type {:?} when prev and current log is equal",
                    log.log_type
                ),
            }
        } else {
            if log.log_type == LogType::START {
                let prev_entity_name = entity_flow_id_map.get(&prev.unwrap().entity_name).unwrap();
                stack.push(prev_entity_name.clone());
                let entity_flow_id = Uuid::new_v4().to_string();
                entity_flow_id_map.insert(log.entity_name.clone(), entity_flow_id.clone());

                let entity_flow = EntityFlow {
                    flow: Vec::new(),
                    name: log.entity_name.clone(),
                };
                flow_map.insert(entity_flow_id, entity_flow);
            } else if prev.unwrap().log_type != LogType::END {
                panic!("Invalid transition between logs.");
            }
        }
        prev = Some(log);
    }

    // Output the generated entity flow in a human-readable format
    println!("\n--- Visual Flow Output ---");
    for (id, entity_flow) in flow_map.iter() {
        println!("\n{}: {}", id, entity_flow.name);
        println!("Flow:");
        for (i, entry) in entity_flow.flow.iter().enumerate() {
            println!("  {}: {}", i + 1, entry);
        }
    }
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
            entity_name: String::from("main"),
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
            log_type: LogType::END,
            value: None,
        },
    ];

    generate_visual_flow(&double_self_fn_call);
}
