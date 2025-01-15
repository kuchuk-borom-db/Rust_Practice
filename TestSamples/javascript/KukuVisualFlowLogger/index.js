import {AsyncLocalStorage} from "node:async_hooks";

/**
 * AsyncLocalStorage is used to create Task Scoped storage space. This enables us to isolate independent call stacks.
 * AsyncLocalStorage.getStore() returns the store for that specific task scope.
 */
export class KukuVisualFlowLoggerManual {

    async run(fn) {
        return await this.asyncLocalStorage.run({
            "operationId": this._generateOperationID(),
            "entries": [],
            "counter": 0,
        }, async () => {
            const result = await fn();
            await this.sendLogToServer();
            return result;
        });
    }

    _getStoreValue(name) {
        const store = this.asyncLocalStorage.getStore();
        return store[name];
    }

    constructor(url) {
        this.asyncLocalStorage = new AsyncLocalStorage();
        this.url = url;
    }

    _generateOperationID() {
        return `${Date.now()}-${Math.random().toString(36).substring(2, 15)}`;
    }

    _getCounter() {
        const current = this.asyncLocalStorage.getStore()["counter"];
        this.asyncLocalStorage.getStore()["counter"] += 1;
        return current;
    }

    START = (name) => {
        this.asyncLocalStorage.getStore()["entries"].push(new VisLogEntry(this._getStoreValue("operationId"), name, "START", null, this._getCounter()));
    }
    END = (name) => {
        this.asyncLocalStorage.getStore()["entries"].push(new VisLogEntry(this._getStoreValue("operationId"), name, "END", null, this._getCounter()));
    }
    STORE = (name, value) => {
        this.asyncLocalStorage.getStore()["entries"].push(new VisLogEntry(this._getStoreValue("operationId"), name, "STORE", value, this._getCounter()));
    }
    log = (name, log) => {
        this.asyncLocalStorage.getStore()["entries"].push(new VisLogEntry(this._getStoreValue("operationId"), name, "LOG", log, this._getCounter()));
    }

    async sendLogToServer() {
        const entries = this.asyncLocalStorage.getStore()["entries"];
        const jsonEntries = JSON.stringify(entries);
        const response = await fetch(this.url, {
            body: jsonEntries,
            headers: {
                'Content-Type': 'application/json'
            },
            method: "POST"
        });

        if (!response.ok) {
            console.log(`\nError = ${response.statusText}\n`);
            return;
        }
        const base64Response = await response.text();
        console.log(base64Response);
        const decodedDiagram = atob(base64Response); // Decode base64 to string
        console.log(decodedDiagram);
        return decodedDiagram;
    }
}


class VisLogEntry {
    constructor(operationID, name, logType, value, sequence) {
        this.operationId = operationID;   // Maps to operation_id in JSON
        this.name = name;
        this.logType = logType;           // Maps to log_type in JSON
        this.value = value;
        this.sequence = sequence;        // Matches sequence
    }
}
//TODO Add a way to define START and it returns a custom mini logger which can be used to log stuff and then call log.end with it to end the function call

