use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize)]
pub struct VisFlowOperationEntity {
    pub operation_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl VisFlowOperationEntity {
    pub(crate) fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        VisFlowOperationEntity {
            operation_id: row.get("id"),
            created_at: row.get("created"),
            updated_at: row.get("updated"),
        }
    }
}

impl Display for VisFlowOperationEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VisFlowOperationEntity {{ operation_id: {}, created_at: {}, updated_at: {} }}",
            self.operation_id,
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.updated_at.format("%Y-%m-%d %H:%M:%S")
        )
    }
}
