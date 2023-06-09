use std::time::Instant;

use clap::Parser;
use gitraider::config::Config;
use gitraider::raider::RepoRaider;

fn main() {
    let conf = Config::parse();
    let start: Instant = Instant::now();

    // Recursively find directories that are git repositories
    let mut raider = RepoRaider::new(conf.path, conf.dry_run);
    raider.find_repos();

    // Check out branch that matches regex pattern
    raider.checkout_branch(conf.branch_pattern.as_str());

    // Match files with regex pattern
    if let Some(file_pattern) = conf.file_pattern {
        raider.match_files(file_pattern.as_str());
    }

    // Match lines in files that match regex pattern
    if let Some(content_pattern) = conf.line_pattern {
        raider.match_lines(content_pattern.as_str());
    }

    // Create replace patterns for each pattern
    if let Some(select) = conf.line_select_pattern {
        conf.line_replace_pattern.map_or_else(
            || {
                panic!("ERROR: No replace flag specified");
            },
            |replace| {
                raider.replace(select.as_str(), replace.as_str());
            },
        );
    }

    // Apply replace patterns to files
    raider.apply();

    // Stage matches
    raider.stage();

    // Commit changes with message
    if let Some(commit_message) = conf.commit {
        raider.commit(commit_message.as_str());

        // If push flag is set, push to remote
        if conf.push && conf.username.is_some() {
            raider.remote_push(conf.username.expect("Error unwrapping username").as_str());
        }
        // If username or password was not set then throw an error
        else if conf.push {
            panic!("ERROR: Git username must be specified for push");
        }
    }

    // Print assessment for found Directories, Pages and Matches
    if conf.assess {
        assessment(&raider);
    }

    println!("Elapsed: {:.3?}", start.elapsed());
}

/// Print assessment
fn assessment(raider: &RepoRaider) {
    // gitraider::func::paths_info_print(&raider.get_dirs(), "found directories (repos)", 5);
    println!("REPORT");
    println!("Fn: - Matched files, number of matched lines");
    println!("  Ln: - Original line, line number");
    println!("  Rn: - Replace line (if present), line number");
    println!("GIT REPOSITORIES");
    for dir in &raider.dirs {
        if dir.pages.iter().any(|p| !p.matches.is_empty()) {
            let branch_name =
                gitraider::git::get_branch_name(dir.repo.as_ref().expect("Folder not a git repo"))
                    .expect("Error getting repo branch name");
            println!("\nRepository: {}", dir.relative_path.display());
            println!("Branch: {branch_name}");
            dir.pages.iter().for_each(|p| {
                println!("  F{}: {}", p.matches.len(), p.relative_path.display());
                // Loop through matches
                p.matches.iter().for_each(|m| {
                    println!("    L{}: {}", m.line, m.content);
                    // Print replace line if Some
                    if let Some(r) = &m.replace {
                        println!("    R{}: {}", m.line, r);
                    }
                });
            });
        }
    }
}
