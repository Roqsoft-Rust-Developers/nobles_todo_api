// use config::Environment;
// use serde::Deserialize;
// use config::ConfigError;

// #[derive(Deserialize)]
// pub struct ServerConfig{
//     pub host:String,
//     pub port:i32
// }

// #[derive(Deserialize)]
// pub struct Config{
//     pub server:ServerConfig
// }

// impl Config{
//     pub fn from_env()->Result<Self, ConfigError>{
//         let mut cfg = config::Config::new();
//         cfg.merge(Environment::default())?;
//         cfg.try_into()


//     }
// }

use config::{Config as ConfigSource, ConfigBuilder, Environment, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let builder = ConfigBuilder::default()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::default())
            .build()?;

        builder.try_deserialize()
    }
}