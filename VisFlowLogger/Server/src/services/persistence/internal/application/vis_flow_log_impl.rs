use crate::services::persistence::api::model::vis_flow_log_model::{
    VisFlowLogEntity, VisFlowLogEntry,
};
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
    async fn save_log(&self, logs: &Vec<&VisFlowLogEntry>) -> bool {
        println!("Saving logs :- ");
        logs.iter().for_each(|log| println!("{}", log));
        if logs.is_empty() {
            return true;
        }

        let mut query = String::from(
            "INSERT INTO logs (id, operation_id, block_name, log_type, log_value, sequence) VALUES ",
        );
        let mut value_placeholders = Vec::new();

        for (idx, _) in logs.iter().enumerate() {
            let offset = idx * 6;

            value_placeholders.push(format!(
                "(${},${},${},${},${},${})",
                offset + 1,
                offset + 2,
                offset + 3,
                offset + 4,
                offset + 5,
                offset + 6
            ));
        }

        query.push_str(&value_placeholders.join(","));

        // Create the query with all placeholders
        let mut query = sqlx::query(&query);

        // Bind all values
        for log in logs {
            let sequence = i32::try_from(log.sequence).unwrap_or_default();
            let id = Uuid::new_v4();

            query = query
                .bind(id.to_string())
                .bind(&log.operation_id)
                .bind(&log.block_name)
                .bind(&log.log_type)
                .bind(log.log_value.as_deref().unwrap_or_default())
                .bind(sequence);
        }

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
    ) -> Result<Vec<VisFlowLogEntity>, String> {
        match sqlx::query("SELECT * FROM logs WHERE operation_id = $1 ORDER BY sequence")
            .bind(operation_id)
            .fetch_all(&self.db)
            .await
        {
            Ok(rows) => {
                let logs = rows
                    .iter()
                    .map(|row| VisFlowLogEntity::from_row(row))
                    .collect();
                Ok(logs)
            }
            Err(err) => Err(format!("Error fetching logs: {}", err)),
        }
    }
}
