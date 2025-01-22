export interface BlockData {
    caller: string | null; // Allow null
    name: string;
    flow: BlockFlow[];
}

export interface BlockFlow {
    flowPointerId: string | null; // Allow null
    flowId: string;
    flowType: BlockFlowType;
    value: string | null;
}

export enum BlockFlowType {
    Log,
    CallStore,
    Call,
    ExternCall,
    ExternCallStore,
}