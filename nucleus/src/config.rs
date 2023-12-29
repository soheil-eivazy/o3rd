use crate::error::{Error, Result};
use std::sync::OnceLock;
use std::env;
use std::str::FromStr;

pub fn load_config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}


#[allow(non_snake_case)]
pub struct Config {
    pub MONGO_DB_URI: String,
}

impl Config {
    fn load() -> Result<Config> {
        dotenvy::dotenv().ok();

        Ok(Config {
            MONGO_DB_URI: get_env_parse::<String>("MONGO_DB_URL")?,
        })
    }
}

pub fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::MissingEnv(name))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
	let val = get_env(name)?;
	val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}
