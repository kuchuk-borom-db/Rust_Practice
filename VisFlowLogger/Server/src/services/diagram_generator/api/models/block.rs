use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DGBlock {
    /// Key of a block that called it.
    pub caller: Option<String>,
    pub name: String,
    pub flow: Vec<DGBlockFlow>,
}

impl Display for DGBlock {
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

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DGBlockFlow {
    pub flow_pointer_id: Option<String>,
    pub flow_id: String,
    pub flow_type: DGBlockFlowType,
    pub value: Option<String>,
}

impl Display for DGBlockFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DGBlockFlow {{ flow_pointer_id: {:?}, flow_id: {}, flow_type: {}, value: {:?} }}",
            self.flow_pointer_id, self.flow_id, self.flow_type, self.value
        )
    }
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum DGBlockFlowType {
    Call,
    CallStore,
    Log,
    ExternalCall,
    ExternalCallStore,
}

impl Display for DGBlockFlowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            DGBlockFlowType::Call => "Call",
            DGBlockFlowType::CallStore => "CallStore",
            DGBlockFlowType::Log => "Log",
            DGBlockFlowType::ExternalCall => "ExternalCall",
            DGBlockFlowType::ExternalCallStore => "ExternalCallStore",
        };
        write!(f, "{}", type_str)
    }
}
