<script lang="ts">
    import type { BlockData } from '../models/blockData';
    import Flow from './Flow.svelte';

    export let blockID: string;
    export let blockData: BlockData;
    export let blocks: Record<string, BlockData>;
    export let isOpenedFromFlow: boolean;
</script>

<!-- Block container with neon animating gradient border -->
<div class="block-container">
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
        padding: 1.5rem; /* Padding inside the block */
        background: #1a202c; /* Dark background for the block */
        border-radius: 0.5rem; /* Rounded corners */
    }

    /* Neon animating gradient border */
    .block-container::before {
        content: '';
        position: absolute;
        top: -2px; /* Offset for the border */
        left: -2px; /* Offset for the border */
        right: -2px; /* Offset for the border */
        bottom: -2px; /* Offset for the border */
        z-index: -1; /* Place behind the content */
        background: linear-gradient(45deg, #3b82f6, #10b981, #f59e0b, #3b82f6);
        background-size: 300% 300%; /* Larger background size for animation */
        border-radius: inherit; /* Match the border radius of the container */
        animation: neon-border 4s linear infinite; /* Animation for the gradient */
    }

    /* Neon glow effect */
    .block-container::after {
        content: '';
        position: absolute;
        top: -2px;
        left: -2px;
        right: -2px;
        bottom: -2px;
        z-index: -2; /* Place behind the gradient border */
        background: inherit;
        border-radius: inherit;
        filter: blur(10px); /* Blur effect for the glow */
        opacity: 0.5; /* Semi-transparent glow */
        animation: neon-border 4s linear infinite; /* Sync with the border animation */
    }

    /* Keyframes for the neon border animation */
    @keyframes neon-border {
        0% {
            background-position: 0% 50%;
        }
        50% {
            background-position: 100% 50%;
        }
        100% {
            background-position: 0% 50%;
        }
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
        gap: 20px;
    }
</style>