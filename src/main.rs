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
fn generate_visual_flow_v2(logs: &Vec<Entry>) {
    let mut root_entity: Option<String> = None;
    let mut prev: Option<&EntityFlow> = None;
    let mut id_flow_map: HashMap<String, EntityFlow> = HashMap::new();
    let mut stack: Vec<&EntityFlow> = Vec::new();
    for log in logs {
        if prev.is_none() {
            /*
            This is the first iteration with START as the log_type.
            Generate ID for the entity flow and create an empty entity flow.
            Set root_entity and prev.
             */
            match log.log_type {
                LogType::START => {}
                _ => panic!(
                    "Starting log has invalid log type {:?}. It needs to be of type START",
                    log.log_type
                ),
            }
            //TODO : Convert into helper function
            let entity_flow_id: String = Uuid::new_v4().to_string();
            let entity_flow: EntityFlow = EntityFlow {
                id: entity_flow_id.clone(),
                name: log.entity_name.clone(),
                flow: Vec::new(),
            };
            id_flow_map.insert(entity_flow_id.clone(), entity_flow);
            prev = Some(id_flow_map.get(&entity_flow_id).unwrap());
            root_entity = Some(entity_flow_id.clone());
            continue;
        }
        let prev_entity_flow = prev.unwrap();
        /*
        If prev is defined then the entity name may or may not be the same.
        If they are NOT same then it MUST be a start of another function call.
         */
        if prev_entity_flow.name != log.entity_name {
            /*
            If previous and current log entity names do not match. It should be a function call that was called by "prev".
            Thus, prev much be added to stack which will be popped when current entity's function ends.
            This is a new function call thus it is a new entity_flow and must be assigned a UID and added to entity_flow.
            */
            match log.log_type {
                LogType::START => {}
                _ => panic!(
                    "Current log & Prev log have distinct names BUT has invalid log type {:?}. It needs to be of type START",
                    log.log_type
                ),
            }
            //TODO: Convert into a helper function
            stack.push(prev_entity_flow);
            let entity_flow_id: String = Uuid::new_v4().to_string();
            let entity_flow: EntityFlow = EntityFlow {
                id: entity_flow_id.clone(),
                name: log.entity_name.clone(),
                flow: Vec::new(),
            };
            id_flow_map.insert(entity_flow_id.clone(), entity_flow);
            prev = Some(&id_flow_map.get(&entity_flow_id).unwrap());
            continue;
        }
        /*
        Prev is defined and current entity is the same as prev entity.
        If the current entity's log_type is START. It's a recursive function call.
        If the current entity's log_type is END then it's the end of current entity's function call and MUST be linked to it's parent entity by popping stack.
        */
        //let mut current_flow = *id_flow_map.get(prev.clone().unwrap().id.as_str()).unwrap();

        match log.log_type {
            LogType::START => {
                //Recursive function call
                stack.push(prev_entity_flow);

                let entity_flow_id: String = Uuid::new_v4().to_string();
                let entity_flow: EntityFlow = EntityFlow {
                    id: entity_flow_id.clone(),
                    name: log.entity_name.clone(),
                    flow: Vec::new(),
                };
                id_flow_map.insert(entity_flow_id.clone(), entity_flow);
                prev = Some(&id_flow_map.get(&entity_flow_id).unwrap());
                continue;
            }
            LogType::END => {
                let  current_flow = id_flow_map.get_mut(&prev_entity_flow.id).unwrap();
                // End of current entity's function
                let caller_entity_option = stack.pop();
                if caller_entity_option.is_some() {
                    let mut caller_entity: EntityFlow = caller_entity_option.unwrap().clone();
                    if caller_entity.flow.is_empty()
                        || caller_entity.flow.last().unwrap() != "::STORE::"
                    {
                        caller_entity
                            .flow
                            .push(format!("::CALL::{}", current_flow.id));
                    } else {
                        caller_entity.flow.pop();
                        caller_entity
                            .flow
                            .push(format!("::CALL_STORE::{}", current_flow.id));
                    }
                    id_flow_map.insert(caller_entity.id.clone(), caller_entity);
                }
                //If popped stack is none then it MUST be the end of the root entity's function. i.e end of visual flow log life cycle.
                else {
                    let root_entity = root_entity.clone().unwrap();
                    if current_flow.id != root_entity {
                        panic!("Popped stack is empty but current entity is not root entity ");
                    }
                }
            }
            LogType::STORE => {
                let  current_flow = id_flow_map.get_mut(&prev_entity_flow.id).unwrap();

                //Denotes that the next log needs to have a self loop to signify that it's return value is used
                current_flow.flow.push("::STORE::".to_string());
            }
            LogType::LOG => {
                let  current_flow = id_flow_map.get_mut(&prev_entity_flow.id).unwrap();
                if current_flow.flow.last_mut().unwrap_or(&mut "".to_string()) == "::STORE::" {
                    current_flow.flow.pop();
                    current_flow.flow.push(format!(
                        "::CALL_STORE::{}",
                        log.value.as_ref().unwrap_or(&"".to_string())
                    ));
                } else {
                    current_flow
                        .flow
                        .push(log.value.clone().unwrap().to_string());
                }
            }
        }
        let  current_flow = id_flow_map.get_mut(&prev_entity_flow.id).unwrap();
        prev = Some(&id_flow_map.get(&current_flow.id).unwrap());
    }
}
/*
fn generate_visual_flow(logs: &Vec<Entry>) {
    let mut prev: Option<&EntityFlow> = None;
    let mut root_entity: Option<String> = None;
    let mut id_flow_map: HashMap<String, &EntityFlow> = HashMap::new();
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
            // Set entity_name_map for the entity.
            let entity_flow_id: String = Uuid::new_v4().to_string();
            let entity_flow: EntityFlow = EntityFlow {
                id: entity_flow_id.clone(),
                name: log.entity_name.clone(),
                flow: Vec::new(),
            };
            id_flow_map.insert(entity_flow_id.clone(), &entity_flow);
            root_entity = Some(entity_flow_id);
            prev = Some(&entity_flow);
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

            if !id_flow_map.contains_key(entity_id) {
                panic!("entity_flow doesn't exist {}", entity_id);
            }

            let log_type: &LogType = &log.log_type;
            match log_type {
                LogType::STORE => {
                    id_flow_map
                        .get_mut(entity_id)
                        .unwrap()
                        .flow
                        .push("STORE::".to_string());
                }
                LogType::LOG => {
                    let log_val = log.value.as_ref().unwrap().clone();
                    id_flow_map.get_mut(entity_id).unwrap().flow.push(log_val);
                }
                LogType::END => {
                    let parent_entity_id_option: Option<String> = stack.pop();
                    if parent_entity_id_option.is_none() {
                        println!("parent_entity_id is NONE for entity {}", log.entity_name);
                        continue;
                    }
                    let parent_entity_id = parent_entity_id_option.unwrap();
                    let parent_entity_flow_option = id_flow_map.get_mut(&parent_entity_id);
                    let parent_entity_flow = parent_entity_flow_option.unwrap();
                    let parent_flow: &mut Vec<String> = &mut parent_entity_flow.flow;

                    if parent_flow.is_empty() {
                        parent_flow.push(format!(
                            "CALL::{}",
                            entity_flow_id_map.get(&log.entity_name).unwrap()
                        ));
                    } else {
                        let last_flow = parent_flow.last_mut().unwrap();
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
                        id: entity_flow_id,
                        flow: Vec::new(),
                        name: log.entity_name.clone(),
                    };
                    id_flow_map.insert(entity_flow_id, entity_flow);
                }
            }
        } else {
            if log.log_type == LogType::START {
                let prev_entity_name = entity_flow_id_map.get(&prev.unwrap().entity_name).unwrap();
                stack.push(prev_entity_name.clone());
                let entity_flow_id = Uuid::new_v4().to_string();
                entity_flow_id_map.insert(log.entity_name.clone(), entity_flow_id.clone());

                let entity_flow = EntityFlow {
                    id: entity_flow_id,
                    flow: Vec::new(),
                    name: log.entity_name.clone(),
                };
                id_flow_map.insert(entity_flow_id, entity_flow);
            } else if prev.unwrap().log_type != LogType::END {
                panic!("Invalid transition between logs.");
            }
        }
        prev = Some(log);
    }
    let root_key = root_entity.unwrap();
    if let Some(starting_entity) = id_flow_map.remove(&root_key) {
        id_flow_map.insert("START".to_string(), starting_entity);
    }
    // Output the generated entity flow in a human-readable format
    println!("\n--- Visual Flow Output ---");
    for (id, entity_flow) in id_flow_map.iter() {
        println!("\n{}: {}", id, entity_flow.name);
        println!("Flow:");
        for (i, entry) in entity_flow.flow.iter().enumerate() {
            println!("  {}: {}", i + 1, entry);
        }
    }
}
*/

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

    generate_visual_flow_v2(&double_self_fn_call);
}
