use crate::services::persistence::api::model::vis_flow_log_model::VisFlowLogModel;
use crate::services::persistence::api::services::vis_flow_log::VisFlowLog;
use crate::services::persistence::internal::common::db::init_database;
use sqlx::{Pool, Postgres};
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
impl VisFlowLog for VisFlowLogImpl {
    async fn save_log(&self, logs: Vec<VisFlowLogModel>) -> bool {
        if logs.is_empty() {
            return true;
        }

        let mut query = String::from("INSERT INTO logs (id, operation_id, block_name, log_type, log_value, sequence) VALUES ");
        let mut param_values = Vec::new();
        let mut param_placeholders = Vec::new();
        let mut param_counter = 1;

        for log in logs {
            let id = Uuid::new_v4().to_string();

            // Add parameter placeholders
            param_placeholders.push(format!(
                "(${}, ${}, ${}, ${}, ${}, ${})",
                param_counter,
                param_counter + 1,
                param_counter + 2,
                param_counter + 3,
                param_counter + 4,
                param_counter + 5
            ));

            // Add values to bind
            param_values.push(id);
            param_values.push(log.operation_id);
            param_values.push(log.block_name);
            param_values.push(log.log_type);
            param_values.push(log.value.unwrap_or_default());
            param_values.push(log.sequence.to_string());

            param_counter += 6;
        }

        query.push_str(&param_placeholders.join(", "));

        match sqlx::query(&query)
            .bind_all(param_values)
            .execute(&self.db)
            .await
        {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error saving logs: {}", e);
                false
            }
        }
    }
}
