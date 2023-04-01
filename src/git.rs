use git2::{Branch, BranchType, Branches, Oid, Repository};
use std::path::PathBuf;

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
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("    Failed to set HEAD");

    let head = repo.head().unwrap().target().unwrap();
    println!("    Success checkout {} {}", refname, &head);

    Ok(head)
}

/// Stage all changes
pub fn stage_all(repo: &mut Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

/// Commit stages changes
/// TODO: Fix `{ code: -15, klass: 11, message: "failed to create commit: current tip is not the first parent" }'`
pub fn commit(repo: &mut Repository, msg: &str) -> Result<(), git2::Error> {
    let signature = repo.signature().unwrap();
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(Some("HEAD"), &signature, &signature, msg, &tree, &[])?;
    Ok(())
}
