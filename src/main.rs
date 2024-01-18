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

fn prompt_get_selected_branches(all_branches: &Vec<String>) -> Vec<String> {
    let question = requestty::Question::multi_select("branches")
        .message("Select branches to delete")
        .choices(all_branches)
        .on_esc(requestty::OnEsc::Terminate)
        .build();

    let result = match requestty::prompt_one(question) {
        Ok(result) => result,
        Err(requestty::ErrorKind::Aborted) => {
            return vec![];
        }
        Err(_) => {
            println!("Failed to get selected branches");
            return vec![];
        }
    };

    let selected_branches: Vec<String> = result
        .as_list_items()
        .unwrap()
        .iter()
        .map(|i| i.text.clone())
        .collect();

    selected_branches
}

fn main() {
    let ignore_branches = vec!["master", "main"];
    let branches = get_local_branches(ignore_branches);

    if branches.is_empty() {
        println!("No branches to delete");
        return;
    }

    let selected_branches = prompt_get_selected_branches(&branches);
    if selected_branches.is_empty() {
        return;
    }

    let count = delete_branches(selected_branches);

    println!("{} branches deleted successfully", count);
}
