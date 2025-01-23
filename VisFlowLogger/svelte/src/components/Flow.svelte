<script lang="ts">
    import type { BlockData, BlockFlow } from '../models/blockData';
    import { BlockFlowType } from '../models/blockData';
    import Block from './Block.svelte';
    import { slide } from 'svelte/transition';

    export let flow: BlockFlow;
    export let blocks: Record<string, BlockData>;
    let isExpanded = false; // Tracks whether the section is expanded or collapsed

    // Toggle expand/collapse
    function toggleExpand(event: MouseEvent) {
        event.stopPropagation();
        isExpanded = !isExpanded;
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
<div on:click={toggleExpand}
     class="flow-container {getFlowTypeColor(flow.flowType)} p-4 rounded-lg shadow-md text-white cursor-pointer relative bg-gray-800 transition-colors duration-200">
    <!-- Flow header -->
    <div class="flex items-center justify-between mb-2">
        <h3 class="text-lg font-semibold">{getFlowTypeLabel(flow.flowType)}</h3>
        <span class="text-sm bg-black/20 px-2 py-1 rounded">{flow.flowId}</span>
    </div>
    <p class="text-sm mb-2">Pointer ID: {flow.flowPointerId}</p>
    {#if flow.value}
        <p class="text-sm">Value: {flow.value}</p>
    {/if}

    <!-- Sub-block (collapsible) -->
    {#if isExpanded && flow.flowPointerId}
        <div transition:slide|local={{duration: 200}} class="sub-block-container relative opacity-0 animate-fade-in">
            <div class="connecting-line"></div>
            {#if flow.flowType === BlockFlowType.CallStore}
                <div class="stored-value-banner bg-green-700/50 p-2 rounded-t-lg">
                    <p class="text-sm font-semibold">Storing result: <span class="font-mono">{flow.value}</span></p>
                </div>
            {/if}
            <Block blockID={flow.flowPointerId} blockData={blocks[flow.flowPointerId]} {blocks}
                   isOpenedFromFlow={true}/>
        </div>
    {/if}
</div>

<style>
    .flow-container {
        margin: 10px 0;
        width: 100%;
        overflow: visible; /* Change to visible to prevent cutting off content */
    }

    .sub-block-container {
        margin-top: 10px;
        padding-left: 20px;
        overflow: visible; /* Change to visible to prevent cutting off content */
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
        margin-bottom: -10px;
    }

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
</style>