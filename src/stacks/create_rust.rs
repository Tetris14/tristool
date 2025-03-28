use std::process::Command as StdCommand;

pub fn create_rust(project_name: &str) {
    let mut output = StdCommand::new("cargo")
        .arg("new")
        .arg(project_name)
        .spawn()
        .expect("Failed to execute command");
    let _ = output.wait().expect("Command wasn't running");
    println!("Rust project created successfully!");
}
