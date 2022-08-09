# Vimania: Todo list management in VIM

[![PyPI Version][pypi-image]][pypi-url]
[![Build Status][build-image]][build-url]
[![Code Coverage][coverage-image]][coverage-url]

> modern markdown todo list management in VIM

## Key features:
- Centralized todo list management with embedded database, keep your todo items within the context/file where they
  belong but have a centralized view on it
- no more missing, obsolete or duplicated todos
- Synchronization of todo status between Markdown files and database
- todo lists within code fences in markdown are ignored
- DB entry has a link to the task's source file, so by looking in the DB any todo can be located.
- Todos are removed from database when removed from markdown file with `dd`

## Installation
- vim needs to be configured with python support.
- `pip` must be in path in order to install required dependencies into `vimania/pythonx` (no pollution of system python).

1. Install `https://github.com/sysid/vimania` with your favourite VIM plugin manager
2. Install python `requirements.txt` into `<vimplugins>/vimania/pythonx`
3. Install CLI interface: `make install` (requires pipx)

Example:  
`Plug 'https://github.com/sysid/vimania.git', {'do': 'pip install -r pythonx/requirements.txt --target pythonx'}`


## CLI interface
- `vimania` provides a CLI interface with full-text search capabilities to your todo database:

```bash
vimania-todos -h
vimania-todos search
```
The CLI interface is identical to the [twbm](https://github.com/sysid/twbm.git) interface.


### Insert Todos convenience method:
I recommend configuring two [UltiSnips](https://github.com/SirVer/ultisnips) snippets:
```
snippet todo "todo for Vimania"
- [ ] ${1:todo}
endsnippet
```


### Dependency
Optional:
[twbm](https://github.com/sysid/twbm) for seamless bookmark manager integration
[UltiSnips](https://github.com/SirVer/ultisnips) for easy uri and todo creation


### Configuration
Vimenia needs to know, where your Todos database is located:
`TW_VIMANIA_DB_URL="sqlite:///$HOME/vimania/todos.db"`


# Implementation Details
## Architecture
![Component](doc/component-vimenia.png)


## Todo Management Details
- Todos are recognized via the format: `- [ ] todo`
- On opening Vimania scans the markdown files and updates existing todos with the current state from the database
- On saving Vimania scans the markdown and saves new or updated todos to the database
- Vimania inserts a DB identifier ('%99%') into the markdown item in order to establish a durable link between DB and
  markdown item
- The identifier is hidden via VIM's `conceal` feature
- todo items are deleted by deleting (`dd`) in normal mode. This triggers a DB update
- todo items deleted by `dd` in visual mode are NOT delete from DB. This is useful to move tasks from one file to
  another. Otherwise, you always can move an item by just deleting it in one file and paste in to another file AND then
  remove the database id ('%99%'). So Vimania kust creates a new entry/link.

### Example todo file
After saving the file, the identifiers have been added and the items are saved in DB:

```markdown
-%1% [ ] purchase piano -%2% [ ] [AIMMS book](file:~/dev/pyomo/tutorial/AIMMS_modeling.pdf)
-%7% [ ] list repos ahead/behind remote
```

## Caveat
- Deleting markdown todo items outside Vimenia will cause inconsistency between the DB and the markdown state.
- Always use `dd` to delete a markdown item in order to trigger the corresponding DB update
- Never change the identifier '%99%' manually.
- Todo items are always synced from the DB when opening a markdown file, so changes not written back to DB will be
  lost.

Markdown content other than todo items can be changed arbitrarily, of course.

### Fixing inconsistent state
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
However, try to avoid bringing up PyCharm because it tries to index the entire dependency tree.

## VimaniaManager (VIM Interface)
- cannot be tested within PyCharm, needs to be called from VIM.

## Testing
`make test`
`make test-vim`

### VIM bridge
- For python changes it is important to restart vim after every change in order to enforce proper reload:
  this is best automated with a Vader script: `run_tests.sh testfile` in tests directory.
- vimscript changes can be reloaded as usual


## Credits
It is inspired by and recommends to use [UltiSnips](https://github.com/SirVer/ultisnips).


## Changelog
[CHANGELOG.md](https://github.com/sysid/vimania/blob/master/CHANGELOG.md)

<!-- Badges -->

[pypi-image]: https://badge.fury.io/py/vimania.svg
[pypi-url]: https://pypi.org/project/vimania/
[build-image]: https://github.com/sysid/vimania/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/sysid/vimania/actions/workflows/build.yml
[coverage-image]: https://codecov.io/gh/sysid/vimania/branch/master/graph/badge.svg
[coverage-url]: https://codecov.io/gh/sysid/vimania
