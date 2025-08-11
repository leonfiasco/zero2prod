use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;







#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize logging ONCE via telemetry
      let subscriber = get_subscriber(
        "zero2prod".into(), "info".into(), std::io::stdout
    );
    init_subscriber(subscriber); // ← This handles `LogTracer` and global default

    let configuration = get_configuration().expect("Failed to read configuration.");

    // No longer async, given that we don't actually try to connect!
    let connection_pool = PgPoolOptions::new()
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.connect_options());
    let address = format!(
    "{}:{}",
    std::env::var("HOST").unwrap_or(configuration.application.host),
    std::env::var("PORT").unwrap_or(configuration.application.port.to_string())
);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
