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
    pub dry_run: bool,
}

/// Repo Raider Implementation
impl RepoRaider {
    #[must_use]
    pub fn new(path: String, dry_run: bool) -> Self {
        let abs_path = fs::canonicalize(path).expect("Error generating absolute path");
        Self {
            path: abs_path,
            dirs: Vec::new(),
            dry_run,
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
                        changes: false,
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
                        // An error usually results from non utf-8 encoded file
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
                    let replace_string = res.to_string();

                    println!("Repo {}", &dir.relative_path.display());
                    // Create a replace string only if replaced string does not match line content
                    if replace_string != mat.content {
                        // Mark Page as one that has changes
                        page.changes = true;
                        // Set replace string
                        mat.replace = Some(replace_string);
                        println!("  O {:<3} {}", mat.line, mat.content);
                        println!(
                            "  R {:<3} {}",
                            mat.line,
                            mat.replace
                                .as_ref()
                                .expect("Error unwrapping replace reference")
                        );
                    }
                });
            });
        });
    }

    /// Apply replace pattern to all Match structs
    /// for every Page struct in every Directory struct
    pub fn apply(&mut self) {
        self.dirs.iter_mut().for_each(|dir| {
            dir.pages
                .iter_mut()
                // Filter only pages that have changes
                .filter(|p| p.changes)
                .for_each(|page| {
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

                    // Check if in dry run mode
                    if self.dry_run {
                        println!("Would have written to {}", page.path.display());
                    } else {
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
                    }
                });
        });
    }

    /// Stage all matches  
    pub fn stage(&mut self) {
        self.dirs.iter_mut().for_each(|dir| {
            if let Some(repo) = &mut dir.repo {
                // Stage all changes
                // git::stage_all(repo).expect("Error staging all files");

                // Filter only pages that have changes
                dir.pages
                    .clone()
                    .into_iter()
                    .filter(|p| p.changes)
                    .for_each(|p| {
                        // Get file path relative to repository root
                        let file_repo_path = p
                            .relative_path
                            .strip_prefix(&dir.relative_path)
                            .expect("Error stripping Path prefix");

                        // Check if in dry run mode
                        if self.dry_run {
                            println!("Would have staged '{}'", file_repo_path.display());
                        } else {
                            match git::stage_file(repo, file_repo_path) {
                                Ok(_) => {
                                    println!("Staged '{}'", file_repo_path.display());
                                }
                                Err(_) => {
                                    println!("Error staging '{}'", file_repo_path.display());
                                }
                            }
                        }
                    });
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
                // Check if there are is at least one Match to commit
                let do_commit = dir.pages.iter().any(|p| p.changes);
                if do_commit && !self.dry_run {
                    git::commit(repo, msg).expect("Error committing changes");
                } else if do_commit {
                    println!("    Would have committed {}", dir.relative_path.display());
                };
            } else {
                println!(
                    "Skipping {} not a git repository",
                    dir.relative_path.display()
                );
            }
        });
    }

    /// Push changes to remote
    pub fn remote_push(&self, username: &str) {
        self.dirs.iter().for_each(|dir| {
            let repo = dir.repo.as_ref().expect("Error unwrapping repo");

            if self.dry_run {
                println!(
                    "Would have pushed {} to remote",
                    dir.relative_path.display()
                );
            } else {
                println!("Pushing {} to remote", dir.relative_path.display());
                git::push(repo, username).expect("Error pushing repository");
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
        let raider = RepoRaider::new(path.clone(), false);
        assert_ne!(raider.path.to_str().unwrap(), path.as_str());
    }

    #[test]
    fn raider_find_dirs() {
        let path = "./".to_string();
        let mut raider = RepoRaider::new(path, false);

        raider.find_dirs("src");
        assert_ne!(raider.get_dirs().len(), 0);
    }

    #[test]
    fn raider_find_repos() {
        let path = "../".to_string();
        let mut raider = RepoRaider::new(path, false);

        raider.find_repos();
        assert_ne!(raider.get_dirs().len(), 0);
    }

    #[test]
    fn raider_match_files() {
        let path = "../".to_string();
        let mut raider = RepoRaider::new(path, false);

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
        let mut raider = RepoRaider::new(path, false);

        raider.find_repos();
        raider.match_files("main.rs");
        raider.match_lines("RepoRaider");

        raider.get_matches().iter().for_each(|m| {
            assert!(m.content.contains("RepoRaider"));
        });
    }
}
