use crate::error::{Error, Result};
use std::env;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - Cause: {ex:?}")
        })
    })
}

pub struct Config {

}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config{})
    }
}

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::MissingEnv(name))
}