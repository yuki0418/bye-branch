use std::process::Command;

fn get_local_branches(ignore_branches: Vec<&str>) -> Vec<String> {
    let output = Command::new("git")
        .arg("branch")
        .output()
        .expect("failed to execute git branch");

    let branches_str =
        String::from_utf8(output.stdout).expect("failed to convert output to string");

    let branches: Vec<String> = branches_str
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with('*') && !ignore_branches.contains(s))
        .map(|s| s.to_string())
        .collect();

    branches
}

fn delete_branches(branches: Vec<String>) -> i32 {
    let mut count = 0;
    branches.iter().for_each(|b| {
        match Command::new("git").arg("branch").arg("-D").arg(b).output() {
            Ok(_) => count += 1,
            Err(_) => println!("Failed to delete branch {}", b),
        }
    });

    count
}

fn main() {
    let ignore_branches = vec!["master", "main"];
    let branches = get_local_branches(ignore_branches);
    let count = delete_branches(branches);

    println!("{} branches deleted", count);
}
