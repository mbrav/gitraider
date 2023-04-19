[![Release](https://github.com/mbrav/git_raider/actions/workflows/release.yml/badge.svg)](https://github.com/mbrav/git_raider/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-yellow.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![tokei](https://tokei.rs/b1/github/mbrav/git_raider?category=lines)](https://tokei.rs/b1/github/mbrav/git_raider)

# git_raider

Mass git repository search, replace and commit tool written in Rust

**⚠️WARNING⚠️** This tool is designed to make changes to hundreds of git repositories, as well as pushing these changes to remote if `--push` flag is used. This project is still WIP so please test on repositories that you have a safe copy.

## Install binary on Linux

To install the latest version of the binary to `/usr/local/bin/` copy the following into your terminal:

```bash
latest_ver=$(curl https://raw.githubusercontent.com/mbrav/git_raider/main/latest)
file_name=gitraider_$latest_ver-stable-x86_64-unknown-linux-gnu.tar.gz
curl -L -o /tmp/$file_name https://github.com/mbrav/git_raider/releases/download/$latest_ver/$file_name
tar -xvf /tmp/$file_name -C /tmp/
sudo cp /tmp/target/release/gitraider /usr/local/bin/
gitraider -V 
```

If successful, you will get the following after the end:

```text
git_raider 0.1.6
```

## Run from source

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
  -c, --commit <TXT>     Specify commit message. No commit if empty [env: COMMIT_MSG=]
      --push             Specify wether to push commit [env: PUSH_CHANGES=]
      --dry              Run program in dry mode without altering files and writing to git history [env: DRY_RUN=]
  -d, --display          Display results at the end of program execution [env: DISPLAY_RES=]
  -h, --help             Print help
  -V, --version          Print version
```

## Example

As an example, say we have a backend team that is tired of causing outages every other commit because the same Apache Kafka cluster used in production is also used in development (thankfully, this is a made up story). A new Kafka cluster for development was setup and now we need to modify hundreds of repositories in development branches to use a new domain pointing to Kafka's development bootstrap server.

We need to modify **values.yaml** and **config.env** files in hundreds of git repositories that are cloned to **/home/user/git_repos**. But all these repositories need to be modified under the git branch *development*. Specifically, we need to select lines that contain *"prod-kafka.backend:9092"* and replace *"prod-kafka"* hostname with *"dev-kafka"*.  We can do the following:

```bash
cargo run -- \
  -p "/home/user/git_repos" \
  -b "development\$" \
  -f "values.yaml|config.env" \
  -l "prod-kafka.backend:9092" \
  -s "prod-kafka" \
  -r "dev-kafka" \
  -c "Change bootstrap server url from prod-kafka to dev-kafka" \
  --push -d
```

After running the command with the `-d` flag we get the following report:

```text
Repo mbrav/test-repo
  Checking out development
    Success checking out branch 'development' 0290ec568bbd541420454e64b5a7dda6a9642554
    Staged 'values.yaml'
    Success commit 'Change bootstrap server url from prod-kafka to dev-kafka' d93cb354791ccb4a540b767c70ea480d4cbd580a
REPORT                                                                                               
Fn - Matched file with number of matched lines                                                                           
  On - Original line, line number                                                                                     
  Rn - Replace line (if present), line number 
PROJECTS
Project: mbrav/test-repo
  F2: mbrav/test-repo/values.yaml
    F1:   kafka_bootstrapservers: "prod-kafka.backend:9092"
    R1:   kafka_bootstrapservers: "test-kafka.backend:9092"
    F4:   kafka_url: prod-kafka.backend:9092
    R4:   kafka_url: test-kafka.backend:9092
Elapsed: 39.170ms
```

## TODO

For base functionality to be completed, the following must still be finished:

- [x] ~~Create new commit with specified message~~;
- [x] ~~Add more elaborate commit changes checks to avoid making duplicate changes and commits~~;
- [x] ~~Make `dry-run` mode more functional.~~
- [ ] Push changes to remote after successful commit. Because liggit2, the underlying C library that git2 Rust library offers bindings to, does not support parsing `~/.ssh/` configs, coming up with a workaround is still WIP. Relevant issues:
  - rust-lang/git2-rs#362
  - libgit2/libgit2#5640
  - libgit2/libgit2#4338
- [ ] Add undo mechanics based on already done changes to avoid deleting and recreating all repositories after each unsuccessful run;
- [ ] Print current branch name is results assesment as well as commit info, etc
- [ ] Add optional pull from remote before a commit to branch;
