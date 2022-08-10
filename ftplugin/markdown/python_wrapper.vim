" vim: fdm=marker ts=2 sts=2 sw=2 fdl=0
" convert_string.vim
if g:twvim_debug | echom "-D- Sourcing " expand('<sfile>:p') | endif
let s:script_dir = fnamemodify(resolve(expand('<sfile>', ':p')), ':h')

if !has("python3")
  echohl ErrorMsg | echo  "ERROR: vim has to be compiled with +python3 to run this" | echohl None
  finish
endif

" only load it once
if exists('g:vimania_todos_wrapper')
  finish
endif

let g:vimania#PythonScript = expand('<sfile>:r') . '.py'
call TwDebug(printf("Vimania PythonScript: %s", g:vimania#PythonScript))
execute 'py3file ' . g:vimania#PythonScript
"py3file /Users/Q187392/dev/vim/vimania-todos/pythonx/vimania/python_wrapper.py
"py3file /Users/Q187392/dev/vim/vimania/plugin/python_wrapper.py

function! VimaniaTodo(args, path)
  call TwDebug(printf("Vimania args: %s, path: %s", a:args, a:path))
  python3 xTodosMgr.create_todo(vim.eval('a:args'), vim.eval('a:path'))
endfunction
command! -nargs=1 VimaniaTodo call VimaniaTodo(<f-args>, expand('%:p'))
"noremap Q :VimaniaTodo - [ ] todo vimania<CR>

function! VimaniaLoadTodos()
  "call TwDebug(printf("Vimania args: %s, path: %s", a:args, a:path))
  python3 xTodosMgr.load_todos()
endfunction
command! -nargs=0 VimaniaLoadTodos call VimaniaLoadTodos()
"noremap Q :VimaniaLoadTodos<CR>

function! VimaniaDebug()
  "call TwDebug(printf("Vimania args: %s, path: %s", a:args, a:path))
  python3 xTodosMgr.debug()
endfunction
command! -nargs=0 VimaniaDebug call VimaniaDebug()
"noremap Q :VimaniaDebug<CR>

function! VimaniaThrowError()
  "call TwDebug(printf("Vimania args: %s, path: %s", a:args, a:path))
  python3 xTodosMgr.throw_error()
endfunction
command! -nargs=0 VimaniaThrowError call VimaniaThrowError()
"noremap Q :VimaniaDebug<CR>

function! VimaniaHandleTodos(args)
  "call TwDebug(printf("Vimania args: %s, path: %s", a:args, a:path))
  python3 xTodosMgr.handle_todos(vim.eval('a:args'))
endfunction
command! -nargs=1 VimaniaHandleTodos call VimaniaHandleTodos(<f-args)

function! VimaniaDeleteTodo(args, path)
  call TwDebug(printf("Vimania args: %s, path: %s", a:args, a:path))
  python3 xTodosMgr.delete_todo(vim.eval('a:args'), vim.eval('a:path'))
endfunction
command! -nargs=1 VimaniaDeleteTodo call VimaniaDeleteTodo(<f-args>, expand('%:p'))
"noremap Q :VimaniaDeleteTodo - [ ] todo vimania<CR>

function! Xxx(args)
  call TwDebug(printf("Vimania args: %s", a:args))
  python3 xTodosMgr.xxx(vim.eval('a:args'))
endfunction
command! -nargs=1 Xxx call Xxx(<f-args>)
noremap Q :Xxx doSomething with this<CR>

let g:vimania_todos_wrapper = 1
