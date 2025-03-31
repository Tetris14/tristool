mod git;
mod stacks;
mod utils;

use clap::{Arg, Command};
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use git::create_git_repo::create_git_repo;
use stacks::create_c::create_c;
use stacks::create_cpp::create_cpp;
use stacks::create_expo::create_expo;
use stacks::create_nestjs::create_nestjs;
use stacks::create_nextjs::create_nextjs;
use stacks::create_rust::create_rust;
use utils::list_projects::list_projects;
use utils::project_manager::save_project;

fn main() {
    let matches = Command::new("tristool")
        .version("1.0")
        .author("Tristan Gory <tristan@example.com>")
        .about("A simple CLI tool")
        .arg(
            Arg::new("command")
                .help("The command to execute (e.g., 'new')")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("name")
                .help("The name of the project")
                .required(false)
                .index(2),
        )
        .get_matches();

    let command = matches.get_one::<String>("command").unwrap();
    if command == "new" {
        let mut project_stack = "Default";
        println!("Creating a new project...");
        let mut project_name = "project-created-by-tristool";
        if let Some(name) = matches.get_one::<String>("name") {
            println!("Project name: {}", name);
            project_name = name;
        }
        println!("Please select a template:");
        let choices = vec![
            style("NestJS").cyan().to_string(),
            style("NextJS").blue().to_string(),
            style("React Native with Expo").green().to_string(),
            style("Rust").red().to_string(),
            style("C").yellow().to_string(),
            style("C++").magenta().to_string(),
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&choices)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                project_stack = "NestJS";
                create_nestjs(project_name);
            }
            1 => {
                project_stack = "NextJS";
                create_nextjs(project_name);
            }
            2 => {
                project_stack = "React Native with Expo";
                create_expo(project_name);
            }
            3 => {
                project_stack = "Rust";
                create_rust(project_name);
            }
            4 => {
                project_stack = "C";
                create_c(project_name);
            }
            5 => {
                project_stack = "C++";
                create_cpp(project_name);
            }
            _ => println!("Invalid selection"),
        }
        create_git_repo(project_name);
        let project_path = std::fs::canonicalize(project_name).unwrap();
        save_project(
            project_name,
            &project_path.display().to_string(),
            project_stack,
        );
    } else if command == "list" {
        println!();
        list_projects();
    } else {
        println!("Unknown command: {}", command);
    }
}
