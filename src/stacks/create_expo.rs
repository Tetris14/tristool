use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use std::process::Command as StdCommand;

pub fn create_expo(project_name: &str) {
    let choices = vec![
        style("Yes!").green().to_string(),
        style("No.").red().to_string(),
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to use a template ?")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();
    if selection == 0 {
        println!("You selected: {}", choices[selection]);
        let mut output = StdCommand::new("npx")
            .arg("create-expo-app")
            .arg(project_name)
            .arg("--template")
            .spawn()
            .expect("Failed to execute command");
        let _ = output.wait().expect("Command wasn't running");
        println!("React Native with Expo project created successfully!");
        return;
    }
    let mut output = StdCommand::new("npx")
        .arg("create-expo-app")
        .arg(project_name)
        .spawn()
        .expect("Failed to execute command");
    let _ = output.wait().expect("Command wasn't running");
    println!("React Native with Expo project created successfully!");
}
