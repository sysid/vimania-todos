diesel::table! {
    vimania_todos (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        todo -> Text,
        metadata -> Text,
        tags -> Text,
        desc -> Text,
        path -> Text,
        flags -> Integer,
        last_update_ts -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    vimania_todos_fts (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        todo -> Text,
        metadata -> Text,
        tags -> Text,
        desc -> Text,
        path -> Text,
        flags -> Integer,
        last_update_ts -> Timestamp,
        created_at -> Timestamp,
    }
}
