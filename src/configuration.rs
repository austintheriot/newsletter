use secrecy::{ExposeSecret, Secret};
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};
use tracing::log::LevelFilter;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let mut pg_connection_options = self.without_db().application_name(&self.name);
        // cut down some of the noise by filtering out the Trace logs
        pg_connection_options.log_statements(LevelFilter::Trace);
        pg_connection_options
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

/// These are the possible runtime configurations and can be specified
/// by the APP_ENVIRONMENT env variable
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a recognized environment. Use `local` or `production` instead.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // this will determine which configuration to use at runtime
    // and can be specified manually with a flag: APP_ENVIRONMENT
    // (normally this is set from Docker when running in production)
    let app_environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| String::from("local"))
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT env variable");

    let settings = config::Config::builder()
        // base configuration for all environments
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        // set specific properties depending on current environment (allows us to
        // expose a port publicly in production but keep it local to our machine in development)
        .add_source(config::File::from(
            configuration_directory.join(format!("{}.yaml", app_environment.as_str())),
        ))
        // allows overriding any configuration settings at runtime from the command line
        // e.g. APP_APPLICATION__PORT=1234 sets the value for Settings.application.port
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub fn get_configuration_with_randomized_database_name() -> Result<Settings, config::ConfigError> {
    let mut configuration = get_configuration()?;
    configuration.database.name = Uuid::new_v4().to_string();

    Ok(configuration)
}
