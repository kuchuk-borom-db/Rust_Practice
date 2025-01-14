import {AsyncLocalStorage} from "node:async_hooks";

/**
 * AsyncLocalStorage is used to create Task Scoped storage space. This enables us to isolate independent call stacks.
 * AsyncLocalStorage.getStore() returns the store for that specific task scope.
 */
export class KukuVisualFlowLoggerManual {

    async run(fn) {
        return await this.asyncLocalStorage.run({
            operationId: this._generateOperationID(),
            entries: []
        }, async () => {
            const result = await fn();
            await this.sendLogToServer();
            return result;
        });
    }

    constructor(config) {
        this.asyncLocalStorage = new AsyncLocalStorage();
    }

    _generateOperationID() {
        return `${Date.now()}-${Math.random().toString(36).substring(2, 15)}`;
    }

    START = (name) => {
        this.asyncLocalStorage.getStore().entries.push(new VisLogEntry(this.asyncLocalStorage.getStore().operationId, name, "START", null));
    }
    END = (name) => {
        this.asyncLocalStorage.getStore().entries.push(new VisLogEntry(this.asyncLocalStorage.getStore().operationId, name, "END", null));
        console.log(`\nEND of ${name} with stack ${JSON.stringify(this.asyncLocalStorage.getStore().entries)}`);
    }
    STORE = (name) => {
        this.asyncLocalStorage.getStore().entries.push(new VisLogEntry(this.asyncLocalStorage.getStore().operationId, name, "STORE", null));
    }
    LOG = (name, log) => {
        this.asyncLocalStorage.getStore().entries.push(new VisLogEntry(this.asyncLocalStorage.getStore().operationId, name, "LOG", log));
    }

    async sendLogToServer() {
        const entries = this.asyncLocalStorage.getStore().entries;
    }
}


class VisLogEntry {
    constructor(operationID, name, logType, value) {
        this.operationId = operationID;
        this.name = name;
        this.logType = logType;
        this.value = value;
    }
}


