use std::io::Write;

use anyhow::{anyhow, Result};

use crate::task::*;
use crate::task_manager::*;

enum ManagerCommand {
    Create,
    Remove(usize),
    Edit(usize),
    List,
    Quit,
}

pub struct App {
    m_task_manager: TaskManager,
}

impl App {
    pub fn new() -> App {
        return App {
            m_task_manager: TaskManager::new(),
        };
    }

    fn print_prompt(&self) {
        println!("\n|- --------------------- -|");
        println!("|       TaskManager       |");
        println!("|- --------------------- -|");
        println!("|- create                -|");
        println!("|- remove <idx>          -|");
        println!("|- edit   <idx>          -|");
        println!("|- list                  -|");
        println!("|- quit                  -|");
        println!("|- --------------------- -|\n");
        print!("|> ");
        std::io::stdout().flush().unwrap();
    }

    fn read_input() -> Result<ManagerCommand> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();
        let command = match iter.next() {
            Some(command) => command,
            None => return Ok(ManagerCommand::Quit),
        };

        match command.to_lowercase().as_str() {
            "create" => {
                if let None = iter.next() {
                    return Ok(ManagerCommand::Create);
                } else {
                    return Err(anyhow!("create command does not take arguments"));
                }
            }
            "remove" => {
                let index = match iter.next() {
                    Some(index) => index.parse::<usize>()?,
                    None => return Err(anyhow!("remove command takes one argument")),
                };

                if let Some(_) = iter.next() {
                    return Err(anyhow!("remove command takes only one argument"));
                }

                return Ok(ManagerCommand::Remove(index));
            }
            "edit" => {
                let index = match iter.next() {
                    Some(index) => index.parse::<usize>()?,
                    None => return Err(anyhow!("edit command takes one argument")),
                };

                if let Some(_) = iter.next() {
                    return Err(anyhow!("edit command takes only one argument"));
                }

                return Ok(ManagerCommand::Edit(index));
            }
            "list" => {
                if let Some(_) = iter.next() {
                    return Err(anyhow!("list command does not take arguments"));
                }

                return Ok(ManagerCommand::List);
            }
            "quit" => {
                if let Some(_) = iter.next() {
                    return Err(anyhow!("quit command does not take arguments"));
                }

                return Ok(ManagerCommand::Quit);
            }
            command => return Err(anyhow!("unknown command: {}", command)),
        }
    }

    fn handle_create(&mut self) -> Result<()> {
        let mut input = String::new();
        let task = TaskBuilder::new();

        println!("-| Context: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            context => task.with_context(context.into()),
        };
        input.clear();

        println!("-| Description: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            description => task.with_description(description.into()),
        };
        input.clear();

        println!("-| Days to start: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            days_to_start => {
                let days_to_start = days_to_start.parse::<usize>()?;
                task.with_days_to_start(days_to_start)
            }
        };
        input.clear();

        println!("-| Days to end: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            days_to_end => {
                let days_to_end = days_to_end.parse::<usize>()?;
                task.with_days_to_end(days_to_end)
            }
        };
        input.clear();

        println!("-| Weight: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            weight => {
                let weight = match weight.to_lowercase().as_str() {
                    "high" => Weight::High,
                    "medium" => Weight::Med,
                    "low" => Weight::Low,
                    _ => return Err(anyhow!("unknown weight: {}", weight)),
                };
                task.with_weight(weight)
            }
        };

        let task = task.build();
        self.m_task_manager.add_task(task);
        return Ok(());
    }

    fn handle_edit(&mut self, index: usize) -> Result<()> {
        let mut input = String::new();
        let task = match self.m_task_manager.take_task(index) {
            Some(task) => task.edit(),
            None => return Err(anyhow!("index out of bounds")),
        };

        println!("-| Context: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            context => task.with_context(context.into()),
        };
        input.clear();

        println!("-| Description: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            description => task.with_description(description.into()),
        };
        input.clear();

        println!("-| Days to start: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            days_to_start => {
                let days_to_start = days_to_start.parse::<usize>()?;
                task.with_days_to_start(days_to_start)
            }
        };
        input.clear();

        println!("-| Days to end: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            days_to_end => {
                let days_to_end = days_to_end.parse::<usize>()?;
                task.with_days_to_end(days_to_end)
            }
        };
        input.clear();

        println!("-| Weight: ");
        print!("   |> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let task = match input.trim() {
            "" => task,
            weight => {
                let weight = match weight.to_lowercase().as_str() {
                    "high" => Weight::High,
                    "medium" => Weight::Med,
                    "low" => Weight::Low,
                    _ => return Err(anyhow!("unknown weight: {}", weight)),
                };
                task.with_weight(weight)
            }
        };

        let task = task.build();
        self.m_task_manager.add_task(task);
        return Ok(());
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.print_prompt();
            let input = App::read_input();
            match input {
                Ok(ManagerCommand::Create) => match self.handle_create() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("   Error: {}", e);
                    }
                },
                Ok(ManagerCommand::Remove(index)) => {
                    let result = self.m_task_manager.take_task(index);
                    if let None = result {
                        println!("   Error: index out of bounds");
                    }
                }
                Ok(ManagerCommand::Edit(index)) => match self.handle_edit(index) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("   Error: {}", e);
                    }
                },
                Ok(ManagerCommand::List) => {
                    if self.m_task_manager.is_empty() {
                        println!("   Error: no tasks to list")
                    } else {
                        println!("{}", self.m_task_manager);
                    }
                }
                Ok(ManagerCommand::Quit) => {
                    break;
                }
                Err(e) => {
                    println!("   Error: {}", e);
                }
            }
        }

        return Ok(());
    }
}
