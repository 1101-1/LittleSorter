# LittleSorter

A little sorter that sort all files to folders by it extension.

## USAGE 
If you want to sort files without copy, you need to add `-d` CLI arg `flag`.

For example:

`cargo run -- <path/to/files> <flag>`

`<flag>` is optional, `<path/to/files>` required.


## NOTE:
MAKE SURE if you do not have files without extension!!! Then sorter return an error.

if you use `-d` flag, folders do not deleted after moving files.