use sqlx::{Database, Pool};

pub fn connect_database<T: Database>() -> Pool<T> {}

