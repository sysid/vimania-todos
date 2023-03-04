use std::fmt;
use std::fmt::Debug;

use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sql_types::{Integer, Text};
use diesel::{sql_query, Connection, RunQueryDsl, SqliteConnection};
use log::debug;
use stdext::function_name;

use crate::models::{NewTodo, TagsFrequency, Todo};
use crate::schema::vimania_todos::dsl::vimania_todos;
use crate::schema::vimania_todos::{desc, flags, metadata, parent_id, path, tags, todo};

// use crate::schema::bookmarks;

// #[derive(Debug)]
pub struct Dal {
    // #[allow(dead_code)]
    url: String,
    pub conn: SqliteConnection,
}

impl Dal {
    pub fn new(url: String) -> Self {
        debug!("({}:{}) {:?}", function_name!(), line!(), url);
        Self {
            conn: Dal::establish_connection(&url),
            url,
        }
    }

    fn establish_connection(database_url: &str) -> SqliteConnection {
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|e| panic!("Error connecting to {}: {:?}", database_url, e))
    }
    // pub fn delete_todo(&mut self, id_: i32) -> Result<Vec<Todo>, DieselError> {
    //     // diesel::delete(bookmarks.filter(id.eq(1))).execute(&mut self.conn)
    //     diesel::delete(vimania_todos.filter(id.eq(id_))).get_results(&mut self.conn)
    // }

    /**
     * Delete a todo_ and compact the table
     *
     * # Return the number of deleted rows
     */
    pub fn delete_todo2(&mut self, id_: i32) -> Result<usize, DieselError> {
        sql_query("BEGIN TRANSACTION;").execute(&mut self.conn)?;

        // Gotcha: 'returning *' not working within transaction
        let n = sql_query(
            "
            DELETE FROM vimania_todos
            WHERE id = ?;
        ",
        )
        .bind::<Integer, _>(id_)
        .execute(&mut self.conn);
        debug!("({}:{}) Deleting {:?}", function_name!(), line!(), id_);

        // database compaction
        sql_query(
            "
            UPDATE vimania_todos
            SET id = id - 1
            WHERE id > ?;
        ",
        )
        .bind::<Integer, _>(id_)
        .execute(&mut self.conn)?;
        debug!("({}:{}) {:?}", function_name!(), line!(), "Compacting");

        sql_query("COMMIT;").execute(&mut self.conn)?;
        debug!(
            "({}:{}) Deleted and Compacted, n: {:?}",
            function_name!(),
            line!(),
            n
        );

        Ok(n?)
    }
    pub fn clean_table(&mut self) -> Result<(), DieselError> {
        sql_query("DELETE FROM vimania_todos WHERE id != 1;").execute(&mut self.conn)?;
        debug!("({}:{}) {:?}", function_name!(), line!(), "Cleaned table.");
        Ok(())
    }
    pub fn update_todo(&mut self, bm: Todo) -> Result<Vec<Todo>, DieselError> {
        diesel::update(vimania_todos.find(bm.id))
            .set((
                parent_id.eq(bm.parent_id),
                todo.eq(bm.todo),
                metadata.eq(bm.metadata),
                tags.eq(bm.tags),
                desc.eq(bm.desc),
                path.eq(bm.path),
                flags.eq(bm.flags),
            ))
            .get_results(&mut self.conn)
    }

    pub fn insert_todo(&mut self, bm: NewTodo) -> Result<Vec<Todo>, DieselError> {
        diesel::insert_into(vimania_todos)
            .values(bm)
            .get_results(&mut self.conn)
    }

    pub fn get_todo_by_id(&mut self, id_: i32) -> Result<Todo, DieselError> {
        // Ok(sql_query("SELECT id, URL, metadata, tags, desc, flags, last_update_ts FROM bookmarks").load::<Bookmark2>(conn)?)
        let bms = sql_query(
            "SELECT id, parent_id, todo, metadata, tags, desc, path, flags, last_update_ts, created_at FROM vimania_todos \
            where id = ?;",
        );
        let bm = bms.bind::<Integer, _>(id_).get_result(&mut self.conn);
        Ok(bm?)
    }

    pub fn get_todos_by_todo(&mut self, todo_: String) -> Result<Vec<Todo>, DieselError> {
        // Ok(sql_query("SELECT id, URL, metadata, tags, desc, flags, last_update_ts FROM bookmarks").load::<Bookmark2>(conn)?)
        let bms = sql_query(
            "SELECT id, parent_id, todo, metadata, tags, desc, path, flags, last_update_ts, created_at FROM vimania_todos \
            where todo = ?;",
        );
        let bms = bms.bind::<Text, _>(todo_).get_results(&mut self.conn);
        Ok(bms?)
    }

    pub fn get_todos(&mut self, query: &str) -> Result<Vec<Todo>, DieselError> {
        if query == "" {
            // select all
            return Ok(vimania_todos.load::<Todo>(&mut self.conn)?);
        }
        self.get_todos_fts(query)
    }

    pub fn get_todos_fts(&mut self, fts_query: &str) -> Result<Vec<Todo>, DieselError> {
        // Ok(sql_query("SELECT id, URL, metadata, tags, desc, flags, last_update_ts FROM bookmarks").load::<Bookmark2>(conn)?)
        let bms = sql_query(
            "SELECT id, parent_id, todo, metadata, tags, desc, path, flags, last_update_ts, created_at FROM vimania_todos_fts \
            where vimania_todos_fts match ? \
            order by rank",
        );
        let bms = bms.bind::<Text, _>(fts_query).get_results(&mut self.conn);
        Ok(bms?)
    }

    pub fn todo_exists(&mut self, url: &str) -> Result<bool, DieselError> {
        // check if a bookmark exists, exact match
        let bms = sql_query(
            "SELECT id, parent_id, todo, metadata, tags, desc, path, flags, last_update_ts, created_at FROM vimania_todos \
            where todo = ?;",
        );
        let bms = bms
            .bind::<Text, _>(url)
            .get_results::<Todo>(&mut self.conn)?;
        Ok(bms.len() > 0)
    }

    /// get frequency based ordered list of all tags
    pub fn get_all_tags(&mut self) -> Result<Vec<TagsFrequency>, DieselError> {
        let tags_query = sql_query(
            "
            -- name: get_all_tags
            with RECURSIVE split(tags, rest) AS (
                SELECT '', tags || ','
                FROM vimania_todos
                UNION ALL
                SELECT substr(rest, 0, instr(rest, ',')),
                       substr(rest, instr(rest, ',') + 1)
                FROM split
                WHERE rest <> '')
            SELECT tags as tag, count(tags) as n
            FROM split
            WHERE tags <> ''
            group by tags
            ORDER BY 2 desc;
        ",
        );
        let tags_result = tags_query.get_results(&mut self.conn);
        Ok(tags_result?)
    }

    /// get ordered vector of tags
    pub fn get_all_tags_as_vec(&mut self) -> Vec<String> {
        let all_tags = self.get_all_tags().unwrap(); //todo handle error
        let mut all_tags: Vec<String> = all_tags.into_iter().map(|t| t.tag).collect();
        debug!("({}:{}) {:?}", function_name!(), line!(), all_tags);
        all_tags.sort();
        all_tags
    }
    /// get frequency based ordered list of related tags for a given tag
    pub fn get_related_tags(&mut self, tag: &str) -> Result<Vec<TagsFrequency>, DieselError> {
        let search_tag = format!("%,{},%", tag);
        let tags_query = sql_query(
            "
            -- name: get_related_tags
            with RECURSIVE split(tags, rest) AS (
                SELECT '', tags || ','
                FROM vimania_todos
                WHERE tags LIKE :tag_query
                -- WHERE tags LIKE ?
                UNION ALL
                SELECT substr(rest, 0, instr(rest, ',')),
                       substr(rest, instr(rest, ',') + 1)
                FROM split
                WHERE rest <> '')
            SELECT tags as tag, count(tags) as n
            FROM split
            WHERE tags <> ''
            group by tags
            ORDER BY 2 desc;
        ",
        );
        let tags_result = tags_query
            .bind::<Text, _>(search_tag)
            .get_results(&mut self.conn);
        Ok(tags_result?)
    }
}

impl Debug for Dal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.url)
    }
}
