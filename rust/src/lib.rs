pub mod dal;
pub mod environment;
mod handle_buffer;
pub mod helper;
pub mod models;
pub mod schema;
pub mod tag;
mod vim_todo;

use log::debug;
use stdext::function_name;
use pyo3::prelude::*;
use crate::dal::Dal;
use crate::handle_buffer::Line;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b + 100).to_string())
}

pub fn sum_as_string2(a: usize, b: usize) -> anyhow::Result<String> {
    Ok((a + b + 100).to_string())
}

#[pyfunction]
fn handle_it(lines: Vec<String>, path: String, read: bool) -> PyResult<Vec<String>> {
    debug!("({}:{}) {:?}, {:?}, {:?}", function_name!(), line!(), lines, path, read);
    // Ok(handle_buffer::handle_it(lines, path, read))
    Ok(vec!["bla".to_string(), "blub".to_string()])
}

fn _handle_it(lines: Vec<String>, path: String, read: bool) -> anyhow::Result<Vec<String>> {
    debug!("({}:{}) {:?}, {:?}, {:?}", function_name!(), line!(), lines, path, read);
    let mut new_lines = Vec::<String>::new();
    let mut is_in_code_fence = false;

    for l in lines {
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
        // do not evaluate text within code fences
        if l.trim().starts_with("```") {
            is_in_code_fence = !is_in_code_fence;
        }
        if is_in_code_fence {
            new_lines.push(l.to_owned());
            continue;
        }

        let mut line = Line::new(l, path.to_owned());
        let new_line = if read { line.handle_read()? } else { line.handle()? };

        if let Some(new_line) = new_line {
            new_lines.push(new_line);
        }
    }
    return Ok(new_lines);
}

/// A Python module implemented in Rust.
#[pymodule]
fn _vimania_todos(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(handle_it, m)?)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use log::debug;
    // use log::debug;
    use super::*;
    use rstest::*;
    use stdext::function_name;
    use crate::environment::VIMANIA_TEST_DB_URL;

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

    #[fixture]
    pub fn dal() -> Dal {
        helper::init_logger();
        let mut dal = Dal::new(String::from(VIMANIA_TEST_DB_URL));
        helper::init_db(&mut dal.conn).expect("Error DB init");
        dal
    }

    #[rstest]
    #[case(vec ! ["- [ ] bla bub ()"], vec ! ["-%13% [ ] bla bub ()"])]
    fn test_handle_it(mut dal: Dal, #[case] lines: Vec<&str>, #[case] expected: Vec<&str>) {
        debug!("({}:{}) {:?}, {:?}", function_name!(), line!(), lines, expected);
        let lines: Vec<String> = lines.into_iter().map(String::from).collect();
        let expected: Vec<String> = expected.into_iter().map(String::from).collect();
        let result = _handle_it(lines, "testpath".to_string(), false).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    fn xxx() {
        let x = vec!["a", "b", "c"];
        let strings: Vec<String> = x.into_iter().map(String::from).collect();
        println!("{:?}", strings)
        // let strings: Vec<String> = ["a", "b", "c"].map(String::from).to_vec();
    }

    //// must be commented out, ignore not enough
    // #[rstest]
    // #[case(1, 2, "103".to_string())]
    // #[ignore = "Error: symbol not found in flat namespace '_PyBytes_AsString'"]
    // fn test_sum_as_string(#[case] x: usize, #[case] y: usize, #[case] expected: String) {
    //     debug!("({}:{}) {:?}", function_name!(), line!(), expected);
    //     assert_eq!(sum_as_string(x, y).unwrap(), expected);
    // }

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
