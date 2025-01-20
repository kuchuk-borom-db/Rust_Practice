import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Diagram from "./pages/visualiser/reactFlow/Diagram";

const data = {
    "9e1a92ed-64de-4ee3-a3fc-189b993ba559": {
        caller: "60ce68ed-a1bd-4a5e-8a40-bef5d8f5ddde",
        name: "sum",
        flow: [
            {
                flow_pointer_id: null,
                flow_id: "ac0d814c-ad94-4166-8698-23d53d4dbffd",
                flow_type: "Log",
                value: "1 + 1 = 2",
            },
        ],
    },
    START: {
        caller: null,
        name: "main",
        flow: [
            {
                flow_pointer_id: null,
                flow_id: "7745bd7b-ecec-484a-ae39-0c5eb657a181",
                flow_type: "Log",
                value: "Adding 2 and 1",
            },
            {
                flow_pointer_id: "60ce68ed-a1bd-4a5e-8a40-bef5d8f5ddde",
                flow_id: "a31b67f4-92ff-4f06-8864-e2bba31c7bbf",
                flow_type: "CallStore",
                value: "sum = 3",
            },
            {
                flow_pointer_id: "16caf84f-8836-44d0-851e-5a9d8763f7f7",
                flow_id: "27e46faa-c117-4753-b1d2-ee1028020641",
                flow_type: "Call",
                value: null,
            },
        ],
    },
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <Diagram data={data}/>, document.getElementById("root")
);

