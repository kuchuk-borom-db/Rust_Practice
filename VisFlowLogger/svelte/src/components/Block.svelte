<script lang="ts">
    import type {BlockData} from '../models/blockData';
    import Flow from './Flow.svelte';

    export let blockID: string;
    export let blockData: BlockData;
    export let blocks: Record<string, BlockData>;
    export let isOpenedFromFlow: boolean;

    // Per-block orientation state
    let isHorizontal = false;

    // Function to toggle orientation for this block
    function toggleOrientation(event: MouseEvent) {
        event.stopPropagation(); // Stop event propagation
        isHorizontal = !isHorizontal;
    }
</script>

<!-- Block container -->
<div class="block-container {isHorizontal ? 'horizontal' : 'vertical'}">
    <!-- Block header -->
    <div class="flex items-center justify-between mb-4">
        <h2 class="text-xl font-semibold">Block ID: {blockID}</h2>
        <!-- Orientation toggle button -->
        <button
                on:click={toggleOrientation}
                class="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded-lg text-white text-sm transition-colors"
        >
            {isHorizontal ? 'Vertical' : 'Horizontal'}
        </button>
    </div>
    <p class="text-sm text-gray-300">Caller: {blockData.caller}</p>
    <p class="text-sm text-gray-300">Name: {blockData.name}</p>

    <!-- Render flows inside the block -->
    <div class="flows-container {isHorizontal ? 'horizontal' : 'vertical'} mt-4">
        {#each blockData.flow as flow}
            <Flow {blockID} {flow} {blocks} {isHorizontal}/>
        {/each}
    </div>
</div>

<style>
    .block-container {
        position: relative;
        color: white;
        overflow: visible; /* Allow content to overflow */
        width: 100%; /* Expand to fill the container */
        height: 100%; /* Expand to fill the container */
        padding: 1.5rem;
        background: #1a202c;
        border-radius: 0.5rem;
    }

    /* Vertical layout */
    .block-container.vertical {
        margin: 10px 0;
    }

    /* Horizontal layout */
    .block-container.horizontal {
        margin: 0 10px;
        white-space: nowrap; /* Prevent text wrapping */
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
        align-items: flex-start; /* Align items to the top */
    }
</style>