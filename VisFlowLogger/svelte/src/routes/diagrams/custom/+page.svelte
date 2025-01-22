<script lang="ts">
    import {BlockFlowType} from '../../../models/blockData';
    import Block from "../../../components/Block.svelte"; // Adjust the import path as needed

    // Example block data
    const blockID = 'exampleBlockID';
    const blockData = {
        caller: 'exampleCaller',
        name: 'exampleName',
        flow: [
            {
                flowPointerId: 'pointer1',
                flowId: 'flow1',
                flowType: BlockFlowType.Log,
                value: 'This is a log flow',
            },
            {
                flowPointerId: 'pointer2',
                flowId: 'flow2',
                flowType: BlockFlowType.CallStore,
                value: 'This is a call store flow',
            },
        ],
    };
</script>

<main>
    <h1>Dummy Page</h1>
    <p>This page demonstrates a single Block component.</p>

    <!-- Render the Block component -->
    <Block {blockID} {blockData}/>
</main>

<style>
    main {
        padding: 20px;
        font-family: Arial, sans-serif;
    }

    h1 {
        color: #333;
    }

    p {
        color: #666;
    }
</style>