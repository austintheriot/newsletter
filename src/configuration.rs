use secrecy::{ExposeSecret, Secret};
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
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn connection_string_without_db_name(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
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
    // and can be specified with a flag: APP_ENVIRONMENT
    let app_environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| String::from("local"))
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT env variable");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        // override specific configuration properties depending on current environment
        .add_source(config::File::from(
            configuration_directory.join(format!("{}.yaml", app_environment.as_str())),
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}

pub fn get_configuration_with_randomized_database_name() -> Result<Settings, config::ConfigError> {
    let mut configuration = get_configuration()?;
    configuration.database.name = Uuid::new_v4().to_string();

    Ok(configuration)
}
