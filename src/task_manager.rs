use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::task::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskManager {
    m_tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            m_tasks: Vec::new(),
        }
    }

    fn sort_tasks(&mut self) {
        self.m_tasks.sort_unstable_by(|a, b| {
            if a.get_priority().is_infinite() && b.get_priority().is_infinite() {
                return b.get_weight().cmp(&a.get_weight());
            }

            b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn add_task(&mut self, task: Task) {
        self.m_tasks.push(task);
        self.sort_tasks();
    }

    pub fn add_tasks(&mut self, tasks: Vec<Task>) {
        self.m_tasks.extend(tasks);
        self.sort_tasks();
    }

    pub fn peek_task(&self, index: usize) -> Option<&Task> {
        return self.m_tasks.get(index);
    }

    pub fn take_task(&mut self, index: usize) -> Option<Task> {
        if index >= self.m_tasks.len() {
            None
        } else {
            Some(self.m_tasks.remove(index))
        }
    }

    pub fn map<F>(&mut self, f: F)
    where
        F: FnMut(Task) -> Task,
    {
        self.m_tasks = self.m_tasks.drain(..).map(f).collect();
        self.sort_tasks();
    }

    pub fn is_empty(&self) -> bool {
        self.m_tasks.is_empty()
    }
}

impl Display for TaskManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n| Idx | Context    | Description               | Start  | End    | Finish | Weight |"
        )?;

        for (num, task) in self.m_tasks.iter().enumerate() {
            write!(f, "| {:3} {}", num, task)?;
        }

        Ok(())
    }
}
