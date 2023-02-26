use std::collections::LinkedList;

use anyhow::anyhow;
use diesel::result::DatabaseErrorKind;
use itertools::Itertools;
use log::{debug, error, info, warn};
use regex::{Captures, Regex};
use stdext::function_name;

use crate::dal::Dal;
use crate::environment::CONFIG;
use crate::models;
use crate::vim_todo::{TodoStatus, VimTodo};

#[derive(Debug)]
struct Line<'a> {
    line: String,
    running_todos: LinkedList<&'a Line<'a>>,
    parent_id: Option<i32>,
    path: String,
    todo: VimTodo,
}

impl<'a> Line<'a> {
    fn new(line: String, path: String, running_todos: LinkedList<&'a Line<'a>>) -> Self {
        Self {
            line: line.clone(),
            running_todos,
            parent_id: None,
            path,
            todo: VimTodo::new(line),
        }
    }
    pub fn handle(&mut self) -> anyhow::Result<Option<String>> {
        /**
         * Handles a vim buffer line
         *
         * 1. Determines the parent id if applicable
         * 2. updates the DB accordingly:
         *     - creates new entry if new todos
         *     - updates existing entry if changes in existing todos (id unchanged)
         *
         * returns updated line or None for deletion in buffer
         */
        match self.todo.status() {
            TodoStatus::ToDelete => {
                // self.delete_todo()?;
                return Ok(None);  // remove from vim buffer
            }
            _ => {
                // if todo.code() == "" {  // new vimania-todo_
                //     let code = self.create_todo()?;
                //     todo.set_code(code);
                // } else {
                //     // update existing vimania-todo_
                //     *todo = self.update_todo()?.unwrap();
                // }
            }
        }
        return Ok(Some(self.todo.line()));
    }


    /*
    pub fn handle_read(&mut self) -> anyhow::Result<Option<String>> {
        if let Some(todo) = &mut self.todo {
            match todo.status() {
                TodoStatus::ToDelete => anyhow::Error::msg("Should not happen.").context("handle_read"),
                _ => {
                    if todo.code() == "" {
                        warn!("({}:{}) Creating {:?} in read mode. Should only happen when re-initializing a re-set file", function_name!(), line!(), todo);
                        todo.add_code(self.create_code());
                    } else {
                        todo = self.update_buffer_from_db();
                    }
                }
                // _ => unreachable!(), // or return an error instead of panicking
            }
        }
        Ok(Some(self.line()))
    }
    */

    pub fn create_todo(&self) -> anyhow::Result<(i32)> {
        let fts_query = format!("\"{}\"", self.todo.todo());
        debug!("({}:{}) {:?}", function_name!(), line!(), fts_query);
        let todos = Dal::new(CONFIG.db_url.clone()).get_todos_by_todo(self.todo.todo())?;

        let new_todo = models::NewTodo {
            parent_id: self.parent_id,
            todo: self.todo.todo(),
            metadata: "".to_string(),
            tags: self.todo.tags_db_formatted(),
            desc: "".to_string(),
            path: self.path.to_string(),
            flags: self.todo.status() as i32,
        };

        if todos.len() > 0 {
            let active_todos: Vec<_> = todos
                .iter()
                .filter(|&todo| todo.flags < TodoStatus::Done as i32)
                .cloned()
                .collect();
            if active_todos.len() > 0 {
                return Err(anyhow!("Todo {:?} already exists and is active.", self.todo.todo()));
            }
        }
        debug!("({}:{}) Creating {:?}.", function_name!(), line!(), new_todo);
        let inserted = Dal::new(CONFIG.db_url.clone()).insert_todo(new_todo)?;
        return Ok(inserted[0].id);
    }

    fn update_todo(&self) -> anyhow::Result<Option<VimTodo>> {
        let code = self.todo.code().parse::<i32>()?;
        let mut todo = Dal::new(CONFIG.db_url.clone()).get_todo_by_id(code);
        match todo {
            Ok(mut todo) => {
                todo.todo = self.todo.todo();
                todo.tags = self.todo.tags_db_formatted();
                todo.parent_id = self.parent_id;
                todo.path = self.path.to_string();
                todo.flags = self.todo.status() as i32;
                debug!("({}:{}) Updating in database: {:?}", function_name!(), line!(), todo);

                Dal::new(CONFIG.db_url.clone()).update_todo(todo.clone())?;
                Ok(Some(self.todo.clone()))
            }
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    info!("Cannot update non existing todo: {}", self.todo.todo());
                    info!("Deleting from vim");
                    Ok(None)
                }
                other_error => Err(anyhow!("Error: {}", other_error)),
            }
        }
    }

    /*
    fn calc_parent_id(&self) {
        error!("({}:{}) Not implemented{:?}", function_name!(), line!(), self);
    }
    fn delete_todo(&self) {
        todo!()
    }
     */
}

#[cfg(test)]
mod test {
    use log::debug;
    use rstest::*;
    use stdext::function_name;

    use crate::environment::VIMANIA_TEST_DB_URL;
    use crate::helper;

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

    #[fixture]
    pub fn dal() -> Dal {
        helper::init_logger();
        let mut dal = Dal::new(String::from(VIMANIA_TEST_DB_URL));
        helper::init_db(&mut dal.conn).expect("Error DB init");
        dal
    }

    #[rstest]
    fn test_update_todo(mut dal: Dal) {
        // init linked list

        let mut l = Line::new("- [ ] bla blub ()".to_string(), "testpath".to_string(), LinkedList::new());
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
        let id = l.create_todo().unwrap();
        l.todo.set_code(id);
        l.todo.set_todo("updated bla blub ()".to_string());
        l.todo.set_tags("tag1,tag2".to_string());
        l.update_todo().unwrap();

        let updated = dal.get_todo_by_id(id).unwrap();
        assert_eq!(updated.todo, "updated bla blub ()");
    }

    #[rstest]
    fn test_new() {
        // init linked list

        let l = Line::new("- [ ] bla blub ()".to_string(), "testpath".to_string(), LinkedList::new());
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
    }

    #[rstest]
    #[case("- [ ] todo yyy", "testpath")]
    fn test_create_todo(
        mut dal: Dal,
        #[case] line: &str,
        #[case] path: &str,
    ) {
        let mut l = Line::new(line.to_string(), "testpath".to_string(), LinkedList::new());
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
        l.create_todo().unwrap();

        let todos = dal.get_todos("").unwrap();
        assert_eq!(todos.len(), 13);
    }
}
/*
    #[rstest]
    fn test_handle_read() {
        let l = Line::new("- [ ] bla blub ()", "testpath", None);
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
    }


    #[rstest]
    fn test_line() {
        let l = Line::new("-%123% [x] this is a text describing a task ", "testpath", None);
        debug!("({}:{}) {:?}", function_name!(), line!(), l.line());
        assert_eq!(l.line(), "-%123% [x] this is a text describing a task ");
        assert_eq!(l.line(), l._line);
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

    #[rstest]
    #[case("-%1% [x] todo 1", "-%1% [x] todo 1")]
    fn test_handle(
        mut dal: Dal,
        #[case] line: &str,
        #[case] result: &str,
    ) {
        let mut l = Line::new(line, "testpath", None);
        // debug!("({}:{}) {:?}", function_name!(), line!(), l);
        let new_line = l.handle().unwrap().unwrap();
        assert_eq!(new_line, result);
    }

    #[rstest]
    fn test_handle_update(mut dal: Dal) {
        let mut l = Line::new("- [x] updateable task", "testpath", None);
        // debug!("({}:{}) {:?}", function_name!(), line!(), l);
        let new_line = l.handle().unwrap().unwrap();  // add to db
        l.todo.unwrap().todo = "xxxxxxxxxxxxxxxxxxxxx".to_string();
        let new_line = l.handle().unwrap().unwrap();  // update to db
        assert_eq!(new_line, "-%13% [x] xxxxxxxxxxxxxxxxxxxxx");
    }
}

 */
