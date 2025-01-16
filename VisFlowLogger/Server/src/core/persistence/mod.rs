mod api;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn connect_database() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://neondb_owner:Pd9koTsqw3xr@ep-damp-lab-a1lf2lzs.ap-southeast-1.aws.neon.tech/neondb?sslmode=require")
        .await
        .expect("Failed to connect to the database")
}
