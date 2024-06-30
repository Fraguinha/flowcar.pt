use sqlx::postgres;
use std::env;

pub const ROW_LIMIT: i32 = 12;

pub async fn db() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&get_database_url())
        .await
}

fn get_database_url() -> String {
    let user = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();
    let service = env::var("POSTGRES_SERVICE").unwrap();
    let port = env::var("POSTGRES_PORT").unwrap();
    let db = env::var("POSTGRES_DB").unwrap();

    format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, service, port, db
    )
}
