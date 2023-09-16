# Brokr

Brokr is a tool, to find broken links in your project, 

### Install

To install brokr run 
```sh
cargo install --git https://github.com/0xMimir/brokr
```

### Usage

To find broken links in your project just run 
```sh
brokr
or
brokr --source-dir PATH-TO-YOUR-PROJECT
```
Output:
```
Finding links in: "./tests/example/index.html"
Finding links in: "./tests/example/random.txt"
Finding links in: "./tests/example/README.md"
Finding links in: "./README.md"

Found 3 invalid links

File: "./tests/example/index.html"
Url: https://somewhere.nowhere/there

File: "./tests/example/random.txt"
Url: https://link.random/there

File: "./tests/example/README.md"
Url: https://random.link.here/da
```

By default brokr will scan links in html, txt and md files, but you can specify what files you want to be scanned, for example to search only html files:
```sh
brokr --extensions html
Finding links in: "./tests/example/index.html"

Found 1 invalid links

File: "./tests/example/index.html"
Url: https://somewhere.nowhere/there
```

or to search only html and markdown files:
```sh
brokr --extensions html,md
Finding links in: "./tests/example/index.html"
Finding links in: "./tests/example/README.md"
Finding links in: "./README.md"

Found 2 invalid links

File: "./tests/example/index.html"
Url: https://somewhere.nowhere/there

File: "./tests/example/README.md"
Url: https://random.link.here/da
```