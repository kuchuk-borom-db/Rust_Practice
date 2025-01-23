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

export const testData: Record<string, BlockData> = {
    "START": {
        caller: null,
        name: "main",
        flow: [
            {
                flowPointerId: null,
                flowId: "e14e2dee-9c6a-4644-8481-8f0d5a74aabc",
                flowType: BlockFlowType.Log,
                value: "Initializing application"
            },
            {
                flowPointerId: "9ac3b255-97a2-4310-a795-45559b27c851",
                flowId: "e1daebe5-f55d-4c35-bffe-7dd2db8e06f9",
                flowType: BlockFlowType.CallStore,
                value: "sum = 3"
            },
            {
                flowPointerId: "65351ad5-060c-428c-9d07-0a3685cb4ba0",
                flowId: "a805b838-221f-4057-b78f-3318d7dc1a02",
                flowType: BlockFlowType.Call,
                value: null
            },
            {
                flowPointerId: "b2c3d4e5-f6g7-4859-9a1b-c2d3e4f5g6h7",
                flowId: "c3d4e5f6-g7h8-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Call,
                value: null
            }
        ]
    },
    "9ac3b255-97a2-4310-a795-45559b27c851": {
        caller: "START",
        name: "sum",
        flow: [
            {
                flowPointerId: null,
                flowId: "11b27065-fe51-4239-ad08-f25c1b1e9df8",
                flowType: BlockFlowType.Log,
                value: "2 + 1 = 3"
            },
            {
                flowPointerId: "9dd4c31a-e4e4-47c8-a8bd-599121c98c5e",
                flowId: "675d7f8b-38dc-4cc2-95b2-6402c6f46c8a",
                flowType: BlockFlowType.Call,
                value: null
            },
            {
                flowPointerId: "a1b2c3d4-e5f6-4859-9a1b-c2d3e4f5g6h7",
                flowId: "d4e5f6g7-h8i9-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.CallStore,
                value: "result = 5"
            }
        ]
    },
    "9dd4c31a-e4e4-47c8-a8bd-599121c98c5e": {
        caller: "9ac3b255-97a2-4310-a795-45559b27c851",
        name: "sum",
        flow: [
            {
                flowPointerId: null,
                flowId: "dedbeb82-b326-48ed-9e92-67fb13521941",
                flowType: BlockFlowType.Log,
                value: "1 + 1 = 2"
            },
            {
                flowPointerId: "b2c3d4e5-f6g7-4859-9a1b-c2d3e4f5g6h7",
                flowId: "e5f6g7h8-i9j0-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Call,
                value: null
            }
        ]
    },
    "65351ad5-060c-428c-9d07-0a3685cb4ba0": {
        caller: "START",
        name: "foo",
        flow: [
            {
                flowPointerId: null,
                flowId: "e0188ee4-741c-49a7-979f-76d69c206c5a",
                flowType: BlockFlowType.Log,
                value: "foo called"
            },
            {
                flowPointerId: "c3d4e5f6-g7h8-495a-1b2c-d3e4f5g6h7i8",
                flowId: "f6g7h8i9-j0k1-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.CallStore,
                value: "foo_result = 10"
            }
        ]
    },
    "b2c3d4e5-f6g7-4859-9a1b-c2d3e4f5g6h7": {
        caller: "9dd4c31a-e4e4-47c8-a8bd-599121c98c5e",
        name: "multiply",
        flow: [
            {
                flowPointerId: null,
                flowId: "g7h8i9j0-k1l2-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Log,
                value: "3 * 2 = 6"
            },
            {
                flowPointerId: "d4e5f6g7-h8i9-495a-1b2c-d3e4f5g6h7i8",
                flowId: "h8i9j0k1-l2m3-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Call,
                value: null
            }
        ]
    },
    "a1b2c3d4-e5f6-4859-9a1b-c2d3e4f5g6h7": {
        caller: "9ac3b255-97a2-4310-a795-45559b27c851",
        name: "subtract",
        flow: [
            {
                flowPointerId: null,
                flowId: "i9j0k1l2-m3n4-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Log,
                value: "5 - 2 = 3"
            }
        ]
    },
    "c3d4e5f6-g7h8-495a-1b2c-d3e4f5g6h7i8": {
        caller: "65351ad5-060c-428c-9d07-0a3685cb4ba0",
        name: "bar",
        flow: [
            {
                flowPointerId: null,
                flowId: "j0k1l2m3-n4o5-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Log,
                value: "bar called"
            },
            {
                flowPointerId: "e5f6g7h8-i9j0-495a-1b2c-d3e4f5g6h7i8",
                flowId: "k1l2m3n4-o5p6-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.CallStore,
                value: "bar_result = 15"
            }
        ]
    },
    "d4e5f6g7-h8i9-495a-1b2c-d3e4f5g6h7i8": {
        caller: "b2c3d4e5-f6g7-4859-9a1b-c2d3e4f5g6h7",
        name: "divide",
        flow: [
            {
                flowPointerId: null,
                flowId: "l2m3n4o5-p6q7-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Log,
                value: "6 / 2 = 3"
            }
        ]
    },
    "e5f6g7h8-i9j0-495a-1b2c-d3e4f5g6h7i8": {
        caller: "c3d4e5f6-g7h8-495a-1b2c-d3e4f5g6h7i8",
        name: "baz",
        flow: [
            {
                flowPointerId: null,
                flowId: "m3n4o5p6-q7r8-495a-1b2c-d3e4f5g6h7i8",
                flowType: BlockFlowType.Log,
                value: "baz called"
            }
        ]
    }
};