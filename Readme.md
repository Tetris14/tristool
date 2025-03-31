# Tristool

Tristool is a command-line interface (CLI) tool designed to streamline the process of creating new projects. It currently supports generating projects in the following frameworks and languages:

- C
- C++
- Next.js
- Nest.js
- Expo
- Rust

## Features

- Quickly scaffold new projects in supported languages and frameworks.
- Simple and efficient CLI interface.

## Installation

Tristool is developed in Rust. To build the binary and make it available globally on your system, follow these steps:

1. Build the project in release mode:

```bash
cargo build --release
```

2. Copy the binary to a directory in your system's PATH:

```bash
sudo cp target/release/tristool /usr/local/bin/
```

## Usage

To verify the installation, check the version of Tristool:

```bash
tristool --version
```

To create a new project, use the following command:

```bash
tristool new <project_name>
```
