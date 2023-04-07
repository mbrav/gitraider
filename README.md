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

## Example

```bash
cargo run -- \
  -p "/home/user/git_repos" \
  -b "development\$" \
  -f "values.yaml|config.env" \
  -l "prod-kafka.local:9092" \
  -s "prod-kafka" \
  -r "dev-kafka" \
  -m "Change Apache Kafka server url from 'prod-kafka' to 'dev-kafka'" \
  --push -d
```

In this example, the command will search for all repos that exist in `/home/user/git_repos`, checkout all branches that end with "`development`", search only in `values.yaml` and `config.env` files, find all lines that contain "`prod-kafka.local:9092`", select "`prod-kafka`" in all matched lines and replace it with "`dev-kafka`". Lastly, add commit with `-m` and push with `--push` changes to remote.

### TODO

For base functionality to be completed, the following must still be finished:

* ~~Create new commit with specified message~~;
* Push changes to remote after successful commit;
* Make `dry-run` mode more functional.

## Run

To run, first install Rust's tool chain. Then build:

```bash
cargo run -- --help
```

You will get the following result showing you a help dialogue:

```text
Mass git repository search, replace and commit tool

Usage: gitraider [OPTIONS]

Options:
  -p, --path <PATH>      Path to repositories [env: REPO_PATH=] [default: ../repos]
  -b, --branch <REGEX>   Specify Regex pattern for branches to checkout [env: REPO_BRANCH=] [default: .*]
  -f, --file <REGEX>     Specify Regex pattern for filename [env: FILE_PATTERN=]
  -l, --line <REGEX>     Specify Regex pattern for selecting lines [env: LINE_PATTERN=]
  -s, --select <REGEX>   Specify Regex pattern for selecting parts of a line [env: LINE_SELECT=]
  -r, --replace <REGEX>  Specify Regex pattern for replacing lines selected by --select [env: LINE_REPLACE=]
  -m, --message <TXT>    Specify commit message. No commit if empty [env: COMMIT_MSG=]
      --push             Specify wether to push commit [env: PUSH_CHANGES=]
      --dry              Run program in dry mode without altering files and writing to git history [env: DRY_RUN=]
  -d, --display          Display results at the end of program execution [env: DISPLAY_RES=]
  -h, --help             Print help
  -V, --version          Print version
```
