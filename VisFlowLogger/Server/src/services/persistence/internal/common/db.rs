use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn init_database() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .connect("postgres://user:pass@localhost/vis_flow_log")
        .await
}
