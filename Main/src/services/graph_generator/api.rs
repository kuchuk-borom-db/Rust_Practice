pub mod model {
    pub mod errors {
        #[derive(Debug)]
        pub struct GraphError;
    }
    #[derive(PartialEq, Debug, Clone)]
    pub enum FlowType {
        STORE,
        CALL,
        CallStore,
        LOG,
        ExternalCall,
        ExternalCallStore,
    }
    #[derive(Debug, Clone)]
    pub struct Flow {
        pub flow_pointer_id: Option<String>,
        pub flow_id: String,
        pub flow_type: FlowType,
        pub value: Option<String>,
    }
    #[derive(Debug, Clone)]
    pub struct VisEntity {
        pub id: String, //For internal use.
        pub caller: Option<String>,
        pub name: String,
        pub flow: Vec<Flow>,
    }
}

use crate::services::graph_generator::api::model::errors::GraphError;
use crate::services::graph_generator::api::model::VisEntity;
use crate::services::graph_generator::repo::api::model::VisLog;
use std::collections::HashMap;

pub trait GraphGeneratorTrait {
    fn generate_graph(
        &self,
        entries: Vec<VisLog>,
    ) -> Result<HashMap<String, VisEntity>, GraphError>;
}

pub struct Factory;
impl Factory {
    pub fn factory() -> impl GraphGeneratorTrait {
        crate::services::graph_generator::internal::model::GGImpl {}
    }
}
