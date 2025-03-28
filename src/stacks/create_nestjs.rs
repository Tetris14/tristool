use std::process::Command as StdCommand;

pub fn create_nestjs(project_name: &str) {
    let mut output = StdCommand::new("nest")
        .arg("new")
        .arg(project_name)
        .spawn()
        .expect("Failed to execute command");
    let _ = output.wait().expect("Command wasn't running");
    println!("NestJS project created successfully!");
}
