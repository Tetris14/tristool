use chrono;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    path: String,
    date: String,
    stack: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectList {
    projects: Vec<Project>,
}

fn get_storage_path() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".tristool");
    path.push("projects.json");
    return path;
}

pub fn save_project(project_name: &str, project_path: &str, stack: &str) {
    let file_path = get_storage_path();

    fs::create_dir_all(file_path.parent().unwrap()).unwrap();

    let mut project_list = if file_path.exists() {
        let mut file = File::open(&file_path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str(&data).unwrap_or(ProjectList { projects: vec![] })
    } else {
        ProjectList { projects: vec![] }
    };

    project_list.projects.push(Project {
        name: project_name.to_string(),
        path: project_path.to_string(),
        date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        stack: stack.to_string(),
    });

    // Save back to the file
    let json = serde_json::to_string_pretty(&project_list).unwrap();
    let mut file = File::create(&file_path).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!(
        "Project '{}' saved successfully at path '{}'.",
        project_name, project_path
    );
}
