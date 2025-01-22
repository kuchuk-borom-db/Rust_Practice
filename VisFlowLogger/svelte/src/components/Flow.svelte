<script lang="ts">
    import type { BlockFlow } from '../models/blockData'; // Import BlockFlow as a type
    import { BlockFlowType } from '../models/blockData'; // Import BlockFlowType as a value
    export let blockID: string;
    export let flow: BlockFlow;

    // Helper function to get the flow type label
    function getFlowTypeLabel(flowType: BlockFlowType): string {
        switch (flowType) {
            case BlockFlowType.Log:
                return 'Log';
            case BlockFlowType.CallStore:
                return 'Call Store';
            case BlockFlowType.Call:
                return 'Call';
            case BlockFlowType.ExternCall:
                return 'Extern Call';
            case BlockFlowType.ExternCallStore:
                return 'Extern Call Store';
            default:
                return 'Unknown';
        }
    }

    // Helper function to get the flow type color
    function getFlowTypeColor(flowType: BlockFlowType): string {
        switch (flowType) {
            case BlockFlowType.Log:
                return 'bg-blue-500';
            case BlockFlowType.CallStore:
                return 'bg-green-500';
            case BlockFlowType.Call:
                return 'bg-purple-500';
            case BlockFlowType.ExternCall:
                return 'bg-yellow-500';
            case BlockFlowType.ExternCallStore:
                return 'bg-red-500';
            default:
                return 'bg-gray-500';
        }
    }
</script>

<!-- Flow container -->
<div class="flow-container {getFlowTypeColor(flow.flowType)} p-4 rounded-lg shadow-md text-white">
    <div class="flex items-center justify-between mb-2">
        <h3 class="text-lg font-semibold">{getFlowTypeLabel(flow.flowType)}</h3>
        <span class="text-sm bg-black/20 px-2 py-1 rounded">{flow.flowId}</span>
    </div>
    <p class="text-sm mb-2">Pointer ID: {flow.flowPointerId}</p>
    {#if flow.value}
        <p class="text-sm">Value: {flow.value}</p>
    {/if}
</div>

<style>
    .flow-container {
        max-width: 300px;
        margin: 10px 0;
    }
</style>