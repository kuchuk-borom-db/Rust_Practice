use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;

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
