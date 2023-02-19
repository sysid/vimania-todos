use log::{debug, info};
use rstest::{fixture, rstest};
use std::collections::HashSet;
use stdext::function_name;
use vimania_todos::dal::Dal;
use vimania_todos::environment::VIMANIA_TEST_DB_URL;
use vimania_todos::helper;
use vimania_todos::models::NewTodo;

#[fixture]
pub fn dal() -> Dal {
    helper::init_logger();
    let mut dal = Dal::new(String::from(VIMANIA_TEST_DB_URL));
    helper::init_db(&mut dal.conn).expect("Error DB init");
    dal
}

#[rstest]
fn test_init_db(mut dal: Dal) {
    helper::init_db(&mut dal.conn).expect("Error DB init");
    info!("Init DB");
    assert!(true);
}

#[rstest]
fn test_get_todo_by_id(mut dal: Dal) {
    let todo = dal.get_todo_by_id(1);
    println!("The todos are: {:?}", todo);
    assert_eq!(todo.unwrap().id, 1);
}

#[rstest]
#[should_panic(expected = "NotFound")]
fn test_get_todo_by_id_non_existing(mut dal: Dal) {
    let bm = dal.get_todo_by_id(99999);
    println!("The todos are: {:?}", bm);
    assert_eq!(bm.unwrap().id, 1);
}

#[rstest]
#[case("xxxxx", 1)]
#[case("xx*", 1)]
#[case("", 12)]
#[case("xxxxxxxxxxxxxxxxx", 0)]
fn test_get_todos(mut dal: Dal, #[case] input: &str, #[case] expected: i32) {
    let bms = dal.get_todos(input);
    println!("The todos are: {:?}", bms);
    assert_eq!(bms.unwrap().len() as i32, expected);
}

#[rstest]
#[case("todo 1", true)]
#[case("DOES NOT EXIST", false)]
fn test_td_exists(mut dal: Dal, #[case] input: &str, #[case] expected: bool) {
    let exists = dal.todo_exists(input);
    // println!("The bookmarks are: {:?}", bms);
    assert_eq!(exists.unwrap(), expected);
}

#[rstest]
fn test_insert_todo(mut dal: Dal) {
    // init_db(&mut dal.conn).expect("Error DB init");
    #[allow(non_snake_case)]
    let new_bm = NewTodo {
        parent_id: None,
        todo: String::from("todo from test insert_todo"),
        metadata: String::from(""),
        tags: String::from(",xxx,"),
        desc: String::from("sysid descript"),
        path: "".to_string(),
        flags: 0,
    };
    let bms = dal.insert_todo(new_bm);
    println!("The Todos are: {:?}", bms);
    assert_eq!(bms.unwrap()[0].id, 13);
}

#[allow(non_snake_case)]
#[rstest]
fn test_update_todo(mut dal: Dal) {
    let mut bm = dal.get_todo_by_id(1).unwrap();
    // init_db(&mut dal.conn).expect("Error DB init");
    bm.todo = String::from("todo from test update_todo");
    let bms = dal.update_todo(bm);
    println!("The Todos are: {:?}", bms);
    assert_eq!(bms.unwrap()[0].todo, "todo from test update_todo");
}

#[rstest]
fn test_clean_table(mut dal: Dal) {
    let _bms = dal.clean_table();
    let mut ids = Vec::new();
    let bms = dal.get_todos("").unwrap();
    for (i, _bm) in bms.iter().enumerate() {
        ids.push(bms[i].id)
    }
    // println!("The ids are: {:?}", ids);
    assert!(ids.contains(&1));
    assert_eq!(ids.len(), 1);
}

#[rstest]
fn test_delete_todo2(mut dal: Dal) {
    let n = dal.delete_todo2(4).unwrap(); // asdf2
    let mut ids = Vec::new();
    assert_eq!(n, 1);

    let bms = dal.get_todos("").unwrap();
    for (i, _bm) in bms.iter().enumerate() {
        ids.push(bms[i].id)
    }
    println!("The ids are: {:?}", ids);
    assert!(!ids.contains(&12));
    assert_eq!(ids.len(), 11);

    let bms = dal.get_todos("todo 4").unwrap();
    assert_eq!(bms.len(), 0);
}

