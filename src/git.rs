use git2::{Branch, BranchType, Branches, Commit, Oid, PushOptions, RemoteCallbacks, Repository};
use std::path::{Path, PathBuf};

/// Get repo from path
pub fn get_repo(path: &PathBuf) -> Result<Repository, git2::Error> {
    let repo = Repository::open(path)?;
    Ok(repo)
}

/// Get all branches in a repo
pub fn get_branches(repo: &Repository) -> Result<Branches, git2::Error> {
    let branches = repo.branches(Some(BranchType::Local))?;
    Ok(branches)
}

/// Get a branches refname
#[must_use]
pub fn get_ref<'a>(branch: &'a Branch) -> &'a str {
    let refname = branch
        .name()
        .ok()
        .unwrap()
        .expect("Error getting branch's ref");
    refname
}

/// Checkout a branch in a repo using Branch struct
pub fn checkout_branch(repo: &Repository, branch: &Branch) -> Result<Oid, git2::Error> {
    let refname = get_ref(branch);
    println!("  Checking out {}", &refname);

    let (object, reference) = repo.revparse_ext(refname).expect("    Object not found");

    repo.checkout_tree(&object, None)
        .expect("    Failed to checkout");

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().expect("Error unwrapping refname")),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("    Failed to set HEAD");

    let head = repo
        .head()
        .expect("Error unwrapping repo head")
        .target()
        .expect("Error head target");
    println!("  Success branch checkout '{}' {}", refname, head);

    Ok(head)
}

/// Stage all changes
pub fn stage_all(repo: &mut Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

/// Stage specific files
pub fn stage_file(repo: &mut Repository, file: &Path) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    index.add_path(file)?;
    index.write()?;
    Ok(())
}

/// Commit staged changes
pub fn commit(repo: &mut Repository, msg: &str) -> Result<(), git2::Error> {
    // Gather git objects
    let mut index = repo.index().expect("Error unwrapping repo index");
    let oid = index.write_tree().expect("Error unwrapping index tree");
    let signature = repo.signature().expect("Error getting user's signature");
    let parent_commit = get_last_commit(repo).expect("Error getting last commit");
    let tree = repo.find_tree(oid).expect("Error unwrapping tree");

    // Create new commit
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        msg,
        &tree,
        &[&parent_commit],
    )?;

    // Get new commit
    let new_commit = get_last_commit(repo).expect("Error getting new commit");
    let new_msg = new_commit.message().expect("Error getting commit message");
    let new_head = new_commit.id();

    // Check if commit message matches
    if new_msg == msg {
        println!("    Success commit '{}' {}", new_msg, new_head)
    } else {
        println!("    Warning, commit message mismatch '{}'", new_msg)
    }

    Ok(())
}

/// Push changes to remote
pub fn push(repo: &Repository) -> Result<(), git2::Error> {
    // Setup remote
    let mut opts = PushOptions::default();
    let callbacks = RemoteCallbacks::new();
    opts.remote_callbacks(callbacks);

    // Assume the remote's name is "origin"
    let mut remote = repo.find_remote("origin")?;

    // Push to remote
    remote.push::<&str>(&[], Some(&mut opts))?;

    Ok(())
}

/// Get last commit
fn get_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let commit = repo
        .head()
        .expect("Error unwrapping commit from head")
        .peel_to_commit()?;
    Ok(commit)
}
