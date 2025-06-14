# minigrep

find specific string in a specific file using command line.

Features:

- Accepting Command Line Arguments

  - arguments:
    1. the file path
    2. a string to search for

## Setup

```sh
$ cargo install

$ cargo test
```

## Usage

Case insensitive search of `who` inside `sample.txt` file:

```sh
$ cargo run -- who sample.txt -i
```

Case sensitive search of `who` inside `sample.txt` file:

```sh
$ cargo run -- who sample.txt
```
