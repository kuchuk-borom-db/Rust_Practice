import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import MermaidPage from "./pages/visualiser/mermaid/Index";
import DS3Diagram from "./pages/visualiser/reactFlow/Index";

const data =
    `
    flowchart TB
        subgraph aa0b31d1-eec0-41f8-9711-d25b5946d68d["addMoney"]
                03d64210-8544-4234-b4fa-6b59877e3259(["Adding money (100) to balance 1000"])
                69922f1e-8c76-4a0e-8538-bb17af3b7b81(["Balance = 1100"])
                03d64210-8544-4234-b4fa-6b59877e3259 ==> 69922f1e-8c76-4a0e-8538-bb17af3b7b81
        end
style aa0b31d1-eec0-41f8-9711-d25b5946d68d fill:#8D3EC1
        subgraph 32577473-bbe5-4839-b9e9-9c1b11a72582["calculateInterest"]
                95f44d65-0336-4cf3-aaa0-1af85eb05f85(["Calculating interest for 1100"])
                6c279785-9da0-495b-8fcd-be95ea9f910a(["Interest = 110"])
                95f44d65-0336-4cf3-aaa0-1af85eb05f85 ==> 6c279785-9da0-495b-8fcd-be95ea9f910a
        end
style 32577473-bbe5-4839-b9e9-9c1b11a72582 fill:#CF4723
        subgraph f69b39ae-7d96-4f58-8491-d3d167a745d5["getCompoundInterest"]
                3cec1732-0033-49ce-902a-309502262bf1(["Getting CI for 1100, 1"])
                07e3f9f5-8dc1-404a-b0a6-5b767f6460dc[/"interest = 110"/]
                3cec1732-0033-49ce-902a-309502262bf1 ==> 07e3f9f5-8dc1-404a-b0a6-5b767f6460dc
                9ad392b7-400d-4b72-b0c4-899edb548563[/"Updated Balance = 1210"/]
                07e3f9f5-8dc1-404a-b0a6-5b767f6460dc ==> 9ad392b7-400d-4b72-b0c4-899edb548563
                273d06a1-8970-4b33-bfc1-f66a071828a1[/"CI = 1210"/]
                9ad392b7-400d-4b72-b0c4-899edb548563 ==> 273d06a1-8970-4b33-bfc1-f66a071828a1
        end
style f69b39ae-7d96-4f58-8491-d3d167a745d5 fill:#CF99E7
        subgraph e8b69b46-753b-440a-b886-adff917560c6["addMoney"]
                4111ee00-1fba-42d4-9dfa-8e86b0e98c32(["Adding money (110) to balance 1100"])
                e18e5db6-a07d-4682-8e67-be5ffb1c20b1(["Balance = 1210"])
                4111ee00-1fba-42d4-9dfa-8e86b0e98c32 ==> e18e5db6-a07d-4682-8e67-be5ffb1c20b1
        end
style e8b69b46-753b-440a-b886-adff917560c6 fill:#931798
        subgraph 3d3d267d-2636-40b1-8704-8c224fe60b63["logTransaction"]
                13fd52e9-62f0-40cf-9830-9b513e9e369e(["Logging transaction 2025-01-15T04:04:35.958Z: Investment matured: $1210"])
        end
style 3d3d267d-2636-40b1-8704-8c224fe60b63 fill:#89DD3B
        subgraph 1f7f5e73-ec71-4f28-8de9-0b2fe396840e["getCompoundInterest"]
                1945901c-d40e-43e1-a52a-c592359b4ab1(["Getting CI for 1000, 2"])
                da39b64b-93e2-4d6c-9533-0726c93ada4d[/"interest = 100"/]
                1945901c-d40e-43e1-a52a-c592359b4ab1 ==> da39b64b-93e2-4d6c-9533-0726c93ada4d
                b3f726bb-490e-4ecd-9624-655992ef4c6d[/"Updated Balance = 1100"/]
                da39b64b-93e2-4d6c-9533-0726c93ada4d ==> b3f726bb-490e-4ecd-9624-655992ef4c6d
                3c392ccf-0577-4758-8151-f1932910e814[/"CI = 1210"/]
                b3f726bb-490e-4ecd-9624-655992ef4c6d ==> 3c392ccf-0577-4758-8151-f1932910e814
        end
style 1f7f5e73-ec71-4f28-8de9-0b2fe396840e fill:#BC5A86
        subgraph START["calculateInvestment"]
                17ee683d-da14-4938-bb87-6e46169b4000(["Calculating investment for 1000"])
                efda083f-dfc3-4d04-8569-3659ce331ca3[/"final Amount = 1210"/]
                17ee683d-da14-4938-bb87-6e46169b4000 ==> efda083f-dfc3-4d04-8569-3659ce331ca3
                63918667-0ba5-4a9c-987f-a3cb4e481c11["logTransaction"]
                efda083f-dfc3-4d04-8569-3659ce331ca3 ==> 63918667-0ba5-4a9c-987f-a3cb4e481c11
        end
style START fill:#885B23
        subgraph 34e62709-eafb-409c-9979-890ba135adb4["calculateInterest"]
                10ee817e-3e42-41aa-bc40-3c3448354614(["Calculating interest for 1000"])
                f1bfb3f9-3cb0-4d35-bcbf-807ec2981fa1(["Interest = 100"])
                10ee817e-3e42-41aa-bc40-3c3448354614 ==> f1bfb3f9-3cb0-4d35-bcbf-807ec2981fa1
        end
style 34e62709-eafb-409c-9979-890ba135adb4 fill:#4A29A3
        subgraph 5d86cbbc-dde3-4391-8eac-700db905dc07["getCompoundInterest"]
                d92dd30e-bd22-4b82-be00-6461001ef9e1(["Getting CI for 1210, 0"])
                6e4eb34b-667c-4d9f-9561-29e9c83a6419(["Year == 0. Returning 1210"])
                d92dd30e-bd22-4b82-be00-6461001ef9e1 ==> 6e4eb34b-667c-4d9f-9561-29e9c83a6419
        end
style 5d86cbbc-dde3-4391-8eac-700db905dc07 fill:#203318





07e3f9f5-8dc1-404a-b0a6-5b767f6460dc ...-o 95f44d65-0336-4cf3-aaa0-1af85eb05f85
6c279785-9da0-495b-8fcd-be95ea9f910a ---> 07e3f9f5-8dc1-404a-b0a6-5b767f6460dc
9ad392b7-400d-4b72-b0c4-899edb548563 ...-o 4111ee00-1fba-42d4-9dfa-8e86b0e98c32
e18e5db6-a07d-4682-8e67-be5ffb1c20b1 ---> 9ad392b7-400d-4b72-b0c4-899edb548563
273d06a1-8970-4b33-bfc1-f66a071828a1 ...-o d92dd30e-bd22-4b82-be00-6461001ef9e1
6e4eb34b-667c-4d9f-9561-29e9c83a6419 ---> 273d06a1-8970-4b33-bfc1-f66a071828a1




da39b64b-93e2-4d6c-9533-0726c93ada4d ...-o 10ee817e-3e42-41aa-bc40-3c3448354614
f1bfb3f9-3cb0-4d35-bcbf-807ec2981fa1 ---> da39b64b-93e2-4d6c-9533-0726c93ada4d
b3f726bb-490e-4ecd-9624-655992ef4c6d ...-o 03d64210-8544-4234-b4fa-6b59877e3259
69922f1e-8c76-4a0e-8538-bb17af3b7b81 ---> b3f726bb-490e-4ecd-9624-655992ef4c6d
3c392ccf-0577-4758-8151-f1932910e814 ...-o 3cec1732-0033-49ce-902a-309502262bf1
273d06a1-8970-4b33-bfc1-f66a071828a1 ---> 3c392ccf-0577-4758-8151-f1932910e814

efda083f-dfc3-4d04-8569-3659ce331ca3 ...-o 1945901c-d40e-43e1-a52a-c592359b4ab1
3c392ccf-0577-4758-8151-f1932910e814 ---> efda083f-dfc3-4d04-8569-3659ce331ca3
63918667-0ba5-4a9c-987f-a3cb4e481c11 ...-o 13fd52e9-62f0-40cf-9830-9b513e9e369e





BEGIN(("START")):::starting ==> 17ee683d-da14-4938-bb87-6e46169b4000
 63918667-0ba5-4a9c-987f-a3cb4e481c11 ==> END(("END")):::starting
classDef starting fill:#FF5733;

    `


const graph = {
    "START": {
        "caller": null,
        "name": "main",
        "flow": [
            {
                "flow_pointer_id": null,
                "flow_id": "e14e2dee-9c6a-4644-8481-8f0d5a74aabc",
                "flow_type": "Log",
                "value": "Adding 2 and 1"
            },
            {
                "flow_pointer_id": "9ac3b255-97a2-4310-a795-45559b27c851",
                "flow_id": "e1daebe5-f55d-4c35-bffe-7dd2db8e06f9",
                "flow_type": "CallStore",
                "value": "sum = 3"
            },
            {
                "flow_pointer_id": "65351ad5-060c-428c-9d07-0a3685cb4ba0",
                "flow_id": "a805b838-221f-4057-b78f-3318d7dc1a02",
                "flow_type": "Call",
                "value": null
            }
        ]
    },
    "9ac3b255-97a2-4310-a795-45559b27c851": {
        "caller": "START",
        "name": "sum",
        "flow": [
            {
                "flow_pointer_id": null,
                "flow_id": "11b27065-fe51-4239-ad08-f25c1b1e9df8",
                "flow_type": "Log",
                "value": "2 + 1 = 3"
            },
            {
                "flow_pointer_id": "9dd4c31a-e4e4-47c8-a8bd-599121c98c5e",
                "flow_id": "675d7f8b-38dc-4cc2-95b2-6402c6f46c8a",
                "flow_type": "Call",
                "value": null
            }
        ]
    },
    "9dd4c31a-e4e4-47c8-a8bd-599121c98c5e": {
        "caller": "9ac3b255-97a2-4310-a795-45559b27c851",
        "name": "sum",
        "flow": [
            {
                "flow_pointer_id": null,
                "flow_id": "dedbeb82-b326-48ed-9e92-67fb13521941",
                "flow_type": "Log",
                "value": "1 + 1 = 2"
            }
        ]
    },
    "65351ad5-060c-428c-9d07-0a3685cb4ba0": {
        "caller": "START",
        "name": "foo",
        "flow": [
            {
                "flow_pointer_id": null,
                "flow_id": "e0188ee4-741c-49a7-979f-76d69c206c5a",
                "flow_type": "Log",
                "value": "foo called"
            }
        ]
    }
}

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <DS3Diagram data={graph}/>
);

/*
this is a jsx page that can view blocks and nodes. flow nodes of a block may call another block or even callStore.
If its a call then its simply calling another block and then moving on

If its callStore then its calling that block's first flow node and the last flow node of the called block must point back to the caller flow node.

Currently there is no indicator showing the last flowNode of a called block returning back to the caller flow node of parent block.

Do what you think is necessary to make callStore's operation directed showing the return value i.e last flow node of called block being returned to caller flow node
And instead of horizontal view occupying while width make it use only as much as required and expand when its inner content expand and add a zoom in and zoom out and moving around feature
Write the whole code
 */

/*
this is a jsx page that can view blocks and nodes. flow nodes of a block may call another block or even callStore.
If its a call then its simply calling another block and then moving on

If its callStore then its calling that block's first flow node and the last flow node of the called block must point back to the caller flow node.

currently its custom compoents being used to show the flow but i want you to rewrite the whole thing from scratch using reactFlow
 but keeping the same style. Use arrows to represent connections and instead of nested blocks. And flow nodes should be inside their respective blocks with arrows pointing properly

 a call / callStore flow node of a block will pop up another block which will be the vlock that was called by the caller flow node
  and its going to be linked using arrows between the caller flow node and the called block's firest flow node.
  if its callStore then the last flow node of the called block needs to point back an arrow to the caller flow node. thats how we show that its return value is used.
  We should have the ability to open and close the flow node of type call and callStore
 */