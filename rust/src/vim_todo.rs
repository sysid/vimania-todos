#[allow(dead_code)]
use std::fmt::Display;

use crate::models::Todo;
use itertools::Itertools;
use log::debug;
use regex::Regex;
use stdext::function_name;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum TodoStatus {
    Open = 1,
    InProgress = 2,
    Done = 4,
    ToDelete = 8,
    Invalid = 16,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
pub struct VimTodo {
    pub is_todo: bool,
    line_: String,
    level_: String,
    fill0_: String,
    code_: String,
    fill1_: String,
    status_: String,
    fill2_: String,
    todo_: String,
    tags_: String,
}

impl VimTodo {
    pub fn from(todo: Todo) -> Self {
        let mut vim_todo = VimTodo::default();
        vim_todo.is_todo = true;
        vim_todo.set_code(todo.id);
        vim_todo.set_status(todo.flags);
        vim_todo.set_todo(todo.todo);
        vim_todo.set_tags(todo.tags);
        debug!("({}:{}) ::from: {:?}", function_name!(), line!(), vim_todo);
        vim_todo
    }

    pub fn new(line_: String) -> Self {
        #[allow(non_snake_case)]
        let TODO_PATTERN = Regex::new(
            r"^(\t*)(\s*[-*]\s?)(%\d+%)?(.?)(\[[ \-xXdD]{1}\])(\s+)([^{}]+?)(\{t:.+\})?$",
        )
        .unwrap();
        //// GOTCHA/BUG: Multiline not working
        // let TODO_PATTERN = Regex::new(r"(?x)
        //     ^
        //     (?P<a>\t*)   # 1 tab indentation (hierarchy)
        //     (?P<b>\s*[-*]\s?)
        //     (?P<c>%\d+%)?
        //     (?P<d>.?)
        //     (?P<e>\[[ \-xXdD]{1}\])
        //     (?P<f>\s+)
        //     (?P<g>[^{}]+?)
        //     (?P<h>\{t:.+\})?
        //     $
        //     ").unwrap();
        // let match_ = TODO_PATTERN.captures(line).unwrap();
        if let Some(captures) = TODO_PATTERN.captures(line_.as_str()) {
            debug!("({}:{}) {:?}", function_name!(), line!(), captures);
            Self {
                is_todo: true,
                line_: line_.clone(),
                level_: captures
                    .get(1)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
                fill0_: captures
                    .get(2)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
                code_: captures
                    .get(3)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
                fill1_: captures
                    .get(4)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
                status_: captures
                    .get(5)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
                fill2_: captures
                    .get(6)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
                todo_: captures
                    .get(7)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
                tags_: captures
                    .get(8)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
            }
        } else {
            Self {
                is_todo: false,
                line_,
                ..Default::default()
            }
        }
    }

    pub fn todo(&self) -> String {
        self.todo_.trim().to_owned()
    }
    pub fn set_todo(&mut self, todo: String) {
        self.todo_ = todo;
    }

    pub fn status(&self) -> TodoStatus {
        let status = self
            .status_
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .to_lowercase();
        match status.as_str() {
            " " => TodoStatus::Open,
            "-" => TodoStatus::InProgress,
            "x" => TodoStatus::Done,
            "d" => TodoStatus::ToDelete,
            _ => TodoStatus::Invalid,
        }
    }

    pub fn set_status(&mut self, value: i32) {
        match value {
            1 => self.status_ = String::from("[ ]"),
            2 => self.status_ = String::from("[-]"),
            4 => self.status_ = String::from("[x]"),
            _ => unreachable!(), // or return an error instead of panicking
        }
    }

    pub fn tags(&self) -> Vec<String> {
        let tags = self.tags_.trim_start_matches("{t:").trim_end_matches("}");
        if !tags.is_empty() && tags != ",," {
            tags.split(',')
                .map(|tag| tag.trim().to_owned())
                .sorted()
                .collect()
        } else {
            vec![]
        }
    }

    pub fn set_tags(&mut self, tags: String) {
        self.tags_ = format!("{{t:{}}}", tags
            .split(',')
            .map(|tag| tag.trim().to_owned())
            .filter(|tag| !tag.is_empty())
            .sorted()
            .join(","));
        if self.tags_ == "{t:}" {
            self.tags_ = String::new();
        }
    }

    pub fn tags_db_formatted(&self) -> String {
        format!(",{},", self.tags().join(","))
    }

    pub fn code(&self) -> String {
        self.code_
            .trim_start_matches('%')
            .trim_end_matches('%')
            .to_owned()
    }

    pub fn set_code(&mut self, code: i32) {
        self.code_ = format!("%{}%", code);
    }

    pub fn line(&self) -> String {
        if self.is_todo {
            format!(
                "-{} {} {}{}",
                self.code_, self.status_, self.todo_, self.tags_,
            )
        } else {
            self.line_.clone()
        }
    }
}

impl Display for VimTodo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.line())
    }
}

#[cfg(test)]
mod test {
    use crate::models::Todo;
    use log::debug;
    use rstest::*;
    use stdext::function_name;

    use crate::vim_todo::{TodoStatus, VimTodo};

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
    #[case("- [ ] bla blub ()", "bla blub ()", vec![], TodoStatus::Open)]
    #[case("- [ ] bla blub ()   {t:bla,blub}", "bla blub ()", vec!["bla".to_string(), "blub".to_string()], TodoStatus::Open)]
    // #[case("- [ ] bla blub ()   {t:bla,blub}  asdf", "bla blub ()", vec!["bla".to_string(), "blub".to_string()], TodoStatus::Open)]  // TODO: not working
    #[case("- [x] bla blub ()", "bla blub ()", vec![], TodoStatus::Done)]
    #[case("- [D] bla blub ()", "bla blub ()", vec![], TodoStatus::ToDelete)]
    #[case("- [-] bla blub ()", "bla blub ()", vec![], TodoStatus::InProgress)]
    #[case("- [b] xxxx: invalid", "", vec![], TodoStatus::Invalid)]
    #[case("[ ] xxxx: invalid", "", vec![], TodoStatus::Invalid)]
    #[case("[  xxxx: invalid", "", vec ! [], TodoStatus::Invalid)]
    fn test_new(
        #[case] line: &str,
        #[case] todo_: String,
        #[case] tags: Vec<String>,
        #[case] status: TodoStatus,
    ) {
        let l = VimTodo::new(line.to_string());
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
        assert_eq!(l.todo(), todo_);
        assert_eq!(l.tags(), tags);
        assert_eq!(l.status(), status);
        assert_eq!(l.line(), line.to_string());
    }

    #[rstest]
    fn test_code() {
        let mut todo = super::VimTodo::default();
        todo.code_ = String::from("%123%");
        assert_eq!(todo.code(), "123");
    }

    #[rstest]
    #[case("zzz,xxx", "{t:xxx,zzz}")]
    #[case(",xxx,", "{t:xxx}")]
    #[case(",,", "")]
    fn test_set_tags2(#[case] tags: &str, #[case] expected: &str) {
        let mut todo = VimTodo::new("- [ ] bla".to_string());
        todo.set_tags(tags.to_string());
        debug!("({}:{}) {:?}", function_name!(), line!(), todo);
        assert_eq!(todo.tags_, expected);
    }

    #[rstest]
    #[case("- [ ] bla blub ()   {t:bla,blub}", "- [ ] bla blub ()   {t:bla,blub}")]
    fn test_line(#[case] line: &str, #[case] expected: &str) {
        let todo = VimTodo::new(line.to_string());
        debug!("({}:{}) {:?}", function_name!(), line!(), todo);
        assert_eq!(todo.line(), expected);
    }

    #[rstest]
    fn test_set_tags() {
        let mut todo = VimTodo::new("- [ ] bla blub ()   {t:bla,blub}".to_string());
        todo.set_tags("zzz,xxx".to_string());
        debug!("({}:{}) {:?}", function_name!(), line!(), todo);
        assert_eq!(todo.tags(), vec![String::from("xxx"), String::from("zzz")]);
    }

    #[rstest]
    fn test_set_tags_semantic_empty() {
        let mut todo = VimTodo::new("- [ ] bla blub ()   {t:bla,blub}".to_string());
        todo.set_tags(",,".to_string());
        debug!("({}:{}) {:?}", function_name!(), line!(), todo);
        assert_eq!(todo.tags(), Vec::<String>::new());
    }

    #[rstest]
    fn test_from() {
        let todo = Todo {
            id: 1,
            parent_id: None,
            todo: "bla blub".to_string(),
            metadata: "".to_string(),
            tags: "".to_string(),
            desc: "".to_string(),
            path: "".to_string(),
            flags: 1,
            last_update_ts: Default::default(),
            created_at: Default::default(),
        };
        debug!("({}:{}) {:?}", function_name!(), line!(), todo);
        let vim_todo = VimTodo::from(todo);
        debug!("({}:{}) {:?}", function_name!(), line!(), vim_todo);
        assert_eq!(vim_todo.todo(), "bla blub");
        assert_eq!(vim_todo.status(), TodoStatus::Open);
    }
}
