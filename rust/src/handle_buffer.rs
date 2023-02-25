use itertools::Itertools;
use log::debug;
use std::collections::LinkedList;
use stdext::function_name;

use crate::vim_todo::VimTodo;
use regex::{Captures, Regex};

#[derive(Debug, PartialEq, Eq)]
enum MatchEnum {
    ALL = 0,
    LEVEL = 1,
    FILL0 = 2,
    CODE = 3,
    FILL1 = 4,
    STATUS = 5,
    FILL2 = 6,
    TODO = 7,
    TAGS = 8,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Default)]
struct ParsedTodo<'a> {
    line: &'a str,
    all: &'a str,
    level: &'a str,
    fill0: &'a str,
    code: &'a str,
    fill1: &'a str,
    status: &'a str,
    fill2: &'a str,
    todo_: &'a str,
    tags: &'a str,
}

#[derive(Debug)]
struct Line<'a> {
    _line: String,
    is_todo: bool,
    running_todos: LinkedList<&'a Line<'a>>,
    depth: i32,
    parent_id: Option<i32>,
    path: &'a str,
    todo: Option<VimTodo>,
    parsed_todo: Option<ParsedTodo<'a>>,
    // match_: Option<Captures<'a>>,
}

impl<'a> Line<'a> {
    fn new(line: &'a str, path: &'a str, running_todos: Option<LinkedList<&'a Line<'a>>>) -> Self {
        let mut new_running_todos = LinkedList::new();
        if let Some(todos) = running_todos {
            new_running_todos = todos;
        }

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
        if let Some(captures) = TODO_PATTERN.captures(line) {
            debug!("({}:{}) {:?}", function_name!(), line!(), captures);
            let parsed_todo: ParsedTodo = ParsedTodo {
                line,
                all: captures.get(0).map_or("", |m| m.as_str()),
                level: captures.get(1).map_or("", |m| m.as_str()),
                fill0: captures.get(2).map_or("", |m| m.as_str()),
                code: captures.get(3).map_or("", |m| m.as_str()),
                fill1: captures.get(4).map_or("", |m| m.as_str()),
                status: captures.get(5).map_or("", |m| m.as_str()),
                fill2: captures.get(6).map_or("", |m| m.as_str()),
                todo_: captures.get(7).map_or("", |m| m.as_str()),
                tags: captures.get(8).map_or("", |m| m.as_str()),
            };
            debug!("({}:{}) {:?}", function_name!(), line!(), parsed_todo);
            let todo: VimTodo = VimTodo {
                raw_code: parsed_todo.code.to_owned(),
                todo: parsed_todo.todo_.to_owned(),
                raw_status: parsed_todo.status.to_owned(),
                raw_tags: parsed_todo.tags.to_owned(),
                match_: parsed_todo.all.to_owned(),
            };
            debug!("({}:{}) {:?}", function_name!(), line!(), todo);
            Line {
                _line: line.to_owned(),
                is_todo: true,
                running_todos: new_running_todos,
                depth: parsed_todo.level.matches('\t').count() as i32,
                parent_id: None,
                path,
                todo: Some(todo),
                parsed_todo: Some(parsed_todo),
            }
        } else {
            Line {
                _line: line.to_owned(),
                is_todo: false,
                running_todos: new_running_todos,
                depth: 0,
                parent_id: None,
                path,
                todo: None,
                parsed_todo: None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use log::debug;
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
    fn test_tags() {
        let l = Line::new("- [ ] bla bub ()", "testpath", None);
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
    }

    #[rstest]
    #[case("- [ ] bla blub ()", true, 0, "bla blub ()", "")]
    #[case("    - [ ] bla blub ()", true, 0, "bla blub ()", "")]
    #[case("\t- [ ] bla blub ()", true, 1, "bla blub ()", "")]
    #[case("-%9% [ ] with tags {t:todo,py}", true, 0, "with tags ", "{t:todo,py}")]
    fn test_line_parse(
        #[case] line: &str,
        #[case] is_todo: bool,
        #[case] depth: i32,
        #[case] todo_: String,
        #[case] tags: String,
    ) {
        let l = Line::new(line, "testpath", None);
        assert_eq!(l.is_todo, is_todo);
        assert_eq!(l.depth, depth);
        assert_eq!(l.todo.unwrap().todo, todo_);
        assert_eq!(l.parsed_todo.unwrap().tags, tags);
    }
}
