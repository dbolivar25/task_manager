use std::io::Write;

use anyhow::{anyhow, Result};

use crate::task::*;
use crate::task_manager::*;

enum ManagerCommand {
    Create,
    Remove(usize),
    Edit(usize),
    List,
    Tick(usize),
    Help,
    Quit,
    NoOp,
}

pub struct App {
    m_task_manager: TaskManager,
    m_print_help: bool,
}

impl App {
    pub fn new() -> App {
        let file = match std::fs::read_to_string("task_manager.json") {
            Ok(file) => file,
            Err(_) => {
                return App {
                    m_task_manager: TaskManager::new(),
                    m_print_help: false,
                }
            }
        };

        let task_manager = match serde_json::from_str(&file) {
            Ok(task_manager) => task_manager,
            Err(err) => {
                dbg!(&err);
                println!("   Error: failed to load session from file");
                return App {
                    m_task_manager: TaskManager::new(),
                    m_print_help: false,
                };
            }
        };

        App {
            m_task_manager: task_manager,
            m_print_help: false,
        }
    }

    fn print_prompt(&self) {
        println!("\n|- --------------------- -|");
        println!("|-      TaskManager      -|");
        println!("|- --------------------- -|");
        println!("|- create                -|");
        println!("|- remove <idx>          -|");
        println!("|- edit   <idx>          -|");
        println!("|- tick   <days>         -|");
        println!("|- list                  -|");
        println!("|- help                  -|");
        println!("|- quit                  -|");
        println!("|- --------------------- -|\n");
        print!("|> ");
        std::io::stdout().flush().unwrap();
    }

    fn print_help_prompt(&mut self) {
        println!("\n|- ------------------------------------------------- -|");
        println!("|-                     TaskManager                   -|");
        println!("|- ------------------------------------------------- -|");
        println!("|- create        -> add a new task to the list       -|");
        println!("|- remove <idx>  -> remove the n'th task in the list -|");
        println!("|- edit   <idx>  -> edit the n'th task in the list   -|");
        println!("|- tick   <days> -> subtract n days from all tasks   -|");
        println!("|- list          -> list all tasks in the list       -|");
        println!("|- help          -> toggle command descriptions      -|");
        println!("|- quit          -> exit application                 -|");
        println!("|- ------------------------------------------------- -|\n");
        print!("|> ");
        std::io::stdout().flush().unwrap();
    }

    fn read_input() -> Result<ManagerCommand> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();
        let command = match iter.next() {
            Some(command) => command,
            None => return Ok(ManagerCommand::NoOp),
        };

        match command.to_lowercase().as_str() {
            "create" => {
                if iter.next().is_none() {
                    Ok(ManagerCommand::Create)
                } else {
                    Err(anyhow!("create command does not take arguments"))
                }
            }
            "remove" => {
                let index = match iter.next() {
                    Some(index) => index.parse::<usize>()?,
                    None => return Err(anyhow!("remove command takes one argument")),
                };

                if iter.next().is_some() {
                    return Err(anyhow!("remove command takes only one argument"));
                }

                Ok(ManagerCommand::Remove(index))
            }
            "edit" => {
                let index = match iter.next() {
                    Some(index) => index.parse::<usize>()?,
                    None => return Err(anyhow!("edit command takes one argument")),
                };

                if iter.next().is_some() {
                    return Err(anyhow!("edit command takes only one argument"));
                }

                Ok(ManagerCommand::Edit(index))
            }
            "list" => {
                if iter.next().is_some() {
                    return Err(anyhow!("list command does not take arguments"));
                }

                Ok(ManagerCommand::List)
            }
            "tick" => {
                let num_days = if let Some(num_days) = iter.next() {
                    num_days.parse::<usize>()?
                } else {
                    1
                };

                if iter.next().is_some() {
                    return Err(anyhow!("tick command takes at most one argument"));
                }

                Ok(ManagerCommand::Tick(num_days))
            }
            "help" => {
                if iter.next().is_some() {
                    return Err(anyhow!("help command does not take arguments"));
                }

                Ok(ManagerCommand::Help)
            }
            "quit" => {
                if iter.next().is_some() {
                    return Err(anyhow!("quit command does not take arguments"));
                }

                Ok(ManagerCommand::Quit)
            }
            command => Err(anyhow!("unknown command: {}", command)),
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
        Ok(())
    }

    fn handle_edit(&mut self, index: usize) -> Result<()> {
        let mut input = String::new();

        match self.m_task_manager.peek_task(index) {
            Some(task) => {
                println!("\n-| <> | Context    | Description               | Start  | End    | Finish | Weight |");
                println!("-| <> {}", task);
            }
            None => return Err(anyhow!("index out of bounds")),
        };

        let task = self.m_task_manager.take_task(index).unwrap().edit();

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
        Ok(())
    }

    fn handle_tick(&mut self, num_days: usize) -> Result<()> {
        let mut ticked_tasks = Vec::new();

        while let Some(task) = self.m_task_manager.take_task(0) {
            let days_to_start = task.get_days_to_start();
            let days_to_end = task.get_days_to_end();

            let ticked_task = task
                .edit()
                .with_days_to_start(days_to_start.saturating_sub(num_days))
                .with_days_to_end(days_to_end.saturating_sub(num_days))
                .build();
            ticked_tasks.push(ticked_task);
        }

        for task in ticked_tasks {
            self.m_task_manager.add_task(task);
        }

        Ok(())
    }

    fn handle_quit(&mut self) -> Result<()> {
        let mut file = std::fs::File::create("task_manager.json")?;
        let json = serde_json::to_string(&self.m_task_manager)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if self.m_print_help {
                self.print_help_prompt();
            } else {
                self.print_prompt();
            }

            let input = App::read_input();
            match input {
                Ok(ManagerCommand::Create) => match self.handle_create() {
                    Ok(_) => {}
                    Err(e) => {
                        println!("   Error: {}", e);
                    }
                },
                Ok(ManagerCommand::Remove(index)) => {
                    if self.m_task_manager.take_task(index).is_none() {
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
                        print!("{}", self.m_task_manager);
                    }
                }
                Ok(ManagerCommand::Tick(num_days)) => match self.handle_tick(num_days) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("   Error: {}", e);
                    }
                },
                Ok(ManagerCommand::Help) => {
                    self.m_print_help = !self.m_print_help;
                }
                Ok(ManagerCommand::Quit) => {
                    match self.handle_quit() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("   Error: {}", e);
                        }
                    }
                    break;
                }
                Ok(ManagerCommand::NoOp) => {}
                Err(e) => {
                    println!("   Error: {}", e);
                }
            }
        }

        Ok(())
    }
}
