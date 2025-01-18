use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

pub async fn init_database() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .connect("postgres://postgres:kuku@localhost:5432/visual_flow_logger")
        .await
}
