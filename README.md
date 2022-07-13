## LCOUNT

This project is a utility binary for retrieving line counts for projects.

\*\*While it is working, this is not meant to be the fastest/more complete implementation at all.
It was created for some Rust fun and for learning purposes.
if you want something like this but way more powerful look up [SCC](https://github.com/boyter/scc) by Ben Boyter.

## Installation

For now you need to build it with rustc/cargo yourself and add the binary to your path.

I will make it available either on github or via package managers (maybe).

## Examples

1. Retrieve all lines inside this repository

`git clone https://github.com/lpturmel/lcount`

`lcount ./lcount`

2. Retrieve all lines of rust code inside this repository

`lcount ./lcount --extension=rs`
