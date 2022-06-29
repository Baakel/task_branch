use std::process::Command;
use std::io::{self,Write};

fn main() {
    let git_head_name = Command::new("git").args(["symbolic-ref", "HEAD"]).output().expect("Couldn't run symb-refs");
    let full_branch_name = String::from_utf8(git_head_name.stdout).expect("Cannot convert bytes to string");
    let vector_branch: Vec<&str> = full_branch_name.trim().split('/').collect();
    let repo_name_command = Command::new("git").args(["rev-parse", "--show-toplevel"]).output().expect("git path command didn't work");
    let full_repo_name = String::from_utf8(repo_name_command.stdout).expect("Couldn't convert full repo name");
    let basename = Command::new("basename").args([&full_repo_name.trim()]).output().expect("Couldn't run basename");
    let basename_string = String::from_utf8(basename.stdout).expect("Couldn't transform basename to string");
    if vector_branch.len() < 4 {
        return
    }
    if vector_branch[2] != "development" && vector_branch[2] != "main" && !vector_branch[2].is_empty() {
        let mut stdout = io::stdout().lock();
        let existing_task = Command::new("task").args([format!("proj:{}", &basename_string.trim()).as_str(), format!("+{}", &vector_branch[2]).as_str(), format!("prio:{}", &vector_branch[2]).as_str(), format!("/{}/", &vector_branch[3]).as_str(), "list"]).output().expect("Couldn't run the list task command");
        if !existing_task.status.success() {
            println!("There is no task, making a new one");
            let new_task = Command::new("task").args(["add", format!("proj:{}", &basename_string.trim()).as_str(), format!("+{}" ,&vector_branch[2]).as_str(), format!("prio:{}", &vector_branch[2]).as_str(), vector_branch[3]]).output().expect("Couldn't run add task command");
            stdout.write_all(&new_task.stdout).expect("Couldn't write to stdout new task");
            if !&new_task.stderr.is_empty() {
                stdout.write_all(&new_task.stderr).expect("Couldn't write to std err");
            }
        }
        stdout.write_all(&existing_task.stdout).expect("Couldn't write to stdout");
    }
}
