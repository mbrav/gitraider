use std::path::PathBuf;
use std::rc::Rc;

use git2::Repository;

/// Directory struct
// #[derive(Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub repo: Option<Repository>,
    pub pages: Vec<Page>,
    pub relative_path: PathBuf,
}

/// Page struct
/// *named so as to not conflict with `std::fs::File`*
#[derive(Clone, Debug)]
pub struct Page {
    pub path: PathBuf,
    pub matches: Vec<Match>,
    pub relative_path: PathBuf,
    // pub dir: Rc<Directory>,
}

/// Match struct specifying line where a search query was matched
#[derive(Clone, Debug)]
pub struct Match {
    pub line: i16,
    pub content: String,
    pub replace: Option<String>,
    pub page: Rc<Page>,
}

/// Debug features for Directory
#[cfg(feature = "ref_debug")]
impl Drop for Directory {
    fn drop(&mut self) {
        println!(
            "Dropping Directory `{}` Pages {}!",
            self.relative_path.display(),
            self.pages.len()
        );
    }
}

/// Debug features for Page
#[cfg(feature = "ref_debug")]
impl Drop for Page {
    fn drop(&mut self) {
        println!(
            "Dropping Page `{}` Matches {}!",
            self.relative_path.display(),
            self.matches.len()
        );
    }
}

/// Debug features for Match
#[cfg(feature = "ref_debug")]
impl Drop for Match {
    fn drop(&mut self) {
        println!(
            "Dropping Match `{}` Replace `{:?}` for Page `{}`!",
            self.content,
            self.replace,
            self.page.relative_path.display()
        );
    }
}
