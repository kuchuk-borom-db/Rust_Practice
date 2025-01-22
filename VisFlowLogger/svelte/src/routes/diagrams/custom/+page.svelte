<script lang="ts">
    import Block from '../../../components/Block.svelte';
    import {type BlockData, BlockFlowType} from "../../../models/blockData"; // Adjust the import path as needed


    const blocks: Record<string, BlockData> = {
        "START": {
            caller: null, // Note: caller is `null` in the data, but the interface expects `string`. You may need to update the interface.
            name: "main",
            flow: [
                {
                    flowPointerId: null, // Note: flowPointerId is `null` in the data, but the interface expects `string`. You may need to update the interface.
                    flowId: "e14e2dee-9c6a-4644-8481-8f0d5a74aabc",
                    flowType: BlockFlowType.Log, // Convert "Log" to BlockFlowType.Log
                    value: "Adding 2 and 1"
                },
                {
                    flowPointerId: "9ac3b255-97a2-4310-a795-45559b27c851",
                    flowId: "e1daebe5-f55d-4c35-bffe-7dd2db8e06f9",
                    flowType: BlockFlowType.CallStore, // Convert "CallStore" to BlockFlowType.CallStore
                    value: "sum = 3"
                },
                {
                    flowPointerId: "65351ad5-060c-428c-9d07-0a3685cb4ba0",
                    flowId: "a805b838-221f-4057-b78f-3318d7dc1a02",
                    flowType: BlockFlowType.Call, // Convert "Call" to BlockFlowType.Call
                    value: null
                }
            ]
        },
        "9ac3b255-97a2-4310-a795-45559b27c851": {
            caller: "START",
            name: "sum",
            flow: [
                {
                    flowPointerId: null,
                    flowId: "11b27065-fe51-4239-ad08-f25c1b1e9df8",
                    flowType: BlockFlowType.Log, // Convert "Log" to BlockFlowType.Log
                    value: "2 + 1 = 3"
                },
                {
                    flowPointerId: "9dd4c31a-e4e4-47c8-a8bd-599121c98c5e",
                    flowId: "675d7f8b-38dc-4cc2-95b2-6402c6f46c8a",
                    flowType: BlockFlowType.Call, // Convert "Call" to BlockFlowType.Call
                    value: null
                }
            ]
        },
        "9dd4c31a-e4e4-47c8-a8bd-599121c98c5e": {
            caller: "9ac3b255-97a2-4310-a795-45559b27c851",
            name: "sum",
            flow: [
                {
                    flowPointerId: null,
                    flowId: "dedbeb82-b326-48ed-9e92-67fb13521941",
                    flowType: BlockFlowType.Log, // Convert "Log" to BlockFlowType.Log
                    value: "1 + 1 = 2"
                }
            ]
        },
        "65351ad5-060c-428c-9d07-0a3685cb4ba0": {
            caller: "START",
            name: "foo",
            flow: [
                {
                    flowPointerId: null,
                    flowId: "e0188ee4-741c-49a7-979f-76d69c206c5a",
                    flowType: BlockFlowType.Log, // Convert "Log" to BlockFlowType.Log
                    value: "foo called"
                }
            ]
        }
    };

</script>


<main class="p-4">
    <h1 class="text-2xl font-bold mb-4">Dummy Page</h1>
    <p class="text-gray-300 mb-6">This page demonstrates multiple Block components with their flows.</p>

    <!-- Render each block -->
    {#each Object.entries(blocks) as [blockID, blockData]}
        <Block {blockID} {blockData} />
    {/each}
</main>

<style>
    main {
        font-family: Arial, sans-serif;
    }

    h1 {
        color: #333;
    }

    p {
        color: #666;
    }
</style>