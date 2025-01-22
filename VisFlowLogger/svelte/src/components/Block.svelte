<script lang="ts">
    import type { BlockData } from '../models/blockData';
    import Flow from './Flow.svelte';

    export let blockID: string;
    export let blockData: BlockData;
    export let blocks: Record<string, BlockData>;
    export let isOpenedFromFlow: boolean;
</script>

<!-- Glassmorphism container for the block using Tailwind -->
<div class="block-container bg-gray-700 rounded-lg shadow-md p-6 hover:bg-gray-600 transition-colors duration-200">
    <h2 class="text-xl font-semibold mb-2">Block ID: {blockID}</h2>
    <p class="text-sm text-gray-300">Caller: {blockData.caller}</p>
    <p class="text-sm text-gray-300">Name: {blockData.name}</p>

    <!-- Render flows inside the block -->
    <div class="flows-container mt-4">
        {#each blockData.flow as flow}
            <Flow {blockID} {flow} {blocks} />
        {/each}
    </div>
</div>

<style>
    .block-container {
        position: relative;
        color: white;
        overflow: hidden;
        width: fit-content; /* Allow the block to expand based on content */
        min-width: 300px; /* Set a minimum width for better readability */
    }

    h2 {
        margin: 0 0 10px 0;
        font-size: 1.5em;
    }

    p {
        margin: 5px 0;
        font-size: 1em;
        color: rgba(255, 255, 255, 0.8);
    }

    .flows-container {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }
</style>