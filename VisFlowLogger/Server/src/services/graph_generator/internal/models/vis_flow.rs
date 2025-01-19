#[derive(Eq, PartialEq, Hash)]
pub struct VisFlowBlock {
    ///Key of a block that called it.
    pub caller: Option<String>,
    pub name: String,
    pub flow: Vec<BlockFlow>,
}

#[derive(Eq, PartialEq, Hash)]
pub struct BlockFlow {
    pub flow_pointer_id: Option<String>,
    pub flow_id: String,
    pub flow_type: BlockFlowType,
    pub value: Option<String>,
}
#[derive(Eq, PartialEq, Hash)]
pub enum BlockFlowType {
    Call,
    CallStore,
    Log,
    ExternalCall,
    ExternalCallStore,
}
