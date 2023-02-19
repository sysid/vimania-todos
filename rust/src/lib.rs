mod schema;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b + 100).to_string())
}
pub fn sum_as_string2(a: usize, b: usize) -> anyhow::Result<String> {
    Ok((a + b + 100).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _vimania_todos(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}


#[cfg(test)]
mod test {
    use log::debug;
    // use log::debug;
    use super::*;
    use rstest::*;
    use stdext::function_name;

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
    #[case(1, 2, "103".to_string())]
    #[ignore = "Error: symbol not found in flat namespace '_PyBytes_AsString'"]
    fn test_sum_as_string(#[case] x: usize, #[case] y: usize, #[case] expected: String) {
        debug!("({}:{}) {:?}", function_name!(), line!(), expected);
        assert_eq!(sum_as_string(x, y).unwrap(), expected);
    }

    #[rstest]
    #[case(1, 2, "103".to_string())]
    fn test_sum_as_string2(#[case] x: usize, #[case] y: usize, #[case] expected: String) {
        debug!("({}:{}) {:?}", function_name!(), line!(), expected);
        assert_eq!(sum_as_string2(x, y).unwrap(), expected);
    }

    /*
    // Tests are fragile, because they depend on machine specific setup
    #[rstest]
    #[case("", None)]
    #[ignore = "fragile"]
    #[case("~/dev/binx", Some("/Users/Q187392/dev/s/private/devops-binx".to_string()))] // link resolved
    #[ignore = "fragile"]
    #[case("$HOME/dev/binx", Some("/Users/Q187392/dev/s/private/devops-binx".to_string()))]
    #[case("https://www.google.com", None)]
    #[ignore = "fragile"]
    #[case("./tests/resources/bkmr.pptx", Some("/Users/Q187392/dev/s/public/bkmr/bkmr/tests/resources/bkmr.pptx".to_string()))] // link resolved
    fn test_abspath(#[case] x: &str, #[case] expected: Option<String>) {
        assert_eq!(abspath(x), expected);
    }

     */
}
