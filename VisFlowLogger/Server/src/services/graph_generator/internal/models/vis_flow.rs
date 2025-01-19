use std::fmt::{self, Display};

#[derive(Eq, PartialEq, Hash)]
pub struct Block {
    /// Key of a block that called it.
    pub caller: Option<String>,
    pub name: String,
    pub flow: Vec<BlockFlow>,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VisFlowBlock {{ caller: {:?}, name: {}, flow: [{}] }}",
               self.caller,
               self.name,
               self.flow.iter().map(|b| format!("{}", b)).collect::<Vec<String>>().join(", ")
        )
    }
}

#[derive(Eq, PartialEq, Hash)]
pub struct BlockFlow {
    pub flow_pointer_id: Option<String>,
    pub flow_id: String,
    pub flow_type: BlockFlowType,
    pub value: Option<String>,
}

impl Display for BlockFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockFlow {{ flow_pointer_id: {:?}, flow_id: {}, flow_type: {}, value: {:?} }}",
               self.flow_pointer_id,
               self.flow_id,
               self.flow_type,
               self.value
        )
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum BlockFlowType {
    Call,
    CallStore,
    Log,
    ExternalCall,
    ExternalCallStore,
}

impl Display for BlockFlowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            BlockFlowType::Call => "Call",
            BlockFlowType::CallStore => "CallStore",
            BlockFlowType::Log => "Log",
            BlockFlowType::ExternalCall => "ExternalCall",
            BlockFlowType::ExternalCallStore => "ExternalCallStore",
        };
        write!(f, "{}", type_str)
    }
}
