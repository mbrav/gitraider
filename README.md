[![Release](https://github.com/mbrav/git_raider/actions/workflows/release.yml/badge.svg)](https://github.com/mbrav/git_raider/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-yellow.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![tokei](https://tokei.rs/b1/github/mbrav/git_raider?category=lines)](https://tokei.rs/b1/github/mbrav/git_raider)

# git_raider

Mass git repository search, replace and commit tool written in Rust

**⚠️WARNING⚠️** This project is still WIP so please test on repositories that you have a safe copy of.

## Example

As an example, say we have a backend team that is tired of causing outages every other commit because the same Apache Kafka cluster used in production is also used in development (thankfully, this is a made up story). A new Kafka cluster for development was setup and now we need to modify hundreds of repositories in development branches to use a new domain pointing to Kafka's development bootstrap server.

We need to modify **values.yaml** and **config.env** files in hundreds of git repositories that are cloned to **/home/user/git_repos**. But all these repositories need to be modified under the branch `development`. Specifically, we need to select lines that contain *"prod-kafka.backend:9092"* and replace *"prod-kafka"* hostname with *"dev-kafka"*.  We can do the following:

```bash
cargo run -- \
  -p "/home/user/git_repos" \
  -b "development\$" \
  -f "values.yaml|config.env" \
  -l "prod-kafka.backend:9092" \
  -s "prod-kafka" \
  -r "dev-kafka" \
  -m "Change Apache Kafka bootstrap server url from 'prod-kafka' to 'dev-kafka'" \
  --push -d
```

After running the command with the `-d` flag we get the following report:

```text
Repo mbrav/test-repo
  Checking out development
    Success checkout development 0290ec568bbd541420454e64b5a7dda6a9642554
REPORT
Found pages:
M 4: mbrav/test-repo/values.yaml
  O 1   kafka_bootstrapservers1: "prod-kafka.backend:9092"
  R 1   kafka_bootstrapservers1: "test-kafka.backend:9092"
  O 2   kafka_bootstrapservers2: prod-kafka.backend:9092
  R 2   kafka_bootstrapservers2: test-kafka.backend:9092
  O 3   kafka_bootstrapservers3: "prod-kafka.backend:9092"
  R 3   kafka_bootstrapservers3: "test-kafka.backend:9092"
  O 4   kafka_bootstrapservers4: prod-kafka.backend:9092
  R 4   kafka_bootstrapservers4: test-kafka.backend:9092
Elapsed: 39.170ms
```

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
