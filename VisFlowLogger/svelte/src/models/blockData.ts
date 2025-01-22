export interface BlockData {
    caller: string,
    name: string,
    flow: [BlockFlow]
}

export interface BlockFlow {
    flowPointerId: string,
    flowId: string,
    flowType: BlockFlowType,
    value: string | null
}

export enum BlockFlowType {
    Log,
    CallStore,
    Call,
    ExternCall,
    ExternCallStore,
}