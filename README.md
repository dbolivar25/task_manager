# Overview

Rust Task Manager is a simple, yet effective task management application written
in Rust. It allows users to create, manage, and organize tasks with ease. The
application is built with modularity and simplicity in mind, providing a clear
and intuitive interface for managing daily tasks.

## Features

- Task Creation and Management: Users can create, edit, and remove tasks.
- Task Prioritization: Tasks can be prioritized with weights (Low, Medium,
  High).
- Task Listing: Users can list all tasks, providing a clear overview of what
  needs to be done.
- Data Persistence: Tasks are saved in a JSON file, ensuring that your data
  persists across sessions.
- Simple and Robust: The application is designed to be straightforward and easy
  to use, with error handling and data validation for robustness.

## Modules

The project is organized into the following modules:

- app_driver.rs: The main driver of the application, handling user interaction
  and application logic.
- main.rs: The entry point of the application.
- task_manager.rs: Manages task-related operations such as creating, editing,
  and listing tasks.
- task.rs: Defines the Task structure and related functionalities.

# Getting Started

## Prerequisites

Ensure you have Rust installed on your system. You can install Rust via rustup.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/dbolivar25/task_manager
cd task_manager
```

## Running the Application

Compile and run the application using Cargo:

```bash
cargo run
```

# Usage

The application provides a command-line interface for interacting with your
tasks. Use the following commands to manage your tasks:

- Create a new task
- Edit an existing task
- Remove a task
- List all tasks
- Prioritize tasks

Refer to the in-app help for more detailed instructions.
