use std::collections::HashMap;
use uuid::Uuid;

#[derive(PartialEq, Debug)] // Allows binary comparison such as ==, !=
enum LogType {
    LOG,
    STORE,
    START,
    END,
    ExternalCall,
    ExternalCallStore,
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

fn create_entity_flow(
    id_flow_map: &mut HashMap<String, EntityFlow>,
    entity_name: String,
) -> String {
    let entity_flow_id: String = Uuid::new_v4().to_string();
    let entity_flow = EntityFlow {
        id: entity_flow_id.clone(),
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
        let entity_id = create_entity_flow(&mut id_flow_map, first_log.entity_name.clone());
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
                let entity_id = create_entity_flow(&mut id_flow_map, log.entity_name.clone());
                prev_id = Some(entity_id);
                continue;
            }
            //End of a function call. To link it to its caller we pop the stack and Link it based on entity_flow ids.
            LogType::END => {
                //prev_flow will ALWAYS match current flow as for a function to END it at least NEEDS to start previously.
                let prev_flow = id_flow_map.get(prev_entity_id).unwrap();
                if prev_flow.flow.last().unwrap_or(&"_".to_string()) == "::STORE::" {
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
                    //insert the updated caller which has link to the current entity_flow's ID.
                    id_flow_map.insert(caller_id.clone(), updated_caller);
                    //prev_id is now caller_id since we popped back to the caller.
                    prev_id = Some(caller_id);
                    continue;
                } else {
                    //If popped stack is empty it must be the end of the root entity_flow
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
                if let Some(prev_entity) = id_flow_map.get_mut(prev_entity_id) {
                    //If previous flow was ::STORE:: it is NOT valid. STORE is only used to represent that the return value of a function call is stored
                    if prev_entity.flow.last().unwrap_or(&String::new()) == "::STORE::" {
                        panic!("Previous flow log was of type STORE but now it's LOG which is NOT valid for entity_flow {:?}", prev_entity)
                    } else {
                        prev_entity
                            .flow
                            .push(log.value.clone().unwrap_or("_".to_string()));
                    }
                }
            }
            LogType::ExternalCall => {
                if let Some(prev_entity) = id_flow_map.get_mut(prev_entity_id) {
                    //If previous flow was ::STORE:: it is NOT valid. STORE is only used to represent that the return value of a function call is stored
                    if prev_entity.flow.last().unwrap_or(&String::new()) == "::STORE::" {
                        panic!("Previous flow log was of type STORE but now it's LOG which is NOT valid for entity_flow {:?}", prev_entity)
                    }
                    prev_entity.flow.push(format!(
                        "::EXTERNAL_CALL::{}",
                        log.value.clone().unwrap_or("_".to_string())
                    ));
                }
            }
            LogType::ExternalCallStore => {
                if let Some(prev_entity) = id_flow_map.get_mut(prev_entity_id) {
                    //If previous flow was ::STORE:: it is NOT valid. STORE is only used to represent that the return value of a function call is stored
                    if prev_entity.flow.last().unwrap_or(&String::new()) == "::STORE::" {
                        panic!("Previous flow log was of type STORE but now it's LOG which is NOT valid for entity_flow {:?}", prev_entity)
                    }
                    prev_entity.flow.push(format!(
                        "::EXTERNAL_CALL_RETURN::{}",
                        log.value.clone().unwrap_or("_".to_string())
                    ));
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
            value: Option::from(String::from("return 0")),
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
            value: Option::from(String::from("0")),
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
            value: Option::from(String::from("return 0")),
        },
        Entry {
            entity_name: String::from("multiply"),
            log_type: LogType::END,
            value: None,
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::LOG,
            value: Option::from(String::from("0+0 = 0")),
        },
        Entry {
            entity_name: String::from("processData"),
            log_type: LogType::END,
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
            log_type: LogType::ExternalCallStore,
            value: Option::from(String::from("Db repo")),
            //TODO Remove the store logic and instead use another special log type
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::ExternalCall,
            value: Option::from(String::from("External API Call")),
        },
        Entry {
            entity_name: String::from("main"),
            log_type: LogType::END,
            value: None,
        },
    ];
    /*
        {
          "fa891f3e-19de-4bcb-8016-54eea34e34f1": {
            "id": "fa891f3e-19de-4bcb-8016-54eea34e34f1",
            "name": "add",
            "flow": [
              "return 0"
            ]
          },
          "685cbcc3-5708-4131-bfe9-a97224ba8f51": {
            "id": "685cbcc3-5708-4131-bfe9-a97224ba8f51",
            "name": "multiply",
            "flow": [
              "return 0"
            ]
          },
          "771583cc-0bf4-4a73-b522-bb6fe7a2a3ad": {
            "id": "771583cc-0bf4-4a73-b522-bb6fe7a2a3ad",
            "name": "multiply",
            "flow": [
              "return 20"
            ]
          },
          "START": {
            "id": "7e920f7b-4b87-4ac4-a9b5-59402ea49beb",
            "name": "main",
            "flow": [
              "num = 2",
              "::CALL_STORE::92acea72-528a-4866-903c-16d6e68fa9ae",
              "process_data(2) = 24",
              "::CALL::056a8a89-212c-41f4-94f7-cc564c732e48",
              "::EXTERNAL_CALL_RETURN::Db repo",
              "::EXTERNAL_CALL::External API Call"
            ]
          },
          "056a8a89-212c-41f4-94f7-cc564c732e48": {
            "id": "056a8a89-212c-41f4-94f7-cc564c732e48",
            "name": "add2",
            "flow": [
              "return 3"
            ]
          },
          "36486281-03ba-4d0b-aeed-a6f69eaa6ebd": {
            "id": "36486281-03ba-4d0b-aeed-a6f69eaa6ebd",
            "name": "add",
            "flow": [
              "return 4"
            ]
          },
          "fc583f08-ec40-42b2-8c28-b8e130dd31bd": {
            "id": "fc583f08-ec40-42b2-8c28-b8e130dd31bd",
            "name": "processData",
            "flow": [
              "::CALL_STORE::fa891f3e-19de-4bcb-8016-54eea34e34f1",
              "0",
              "::CALL_STORE::685cbcc3-5708-4131-bfe9-a97224ba8f51",
              "0+0 = 0"
            ]
          },
          "92acea72-528a-4866-903c-16d6e68fa9ae": {
            "id": "92acea72-528a-4866-903c-16d6e68fa9ae",
            "name": "processData",
            "flow": [
              "::CALL_STORE::fc583f08-ec40-42b2-8c28-b8e130dd31bd",
              "::CALL_STORE::36486281-03ba-4d0b-aeed-a6f69eaa6ebd",
              "4",
              "::CALL_STORE::771583cc-0bf4-4a73-b522-bb6fe7a2a3ad",
              "24+4 = 24"
            ]
          }
    }
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
            log_type: LogType::ExternalCallStore,
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

    let data = generate_visual_flow_v2(&double_self_fn_call);
    println!("{:?}", data);
}
