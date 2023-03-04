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
pub struct Line {
    line: String,
    parent_id: Option<i32>,
    path: String,
    todo: VimTodo,
}

impl Line {
    pub fn new(line: String, path: String) -> Self {
        Self {
            line: line.clone(),
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
            // TODO calc_parent_id
            TodoStatus::ToDelete => {
                // self.delete_todo()?;
                return Ok(None); // remove from vim buffer
            }
            _ => {
                if self.todo.code() == "" {  // new vimania-todo_
                    let code = self.create_todo()?;
                    self.todo.set_code(code);
                } else {
                    // update existing vimania-todo_
                    if let None = self.update_todo_in_db()? {
                        return Ok(None); // remove from vim buffer
                    }
                }
            }
        }
        return Ok(Some(self.todo.line()));
    }

    pub fn handle_read(&mut self) -> anyhow::Result<Option<String>> {
        match self.todo.status() {
            TodoStatus::ToDelete => Err(anyhow!("({}:{}) Invalid code path: Trying to delete a todo in read mode", function_name!(), line!())),
            _ => {
                if self.todo.code() == "" {
                    warn!("({}:{}) Creating {:?} in read mode. Should only happen when re-initializing a re-set file", function_name!(), line!(), self.todo);
                    let code = self.create_todo()?;
                    self.todo.set_code(code);
                    Ok(Some(self.todo.line()))
                } else {
                    if let Some(todo) = self.update_vimtodo_from_db()? {
                        self.todo = todo;
                        Ok(Some(self.todo.line()))
                    } else {
                        return Ok(None); // remove from vim buffer
                    }
                }
            }
        }
    }

    pub fn create_todo(&self) -> anyhow::Result<i32> {
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
                return Err(anyhow!(
                    "Todo {:?} already exists and is active.",
                    self.todo.todo()
                ));
            }
        }
        debug!(
            "({}:{}) Creating {:?}.",
            function_name!(),
            line!(),
            new_todo
        );
        let inserted = Dal::new(CONFIG.db_url.clone()).insert_todo(new_todo)?;
        return Ok(inserted[0].id);
    }

    fn update_todo_in_db(&self) -> anyhow::Result<Option<VimTodo>> {
        let code = self.todo.code().parse::<i32>()?;
        let mut todo = Dal::new(CONFIG.db_url.clone()).get_todo_by_id(code);
        match todo {
            Ok(mut todo) => {
                todo.todo = self.todo.todo();
                todo.tags = self.todo.tags_db_formatted();
                todo.parent_id = self.parent_id;
                todo.path = self.path.to_string();
                todo.flags = self.todo.status() as i32;
                debug!(
                    "({}:{}) Updating in database: {:?}",
                    function_name!(),
                    line!(),
                    todo
                );

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
            },
        }
    }

    fn update_vimtodo_from_db(&mut self) -> anyhow::Result<Option<VimTodo>> {
        let code = self.todo.code().parse::<i32>()?;
        let todo = Dal::new(CONFIG.db_url.clone()).get_todo_by_id(code);
        match todo {
            Ok(todo) => {
                self.todo = VimTodo::from(todo);
                Ok(Some(self.todo.clone()))
            }
            Err(e) => match e {
                diesel::result::Error::NotFound => {
                    info!("Cannot update non existing todo: {}", self.todo.todo());
                    info!("Deleting from vim");
                    Ok(None)
                }
                other_error => Err(anyhow!("Error: {}", other_error)),
            },
        }
    }

    pub fn delete_todo(&self) -> anyhow::Result<()> {
        if self.todo.code() == "" {
            debug!(
                "({}:{}) Deleting from vim: {:?}",
                function_name!(),
                line!(),
                "Nothing to do."
            );
            return Ok(());
        }
        let code = self.todo.code().parse::<i32>()?;
        let n = Dal::new(CONFIG.db_url.clone()).delete_todo2(code)?;
        Ok(())
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

        let mut l = Line::new(
            "- [ ] bla blub ()".to_string(),
            "testpath".to_string(),
        );
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
        let id = l.create_todo().unwrap();
        l.todo.set_code(id);
        l.todo.set_todo("updated bla blub ()".to_string());
        l.todo.set_tags("tag1,tag2".to_string());
        l.update_todo_in_db().unwrap();

        let updated = dal.get_todo_by_id(id).unwrap();
        assert_eq!(updated.todo, "updated bla blub ()");
    }

    #[rstest]
    fn test_new() {
        // init linked list

        let l = Line::new(
            "- [ ] bla blub ()".to_string(),
            "testpath".to_string(),
        );
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
    }

    #[rstest]
    #[case("- [ ] todo yyy  ", "testpath")]
    fn test_create_todo(mut dal: Dal, #[case] line: &str, #[case] path: &str) {
        let mut l = Line::new(line.to_string(), "testpath".to_string());
        debug!("({}:{}) {:?}", function_name!(), line!(), l);
        l.create_todo().unwrap();

        let todos = dal.get_todos("").unwrap();
        assert_eq!(todos.len(), 13);
    }

    #[rstest]
    #[case("-%13% [d] this is a text for deletion", None)]
    #[case("-%999% [ ] this is a text for deletion", None)]
    #[case("-%1% [x] todo 1", Some("-%1% [x] todo 1".to_string()))]
    #[case("- [x] this is a text describing a task {t:py,todo}", Some("-%13% [x] this is a text describing a task {t:py,todo}".to_string()))]
    fn test_handle(mut dal: Dal, #[case] todo_text: &str, #[case] result: Option<String>) {
        debug!("({}:{}) {:?} {:?}", function_name!(), line!(), todo_text, result);
        let mut l = Line::new(
            todo_text.to_string(),
            "testpath".to_string(),
        );
        let new_line = l.handle().unwrap();
        assert_eq!(new_line, result);
    }

    #[rstest]
    fn test_handle_read(mut dal: Dal) {
        let mut l = Line::new(
            "-%1% [x] this is a text describing a task {t:py,todo}".to_string(),
            "testpath".to_string(),
        );
        let new_line = l.handle_read().unwrap();
        assert_eq!(new_line, Some("-%1% [ ] todo 1{t:ccc,vimania,yyy}".to_string()));
    }

    #[rstest]
    fn test_update_todo_in_db_not_found(mut dal: Dal) {
        let mut l = Line::new(
            "-%999% [ ] bla blub ()".to_string(),
            "testpath".to_string(),
        );
        let result = l.update_todo_in_db().unwrap();
        assert_eq!(result, None);
    }

    #[rstest]
    fn test_update_vimtodo_from_db_not_found(mut dal: Dal) {
        let mut l = Line::new(
            "-%999% [ ] bla blub ()".to_string(),
            "testpath".to_string(),
        );
        let result = l.update_vimtodo_from_db().unwrap();
        assert_eq!(result, None);
    }
}
