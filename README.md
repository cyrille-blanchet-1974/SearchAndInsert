# SearchAndInsert
search a line containing a string and insert an new line after/before<BR>

# SearchAndReplace
Search a string in a file and replace id with another string<BR>
<BR>
<BR>
syntax : search_and_insert /search:search_string /replace:replace_string /fic:file [/only_first] [/keep_old] [/after] [/exact_cmp]<BR>
/search parameter gives the string to search (may be between ")<BR>
/replace parameter gives the string to put instead<BR>
/fic parameters gives the file to work with<BR>    
if /only_first is set, only first searched string is replace event if present many times <BR>
if /keep_old the file before changes is kept (with a .old extention)<BR>
if /after is set then replace is pur after the lien found (default is before)<BR>
if /exact_cmp is set then the line must be exactly the same as /search parameter. Else a line containing serach parameter is enought<BR>
<BR>
Technical note: works with 3 threads <BR>
    -one to read<BR>
    -one to search/replace 
    -one to write<BR>
search is case sensitive