use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct VisFlowLogModel {
    pub operation_id: String,
    pub block_name: String,
    pub log_type: String,
    pub value: Option<String>,
    pub sequence: u32,
}
impl VisFlowLogModel {
    pub fn from_row(row: &PgRow) -> VisFlowLogModel {
        VisFlowLogModel {
            operation_id: row.get("operation_id"),
            block_name: row.get("block_name"),
            log_type: row.get("log_type"),
            value: row.get("value"),
            sequence: row.get::<i32, _>("sequence") as u32,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct VisFlowLogEntity {
    pub id: String,
    pub operation_id: String,
    pub block_name: String,
    pub log_type: String,
    pub value: Option<String>,
    pub sequence: u32,
}
