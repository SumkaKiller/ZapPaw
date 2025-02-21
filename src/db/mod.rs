use sqlx::{Pool, Postgres};
use std::env;
use tracing;

pub async fn init_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    tracing::info!("Подключение к базе данных: {}", database_url);

    let pool = Pool::<Postgres>::connect(&database_url).await?;

    tracing::info!("Успешное подключение к базе данных йоу");
    Ok(pool)
}