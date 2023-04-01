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
    pub fn new(path: String) -> Self {
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
                path: x.to_path_buf(),
                repo: None,
                pages: Vec::new(),
                relative_path: x.strip_prefix(&self.path).unwrap().to_path_buf(),
            })
            .collect()
    }

    /// Searches for directories that are git repositories
    pub fn find_repos(&mut self) {
        self.dirs = func::find_dirs(&self.path, ".git", &false)
            .iter()
            .map(|x| structs::Directory {
                path: x.to_path_buf(),
                repo: Some(git::get_repo(x).expect("Error getting repo")),
                pages: Vec::new(),
                relative_path: x.strip_prefix(&self.path).unwrap().to_path_buf(),
            })
            .collect()
    }

    /// Checks out a branch in all directories that are repos
    pub fn checkout_branch(&mut self, pattern: &str) {
        let re = Regex::new(pattern).unwrap();
        for dir in &mut self.dirs {
            if let Some(repo) = &dir.repo {
                println!("Repo {}", &dir.relative_path.display());
                let branches = git::get_branches(repo).expect("  ERROR unwrapping repo's Branches");
                let mut matches = 0;

                // Loop through branches
                for branch in branches {
                    let b = branch.unwrap().0;
                    let refname = git::get_ref(&b);

                    // If branch's refname matches regex pattern then checkout
                    if re.is_match(refname) {
                        git::checkout_branch(repo, &b).expect("  ERROR checking out branch");
                        matches += 1;

                        // If there were more than on match
                        // Output a warning
                        if matches > 1 {
                            println!("    WARNING: More than one branch matched")
                        }
                    }
                }
            } else {
                println!("   WARNING: folder is not a repository")
            }
        }
    }

    /// Recursively matches for filenames with a specific name and outputs a result vector
    pub fn match_files(&mut self, pattern: &str) {
        for dir in &mut self.dirs {
            let f: Vec<structs::Page> = func::find_files(dir.path.to_str().unwrap(), pattern)
                .iter()
                .map(|x| {
                    structs::Page {
                        path: x.to_path_buf(),
                        matches: Vec::new(),
                        relative_path: x.strip_prefix(&self.path).unwrap().to_path_buf(), // dir: Rc::new(dir.clone()),
                    }
                })
                .collect();
            dir.pages.extend(f);
        }
    }

    /// Recursively searches for a all lines matching a pattern in a file
    /// and saves them as a Match
    pub fn match_lines(&mut self, pattern: &str) {
        let re = Regex::new(pattern).unwrap();
        for dir in &mut self.dirs {
            for page in &mut dir.pages {
                let file = fs::File::open(&page.path).expect("Error reading file");
                let reader = BufReader::new(file);
                for (line, content) in reader.lines().enumerate() {
                    match content {
                        Err(e) => println!("{}. Skipping file {}", e, page.path.display()),
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
            }
        }
    }

    /// Creates a replace string for Match struct
    pub fn replace(&mut self, select: &str, replace: &str) {
        let re = Regex::new(select).unwrap();
        for dir in &mut self.dirs {
            for page in &mut dir.pages {
                for mat in &mut page.matches {
                    let res = re.replace(mat.content.as_str(), replace);
                    mat.replace = Some(res.to_string());
                }
            }
        }
    }

    /// Apply replace pattern to all Match structs
    /// by the line in a result to a file
    pub fn apply(&mut self) {
        for dir in &mut self.dirs {
            for page in &mut dir.pages {
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
            }
        }
    }

    /// Stage all matches  
    pub fn stage(&mut self) {
        for dir in &mut self.dirs {
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
        }
    }

    /// Commit all matches  
    pub fn commit(&mut self, msg: &str) {
        for dir in &mut self.dirs {
            if let Some(repo) = &mut dir.repo {
                // Commit all staged files
                git::commit(repo, msg).expect("Error committing changes");
            } else {
                println!(
                    "Skipping {} not a git repository",
                    dir.relative_path.display()
                );
            }
        }
    }

    /// Gets all folders
    pub fn get_dirs(&self) -> Vec<PathBuf> {
        self.dirs.iter().map(|f| f.path.to_path_buf()).collect()
    }

    /// Gets all pages contained in all folders
    pub fn get_pages(&self) -> Vec<structs::Page> {
        self.dirs
            .iter()
            .flat_map(|f| f.pages.iter().cloned())
            .collect()
    }

    /// Gets all matches in pages
    pub fn get_matches(&self) -> Vec<structs::Match> {
        self.dirs
            .iter()
            .flat_map(|f| f.pages.iter())
            .flat_map(|p| p.matches.iter().cloned())
            .collect()
    }
}
