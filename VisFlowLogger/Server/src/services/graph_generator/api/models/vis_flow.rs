use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GGBlock {
    /// Key of a block that called it.
    pub caller: Option<String>,
    pub name: String,
    pub flow: Vec<GGBlockFlow>,
}


impl Display for GGBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VisFlowBlock {{ caller: {:?}, name: {}, flow: [{}] }}",
            self.caller,
            self.name,
            self.flow
                .iter()
                .map(|b| format!("{}", b))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub struct GGBlockFlow {
    pub flow_pointer_id: Option<String>,
    pub flow_id: String,
    pub flow_type: GGBlockFlowType,
    pub value: Option<String>,
}

impl Display for GGBlockFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DGBlockFlow {{ flow_pointer_id: {:?}, flow_id: {}, flow_type: {}, value: {:?} }}",
            self.flow_pointer_id, self.flow_id, self.flow_type, self.value
        )
    }
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub enum GGBlockFlowType {
    Call,
    CallStore,
    Log,
    ExternalCall,
    ExternalCallStore,
}

impl Display for GGBlockFlowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            GGBlockFlowType::Call => "Call",
            GGBlockFlowType::CallStore => "CallStore",
            GGBlockFlowType::Log => "Log",
            GGBlockFlowType::ExternalCall => "ExternalCall",
            GGBlockFlowType::ExternalCallStore => "ExternalCallStore",
        };
        write!(f, "{}", type_str)
    }
}
