use std::collections::HashSet;

use itertools::Itertools;
use log::debug;
use stdext::function_name;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
enum TodoStatus {
    Open,
    InProgress,
    Done,
    ToDelete,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct VimTodo {
    pub raw_code: String,
    pub todo: String,
    pub raw_status: String,
    pub raw_tags: String,
    pub match_: String,
}

impl VimTodo {
    fn status(&self) -> TodoStatus {
        let status = self
            .raw_status
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .to_lowercase();
        match status.as_str() {
            " " => TodoStatus::Open,
            "-" => TodoStatus::InProgress,
            "x" => TodoStatus::Done,
            "d" => TodoStatus::ToDelete,
            _ => unreachable!(), // or return an error instead of panicking
        }
    }

    fn set_status(&mut self, value: u32) {
        match value {
            1 => self.raw_status = String::from("[ ]"),
            2 => self.raw_status = String::from("[-]"),
            4 => self.raw_status = String::from("[x]"),
            _ => unreachable!(), // or return an error instead of panicking
        }
    }

    fn tags(&self) -> Vec<String> {
        let tags = self
            .raw_tags
            .trim_start_matches("{t:")
            .trim_end_matches("}");
        if !tags.is_empty() {
            tags.split(',')
                .map(|tag| tag.trim().to_owned())
                .sorted()
                .collect()
        } else {
            vec![]
        }
    }

    fn tags_db_formatted(&self) -> String {
        format!(",{},", self.tags().join(","))
    }

    fn code(&self) -> String {
        self.raw_code
            .trim_start_matches('%')
            .trim_end_matches('%')
            .to_owned()
    }

    fn add_code(mut self, code: &str) -> Self {
        self.raw_code = format!("%{}%", code);
        self
    }

    fn vim_line(&self) -> String {
        format!(
            "-{} {} {}{}",
            self.raw_code, self.raw_status, self.todo, self.raw_tags
        )
    }
}

#[cfg(test)]
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

#[cfg(test)]
mod test {
    use log::debug;
    use rstest::*;
    use stdext::function_name;

    use crate::vim_todo::{TodoStatus, VimTodo};

    #[rstest]
    fn test_vim_line() {
        let todo = VimTodo {
            raw_code: String::from(""),
            todo: "todo string".to_string(),
            raw_status: "[ ]".to_string(),
            raw_tags: "".to_string(),
            match_: "- [ ] todo string".to_string(),
        };
        debug!("({}:{}) {:?}", function_name!(), line!(), todo.vim_line());
        assert_eq!(todo.vim_line(), "- [ ] todo string".to_string());
    }

    #[rstest]
    fn test_code() {
        let mut todo = super::VimTodo::default();
        todo.raw_code = String::from("%123%");
        assert_eq!(todo.code(), "123");
    }

    #[rstest]
    fn test_status() {
        let todo = VimTodo {
            raw_code: String::from(""),
            todo: "todo string".to_string(),
            raw_status: "[ ]".to_string(),
            raw_tags: "".to_string(),
            match_: "- [ ] todo string".to_string(),
        };
        debug!("({}:{}) {:?}", function_name!(), line!(), todo);
        assert_eq!(todo.status(), TodoStatus::Open);
    }

    #[rstest]
    fn test_tags() {
        let todo = VimTodo {
            raw_code: String::from(""),
            todo: "todo string".to_string(),
            raw_status: "[ ]".to_string(),
            raw_tags: "{t:zzz,py,todo}".to_string(),
            match_: "- [ ] todo string".to_string(),
        };
        debug!("({}:{}) {:?}", function_name!(), line!(), todo);
        assert_eq!(todo.tags(), vec!["py", "todo", "zzz"]);
    }

    // #[rstest]
    // #[case(None, ",,".to_string(), vec ! [])]
    // fn test_tags(
    //     #[case] tag: Option<String>,
    //     #[case] expected: String,
    //     #[case] expected_vec: Vec<String>,
    // ) {
    //     let tags = Tags::new(tag.clone());
    //     assert_eq!(tags.tag, expected);
    //     assert_eq!(tags.tags, expected_vec);
    //     debug!("({}:{}) {:?}", function_name!(), line!(), tags);
    // }
}
