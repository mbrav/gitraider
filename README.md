[![Release](https://github.com/mbrav/git_raider/actions/workflows/release.yml/badge.svg)](https://github.com/mbrav/git_raider/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-yellow.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![tokei](https://tokei.rs/b1/github/mbrav/git_raider?category=lines)](https://tokei.rs/b1/github/mbrav/git_raider)

# git_raider

Mass git repository search, replace and commit tool written in Rust

This cli tool is designed to **recursively** run through a directory (`-p`/`--path` flag) and find the following:

* Folders that contain a *.git* folder (and hence are git repositories);
* A branch name regex pattern specified with the `-b`/`--branch` flag which checks out a branch that was matched by the regex. A warning will be outputted if more than one branch was matched;
* A file name regex pattern specified with the `-f`/`--file` flag. This will match all files that match the regex pattern;
* A regex pattern specified with the `-l`/`--line` flag that will match *a whole* line in a file that was matched by `-f`/`--file`;
* A regex pattern specified with the `-s`/`--select` flag that can match just *a part* of a line selected with `-l`/`--line`;
* A regex pattern specified with the `-r`/`--replace` flag that will replace content selected with `-s`/`--select` flag;
* A commit message specified with the `-m`/`--message`. If this flag is not passed, no commit will be made.

### TODO

For base functionality to be completed, the following must still be finished:

* Create new commit with specified message;
* Make `dry-run` much better;

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
