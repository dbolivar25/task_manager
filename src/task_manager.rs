use std::{fmt::{Display}};
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weight {
    Low,
    Med,
    High,
}

impl Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:6}", 
            match self {
                Weight::High => "High",
                Weight::Med => "Med",
                Weight::Low => "Low",
        })?;
        
        return Ok(());
    }
}

#[derive(Debug, PartialEq)]
struct Task {
    m_context: String,
    m_description: String,
    m_days_to_start: u32,
    m_days_to_end: u32,
    m_days_to_finish: u32,
    m_weight: Weight,
    m_priority: f32,
}

impl Task {
    fn new(context: String, description: String, days_to_start: u32, days_to_end: u32, weight: Weight) -> Task {
        let days_to_finish = days_to_end - days_to_start;

        let priority = match weight {
            Weight::High => 3.0 / days_to_finish as f32,
            Weight::Med => 2.0 / days_to_finish as f32,
            Weight::Low => 1.0 / days_to_finish as f32,
        };

        return Task { 
                    m_context: context, 
                    m_description: description, 
                    m_days_to_start: days_to_start, 
                    m_days_to_end: days_to_end, 
                    m_days_to_finish: days_to_finish, 
                    m_weight: weight, 
                    m_priority: priority,
                };
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.m_priority.partial_cmp(&other.m_priority);
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "| {:10} | {:25} | {:6} | {:6} | {:6} | {:7} |", self.m_context, self.m_description, self.m_days_to_start, self.m_days_to_end, self.m_days_to_finish, self.m_weight)?;

        return Ok(());
    }
}

#[derive(Debug)]
pub struct TaskManager {
    m_tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        return TaskManager { 
            m_tasks: Vec::new() 
        } 
    }

    pub fn add_task(&mut self, context: String, description: String, days_to_start: u32, days_to_end: u32, weight: Weight) {
        self.m_tasks.push(Task::new(context, description, days_to_start, days_to_end, weight));
        self.m_tasks.sort_unstable_by(|a, b| {
            if a.m_priority.is_infinite() && b.m_priority.is_infinite() {
                return b.m_weight.cmp(&a.m_weight);
            }

            return b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal);
        });
    }

    pub fn remove_task(&mut self, index: u32) {
        self.m_tasks.remove(index as usize);
    }

    pub fn edit_task(&mut self, index: u32, context: String, description: String, days_to_start: u32, days_to_end: u32, weight: Weight) {
        self.remove_task(index);
        self.add_task(context, description, days_to_start, days_to_end, weight);
    }
}

impl Display for TaskManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "|     | Context    | Description               | Start  | End    | Finish | Weight |")?;
        
        for (num, task) in self.m_tasks.iter().enumerate() {
            write!(f, "| {:3} {}", num, task)?;
        }

        return Ok(());
    }
}
