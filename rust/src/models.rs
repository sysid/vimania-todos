#![allow(non_snake_case)]

use crate::tag::Tags;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::Integer;
use diesel::sql_types::Text;
use log::debug;
use stdext::function_name;

use super::schema::vimania_todos;

#[derive(QueryableByName, Debug, PartialOrd, PartialEq)]
pub struct TagsFrequency {
    #[diesel(sql_type = Integer)]
    pub n: i32,
    #[diesel(sql_type = Text)]
    pub tag: String,
}

#[derive(Queryable, QueryableByName, Debug, PartialOrd, PartialEq, Clone)]
#[diesel(table_name = vimania_todos)]
pub struct Todo {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub todo: String,
    pub metadata: String,
    pub tags: String,
    pub desc: String,
    pub path: String,
    pub flags: i32,
    pub last_update_ts: NaiveDateTime,
    pub created_at: NaiveDateTime,
    // pub last_update_ts: DateTime<Utc>,
}

impl Todo {
    pub fn get_tags(&self) -> Vec<String> {
        Tags::normalize_tag_string(Some(self.tags.clone()))
    }
    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = format!(",{},", Tags::clean_tags(tags).join(","));
        debug!("({}:{}) {:?}", function_name!(), line!(), self);
    }
}

#[derive(Insertable, Clone, Debug, PartialOrd, PartialEq)]
#[diesel(table_name = vimania_todos)]
pub struct NewTodo {
    pub parent_id: Option<i32>,
    pub todo: String,
    pub metadata: String,
    pub tags: String,
    pub desc: String,
    pub path: String,
    pub flags: i32,
}

#[cfg(test)]
mod test {
    use crate::models::Todo;
    use chrono::NaiveDate;
    use rstest::*;

    #[fixture]
    fn todo() -> Todo {
        Todo {
            id: 1,
            parent_id: None,
            todo: String::from("this must be done, todo."),
            metadata: String::from(""),
            tags: String::from(",aaa,xxx,"),
            desc: String::from(""),
            path: String::from(""),
            flags: 0,
            last_update_ts: NaiveDate::from_ymd_opt(2016, 7, 8)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap(),
            created_at: Default::default(),
        }
    }

    #[rstest]
    fn test_todo(todo: Todo) {
        println!("{:?}", todo);
    }

    #[rstest]
    fn test_get_tags(todo: Todo) {
        println!("{:?}", todo);
        assert_eq!(todo.get_tags(), vec!("aaa", "xxx"));
    }
    #[rstest]
    fn test_set_tags(mut todo: Todo) {
        println!("{:?}", todo);
        todo.set_tags(vec!["zzz".to_string()]);
        assert_eq!(todo.get_tags(), vec!("zzz".to_string()));
    }
}
