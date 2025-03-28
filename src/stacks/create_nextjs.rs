pub fn create_nextjs(project_name: &str) {
    let mut output = std::process::Command::new("npx")
        .arg("create-next-app")
        .arg(project_name)
        .spawn()
        .expect("Failed to execute command");
    let _ = output.wait().expect("Command wasn't running");
    println!("Next.js project created successfully!");
}
