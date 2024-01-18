use requestty::symbols::SymbolSet;
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

// Windows Terminal renders U+2714 "Heavy Check Mark" as a purple emoji which masks the terminal
// color and makes it impossible to determine if an item is selected or not.
//
// See: https://github.com/microsoft/terminal/issues/15592
//      https://github.com/microsoft/terminal/issues/13110
//
// Fortunately, this doesn't affect U+2713 "Check Mark" so for Windows terminal we use that
// character instead until the emoji issue is fixed.
const WINDOWS_TERMINAL_SYMBOLS: SymbolSet = SymbolSet {
    completed: '\u{2713}', // '✓'
    ..requestty::symbols::UNICODE
};

fn is_windows_terminal() -> bool {
    std::env::var("WT_SESSION").is_ok()
}

fn prompt_get_selected_branches(all_branches: &Vec<String>) -> Vec<String> {
    if is_windows_terminal() {
        requestty::symbols::set(WINDOWS_TERMINAL_SYMBOLS);
    }

    let question = requestty::Question::multi_select("branches")
        .message("Select branches to delete")
        .choices(all_branches)
        .build();

    let result = match requestty::prompt_one(question) {
        Ok(result) => result,
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
    let count = delete_branches(selected_branches);

    println!("{} branches deleted successfully", count);
}
