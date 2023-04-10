.DEFAULT_GOAL := help

MAKE          = make
VERSION       = $(shell cat VERSION)

MAKE    = make
PYTHON	= python
PYTEST	= pytest --log-level=debug --capture=tee-sys --asyncio-mode=auto
PYTOPT	=
VENV	= venv
PIP		= venv/bin/pip

VIM_PLUG="$(HOME)/dev/vim/tw-vim/config/plugins.vim"

app_root = $(PROJ_DIR)
app_root ?= .
pkg_src =  $(app_root)/src/vimania_todos
tests_src = $(app_root)/tests

.PHONY: all
all: clean build upload  ## Build and upload
	@echo "--------------------------------------------------------------------------------"
	@echo "-M- building and distributing"
	@echo "--------------------------------------------------------------------------------"

################################################################################
# Development \
DEVELOPMENT:  ## ############################################################

.PHONY: rsdev
rsdev:  ## rsdev
	maturin develop

.PHONY: rsbuild
rsbuild: clean  ## rsbuild and install in pythonx
	maturin build
	pip install --force-reinstall $(PROJ_DIR)/rust/target/wheels/vimania_todos-0.1.0-cp311-cp311-macosx_11_0_arm64.whl --target pythonx

.PHONY: rstest
rstest:   ## rstest (must run DB test before to init ?!?)
	# nocapture prints error messages !!!
	#BKMR_DB_URL=../db/bkmr.db RUST_LOG=DEBUG pushd bkmr && cargo test --package bkmr -- --test-threads=1  # --nocapture
	#TW_VIMANIA_DB_URL=./rust/tests/data/vimania_todos_test.db RUST_LOG=DEBUG pushd rust && cargo test -- --test-threads=1  # --nocapture
	TW_VIMANIA_DB_URL=$(PROJ_DIR)/tests/data/diesel.db RUST_LOG=DEBUG pushd rust && cargo test -- --test-threads=1  # --nocapture


.PHONY: dev
dev: _confirm clean  ## develop python module and rust, clear pythonx
	charm .

.PHONY: dev-vim
dev-vim:  ## open vim plugin
	vim -c 'OpenSession vimania-todos'

################################################################################
# Testing \
TESTING:  ## ############################################################
.PHONY: test
test:  rstest pytest  ## all test
	:

.PHONY: pytest
pytest:  ## run python tests
	TW_VIMANIA_DB_URL=sqlite:///rust/tests/data/vimania_todos_test.db TW_VIMANIA_RS_URL=rust/tests/data/vimania_todos_test.db python -m pytest -ra --junitxml=report.xml --cov-config=setup.cfg --cov-report=xml --cov-report term --cov=$(pkg_src) -vv tests/

.PHONY: test-vim
test-vim:  test-vim-todos  ## run tests-vim (requires libs in pythonx: make build-vim)

.PHONY: test-vim-todos
test-vim-todos:  ## run tests-vim-todos
	@echo "- > - > - > - > - > - > - > - > - > - > - > - > - > - > - > - > - > - > - > - > "
	pushd tests; ./run_test.sh test_todos.vader; popd
	@echo "- < - < - < - < - < - < - < - < - < - < - < - < - < - < - < - < - < - < - < - < "

.PHONY: coverage
coverage:  ## Run tests with coverage
	python -m coverage erase
	TW_VIMANIA_DB_URL=sqlite:///tests/data/vimania_todos_test.db python -m coverage run --include=$(pkg_src)/* --omit=$(pkg_src)/buku.py -m pytest -ra
	#python -m coverage report -m
	python -m coverage html
	python -m coverage report -m
	python -m coverage xml
	#open htmlcov/index.html  # work on macOS

.PHONY: tox
tox:   ## Run tox
	tox

################################################################################
# Building, Deploying \
BUILDING:  ## ##################################################################

#.PHONY: copy-buku
#copy-buku:  ## copy-buku: copy buku.py from twbm
#	cp $(HOME)/dev/py/twbm/twbm/buku.py $(pkg_src)/buku.py

#.PHONY: build
#build: clean clean-vim ## build
#	@echo "building"
#	#python setup.py sdist
#	cp README.md pythonx/
#	python -m build

#.PHONY: build-vim-dev
#build-vim-dev: _confirm ## copy all python packages into pythonx (for local installation)
#	./scripts/cp_venv.sh dev
#	cp -a ~/dev/py/pure-sql/src/pure_sql ~/dev/vim/vimania/pythonx

#.PHONY: build-vim
#build-vim: _confirm clean-vim ## clean and re-install via pip into pythonx
	#pip install -r pythonx/requirements.txt --target pythonx
	#pip install --force-reinstall /Users/Q187392/dev/s/public/vimania-todos/target/wheels/vimania_todos-0.1.0-cp311-cp311-macosx_11_0_arm64.whl --target pythonx


.PHONY: requirements
requirements:  ## create requirements.txt
	#pipenv lock -r > pythonx/requirements.txt
	vim pythonx/requirements.txt

.PHONY: vim-install
vim-install:  ## vim Plug install
	sed -i.bkp "s#^\"Plug 'https://github.com/sysid/vimania-todos.git'#Plug 'https://github.com/sysid/vimania-todos.git'#" $(VIM_PLUG)
	sed -i.bkp "s#^Plug '~/dev/vim/vimania-todos'#\"Plug '~/dev/vim/vimania-todos'#" $(VIM_PLUG)
	vim -c ':PlugInstall vimania-todos'

.PHONY: vim-uninstall
vim-uninstall:  ## vim Plug uninstall
	[ -d "$(HOME)/.vim/plugged/vimania-todos" ] && rm -fr "$(HOME)/.vim/plugged/vimania-todos"
	sed -i.bkp "s#^\"Plug '~/dev/vim/vimania-todos'#Plug '~/dev/vim/vimania-todos'#" $(VIM_PLUG)
	sed -i.bkp "s#^Plug 'https://github.com/sysid/vimania-todos.git'#\"Plug 'https://github.com/sysid/vimania-todos.git'#" $(VIM_PLUG)

.PHONY: install
install: uninstall
	pipx install $(app_root)

.PHONY: uninstall
uninstall:  ## pipx uninstall
	-pipx uninstall vimania-todos

.PHONY: upload
upload:  ## upload to PyPi
	@echo "upload"
	twine upload --verbose dist/*

.PHONY: bump-major
bump-major:  ## bump-major, tag and push
	bumpversion --commit --tag major
	git push --tags

.PHONY: bump-minor
bump-minor:  ## bump-minor, tag and push
	bumpversion --commit --tag minor
	git push --tags

.PHONY: bump-patch
bump-patch:  ## bump-patch, tag and push
	bumpversion --commit --tag patch
	git push --tags
	#git push  # triggers additional build, but no code change (for bumping workspace must be clean)

################################################################################
# Code Quality \
QUALITY:  ## ############################################################

.PHONY: style
style: isort format  ## perform code style format (black, isort)

.PHONY: format
format:  ## perform black formatting
	black --exclude="buku.py" $(pkg_src) tests

.PHONY: isort
isort:  ## apply import sort ordering
	isort $(pkg_src) --profile black

.PHONY: lint
lint: flake8 mypy ## lint code with all static code checks

.PHONY: flake8
flake8:  ## check style with flake8
	@flake8 $(pkg_src)

.PHONY: mypy
mypy:  ## check type hint annotations
	# keep config in setup.cfg for integration with PyCharm
	mypy --config-file setup.cfg $(pkg_src)

################################################################################
# Documenation \
DOCU:  ## ############################################################

.PHONY: docs
docs: coverage  ## - generate project documentation
	@cd docs; rm -rf source/api/confguard*.rst source/api/modules.rst build/*
	@cd docs; make html

.PHONY: check-docs
check-docs:  ## - quick check docs consistency
	@cd docs; make dummy

.PHONY: serve-docs
serve-docs:  ## - serve project html documentation
	@cd docs/build; python -m http.server --bind 127.0.0.1


################################################################################
# Clean \
CLEAN:  ## ############################################################
.PHONY: clean
clean: clean-build clean-pyc clean-pythonx  ## remove all build, test, coverage and Python artifacts

.PHONY: clean-build
clean-build: ## remove build artifacts
	rm -fr build/
	rm -fr dist/
	rm -fr .eggs/
	find . \( -path ./env -o -path ./venv -o -path ./.env -o -path ./.venv \) -prune -o -name '*.egg-info' -exec rm -fr {} +
	find . \( -path ./env -o -path ./venv -o -path ./.env -o -path ./.venv \) -prune -o -name '*.egg' -exec rm -f {} +

.PHONY: clean-pyc
clean-pyc: ## remove Python file artifacts
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +
	find . -name '__pycache__' -exec rm -fr {} +

.PHONY: clean-pythonx
clean-pythonx:  ## clean-pythonx
	rm -fr pythonx/vimania_todos*

################################################################################
# Misc \
MISC:  ## #####################################################################
define PRINT_HELP_PYSCRIPT
import re, sys

for line in sys.stdin:
	match = re.match(r'^([a-zA-Z0-9_-]+):.*?## (.*)$$', line)
	if match:
		target, help = match.groups()
		print("\033[36m%-20s\033[0m %s" % (target, help))
endef
export PRINT_HELP_PYSCRIPT

.PHONY: help
help:
	@python -c "$$PRINT_HELP_PYSCRIPT" < $(MAKEFILE_LIST)

.PHONY: _confirm
_confirm:
	@echo -n "Are you sure? [y/N] " && read ans && [ $${ans:-N} = y ]
	@echo "Action confirmed by user."
