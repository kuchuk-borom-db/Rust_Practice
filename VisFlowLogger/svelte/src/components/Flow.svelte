<script lang="ts">
    import type {BlockData, BlockFlow} from '../models/blockData';
    import {BlockFlowType} from '../models/blockData';
    import Block from './Block.svelte';
    import {slide} from 'svelte/transition'; // Import slide transition

    export let flow: BlockFlow;
    export let blocks: Record<string, BlockData>;
    export let blockID: string;

    let showSubBlock = false;

    // Toggle between flow and sub-block
    function toggleSubBlock(event: MouseEvent) {
        event.stopPropagation();

        if (flow.flowType === BlockFlowType.Call || flow.flowType === BlockFlowType.CallStore) {
            showSubBlock = !showSubBlock;
        }
    }

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
                return 'bg-red-400';
            case BlockFlowType.Call:
                return 'bg-pink-600';
            case BlockFlowType.ExternCall:
                return 'bg-yellow-600';
            case BlockFlowType.ExternCallStore:
                return 'bg-red-600';
            default:
                return 'bg-gray-600';
        }
    }
</script>

<!-- Flow container -->
<div on:click={toggleSubBlock}
     class="flow-container {getFlowTypeColor(flow.flowType)} hover-{flow.flowType} p-4 rounded-lg shadow-md text-white cursor-pointer relative bg-gray-800 transition-colors duration-200">
    {#if showSubBlock && flow.flowPointerId}
        <!-- Render the sub-block with slide transition and fade effect -->
        <div transition:slide|local={{duration: 200}} class="sub-block-container relative opacity-0 animate-fade-in">
            <!-- Connecting line -->
            <div class="connecting-line"></div>
            <Block blockID={flow.flowPointerId} blockData={blocks[flow.flowPointerId]} {blocks} isOpenedFromFlow={true}/>
            {#if flow.flowType === BlockFlowType.CallStore}
                <!-- Special representation for CallStore sub-blocks -->
                <div class="stored-value-banner bg-green-700/50 p-2 rounded-t-lg">
                    <p class="text-sm font-semibold">Storing result: <span class="font-mono">{flow.value}</span></p>
                </div>
            {/if}
        </div>
    {:else}
        <!-- Render the flow details -->
        <div class="flex items-center justify-between mb-2">
            <h3 class="text-lg font-semibold">{getFlowTypeLabel(flow.flowType)}</h3>
            <span class="text-sm bg-black/20 px-2 py-1 rounded">{flow.flowId}</span>
        </div>
        <p class="text-sm mb-2">Pointer ID: {flow.flowPointerId}</p>
        {#if flow.value}
            <p class="text-sm">Value: {flow.value}</p>
        {/if}
    {/if}
</div>

<style>
    .flow-container {
        margin: 10px 0;
        width: 100%; /* Allow the flow to expand based on content */
    }

    .sub-block-container {
        margin-top: 10px;
        padding-left: 20px; /* Indent the sub-block */
    }

    .connecting-line {
        position: absolute;
        left: 0;
        top: 0;
        bottom: 0;
        width: 2px;
        background-color: rgba(255, 255, 255, 0.3);
    }

    .stored-value-banner {
        margin-bottom: -10px; /* Overlap slightly with the sub-block */
    }

    /* Fade-in animation */
    @keyframes fade-in {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }

    .animate-fade-in {
        animation: fade-in 200ms ease-out forwards;
    }

    /* Hover classes for each flow type */
    .hover-BlockFlowType.Log:hover {
        background-color: #3b82f6; /* Lighter blue */
    }

    .hover-BlockFlowType.CallStore:hover {
        background-color: #ef4444; /* Lighter red */
    }

    .hover-BlockFlowType.Call:hover {
        background-color: #db2777; /* Lighter pink */
    }

    .hover-BlockFlowType.ExternCall:hover {
        background-color: #eab308; /* Lighter yellow */
    }

    .hover-BlockFlowType.ExternCallStore:hover {
        background-color: #dc2626; /* Lighter red */
    }

    .hover-BlockFlowType.Unknown:hover {
        background-color: #6b7280; /* Lighter gray */
    }
</style>