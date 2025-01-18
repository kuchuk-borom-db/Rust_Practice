use serde::Deserialize;
use sqlx::Row;

#[derive(Debug, Deserialize)]
pub struct VisFlowOperationEntity {
    pub operation_id: String,
    pub created_at: String, // yyyy-mm-dd
    pub updated_at: String, // yyyy-mm-dd
}
impl VisFlowOperationEntity {
    pub(crate) fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        VisFlowOperationEntity {
            operation_id: row.get("operation_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
