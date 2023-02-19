create table vimania_todos
(
    id             INTEGER  not null primary key,
    parent_id INTEGER references vimania_todos,
    todo           VARCHAR  not null,
    metadata       VARCHAR  not null default '',
    tags           VARCHAR  not null default '',
    desc           VARCHAR  not null default '',
    path           VARCHAR  not null default '',
    flags          INTEGER  not null default 0,
    last_update_ts DATETIME not null default CURRENT_TIMESTAMP,
    created_at     DATETIME not null default CURRENT_TIMESTAMP
);

CREATE TRIGGER [UpdateLastTime]
    AFTER
        UPDATE
    ON vimania_todos
    FOR EACH ROW
    WHEN NEW.last_update_ts <= OLD.last_update_ts
BEGIN
    update vimania_todos
    set last_update_ts=CURRENT_TIMESTAMP
    where id = OLD.id;
END;

create
    virtual table vimania_todos_fts using fts5
(
    id,
    parent_id UNINDEXED,
    todo,
    metadata,
    tags UNINDEXED,
    "desc",
    "path",
    flags UNINDEXED,
    last_update_ts UNINDEXED,
    created_at UNINDEXED,
    content='vimania_todos',
    content_rowid='id',
    tokenize="porter unicode61"
);

CREATE TRIGGER vimania_todos_ad
    AFTER DELETE
    ON vimania_todos
BEGIN
    INSERT INTO vimania_todos_fts (vimania_todos_fts, rowid, todo, metadata, tags, "desc")
    VALUES ('delete', old.id, old.todo, old.metadata, old.tags, old.desc);
END;

CREATE TRIGGER vimania_todos_ai
    AFTER INSERT
    ON vimania_todos
BEGIN
    INSERT INTO vimania_todos_fts (rowid, todo, metadata, tags, "desc")
    VALUES (new.id, new.todo, new.metadata, new.tags, new.desc);
END;

CREATE TRIGGER vimania_todos_au
    AFTER UPDATE
    ON vimania_todos
BEGIN
    INSERT INTO vimania_todos_fts (vimania_todos_fts, rowid, todo, metadata, tags, "desc")
    VALUES ('delete', old.id, old.todo, old.metadata, old.tags, old.desc);
    INSERT INTO vimania_todos_fts (rowid, todo, metadata, tags, "desc")
    VALUES (new.id, new.todo, new.metadata, new.tags, new.desc);
END;

/*
create table todos_fts_config
(
    k not null primary key,
    v
)
    without rowid;

create table todos_fts_data
(
    id INTEGER primary key,
    block BLOB
);

create table todos_fts_docsize
(
    id INTEGER primary key,
    sz BLOB
);

create table todos_fts_idx
(
    segid not null,
    term  not null,
    pgno,
    primary key (segid, term)
)
    without rowid;

*/
insert into main.vimania_todos (id, parent_id, todo, metadata, tags, desc, path, flags, last_update_ts, created_at)
values (1, null, 'todo 1', 'TEST: entry for bookmark xxxxx', ',ccc,vimania,yyy,', 'nice description b', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (2, null, 'todo 2', 'TEST: entry for bookmark bbbb', ',aaa,bbb,', 'nice description a', 'filepath', 0, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (3, null, 'todo 3', 'bla blub', ',aaa,bbb,', 'nice description a2', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (4, 3, 'todo 4', 'bla blub2', ',aaa,bbb,ccc,', 'nice description a3', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (5, 3, 'todo 5 inconsistency', 'blub3', ',aaa,bbb,ccc,', 'nice description a4', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (6, 3, 'todo 5 inconsistency', 'blub3', ',aaa,bbb,ccc,', 'INCONSISTENCY!!!!!, active at the same time', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (7, 6, 'todo 6', 'uniq test: allow same todos but not active at same time', ',,', '', 'filepath', 4, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (8, 6, 'todo 6', 'uniq test: allow same todos but not active at same time', ',,', '', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (9, 8, 'todo 7', '', ',,', '', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (10, 3, 'todo 8', '', ',,', '', 'filepath', 1, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (11, null, 'todo 9', '', ',,', '', 'filepath', 0, '2023-02-19 13:05:02', '2023-02-19 13:05:02'),
       (12, 11, 'todo 10', '', ',,', '', 'filepath', 4, '2023-02-19 13:05:02', '2023-02-19 13:05:02');
