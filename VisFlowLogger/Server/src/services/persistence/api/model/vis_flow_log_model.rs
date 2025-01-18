pub struct VisFlowLogModel {
    pub operation_id: String,
    pub block_name: String,
    pub log_type: String,
    pub value: Option<String>,
    pub sequence: u32,
}
pub struct VisFlowLogEntity {
    pub id: String,
    pub operation_id: String,
    pub block_name: String,
    pub log_type: String,
    pub value: Option<String>,
    pub sequence: u32,
}
