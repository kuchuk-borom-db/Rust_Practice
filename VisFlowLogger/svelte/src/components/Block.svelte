<script lang="ts">
    import type { BlockData } from '../models/blockData';
    import Flow from './Flow.svelte';
    import { dragState } from './stores/DiagramStores';

    export let blockID: string;
    export let blockData: BlockData;
    export let blocks: Record<string, BlockData>;
    export let isOpenedFromFlow: boolean;

    let isHorizontal = false;

    function toggleOrientation(event: MouseEvent) {
        event.stopPropagation();
        if (!$dragState.isDragging) {
            isHorizontal = !isHorizontal;
        }
    }
</script>

<div class="block-container {isHorizontal ? 'horizontal' : 'vertical'}">
    <div class="flex items-center justify-between mb-4">
        <h2 class="text-xl font-semibold">Block ID: {blockID}</h2>
        <button
                on:click={toggleOrientation}
                class="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded-lg text-white text-sm transition-colors"
                class:pointer-events-none={$dragState.isDragging}
        >
            {isHorizontal ? 'Vertical' : 'Horizontal'}
        </button>
    </div>
    <p class="text-sm text-gray-300">Caller: {blockData.caller}</p>
    <p class="text-sm text-gray-300">Name: {blockData.name}</p>

    <div class="flows-container {isHorizontal ? 'horizontal' : 'vertical'} mt-4">
        {#each blockData.flow as flow}
            <Flow {blockID} {flow} {blocks} {isHorizontal} />
        {/each}
    </div>
</div>

<style>
    .block-container {
        position: relative;
        color: white;
        overflow: hidden;
        width: fit-content;
        min-width: 300px;
        padding: 1.5rem;
        background: #1a202c;
        border-radius: 0.5rem;
    }

    .block-container.vertical {
        margin: 10px 0;
    }

    .block-container.horizontal {
        margin: 0 10px;
        white-space: nowrap;
    }

    .flows-container.vertical {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .flows-container.horizontal {
        display: flex;
        flex-direction: row;
        gap: 20px;
        align-items: flex-start;
    }
</style>