use console::style;
use dirs::home_dir;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct Project {
    name: String,
    path: String,
    date: String,
    stack: String,
}

#[derive(Deserialize, Debug)]
struct ProjectList {
    projects: Vec<Project>,
}

pub fn get_storage_path() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".tristool");
    path.push("projects.json");
    path
}

pub fn list_projects() {
    let file_path = get_storage_path();

    if !file_path.exists() {
        println!("No projects found. The projects.json file doesn't exist yet.");
        return;
    }

    let mut file = File::open(&file_path).expect("Failed to open projects.json file.");
    if file.metadata().unwrap().len() == 2 {
        println!("No projects found. The projects.json file is empty.");
        return;
    }
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read projects.json file.");

    let project_list: ProjectList =
        serde_json::from_str(&data).expect("Failed to parse JSON data.");

    for project in project_list.projects {
        let stack_display = match project.stack.as_str() {
            "NestJS" => style(&project.stack).cyan().to_string(),
            "NextJS" => style(&project.stack).blue().to_string(),
            "React Native with Expo" => style(&project.stack).green().to_string(),
            "Rust" => style(&project.stack).red().to_string(),
            "C" => style(&project.stack).yellow().to_string(),
            "C++" => style(&project.stack).magenta().to_string(),
            _ => style(&project.stack).white().to_string(),
        };
        let name = style("Name:").yellow().to_string();
        let path = style("Path:").blue().to_string();
        let date = style("Date:").green().to_string();

        println!(
            "{} {}, {} {}, {} {}, Stack: {}",
            name, project.name, path, project.path, date, project.date, stack_display
        );
    }
}
