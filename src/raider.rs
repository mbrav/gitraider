use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::rc::Rc;

use regex::Regex;

use crate::func;
use crate::git;
use crate::structs;

/// Repo Raider struct
pub struct RepoRaider {
    pub path: PathBuf,
    pub dirs: Vec<structs::Directory>,
}

/// Repo Raider Implementation
impl RepoRaider {
    #[must_use]
    pub fn new(path: &String) -> Self {
        let abs_path = fs::canonicalize(path).expect("Error generating absolute path");
        Self {
            path: abs_path,
            dirs: Vec::new(),
        }
    }

    /// Searches for directories with a specific name and outputs a result vector
    pub fn find_dirs(&mut self, name: &str) {
        self.dirs = func::find_dirs(&self.path, name, &false)
            .iter()
            .map(|x| structs::Directory {
                path: x.clone(),
                repo: None,
                pages: Vec::new(),
                relative_path: x
                    .strip_prefix(&self.path)
                    .expect("Error prefixing PATH")
                    .to_path_buf(),
            })
            .collect();
    }

    /// Searches for directories that are git repositories
    /// and saves them as a vector of Directory structs
    pub fn find_repos(&mut self) {
        self.dirs = func::find_dirs(&self.path, ".git", &false)
            .iter()
            .map(|x| structs::Directory {
                path: x.clone(),
                repo: Some(git::get_repo(x).expect("Error getting repo")),
                pages: Vec::new(),
                relative_path: x
                    .strip_prefix(&self.path)
                    .expect("Error prefixing Path")
                    .to_path_buf(),
            })
            .collect();
    }

    /// Checks out a branch in all directories that are repos
    pub fn checkout_branch(&mut self, pattern: &str) {
        let re = Regex::new(pattern).expect("Error compiling regex");
        self.dirs.iter_mut().for_each(|dir| {
            if let Some(repo) = &dir.repo {
                println!("Repo {}", &dir.relative_path.display());
                let branches = git::get_branches(repo).expect("  ERROR unwrapping repo's Branches");
                let mut matches = 0;

                // Loop through branches
                for branch in branches {
                    let b = branch.expect("Error unwrapping branch").0;
                    let refname = git::get_ref(&b);

                    // If branch's refname matches regex pattern then checkout
                    if re.is_match(refname) {
                        git::checkout_branch(repo, &b).expect("  ERROR checking out branch");
                        matches += 1;

                        // If there were more than on match
                        // Output a warning
                        if matches > 1 {
                            println!("    WARNING: More than one branch matched");
                        }
                    }
                }
            } else {
                println!("   WARNING: folder is not a repository");
            }
        });
    }

    /// Recursively matches for filenames with a specific name
    /// and saves them as a vector of Page structs
    pub fn match_files(&mut self, pattern: &str) {
        self.dirs.iter_mut().for_each(|dir| {
            let f: Vec<structs::Page> =
                func::find_files(dir.path.to_str().expect("Error unwrapping Path"), pattern)
                    .iter()
                    .map(|x| structs::Page {
                        path: x.clone(),
                        matches: Vec::new(),
                        relative_path: x
                            .strip_prefix(&self.path)
                            .expect("Error prefixing Path")
                            .to_path_buf(),
                    })
                    .collect();
            dir.pages.extend(f);
        });
    }

    /// Recursively searches for all lines matching a pattern in a file
    /// and saves them as a vector of Match structs
    pub fn match_lines(&mut self, pattern: &str) {
        let re = Regex::new(pattern).expect("Error compiling regex");
        self.dirs.iter_mut().for_each(|dir| {
            dir.pages.iter_mut().for_each(|page| {
                // Open File
                let file = fs::File::open(&page.path).expect("Error reading file");

                // Create a buffered reader and loop through file's lines
                let reader = BufReader::new(file);
                for (line, content) in reader.lines().enumerate() {
                    match content {
                        // An usually results from non utf-8 encoded files
                        // i.e. binary files
                        Err(e) => println!("{}. Skipping file {}", e, page.path.display()),

                        // If content is a string and matches Regex,
                        // then save as a new Match struct
                        Ok(content) => {
                            if re.is_match(content.as_str()) {
                                let new_match = structs::Match {
                                    line: line as i16,
                                    content,
                                    replace: None,
                                    page: Rc::new(page.clone()),
                                };
                                page.matches.push(new_match);
                            }
                        }
                    }
                }
            });
        });
    }

    /// Creates a replace string for Match struct
    pub fn replace(&mut self, select: &str, replace: &str) {
        let re = Regex::new(select).expect("Error compiling regex");
        self.dirs.iter_mut().for_each(|dir| {
            dir.pages.iter_mut().for_each(|page| {
                page.matches.iter_mut().for_each(|mat| {
                    let res = re.replace(mat.content.as_str(), replace);
                    mat.replace = Some(res.to_string());
                });
            });
        });
    }

    /// Apply replace pattern to all Match structs
    /// for every Page struct in every Directory struct
    pub fn apply(&mut self) {
        self.dirs.iter_mut().for_each(|dir| {
            dir.pages.iter_mut().for_each(|page| {
                // Open file with buffered reader
                let mut file =
                    BufReader::new(fs::File::open(&page.path).expect("Error opening file"));

                let mut file_contents = String::new();
                file.read_to_string(&mut file_contents)
                    .expect("Unable to read the file");

                // Replace one line for each match if replace string exists
                for mat in &mut page.matches {
                    if let Some(replace) = &mat.replace {
                        // Replace only one match
                        file_contents = file_contents.replacen(&mat.content, replace, 1);
                    }
                }

                // Open file with buffered writer
                let mut file = BufWriter::new(
                    fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(&page.path)
                        .expect("Error opening file"),
                );

                file.write_all(file_contents.as_bytes())
                    .expect("Error writing to file");
            });
        });
    }

    /// Stage all matches  
    pub fn stage(&mut self) {
        self.dirs.iter_mut().for_each(|dir| {
            if let Some(repo) = &mut dir.repo {
                // Stage all changes
                // TODO: Only stage matched files
                git::stage_all(repo).expect("Error staging all files");
            } else {
                println!(
                    "Skipping, {} is not a git repository",
                    dir.relative_path.display()
                );
            }
        });
    }

    /// Commit all matches  
    pub fn commit(&mut self, msg: &str) {
        self.dirs.iter_mut().for_each(|dir| {
            if let Some(repo) = &mut dir.repo {
                // Commit all staged files
                git::commit(repo, msg).expect("Error committing changes");
            } else {
                println!(
                    "Skipping {} not a git repository",
                    dir.relative_path.display()
                );
            }
        });
    }

    /// Gets all folders
    /// TODO: Return mutable pointers instead of cloned data
    #[must_use]
    pub fn get_dirs(&self) -> Vec<PathBuf> {
        self.dirs.iter().map(|f| f.path.clone()).collect()
    }

    /// Gets all matched pages contained in all folders
    /// TODO: Return mutable pointers instead of cloned data
    #[must_use]
    pub fn get_pages(&self) -> Vec<structs::Page> {
        self.dirs
            .iter()
            .flat_map(|f| f.pages.iter().cloned())
            .collect()
    }

    /// Gets all matched lines in pages
    /// TODO: Return mutable pointers instead of cloned data
    #[must_use]
    pub fn get_matches(&self) -> Vec<structs::Match> {
        self.dirs
            .iter()
            .flat_map(|f| f.pages.iter())
            .flat_map(|p| p.matches.iter().cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raider_new() {
        let path = "./".to_string();
        let raider = RepoRaider::new(&path);
        assert_ne!(raider.path.to_str().unwrap(), path.as_str());
    }

    #[test]
    fn raider_find_dirs() {
        let path = "./".to_string();
        let mut raider = RepoRaider::new(&path);

        raider.find_dirs("src");
        assert_ne!(raider.get_dirs().len(), 0);
    }

    #[test]
    fn raider_find_repos() {
        let path = "../".to_string();
        let mut raider = RepoRaider::new(&path);

        raider.find_repos();
        assert_ne!(raider.get_dirs().len(), 0);
    }

    #[test]
    fn raider_match_files() {
        let path = "../".to_string();
        let mut raider = RepoRaider::new(&path);

        raider.find_repos();
        raider.match_files("main.rs");
        assert_ne!(raider.get_pages().len(), 0);

        raider.get_pages().iter().for_each(|page| {
            assert!(page.path.to_string_lossy().contains("main.rs"));
            assert!(page.relative_path.to_string_lossy().contains("main.rs"));
        });
    }

    #[test]
    fn raider_match_file_contents() {
        let path = "../".to_string();
        let mut raider = RepoRaider::new(&path);

        raider.find_repos();
        raider.match_files("main.rs");
        raider.match_lines("RepoRaider");

        raider.get_matches().iter().for_each(|m| {
            assert!(m.content.contains("RepoRaider"));
        });
    }
}
