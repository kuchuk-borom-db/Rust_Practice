<script lang="ts">
    import type { BlockData } from '../models/blockData';
    import Flow from './Flow.svelte';

    export let blockID: string;
    export let blockData: BlockData;
    export let blocks: Record<string, BlockData>;
    export let isOpenedFromFlow: boolean;
    export let isHorizontal: boolean = false;
</script>

<!-- Block container -->
<div class="block-container {isHorizontal ? 'horizontal' : 'vertical'}">
    <h2 class="text-xl font-semibold mb-2">Block ID: {blockID}</h2>
    <p class="text-sm text-gray-300">Caller: {blockData.caller}</p>
    <p class="text-sm text-gray-300">Name: {blockData.name}</p>

    <!-- Render flows inside the block -->
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

    /* Vertical layout */
    .block-container.vertical {
        margin: 10px 0;
    }

    /* Horizontal layout */
    .block-container.horizontal {
        margin: 0 10px;
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
    }
</style>