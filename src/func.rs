use regex::Regex;
use std::fs;
use std::path::PathBuf;

/// Recursively find directories
#[must_use]
pub fn find_dirs(dir: &PathBuf, name: &str, parent: &bool) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let path = entry.expect("Error unpacking path").path();
            if path.is_dir() {
                if fs::read_dir(&path.join(name)).is_ok() {
                    match parent {
                        // Get path the directory itself
                        false => result.push(path.clone()),
                        // Get path of parent of the directory instead of the directory itself
                        true => {
                            if let Some(parent) = path.parent() {
                                result.push(parent.to_path_buf());
                            }
                        }
                    }
                } else {
                    result.append(&mut find_dirs(&path, name, parent));
                }
            }
        }
    }
    result
}

/// Recursively find files
#[must_use]
pub fn find_files(dir: &str, pattern: &str) -> Vec<PathBuf> {
    let re = Regex::new(pattern).expect("Error compiling regex");
    let mut found_files = Vec::new();
    let dir_entries = fs::read_dir(dir).expect("Error reading path");

    for entry in dir_entries {
        let entry = entry.expect("Error unwrapping directory entry");
        let path = entry.path();
        let file_name = path
            .file_name()
            .expect("Error unwrapping file Path")
            .to_str()
            .expect("Error converting Path to str");
        if path.is_file() && re.is_match(file_name) {
            // If path is a file and is a match
            // Push to found files
            found_files.push(path.clone());
        } else if path.is_dir() {
            // Otherwise proceed to recursion
            found_files.append(&mut find_files(
                path.to_str().expect("Error converting Path to str"),
                pattern,
            ));
        }
    }
    found_files
}

/// Prints info about paths
pub fn paths_info_print(list: &Vec<PathBuf>, msg: &str, elements: usize) {
    println!("First {} ({}) {}:", elements, list.len(), msg);
    for f in 0..elements {
        if let Some(val) = list.get(f) {
            println!("{}", val.display());
        } else {
            break;
        }
    }
}
