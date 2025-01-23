<script lang="ts">
    // Use the imported test data
    import Block from "../../../components/Block.svelte";
    import {type BlockData, testData} from "../../../models/blockData";

    const blocks: Record<string, BlockData> = testData;
</script>

<h1 class="text-2xl font-bold mb-6 text-white">Stack Diagram Visualization</h1>

<Block
        blockID="START"
        blockData={blocks["START"]}
        {blocks}
        isOpenedFromFlow={false}
/>


<style>
    :global(body) {
        background-color: #35112f;
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: column;
        align-items: center;
        min-height: max-content;
        min-width: max-content;
        overflow-y: auto; /* Allow vertical scrolling */
    }
</style>