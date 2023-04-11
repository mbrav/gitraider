use clap::{ArgAction, Parser};

/// Mass git repository search, replace and commit tool
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Path to repositories
    #[arg(
        short = 'p',
        long = "path",
        value_name = "PATH",
        default_value = "../repos",
        env = "REPO_PATH"
    )]
    pub path: String,

    /// Specify Regex pattern for branches to checkout
    #[arg(
        short = 'b',
        long = "branch",
        value_name = "REGEX",
        default_value = r".*",
        env = "REPO_BRANCH"
    )]
    pub branch_pattern: String,

    /// Specify Regex pattern for filename
    #[arg(short = 'f', long = "file", value_name = "REGEX", env = "FILE_PATTERN")]
    pub file_pattern: Option<String>,

    /// Specify Regex pattern for selecting lines
    #[arg(short = 'l', long = "line", value_name = "REGEX", env = "LINE_PATTERN")]
    pub line_pattern: Option<String>,

    /// Specify Regex pattern for selecting parts of a line
    #[arg(
        short = 's',
        long = "select",
        value_name = "REGEX",
        env = "LINE_SELECT"
    )]
    pub line_select_pattern: Option<String>,

    /// Specify Regex pattern for replacing lines selected by --select
    #[arg(
        short = 'r',
        long = "replace",
        value_name = "REGEX",
        env = "LINE_REPLACE"
    )]
    pub line_replace_pattern: Option<String>,

    /// Specify commit message. No commit if empty.
    #[arg(short = 'c', long = "commit", value_name = "TXT", env = "COMMIT_MSG")]
    pub commit: Option<String>,

    /// Specify wether to push commit
    #[arg(long = "push", action=ArgAction::SetTrue, env = "PUSH_CHANGES")]
    pub push: bool,

    /// Specify git username for push
    #[arg(long = "username", value_name = "TXT", env = "GIT_USER")]
    pub username: Option<String>,

    /// Specify git username password for push
    #[arg(long = "password", value_name = "TXT", env = "GIT_PASSWORD")]
    pub password: Option<String>,

    /// Run program in dry mode without altering files and writing to git history
    #[arg(long = "dry", action=ArgAction::SetTrue, env = "DRY_RUN")]
    pub dry_run: bool,

    /// Display assessment at the end of program execution
    #[arg(short = 'a', long = "assess", action=ArgAction::SetTrue, env = "DISPLAY_ASSESS")]
    pub assess: bool,
}
