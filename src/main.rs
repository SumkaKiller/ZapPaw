use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod auth;
mod chat;
mod messages;
mod media;
mod notifications;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Не удалось настроить глобальный подписчик логирования");


    info!("Запуск серверной части ZapPaw...");

    // TODO: Реализация сервера Actix-web будет добавлена позже.
}