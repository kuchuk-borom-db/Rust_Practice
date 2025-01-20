import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import FlowGraph from "./pages/VisualFlowInteractive";

const data = {
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
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <FlowGraph data={data}/>
);

