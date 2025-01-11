pub(crate) mod model {
    pub(crate) struct GGImpl;
}

pub(crate) mod application {
    use crate::services::graph_generator::api::model::errors::GraphError;
    use crate::services::graph_generator::api::model::{Flow, FlowType, VisEntity};
    use crate::services::graph_generator::api::GraphGenerator;
    use crate::services::graph_generator::internal::model::GGImpl;
    use crate::services::graph_generator::repo::api::model::{LogType, VisLog};
    use std::collections::HashMap;
    use uuid::Uuid;

    impl GraphGenerator for GGImpl {
        fn generate_graph(
            &self,
            entries: Vec<VisLog>,
        ) -> Result<HashMap<String, VisEntity>, GraphError> {
            let mut id_entity_map: HashMap<String, VisEntity> = HashMap::new();
            let mut root_entity_id: Option<String> = None;
            let mut prev_entity_id: Option<String> = None;
            let mut caller_entity_id_stack: Vec<String> = Vec::new();
            if let Some(first_log) = entries.first() {
                //Validate if the start and the end of the entries are valid.
                {
                    log::debug!("Validating if start and end of entries are valid");
                    if first_log.log_type != LogType::START {
                        return Err(GraphError {});
                    }
                    let last_log = entries.last();
                    if last_log.is_none() {
                        return Err(GraphError {});
                    }
                    if last_log.unwrap().log_type != LogType::END {
                        return Err(GraphError {});
                    }
                    if last_log.unwrap().name != first_log.name {
                        return Err(GraphError {});
                    }
                }
                //Operation exclusive to first element in entries
                {
                    //TODO use lifetime to keep scope leveled variable alive after scope ends
                    let first_vis_entity =
                        create_vis_entity(first_log.name.clone(), None, Some("START".to_string()));
                    let first_vis_entity_id = first_vis_entity.id.clone();
                    prev_entity_id = Some(first_vis_entity_id.clone());
                    root_entity_id = Some(first_vis_entity_id.clone());
                    id_entity_map.insert(first_vis_entity_id, first_vis_entity);
                }
            } else {
                return Err(GraphError {});
            }

            for log in entries.iter().skip(1) {
                let prev_vis_entity = id_entity_map
                    .get_mut(prev_entity_id.as_ref().unwrap())
                    .unwrap();
                match log.log_type {
                    //Start of a new inner VisEntity called by previous VisEntity
                    LogType::START => {
                        //Push caller(previous VisEntity) to stack.
                        caller_entity_id_stack.push(prev_entity_id.as_ref().unwrap().clone());
                        //Create new VisEntityFlow to represent the current VisFlowLog
                        let current_vis_entity = create_vis_entity(log.name.clone(), None, None);
                        let current_vis_entity_id = current_vis_entity.id.clone();
                        id_entity_map.insert(current_vis_entity_id.clone(), current_vis_entity);
                        prev_entity_id = Some(current_vis_entity_id);
                    }
                    //End of Previous VisEntity. Time to link it to the caller stored in stack.
                    LogType::END => {
                        //Validations
                        {
                            //Previous VisEntity and current log's name need to be the same
                            if prev_vis_entity.name != log.name {
                                return Err(GraphError {});
                            }
                            //If Previous VisEntity's flow was ::STORE:: it is NOT valid
                            if !prev_vis_entity.flow.is_empty()
                                && prev_vis_entity.flow.last().unwrap().flow_type == FlowType::STORE
                            {
                                return Err(GraphError {});
                            }
                        }
                        if let Some(caller_entity_id) = caller_entity_id_stack.pop() {
                            let mut caller_entity =
                                id_entity_map.get_mut(&caller_entity_id).unwrap();
                            if caller_entity.flow.is_empty()
                                || caller_entity.flow.last().unwrap().flow_type != FlowType::STORE
                            {
                                caller_entity.flow.push(Flow {
                                    flow_id: log.id.clone(),
                                    flow_type: FlowType::CALL,
                                    //Prev VisEntity ID is valid because current log represents the end of prev VisEntity. We throw Error if this isn't the case in validation scope.
                                    flow_pointer_id: Some(prev_entity_id.as_ref().unwrap().clone()),
                                    value: None,
                                });
                            } else {
                                caller_entity.flow.pop();
                                caller_entity.flow.push(Flow {
                                    flow_id: log.id.clone(),
                                    flow_type: FlowType::CallStore,
                                    //Prev VisEntity ID is valid because current log represents the end of prev VisEntity. We throw Error if this isn't the case in validation scope.
                                    flow_pointer_id: Some(prev_entity_id.as_ref().unwrap().clone()),
                                    value: None,
                                })
                            }
                            //Caller is the latest processed entity and thus, needs to be set as previous Entity
                            prev_entity_id = Some(caller_entity.id.clone());
                        } else {
                            //If no VisEntity called the current VisLog then it HAS to be the root entity
                            if prev_entity_id.as_ref().unwrap() != root_entity_id.as_ref().unwrap()
                            {
                                return Err(GraphError {});
                            }
                        }
                    }
                    LogType::STORE => prev_vis_entity.flow.push(Flow {
                        flow_id: log.id.clone(),
                        flow_type: FlowType::STORE,
                        value: None,
                        flow_pointer_id: None,
                    }),
                    LogType::ExternalCall | LogType::ExternalCallStore | LogType::LOG => {
                        //Validations
                        {
                            //STORE is ONLY used for storing return values of function calls
                            if !prev_vis_entity.flow.is_empty()
                                && prev_vis_entity.flow.last().unwrap().flow_type == FlowType::STORE
                            {
                                return Err(GraphError {});
                            }
                        }
                        prev_vis_entity.flow.push(Flow {
                            flow_id: log.id.clone(),
                            value: Some(log.value.as_ref().unwrap_or(&String::from("_")).clone()),
                            flow_pointer_id: None,
                            flow_type: if log.log_type == LogType::LOG {
                                FlowType::LOG
                            } else if log.log_type == LogType::ExternalCall {
                                FlowType::ExternalCall
                            } else {
                                FlowType::ExternalCallStore
                            },
                        });
                    }
                }
            }
            if !caller_entity_id_stack.is_empty() {
                return Err(GraphError {});
            }
            Ok(id_entity_map)
        }
    }

    //TODO Use queue approach replacing stack & iteration

    ///Creates a new VisEntity
    fn create_vis_entity(
        entity_name: String,
        caller: Option<String>,
        id: Option<String>,
    ) -> VisEntity {
        let entity_flow_id: String = if id.is_some() {
            id.unwrap()
        } else {
            Uuid::new_v4().to_string()
        };
        let entity_flow = VisEntity {
            id: entity_flow_id.clone(),
            caller,
            name: entity_name,
            flow: Vec::new(),
        };
        entity_flow
    }
}
