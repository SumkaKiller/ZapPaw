use actix_web::{App, HttpServer};
use tracing_subscriber::{fmt, EnvFilter};
use ZapPaw::{db, routes};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Настройка логирования: используем переменные окружения или уровень "info" по умолчанию
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();

    tracing::info!("Запуск серверной части ZapPaw...");

    // Инициализация пула подключений к базе данных.
    // Если инициализация не удалась, приложение завершится с сообщением об ошибке.
    let db_pool = db::init_pool()
        .await
        .expect("Не удалось подключиться к базе данных");

    // Запуск HTTP-сервера.
    HttpServer::new(move || {
        App::new()
            // Передаём пул подключений в данные приложения
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            // Регистрируем маршрут /health
            .service(routes::health_check)
    })
    .bind(("127.0.0.1", 8080))? // Убедитесь, что порт 8080 свободен
    .run()
    .await?;

    Ok(())
}
