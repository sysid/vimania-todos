#![allow(unused_imports, unused_variables)]
use log::{debug, error, info, log_enabled, Level};
use rstest::*;
use stdext::function_name;

use std::env;
use vimania_todos::sum_as_string2;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    // env::set_var("SKIM_LOG", "info");
    // env::set_var("TUIKIT_LOG", "info");
    let _ = env_logger::builder()
        // Include all events in tests
        .filter_level(log::LevelFilter::max())
        .filter_module("skim", log::LevelFilter::Info)
        .filter_module("tuikit", log::LevelFilter::Info)
        // Ensure events are captured by `cargo test`
        .is_test(true)
        // Ignore errors initializing the logger if tests race to configure it
        .try_init();
}

#[rstest]
#[case(1, 2, "103".to_string())]
fn test_sum_as_string(#[case] x: usize, #[case] y: usize, #[case] expected: String) {
    debug!("({}:{}) {:?}", function_name!(), line!(), expected);
    assert_eq!(sum_as_string2(x, y).unwrap(), expected);
}
