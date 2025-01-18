use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

pub async fn init_database() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .connect("postgresql://neondb_owner:Pd9koTsqw3xr@ep-damp-lab-a1lf2lzs-pooler.ap-southeast-1.aws.neon.tech/neondb?sslmode=require")
        .await
}
