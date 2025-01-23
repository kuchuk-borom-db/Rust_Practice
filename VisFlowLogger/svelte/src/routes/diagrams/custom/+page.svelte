
<script lang="ts">
    // Use the imported test data
    import Block from "../../../components/Block.svelte";
    import {type BlockData, testData} from "../../../models/blockData";

    const blocks: Record<string, BlockData> = testData;
</script>

<div class="stack-diagram-container">
    <h1 class="text-2xl font-bold mb-6 text-white">Stack Diagram Visualization</h1>

    <Block
            blockID="START"
            blockData={blocks["START"]}
            {blocks}
            isOpenedFromFlow={false}
    />
</div>

<style>

</style>
