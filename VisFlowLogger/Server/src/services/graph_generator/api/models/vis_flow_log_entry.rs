pub enum VisFlowLogEntryLogType {
    Log,
    Store,
    ExternalCall,
    ExternalCallStore,
    Start,
    End,
}
pub struct VisFlowLogEntry {
    pub block_name: String,
    pub log_type: VisFlowLogEntryLogType,
    pub log_value: Option<String>,
}
