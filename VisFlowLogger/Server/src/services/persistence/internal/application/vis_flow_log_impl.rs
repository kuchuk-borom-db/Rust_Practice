use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogModel;
use crate::services::persistence::api::services::vis_flow_log::VisFlowLog;
use crate::services::persistence::internal::common::db::init_database;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub struct VisFlowLogImpl {
    db: Pool<Postgres>,
}

impl VisFlowLogImpl {
    pub async fn new() -> Self {
        VisFlowLogImpl {
            db: init_database().await.unwrap(),
        }
    }
}

#[async_trait]
impl VisFlowLog for VisFlowLogImpl {
    async fn save_log(&self, logs: &Vec<VisFlowLogModel>) -> bool {
        if logs.is_empty() {
            return true;
        }

        let mut query = String::from(
            "INSERT INTO logs (id, operation_id, block_name, log_type, log_value, sequence) VALUES ",
        );
        let mut values = Vec::new();
        let mut value_placeholders = Vec::new();

        for (idx, log) in logs.iter().enumerate() {
            let offset = idx * 6;
            let id = Uuid::new_v4();

            // Create placeholder for this row
            value_placeholders.push(format!(
                "(${},${},${},${},${},${})",
                offset + 1,
                offset + 2,
                offset + 3,
                offset + 4,
                offset + 5,
                offset + 6
            ));

            // Add actual values
            values.push(id.to_string());
            values.push(log.operation_id.clone());
            values.push(log.block_name.clone());
            values.push(log.log_type.clone());
            values.push(log.value.clone().unwrap_or_default());
            values.push(log.sequence.to_string());
        }

        query.push_str(&value_placeholders.join(","));

        // Create the query with the correct number of bindings
        let query = sqlx::query(&query);

        // Bind all values
        let query = values.iter().fold(query, |q, v| q.bind(v));

        match query.execute(&self.db).await {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error saving logs: {}", e);
                false
            }
        }
    }

    async fn get_logs_by_operation_id(
        &self,
        operation_id: String,
    ) -> Result<Vec<VisFlowLogModel>, String> {
        match sqlx::query("SELECT * FROM logs WHERE operation_id = $1 ORDER BY sequence")
            .bind(operation_id)
            .fetch_all(&self.db)
            .await
        {
            Ok(rows) => {
                let logs = rows
                    .iter()
                    .map(|row| VisFlowLogModel::from_row(row))
                    .collect();
                Ok(logs)
            }
            Err(err) => Err(format!("Error fetching logs: {}", err)),
        }
    }
}
