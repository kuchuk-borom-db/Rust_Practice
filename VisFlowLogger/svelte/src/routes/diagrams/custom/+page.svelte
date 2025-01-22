<script lang="ts">
    import {BlockFlowType} from '../../../models/blockData';
    import Block from '../../../components/Block.svelte'; // Adjust the import path as needed
    import Flow from '../../../components/Flow.svelte'; // Import the Flow component

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
    <p>This page demonstrates a single Block component and its flows.</p>

    <!-- Render the Block component -->
    <Block {blockID} {blockData}/>

    <h2 class="text-xl font-bold mt-6 mb-4">Flows</h2>

    <!-- Render a placeholder dummy Flow component for each flow -->
    <div class="flows-container">
        {#each blockData.flow as flow}
            <Flow {blockID} {flow}/>
        {/each}
    </div>
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

    .flows-container {
        margin-top: 20px;
        display: flex;
        flex-direction: column;
        gap: 10px; /* Space between flow components */
    }
</style>