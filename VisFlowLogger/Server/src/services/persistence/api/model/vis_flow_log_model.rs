use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::fmt::Display;
#[derive(Serialize, Deserialize)]
pub struct VisFlowLogEntry {
    pub operation_id: String,
    pub block_name: String,
    pub log_type: String,
    pub log_value: Option<String>,
    pub sequence: u32,
}
#[derive(Serialize, Deserialize)]
pub struct VisFlowLogEntity {
    pub id: String,
    pub operation_id: String,
    pub block_name: String,
    pub log_type: String,
    pub log_value: Option<String>,
    pub sequence: u32,
}

impl VisFlowLogEntity {
    pub fn from_row(row: &PgRow) -> VisFlowLogEntity {
        VisFlowLogEntity {
            id: row.get("id"),
            operation_id: row.get("operation_id"),
            block_name: row.get("block_name"),
            log_type: row.get("log_type"),
            log_value: Some(row.get("log_value")),
            sequence: row.get::<i32, _>("sequence") as u32,
        }
    }
}

impl Display for VisFlowLogEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VisFlowLogEntity {{ id: {}, operation_id: {}, block_name: {}, log_type: {}, log_value: {}, sequence: {} }}",
            self.id,
            self.operation_id,
            self.block_name,
            self.log_type,
            self.log_value.as_deref().unwrap_or("None"),
            self.sequence
        )
    }
}

impl Display for VisFlowLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VisFlowLogEntry {{ operation_id: {}, block_name: {}, log_type: {}, log_value: {}, sequence: {} }}",
            self.operation_id,
            self.block_name,
            self.log_type,
            self.log_value.as_deref().unwrap_or("None"),
            self.sequence
        )
    }
}
