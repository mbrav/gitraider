use std::time::Instant;

use clap::Parser;
use git_raider::config::Config;
// use git_raider::func;
use git_raider::raider::RepoRaider;

fn main() {
    // func::are_you_on_linux();
    let conf = Config::parse();
    let start: Instant = Instant::now();

    let mut raider = RepoRaider::new(&conf.path);

    raider.find_repos();
    raider.checkout_branch(conf.branch_pattern.as_str());

    // Match files
    if let Some(file_pattern) = conf.file_pattern {
        raider.match_files(file_pattern.as_str());
    }

    // Match lines in files
    if let Some(content_pattern) = conf.line_pattern {
        raider.match_lines(content_pattern.as_str());
    }

    // Generate replace patterns for each pattern
    if let Some(line_select_pattern) = conf.line_select_pattern {
        if let Some(line_replace_pattern) = conf.line_replace_pattern {
            raider.replace(line_select_pattern.as_str(), line_replace_pattern.as_str());
        } else {
            panic!("Replace file pattern required for line select pattern");
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

            if conf.push {
                !todo!();
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
    // func::paths_info_print(&raider.get_dirs(), "found directories (repos)", 5);
    println!("REPORT");
    println!("M n - Matched files, number of matched lines");
    println!("  O n - Original line, line number");
    println!("  R n - Replaced line, line number");
    println!("Files:");
    raider.get_pages().iter().for_each(|p| {
        println!("M {}: {}", p.matches.len(), p.relative_path.display());
        p.matches.iter().for_each(|m| {
            println!("  O {:<3} {}", m.line, m.content);
            if let Some(r) = m.replace.as_ref() {
                println!("  R {:<3} {}", m.line, r);
            } else {
                println!("  R None");
            }
        });
    });
}
