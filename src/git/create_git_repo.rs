use console::style;
use std::process::Command as StdCommand;

pub fn create_git_repo(project_name: &str) {
    println!("{}", style("Generating project folder...").cyan());
    let mut output = StdCommand::new("git")
        .arg("init")
        .arg(project_name)
        .spawn()
        .expect("Failed to execute command");
    let _ = output.wait().expect("Command wasn't running");
    let mut output = StdCommand::new("git")
        .arg("add")
        .arg(".")
        .current_dir(project_name)
        .spawn()
        .expect("Failed to execute command");
    let _ = output.wait().expect("Command wasn't running");
    let mut output = StdCommand::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("Initial commit for {}", project_name))
        .current_dir(project_name)
        .spawn()
        .expect("Failed to execute command");
    let _ = output.wait().expect("Command wasn't running");
    println!(
        "{}",
        style("Git repository initialized successfully!").green()
    );
}
