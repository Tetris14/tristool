use console::style;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command as StdCommand;

pub fn create_c(project_name: &str) {
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
