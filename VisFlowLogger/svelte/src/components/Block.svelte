<script lang="ts">
    import type { BlockData } from '../models/blockData'; // Adjust the import path as needed

    export let blockID: string;
    export let blockData: BlockData;
</script>

<!-- Glassmorphism container for the block using Tailwind -->
<div class="block-container">
    <h2 class="text-xl font-semibold mb-2">Block ID: {blockID}</h2>
    <p class="text-sm text-gray-300">Caller: {blockData.caller}</p>
    <p class="text-sm text-gray-300">Name: {blockData.name}</p>
    <!-- Add more details or logic here later -->
</div>

<style>
    .block-container {
        position: relative;
        background: linear-gradient(45deg, rgba(99, 102, 241, 0.8), rgba(168, 85, 247, 0.8), rgba(236, 72, 153, 0.8), rgba(99, 102, 241, 0.8));
        background-size: 400% 400%; /* Larger background size for more movement */
        border-radius: 15px; /* Rounded corners */
        border: 1px solid rgba(255, 255, 255, 0.2); /* Light border */
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Subtle shadow */
        padding: 20px;
        max-width: 400px; /* Adjust as needed */
        margin: 20px auto; /* Center the block */
        color: white; /* Text color */
        overflow: hidden; /* Ensure the gradient doesn't overflow */
        animation: gradientAnimation 6s ease infinite; /* Faster animation */
    }

    /* Gradient animation */
    @keyframes gradientAnimation {
        0% {
            background-position: 0 50%;
        }
        50% {
            background-position: 100% 50%;
        }
        100% {
            background-position: 0 50%;
        }
    }

    h2 {
        margin: 0 0 10px 0;
        font-size: 1.5em;
    }

    p {
        margin: 5px 0;
        font-size: 1em;
        color: rgba(255, 255, 255, 0.8); /* Slightly transparent text */
    }
</style>