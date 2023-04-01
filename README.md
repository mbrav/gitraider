[![Release](https://github.com/mbrav/git_raider/actions/workflows/release.yml/badge.svg)](https://github.com/mbrav/git_raider/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-yellow.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![tokei](https://tokei.rs/b1/github/mbrav/git_raider?category=lines)](https://tokei.rs/b1/github/mbrav/git_raider)


# git_raider

Mass git repository search, replace and commit tool written in Rust

*Description to be added soon*

## Run

To run, first install Rust's tool chain. Then build:

```bash
cargo run -- --help
```

You will get the following result:

```text
Mass git repository search, replace and commit tool

Usage: git-raider [OPTIONS]

Options:
  -p, --path <PATH>      Path to repositories [env: REPO_PATH=] [default: ../repos]
  -b, --branch <REGEX>   Specify Regex pattern for branches to checkout [env: REPO_BRANCH=] [default: .*]
  -f, --file <REGEX>     Specify Regex pattern for filename [env: FILE_PATTERN=]
  -l, --line <REGEX>     Specify Regex pattern for selecting lines [env: LINE_PATTERN=]
  -s, --select <REGEX>   Specify Regex pattern for selecting parts of a line [env: LINE_SELECT=]
  -r, --replace <REGEX>  Specify Regex pattern for replacing lines selected by --select [env: LINE_REPLACE=]
  -m, --message <TXT>    Specify commit message [env: COMMIT_MSG=]
  -d, --display          Display results at the end of program execution [env: DISPLAY_RES=]
  -y, --dry              Run program in dry mode without altering files and writing to git history [env: DRY_RUN=]
  -h, --help             Print help
  -V, --version          Print version
```
