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

By default brokr will scan links in html, txt and md files, but you can specify what files you want to be scanned, for example to search only html files:
```sh
brokr --extensions html
```

or to search only html and markdown files:
```sh
brokr --extensions html,md
```