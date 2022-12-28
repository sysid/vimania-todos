# vimania-todos
# Modern todo-list management in VIM

[![PyPI Version][pypi-image]][pypi-url]

> Manage todos from your markdown files in a SQLITE FTS database.

## Key features:
- todo list management in database, keep your todo items in the context/file where they
  belong, but have a centralized view on it
- no more missing, obsolete or duplicated todos
- sync between Markdown files and database
- todo lists within code fences in markdown are ignored
- DB entry has a back-link to the task's source file, so the associated source file can be identified
- todos are removed from database when removed from markdown file with `dd`

#### Example todo file
After saving the file, the identifiers have been added and the items are saved in DB:
```markdown
-%1% [ ] purchase piano -%2% [ ] [AIMMS book](file:~/dev/pyomo/tutorial/AIMMS_modeling.pdf)
-%7% [ ] list repos ahead/behind remote
```
## Installation
1. Using [vim-plug](https://github.com/junegunn/vim-plug):
```vim
Plug 'https://github.com/sysid/vimania-todos.git', {'do': 'pip install -r pythonx/requirements.txt --target pythonx'}
```
3. Install CLI interface: `pipx vimania-todos`

#### Tested configuration:
- sqlite 3.36.0 (requires update on macOS)
- macOS, Ubuntu
- vim needs to be configured with python support
- `pip` must be in path
- dependencies see [requirements.txt](https://github.com/sysid/vimania-todos/blob/main/pythonx/requirements.txt)
- `sqlite` version > 3.9.0 for full text search support (FTS5)

#### Configuration
```bash
# Default: <vimplugins>/vimania-todos/pythonx/vimania_todos/db/todos.db
export TW_VIMANIA_DB_URL="sqlite:///$HOME/todos.db"
```

#### Insert Todos convenience method:
[UltiSnips](https://github.com/SirVer/ultisnips) snippet:
```
snippet todo "todo for Vimania"
- [ ] ${1:todo}
endsnippet
```

## CLI interface
- `vimania` provides a CLI interface with full-text search capabilities to your todo database:
```bash
vimania-todos --help
vimania-todos search
```
The CLI interface is identical to [twbm](https://github.com/sysid/twbm.git).


## Implementation Details
- Todos are recognized via the format: `- [ ] todo`
- On opening vimania-todo scans the markdown files and updates existing todos with the current state from the database
- On saving vimania-todo scans the markdown and saves new or updated todos to the database
- Vimania-todo inserts a DB identifier ('%99%') into the markdown item in order to establish a durable link between DB and
  markdown item
- The identifier is hidden via VIM's `conceal` feature
- todo items are deleted by deleting (`dd`) in normal mode. This triggers a DB update.
- todo items deleted by `dd` in visual mode are NOT delete from DB. This is useful to move tasks from one file to
  another. Otherwise, you always can move an item by just deleting it in one file and paste in to another file AND then
  remove the database id ('%99%'). So vimania-todo creates a new entry/link.

## Caveat
- Deleting markdown todo items outside VIM-will cause inconsistency between the DB and the markdown state.
- Always use `dd` to delete a markdown item in order to trigger the corresponding DB update
- Never change the identifier '%99%' manually.
- Todo items are always synced from the DB when opening a markdown file, so changes not written back to DB will be
  lost.

## Fixing inconsistent state
Todos in markdown can get out of sync if updates are made outside of vim, e.g. with another text editor. Don't worry,
this can be fixed easily.

#### entry already in DB
- find the corresponding id in the DB
- add the id to the markdown item: `-%99% [ ] markdown item`

#### entry in DB but not in markdown
- you can safely delete the entry in the DB, unless you maintain on purpose todo items in the DB which do not have a
  counterpart in a markdown (I do).

#### Resetting everything (advanced)
Deleting/adding todo items outside the control of Vimania can cause an inconsistent state between the database on the
markdown files. It is possible to re-synchronize the DB and the todo-lists by creating a new database and clearing the
todo items fo their identifier:

1. Reset DB: `cd pythonx/vimania-todos/db; rm todos.db; alembic upgrade head`
2. Clean up existing markdown files:
    - find all affected markdown files: `rg -t md -- '-%\d+%'`
    - edit the markdown files and remove the allocated database-id to allow for
      re-init: `sed -i 's/-%[0-9]\+%/-/' todo.md`


# Development
VIM needs to find vimania dependencies in `pythonx`.

### VIM Interface
- cannot be tested within PyCharm, needs to be called from VIM.
- For python changes it is important to restart vim after every change in order to enforce proper reload:
  this is best automated with a Vader script: `run_tests.sh testfile` in tests directory.
- vimscript changes can be reloaded as usual


### Testing
#### Python: pytest
`make test`

#### VIM: Vader
`make test-vim`

#### Smoke Test
- after installation with [Plug](https://github.com/junegunn/vim-plug) run vader tests

### Architecture
![Component](doc/component-vimenia.png)

### Credits
- It is inspired by and recommends to use [UltiSnips](https://github.com/SirVer/ultisnips).
- DB management is based on [jarun/buku](https://github.com/jarun/buku)


#### Changelog
[CHANGELOG.md](https://github.com/sysid/vimania-todos/blob/master/CHANGELOG.md)

<!-- Badges -->

[pypi-image]: https://badge.fury.io/py/vimania-todos.svg
[pypi-url]: https://pypi.org/project/vimania-todos/
[build-image]: https://github.com/sysid/vimania-todos/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/sysid/vimania-todos/actions/workflows/build.yml
[coverage-image]: https://codecov.io/gh/sysid/vimania-todos/branch/master/graph/badge.svg
[coverage-url]: https://codecov.io/gh/sysid/vimania-todos
