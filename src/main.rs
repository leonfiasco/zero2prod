use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;







#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize logging ONCE via telemetry
      let subscriber = get_subscriber(
        "zero2prod".into(), "info".into(), std::io::stdout
    );
    init_subscriber(subscriber); // ← This handles `LogTracer` and global default

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(
            &configuration.database.connection_string()
        ).expect("Failed to create Postgres connection pool.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
