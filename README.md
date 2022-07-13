## LCOUNT

This project is a utility binary for retrieving line counts for projects.

**While it is functional, this is not meant to be the fastest/more complete implementation at all.
It was created for some Rust fun 🦀 and for learning purposes. if you want something like this but way more powerful look up [SCC](https://github.com/boyter/scc) by Ben Boyter.**

## Installation

For now you need to build it with rustc/cargo yourself and add the binary to your path.

I will make it available either on github or via package managers (maybe).

## Examples

Examples are made by using this repo as a source.

`git clone https://github.com/lpturmel/lcount`

1. Retrieve all lines inside a directory

`lcount ./lcount`

2. Retrieve all lines of rust code inside a directory

`lcount ./lcount --extension=rs`

## Supported extensions
    Rust
    Bicep
    Typescript
    Javascript
    Lua
    C
    Cpp
    CSharp
    Go
    Java
    Kotlin
    Header
    Swift
    Python
    Ruby
    Perl
    Php
    Erlang
    Elixir
    Haskell
    Clojure
    Svelte
    Html
    Css
    Markdown
    Yaml
    Json
    Toml
    Zig

