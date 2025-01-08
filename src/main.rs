use std::collections::HashMap;
use uuid::Uuid;

#[derive(PartialEq, Debug)] //Allows binary comparison such as ==, !=
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
            //root_entity needs to be none too
            if root_entity.is_some() {
                panic!("Root entity is valid when prev is NOT valid.");
            }
            //The log type needs to be start
            if log.log_type != LogType::START {
                panic!("Invalid starting log_type {:?}", log.log_type);
            }
            //entity_name_map should not contain the log's entity_name as key as this is the first iteration.
            if entity_flow_id_map.contains_key(&log.entity_name) {
                panic!(
                    "First Entry but somehow entity_name_map already contains {} key",
                    log.entity_name
                );
            }
            //set entity_name_map for the entity.
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
        //Prev is valid so check if it's entity_name is same as current. If yes then it is part of the flow UNLESS the log type is START
        if prev.unwrap().entity_name == log.entity_name {

            //TODO Check if it's start. If yes then its a function call

            //flow_map needs to have the prev's entity_name key
            let entity_id_option: Option<&String> =
                entity_flow_id_map.get(&prev.unwrap().entity_name);
            if entity_id_option.is_none() {
                panic!(
                    "entity_name_map doesn't exist for entity_name {}",
                    prev.unwrap().entity_name
                );
            }
            let entity_id: &String = entity_id_option.unwrap();
            //flow_map also needs to already contain the entityFlow
            if !flow_map.contains_key(entity_id) {
                panic!("entity_flow doesn't exist {}", entity_id);
            }
            //Based on log type we will store different flow values.
            let log_type: &LogType = &log.log_type;
            if log_type == &LogType::STORE {
                flow_map
                    .get_mut(entity_id)
                    .unwrap()
                    .flow
                    .push("STORE::".parse().unwrap());
            } else if log_type == &LogType::LOG {
                //TODO Self returning arrow if previous flow was STORE
                let log_val = log.value.as_ref().unwrap().clone();
                flow_map.get_mut(entity_id).unwrap().flow.push(log_val);
            } else if log_type == &LogType::END {
                //TODO Throw err if prev is STORE as we cant store end
                //Function has ended and needs to be referred back to it's direct caller (except root entity_flow)
                let parent_entity_id_option: Option<String> = stack.pop();
                //TODO add root entity check as only root entity is allowed to have no parent_entity
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
                        // If the latest flow is not "STORE", add a new flow entry for "CALL::..."
                        parent_flow.push(format!(
                            "CALL::{}",
                            entity_flow_id_map.get(&log.entity_name).unwrap()
                        ));
                    }
                }
            } else {
                //TODO Invalid log type
                panic!(
                    "Invalid log_type {:?} when prev and current log is equal",
                    log.log_type
                );
            }
        }
            //Prev and log is NOT of the same
        else {
            /*
            Rules :- it has to be start log type or prev has to be END log type
             */
            //It can be a start to a new function call OR
            if log.log_type == LogType::START {
                //Push prev's entry flow into stack as this new entity is being started by prev entity.
                let prev_entity_name = entity_flow_id_map.get(&prev.unwrap().entity_name).unwrap();
                stack.push(prev_entity_name.clone());
                //Update entity_name_map
                let entity_flow_id = Uuid::new_v4().to_string();
                entity_flow_id_map.insert(log.entity_name.clone(), entity_flow_id.clone());
                //Create new empty EntryFlow
                let entity_flow = EntityFlow {
                    flow: Vec::new(),
                    name: log.entity_name.clone(),
                };
                //Add the created entryFlow to flow_map
                flow_map.insert(entity_flow_id, entity_flow);
            }
            //It can be an end to a function
            else if prev.unwrap().log_type != LogType::END {
                //TODO Throw exception as prev and current log can only change at another function call start or end
            }
        }
        prev = Some(log);
    }

    println!("Generated entry flow is {:?}", flow_map);
}

fn main() -> () {
    //List of fake entries
    /*
    let logs = vec![
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::START,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: "num = 2".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::STORE,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::START,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::START,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::LOG,
            value: "return 4".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("add"),
            log_type: LogType::END,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: "4".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::STORE,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::START,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::LOG,
            value: "return 20".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::END,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: "24+4 = 24".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::END,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::LOG,
            value: "process_data(2) = 24".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("add2"),
            log_type: LogType::START,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("add2"),
            log_type: LogType::LOG,
            value: "return 3".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("add2"),
            log_type: LogType::END,
            value: "None".parse().unwrap(),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: "None".parse().unwrap(),
        },
    ];

     */
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

    //generate_visual_flow(&single_fn_call_log);
    generate_visual_flow(&single_fn_call_store_log);
    //generate_visual_flow(&double_fn_call_log);
}
