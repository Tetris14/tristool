use clap::{Arg, Command};
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command as StdCommand;

fn main() {
    let matches = Command::new("tristool")
        .version("1.0")
        .author("Tristan Gory <tristan@example.com>")
        .about("A simple CLI tool")
        .arg(
            Arg::new("command") // Positional argument
                .help("The command to execute (e.g., 'new')")
                .required(true) // This argument is mandatory
                .index(1), // First positional argument
        )
        .arg(
            Arg::new("name") // Another positional argument
                .help("The name of the project")
                .required(false) // This argument is optional
                .index(2), // Second positional argument
        )
        .get_matches();

    // Handle the "command" argument
    let command = matches.get_one::<String>("command").unwrap();
    if command == "new" {
        println!("Creating a new project...");
        // Handle the "name" argument (if provided)
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
                // Here you can add the logic to create a NestJS project
                let mut output = StdCommand::new("nest")
                    .arg("new")
                    .arg(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                println!("NestJS project created successfully!");
            }
            1 => {
                let mut output = StdCommand::new("npx")
                    .arg("create-next-app@latest")
                    .arg(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                println!("NextJS project created successfully!");
            }
            2 => {
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
            3 => {
                let mut output = StdCommand::new("cargo")
                    .arg("new")
                    .arg(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                println!("Rust project created successfully!");
            }
            4 => {
                println!("{}", style("Generating project folder...").cyan());
                let mut output = StdCommand::new("mkdir")
                    .arg(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut output = StdCommand::new("touch")
                    .arg("Makefile")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/Makefile", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "# Makefile for compiling all C files in /src with headers in /headers\n\
                    # Output binary will be named '{}'\n\n\
                    CC = gcc\n\
                    CFLAGS = -Iheaders -Wall -Wextra -Werror\n\
                    SRC_DIR = src\n\
                    OBJ_DIR = obj\n\
                    BIN = {}\n\n\
                    SRCS = $(wildcard $(SRC_DIR)/*.c)\n\
                    OBJS = $(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%.o, $(SRCS))\n\n\
                    $(BIN): $(OBJS)\n\
                    \t$(CC) $(CFLAGS) -o $@ $^\n\n\
                    $(OBJ_DIR)/%.o: $(SRC_DIR)/%.c\n\
                    \t@mkdir -p $(OBJ_DIR)\n\
                    \t$(CC) $(CFLAGS) -c $< -o $@\n\n\
                    clean:\n\
                    \trm -rf $(OBJ_DIR) $(BIN)\n\n\
                    .PHONY: clean\n",
                    project_name, project_name
                )
                .expect("Failed to write to Makefile");
                println!("{}", style("Generating src and headers folders...").cyan());
                let mut output = StdCommand::new("mkdir")
                    .arg("src")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut output = StdCommand::new("mkdir")
                    .arg("headers")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                println!("{}", style("Generating main.c and main.h files...").cyan());
                let mut output = StdCommand::new("touch")
                    .arg("main.c")
                    .current_dir(format!("{}/src", project_name))
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/src/main.c", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "#include \"../headers/main.h\"\n\n\
                    int main() {{\n\
                    \tprintf(\"Hello, World!\\n\");\n\
                    \treturn 0;\n\
                    }}\n"
                )
                .expect("Failed to write to main.c");
                let mut output = StdCommand::new("touch")
                    .arg("main.h")
                    .current_dir(format!("{}/headers", project_name))
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/headers/main.h", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "#ifndef MAIN_H\n\
                    #define MAIN_H\n\n\
                    #include <stdio.h>\n\n\
                    #endif // MAIN_H\n"
                )
                .expect("Failed to write to main.h");
                println!("{}", style("Generating .gitignore file...").cyan());
                let mut output = StdCommand::new("touch")
                    .arg(".gitignore")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/.gitignore", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "# Compiled object files\n\
                    obj/\n\n\
                    # Executable files\n\
                    {}\n\n\
                    # Backup files\n\
                    *~\n",
                    project_name
                )
                .expect("Failed to write to .gitignore");
                println!("{}", style("C project created successfully!").green());
            }
            5 => {
                println!("{}", style("Generating project folder...").cyan());
                let mut output = StdCommand::new("mkdir")
                    .arg(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut output = StdCommand::new("touch")
                    .arg("Makefile")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/Makefile", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "# Makefile for compiling all C++ files in /src with headers in /headers\n\
                    # Output binary will be named '{}'\n\n\
                    CXX = g++\n\
                    CXXFLAGS = -Iheaders -Wall -Wextra -Werror -std=c++17\n\
                    SRC_DIR = src\n\
                    OBJ_DIR = obj\n\
                    BIN = {}\n\n\
                    SRCS = $(wildcard $(SRC_DIR)/*.cpp)\n\
                    OBJS = $(patsubst $(SRC_DIR)/%.cpp, $(OBJ_DIR)/%.o, $(SRCS))\n\n\
                    $(BIN): $(OBJS)\n\
                    \t$(CXX) $(CXXFLAGS) -o $@ $^\n\n\
                    $(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp\n\
                    \t@mkdir -p $(OBJ_DIR)\n\
                    \t$(CXX) $(CXXFLAGS) -c $< -o $@\n\n\
                    clean:\n\
                    \trm -rf $(OBJ_DIR) $(BIN)\n\n\
                    .PHONY: clean\n",
                    project_name, project_name
                )
                .expect("Failed to write to Makefile");
                println!("{}", style("Generating src and headers folders...").cyan());
                let mut output = StdCommand::new("mkdir")
                    .arg("src")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut output = StdCommand::new("mkdir")
                    .arg("headers")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                println!(
                    "{}",
                    style("Generating main.cpp and main.h files...").cyan()
                );
                let mut output = StdCommand::new("touch")
                    .arg("main.cpp")
                    .current_dir(format!("{}/src", project_name))
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/src/main.cpp", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "#include \"../headers/main.h\"\n\n\
                    int main() {{\n\
                    \tstd::cout << \"Hello, World!\" << std::endl;\n\
                    \treturn 0;\n\
                    }}\n"
                )
                .expect("Failed to write to main.cpp");
                let mut output = StdCommand::new("touch")
                    .arg("main.h")
                    .current_dir(format!("{}/headers", project_name))
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/headers/main.h", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "#ifndef MAIN_H\n\
                    #define MAIN_H\n\n\
                    #include <iostream>\n\n\
                    #endif // MAIN_H\n"
                )
                .expect("Failed to write to main.h");
                println!("{}", style("Generating .gitignore file...").cyan());
                let mut output = StdCommand::new("touch")
                    .arg(".gitignore")
                    .current_dir(project_name)
                    .spawn()
                    .expect("Failed to execute command");
                let _ = output.wait().expect("Command wasn't running");
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(format!("{}/.gitignore", project_name))
                    .expect("Failed to open file");
                writeln!(
                    file,
                    "# Compiled object files\n\
                    obj/\n\n\
                    # Executable files\n\
                    {}\n\n\
                    # Backup files\n\
                    *~\n",
                    project_name
                )
                .expect("Failed to write to .gitignore");
                println!("{}", style("C++ project created successfully!").green());
            }
            _ => {
                println!("Invalid selection");
            }
        }
    } else {
        println!("Unknown command: {}", command);
        return;
    }
}
