use std::{env, process};

use lazy_static::lazy_static;

pub const VIMANIA_TEST_DB_URL: &'static str = "./tests/data/diesel.db";

// #[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub db_url: String,
}

impl Config {
    fn new() -> Config {
        let db_url = env::var("TW_VIMANIA_DB_URL").unwrap_or(VIMANIA_TEST_DB_URL.to_string());
        // test db_url as path exists
        let path = std::path::Path::new(&db_url);
        if !path.exists() {
            eprintln!("Error: db_url path does not exist: {:?}", db_url);
            process::exit(1);
        }
        Config { db_url }
    }
}

// Create a global configuration singleton
lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[cfg(test)]
mod test {
    use rstest::*;
    use super::*;

    #[ctor::ctor]
    fn init() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    #[rstest]
    fn test_config() {
        println!("Using database at {}", CONFIG.db_url);
        assert_eq!(CONFIG.db_url, String::from(VIMANIA_TEST_DB_URL));
    }
}
