use std::time::Instant;

use clap::Parser;
use git_raider::config::Config;
use git_raider::raider::RepoRaider;

fn main() {
    let conf = Config::parse();
    let start: Instant = Instant::now();

    // Recursively find directories that are git repositories
    let mut raider = RepoRaider::new(&conf.path);
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
    if let Some(line_select_pattern) = conf.line_select_pattern {
        if let Some(line_replace_pattern) = conf.line_replace_pattern {
            raider.replace(line_select_pattern.as_str(), line_replace_pattern.as_str());
        } else {
            panic!("Replace file pattern required for line select");
        }
    }

    // If dry run is not set
    // Do not alter files and stage changes
    // TODO: Make dry run simulate altering files and staging changes
    if !conf.dry_run {
        // Apply replace patterns
        raider.apply();

        // Stage matches
        raider.stage();

        // Commit changes with message
        if let Some(commit_message) = conf.commit {
            raider.commit(commit_message.as_str());

            // If push flag is set, push to remote
            if conf.push {
                raider.push();
            }
        }
    }

    // Print results for found directories, Pages and matches
    if conf.display_results {
        results(&raider);
    }

    println!("Elapsed: {:.3?}", start.elapsed());
}

/// Print results
fn results(raider: &RepoRaider) {
    // git_raider::func::paths_info_print(&raider.get_dirs(), "found directories (repos)", 5);
    println!("REPORT");
    println!("M n - Matched files, number of matched lines");
    println!("  O n - Original line, line number");
    println!("  R n - Replaced line, line number");
    println!("FILES:");
    raider.get_pages().iter().for_each(|p| {
        println!("M {}: {}", p.matches.len(), p.relative_path.display());
        p.matches
            .iter()
            .filter(|m| m.replace.is_some())
            .for_each(|m| {
                println!("  O {:<3} {}", m.line, m.content);
                if let Some(r) = m.replace.as_ref() {
                    println!("  R {:<3} {}", m.line, r);
                } else {
                    println!("  R None");
                }
            });
    });
}
