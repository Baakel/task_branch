use std::process::Command;
use std::io::prelude::*;

fn main() {
    let git_head_name = Command::new("git").args(["symbolic-ref", "HEAD"]).output().expect("Couldn't run symb-refs");
    let full_branch_name = String::from_utf8(git_head_name.stdout).expect("Cannot convert bytes to string");
    let vector_branch: Vec<&str> = full_branch_name.trim().split("/").collect();
    if vector_branch.len() < 4 {
        return
    }
    if vector_branch[2] != "development" && vector_branch[2] != "main" && !vector_branch[2].is_empty() {
        println!("We gon dunit");
    }
    println!("{:?}", vector_branch);
}
