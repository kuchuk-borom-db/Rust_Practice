use crate::services::diagram_generator::api::models::block::{DGBlock, DGBlockFlow, DGBlockFlowType};
use crate::services::graph_generator::api::models::vis_flow::{GGBlock, GGBlockFlow, GGBlockFlowType};

// Convert DGBlockFlowType to GGBlockFlowType
impl From<DGBlockFlowType> for GGBlockFlowType {
    fn from(flow_type: DGBlockFlowType) -> Self {
        match flow_type {
            DGBlockFlowType::Call => GGBlockFlowType::Call,
            DGBlockFlowType::CallStore => GGBlockFlowType::CallStore,
            DGBlockFlowType::Log => GGBlockFlowType::Log,
            DGBlockFlowType::ExternalCall => GGBlockFlowType::ExternalCall,
            DGBlockFlowType::ExternalCallStore => GGBlockFlowType::ExternalCallStore,
        }
    }
}

// Convert DGBlockFlow to GGBlockFlow
impl From<DGBlockFlow> for GGBlockFlow {
    fn from(flow: DGBlockFlow) -> Self {
        GGBlockFlow {
            flow_pointer_id: flow.flow_pointer_id,
            flow_id: flow.flow_id,
            flow_type: flow.flow_type.into(),
            value: flow.value,
        }
    }
}

// Convert DGBlock to GGBlock
impl From<DGBlock> for GGBlock {
    fn from(block: DGBlock) -> Self {
        GGBlock {
            caller: block.caller,
            name: block.name,
            flow: block.flow.into_iter().map(|f| f.into()).collect(),
        }
    }
}

// Convert GGBlockFlowType to DGBlockFlowType
impl From<GGBlockFlowType> for DGBlockFlowType {
    fn from(flow_type: GGBlockFlowType) -> Self {
        match flow_type {
            GGBlockFlowType::Call => DGBlockFlowType::Call,
            GGBlockFlowType::CallStore => DGBlockFlowType::CallStore,
            GGBlockFlowType::Log => DGBlockFlowType::Log,
            GGBlockFlowType::ExternalCall => DGBlockFlowType::ExternalCall,
            GGBlockFlowType::ExternalCallStore => DGBlockFlowType::ExternalCallStore,
        }
    }
}

// Convert GGBlockFlow to DGBlockFlow
impl From<GGBlockFlow> for DGBlockFlow {
    fn from(flow: GGBlockFlow) -> Self {
        DGBlockFlow {
            flow_pointer_id: flow.flow_pointer_id,
            flow_id: flow.flow_id,
            flow_type: flow.flow_type.into(),
            value: flow.value,
        }
    }
}

// Convert GGBlock to DGBlock
impl From<GGBlock> for DGBlock {
    fn from(block: GGBlock) -> Self {
        DGBlock {
            caller: block.caller,
            name: block.name,
            flow: block.flow.into_iter().map(|f| f.into()).collect(),
        }
    }
}
