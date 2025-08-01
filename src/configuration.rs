use secrecy::Secret;
use secrecy::ExposeSecret;


#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
#[derive(Clone)]
pub struct DatabaseSettings {
    pub username: String,
     pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,  self.password.expose_secret(),self.host, self.port, self.database_name
        )
    }
}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file named `configuration.yaml`.
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_deserialize::<Settings>()
}
