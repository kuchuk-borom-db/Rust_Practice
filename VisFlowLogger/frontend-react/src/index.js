import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import MermaidPage from "./pages/visualiser/mermaid/Index";

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

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <MermaidPage code={data}/>
);

