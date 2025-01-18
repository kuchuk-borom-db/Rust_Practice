use crate::services::persistence::api::model::vis_flow_op_model::VisFlowOperationEntity;
use crate::services::persistence::api::services::vis_flow_op::VisFlowOp;
use crate::services::persistence::internal::common::db::init_database;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct VisLogOpImpl {
    db: Pool<Postgres>,
}
impl VisLogOpImpl {
    pub async fn new() -> Self {
        VisLogOpImpl {
            db: init_database().await.unwrap(),
        }
    }
}
#[async_trait]
impl VisFlowOp for VisLogOpImpl {
    async fn upsert(&self, operation_id: &str) -> bool {
        let query = "
        INSERT INTO operations (id, created, updated)
        VALUES ($1, NOW(), NOW())
        ON CONFLICT (id)
        DO UPDATE SET updated = NOW();
    ";

        let result = sqlx::query(query)
            .bind(operation_id)
            .execute(&self.db)
            .await;

        match result {
            Ok(result) => result.rows_affected() > 0,
            Err(e) => {
                eprintln!("Error in upsert: {}", e); // You can log the error for better tracking
                false
            }
        }
    }

    async fn get_operations(&self) -> Result<Vec<VisFlowOperationEntity>, String> {
        let query = "SELECT * FROM operations ORDER BY updated";
        match sqlx::query(query).fetch_all(&self.db).await {
            Ok(rows) => {
                let operations = rows
                    .iter()
                    .map(|row| VisFlowOperationEntity::from_row(row))
                    .collect();
                Ok(operations)
            }
            Err(err) => Err(format!("Error fetching operations: {}", err)),
        }
    }
}
