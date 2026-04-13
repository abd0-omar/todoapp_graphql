//! The todoapp_graphql-config crate contains functionality for parsing as well as accessing the project's documentation.

use dotenvy::dotenv;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use rootcause::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::info;

/// The application configuration.
///
/// This struct is the central point for the entire application configuration. It holds the [`ServerConfig`] as well as [`DatabaseConfig`]and can be extended with any application-specific configuration settings that will be read from the main `app.toml` and the environment-specific configuration files.
///
/// For any setting that appears in both the `app.toml` and the environment-specific file, the latter will override the former so that default settings can be kept in `app.toml` that are overridden per environment if necessary.
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    /// the server configuration: [`ServerConfig`]
    pub server: ServerConfig,
    /// the database configuration: [`DatabaseConfig`]
    pub database: DatabaseConfig,
    /// Redis (e.g. refresh token store). Set via `[redis]` in TOML or `APP_REDIS__URL`.
    pub redis: RedisConfig,
    /// JWT access tokens (HS256). Set via `[jwt]` in TOML or `APP_JWT__SECRET`, `APP_JWT__ACCESS_TOKEN_TTL_SECS`, `APP_JWT__ISSUER`, `APP_JWT__AUDIENCE`.
    pub jwt: JwtConfig,
}

/// The server configuration.
///
/// This struct keeps all settings specific to the server – currently that is the interface the server binds to
/// but more might be added in the future. The struct is provided pre-defined by Gerust and cannot be changed. It
/// **must** be used for the `server` field in the application-specific [`Config`] struct:
///
/// ```rust
/// #[derive(Deserialize, Clone, Debug)]
/// pub struct Config {
///     #[serde(default)]
///     pub server: ServerConfig,
///     pub database: DatabaseConfig,
///     // add your config settings here…
/// }
/// ```
#[derive(Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ServerConfig {
    /// The port to bind to, e.g. 3000
    pub port: u16,

    /// The ip to bind to, e.g. 127.0.0.1 or ::1
    pub ip: IpAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
        }
    }
}

impl ServerConfig {
    /// Returns the full address the server binds to, including both the ip and port.
    ///
    /// This can be used when creating a TCP Listener:
    ///
    /// ```rust
    /// let config: Config = load_config(Environment::Development);
    /// let listener = TcpListener::bind(&config.server.addr).await?;
    /// serve(listener, app.into_make_service()).await?;
    ///  ```
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }
}

/// The database configuration.
///
/// This struct keeps all settings specific to the database – currently that is the database URL to use to connect to the database
/// but more might be added in the future. The struct is provided pre-defined by Gerust and cannot be changed. It
/// **must** be used for the `database` field in the application-specific [`Config`] struct:
///
/// ```rust
/// #[derive(Deserialize, Clone, Debug)]
/// pub struct Config {
///     #[serde(default)]
///     pub server: ServerConfig,
///     pub database: DatabaseConfig,
///     // add your config settings here…
/// }
/// ```
#[derive(Deserialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DatabaseConfig {
    /// The URL to use to connect to the database, e.g. "postgresql://user:password@localhost:5432/database"
    pub url: String,
}

/// Redis connection settings.
#[derive(Deserialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RedisConfig {
    /// Redis URL, e.g. `redis://127.0.0.1:6379`
    pub url: String,
}

/// JWT signing and validation settings (symmetric secret).
#[derive(Deserialize, Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct JwtConfig {
    /// HMAC secret (must be strong in production). Env: `APP_JWT__SECRET`.
    pub secret: String,
    /// Access token time-to-live in seconds. Env: `APP_JWT__ACCESS_TOKEN_TTL_SECS`. Default: 3600.
    #[serde(default = "default_jwt_access_token_ttl_secs")]
    pub access_token_ttl_secs: u64,
    /// Opaque refresh token TTL in Redis (seconds). Env: `APP_JWT__REFRESH_TOKEN_TTL_SECS`. Default: 604800 (7d).
    #[serde(default = "default_jwt_refresh_token_ttl_secs")]
    pub refresh_token_ttl_secs: u64,
    /// Optional issuer (`iss`) claim validation on verify and set on sign. Env: `APP_JWT__ISSUER`.
    pub issuer: Option<String>,
    /// Optional audience (`aud`) claim validation on verify and set on sign. Env: `APP_JWT__AUDIENCE`.
    pub audience: Option<String>,
}

fn default_jwt_access_token_ttl_secs() -> u64 {
    3600
}

fn default_jwt_refresh_token_ttl_secs() -> u64 {
    604800
}

/// Failure while resolving [`Environment`] or loading configuration from Figment.
#[derive(Debug)]
pub enum ConfigError {
    /// `APP_ENVIRONMENT` (or equivalent) was set to an unrecognized value.
    UnknownEnvironment { raw: String },
    /// TOML/env layers could not be merged or deserialized into the target type.
    LoadFailed(figment::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::UnknownEnvironment { raw } => {
                write!(f, r#"Unknown environment: "{raw}"!"#)
            }
            ConfigError::LoadFailed(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::UnknownEnvironment { .. } => None,
            ConfigError::LoadFailed(e) => Some(e),
        }
    }
}

/// Loads the application configuration for a particular environment.
///
/// Depending on the environment, this function will behave differently:
/// * for [`Environment::Development`], the function will load env vars from a `.env` file at the project root if that is present
/// * for [`Environment::Test`], the function will load env vars from a `.env.test` file at the project root if that is present
/// * for [`Environment::Production`], the function will only use the process env vars, and not load a `.env` file
///
/// In case the .env or .env.test files live in another directory,
/// you can set that location using the APP_DOTENV_CONFIG_DIR environment variable.
/// This is useful when they are mounted at separate locations in a Docker container, for example.
///
/// Configuration settings are loaded from these sources (in that order so that latter soruces override former):
/// * the `packages/config/app.toml` file
/// * the `packages/config/environments/<development|production|test>.toml` files depending on the environment
/// * environment variables
pub fn load_config<'a, T>(env: &Environment) -> Result<T, Report<ConfigError>>
where
    T: Deserialize<'a>,
{
    let dotenv_config_dir = env::var("APP_DOTENV_CONFIG_DIR")
        .ok()
        .map(std::path::PathBuf::from);

    match (env, dotenv_config_dir) {
        (Environment::Development, None) => {
            dotenv().ok();
        }
        (Environment::Test, None) => {
            dotenvy::from_filename(".env.test").ok();
        }
        (Environment::Development, Some(mut dotenv_config_dir)) => {
            dotenv_config_dir.push(".env");
            dotenvy::from_filename(dotenv_config_dir).ok();
        }
        (Environment::Test, Some(mut dotenv_config_dir)) => {
            dotenv_config_dir.push(".env.test");
            dotenvy::from_filename(dotenv_config_dir).ok();
        }
        _ => { /* don't use any .env file for production */ }
    }

    let env_config_file = match env {
        Environment::Development => "development.toml",
        Environment::Production => "production.toml",
        Environment::Test => "test.toml",
    };

    let config: T = Figment::new()
        .merge(Serialized::defaults(ServerConfig::default()).key("server"))
        .merge(Toml::file("packages/config/app.toml"))
        .merge(Toml::file(format!(
            "packages/config/environments/{}",
            env_config_file
        )))
        .merge(Env::prefixed("APP_").split("__"))
        .extract()
        .map_err(|e| report!(ConfigError::LoadFailed(e)).attach("Could not read configuration!"))?;

    Ok(config)
}

/// The environment the application runs in.
///
/// The application can run in 3 different environments: development, production, and test. Depending on the environment, the configuration might be different (e.g. different databases) or the application might behave differently.
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    /// The development environment is what developers would use locally.
    Development,
    /// The production environment would typically be used in the released, user-facing deployment of the app.
    Production,
    /// The test environment is using when running e.g. `cargo test`
    Test,
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production"),
            Environment::Test => write!(f, "test"),
        }
    }
}

/// Returns the currently active environment.
///
/// If the `APP_ENVIRONMENT` env var is set, the application environment is parsed from that (which might fail if an invalid environment is set). If the env var is not set, [`Environment::Development`] is returned.
pub fn get_env() -> Result<Environment, Report<ConfigError>> {
    match env::var("APP_ENVIRONMENT") {
        Ok(val) => {
            info!(r#"Setting environment from APP_ENVIRONMENT: "{}""#, val);
            parse_env(&val)
        }
        Err(_) => {
            info!("Defaulting to environment: development");
            Ok(Environment::Development)
        }
    }
}

/// Parses an [`Environment`] from a string.
///
/// The environment can be passed in different forms, e.g. "dev", "development", "prod", etc. If an invalid environment is passed, an error is returned.
pub fn parse_env(env: &str) -> Result<Environment, Report<ConfigError>> {
    let env = &env.to_lowercase();
    match env.as_str() {
        "dev" => Ok(Environment::Development),
        "development" => Ok(Environment::Development),
        "test" => Ok(Environment::Test),
        "prod" => Ok(Environment::Production),
        "production" => Ok(Environment::Production),
        unknown => Err(report!(ConfigError::UnknownEnvironment {
            raw: unknown.to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[derive(Deserialize, PartialEq, Debug)]
    pub struct Config {
        pub server: ServerConfig,
        pub database: DatabaseConfig,
        pub redis: RedisConfig,
        pub jwt: JwtConfig,
        pub app_setting: String,
    }

    #[test]
    fn test_load_config_development() {
        figment::Jail::expect_with(|jail| {
            let config_dir = jail.create_dir("packages/config")?;
            jail.create_file(
                config_dir.join("app.toml"),
                r#"
                app_setting = "Just a TOML App!"
            "#,
            )?;
            let environments_dir = jail.create_dir("packages/config/environments")?;
            jail.create_file(
                environments_dir.join("development.toml"),
                r#"
                app_setting = "override!"
            "#,
            )?;

            jail.set_env("APP_SERVER__IP", "127.0.0.1");
            jail.set_env("APP_SERVER__PORT", "3000");
            jail.set_env(
                "APP_DATABASE__URL",
                "postgresql://user:pass@localhost:5432/my_app",
            );
            jail.set_env("APP_REDIS__URL", "redis://127.0.0.1:6379");
            jail.set_env("APP_JWT__SECRET", "test-jwt-secret-at-least-32-chars!!");
            let config = load_config::<Config>(&Environment::Development).unwrap();

            assert_that!(
                config,
                eq(&Config {
                    server: ServerConfig {
                        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                        port: 3000,
                    },
                    database: DatabaseConfig {
                        url: String::from("postgresql://user:pass@localhost:5432/my_app"),
                    },
                    redis: RedisConfig {
                        url: String::from("redis://127.0.0.1:6379"),
                    },
                    jwt: JwtConfig {
                        secret: String::from("test-jwt-secret-at-least-32-chars!!"),
                        access_token_ttl_secs: 3600,
                        refresh_token_ttl_secs: 604800,
                        issuer: None,
                        audience: None,
                    },
                    app_setting: String::from("override!"),
                })
            );

            Ok(())
        });
    }

    #[test]
    fn test_load_config_test() {
        figment::Jail::expect_with(|jail| {
            let config_dir = jail.create_dir("packages/config")?;
            jail.create_file(
                config_dir.join("app.toml"),
                r#"
                app_setting = "Just a TOML App!"
            "#,
            )?;
            let environments_dir = jail.create_dir("packages/config/environments")?;
            jail.create_file(
                environments_dir.join("test.toml"),
                r#"
                app_setting = "override!"
            "#,
            )?;

            jail.set_env("APP_SERVER__IP", "127.0.0.1");
            jail.set_env("APP_SERVER__PORT", "3000");
            jail.set_env(
                "APP_DATABASE__URL",
                "postgresql://user:pass@localhost:5432/my_app",
            );
            jail.set_env("APP_REDIS__URL", "redis://127.0.0.1:6379");
            jail.set_env("APP_JWT__SECRET", "test-jwt-secret-at-least-32-chars!!");
            let config = load_config::<Config>(&Environment::Test).unwrap();

            assert_that!(
                config,
                eq(&Config {
                    server: ServerConfig {
                        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                        port: 3000,
                    },
                    database: DatabaseConfig {
                        url: String::from("postgresql://user:pass@localhost:5432/my_app"),
                    },
                    redis: RedisConfig {
                        url: String::from("redis://127.0.0.1:6379"),
                    },
                    jwt: JwtConfig {
                        secret: String::from("test-jwt-secret-at-least-32-chars!!"),
                        access_token_ttl_secs: 3600,
                        refresh_token_ttl_secs: 604800,
                        issuer: None,
                        audience: None,
                    },
                    app_setting: String::from("override!"),
                })
            );

            Ok(())
        });
    }

    #[test]
    fn test_load_config_production() {
        figment::Jail::expect_with(|jail| {
            let config_dir = jail.create_dir("packages/config")?;
            jail.create_file(
                config_dir.join("app.toml"),
                r#"
                app_setting = "Just a TOML App!"
            "#,
            )?;
            let environments_dir = jail.create_dir("packages/config/environments")?;
            jail.create_file(
                environments_dir.join("production.toml"),
                r#"
                app_setting = "override!"
            "#,
            )?;

            jail.set_env("APP_SERVER__IP", "127.0.0.1");
            jail.set_env("APP_SERVER__PORT", "3000");
            jail.set_env(
                "APP_DATABASE__URL",
                "postgresql://user:pass@localhost:5432/my_app",
            );
            jail.set_env("APP_REDIS__URL", "redis://127.0.0.1:6379");
            jail.set_env("APP_JWT__SECRET", "test-jwt-secret-at-least-32-chars!!");
            let config = load_config::<Config>(&Environment::Production).unwrap();

            assert_that!(
                config,
                eq(&Config {
                    server: ServerConfig {
                        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                        port: 3000,
                    },
                    database: DatabaseConfig {
                        url: String::from("postgresql://user:pass@localhost:5432/my_app"),
                    },
                    redis: RedisConfig {
                        url: String::from("redis://127.0.0.1:6379"),
                    },
                    jwt: JwtConfig {
                        secret: String::from("test-jwt-secret-at-least-32-chars!!"),
                        access_token_ttl_secs: 3600,
                        refresh_token_ttl_secs: 604800,
                        issuer: None,
                        audience: None,
                    },
                    app_setting: String::from("override!"),
                })
            );

            Ok(())
        });
    }
}
