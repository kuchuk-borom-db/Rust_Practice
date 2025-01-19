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
    async fn upsert(&self, operation_ids: Vec<String>) -> bool {
        if operation_ids.is_empty() {
            return false; // No operations to insert or update
        }

        // Dynamically construct the VALUES part of the query
        let placeholders: Vec<String> = operation_ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("(${}, NOW(), NOW())", i + 1)) // Generates ($1, NOW(), NOW()), ($2, NOW(), NOW()), ...
            .collect();

        let query = format!(
            "
        INSERT INTO operations (id, created, updated)
        VALUES {}
        ON CONFLICT (id)
        DO UPDATE SET updated = NOW();
    ",
            placeholders.join(", ")
        );

        // Bind the operation IDs dynamically
        let mut query_builder = sqlx::query(&query);
        for id in operation_ids {
            query_builder = query_builder.bind(id);
        }

        // Execute the query and handle the result
        let result = query_builder.execute(&self.db).await;

        match result {
            Ok(result) => result.rows_affected() > 0,
            Err(e) => {
                eprintln!("Error in batch upsert: {}", e); // Log the error
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
