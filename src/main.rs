use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use rand::Rng;
use chrono::Local;

fn main() -> std::io::Result<()> {
    let file_name = "commit_log.txt";
    let num_commits = 69; // Change this to the number of commits you want to make

    for i in 1..=num_commits {
        // Append a random number to the file
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_name)?;

        let random_number: u32 = rand::thread_rng().gen();
        writeln!(file, "Commit {}: Random number {}", i, random_number)?;

        // Stage the changes
        Command::new("git")
            .args(&["add", file_name])
            .output()
            .expect("Failed to stage changes");

        // Create a commit
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let commit_message = format!("Commit {} at {}", i, timestamp);
        Command::new("git")
            .args(&["commit", "-m", &commit_message])
            .output()
            .expect("Failed to create commit");

        println!("Created commit {}", i);
    }

    // Push all commits to the remote repository
    Command::new("git")
        .args(&["push"])
        .output()
        .expect("Failed to push commits");

    println!("All commits pushed to remote repository");

    Ok(())
}