@echo off
cargo build --release
set prg=.\target\release\search_and_insert.exe
echo fisrt line > ori.txt
echo second line >> ori.txt
echo third line >> ori.txt
echo last line >> ori.txt
echo original:
echo -------------------------------------------
type ori.txt
echo -------------------------------------------
echo Expected: add a line between frst and second
%prg%  /search:"second line " /replace:"added before second line" /fic:ori.txt /only_first /exact_cmp
echo -------------------------------------------
type ori.txt
echo -------------------------------------------
pause
echo .
echo .
echo Expected: add a line after third line
%prg%  /search:"third line " /replace:"added after third line" /fic:ori.txt /after /only_first /exact_cmp
echo -------------------------------------------
type ori.txt
echo -------------------------------------------
pause
echo .
echo .
echo Expected: add a line befor every lines containing 'line'
%prg%  /search:"line" /replace:"inserted last" /fic:ori.txt /after
echo -------------------------------------------
type ori.txt
echo -------------------------------------------
pause

echo .
echo .
echo Expected: no changes
%prg%  /search:"line" /replace:"last" /fic:ori.txt /after /exact_cmp
echo -------------------------------------------
type ori.txt
echo -------------------------------------------
pause
