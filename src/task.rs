use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Copy)]
pub enum Weight {
    Low,
    Med,
    High,
}

impl Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:6}",
            match self {
                Weight::High => "High",
                Weight::Med => "Med",
                Weight::Low => "Low",
            }
        )?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaskBuilder {
    m_context: String,
    m_description: String,
    m_days_to_start: usize,
    m_days_to_end: usize,
    m_weight: Weight,
}

impl TaskBuilder {
    pub fn new() -> TaskBuilder {
        TaskBuilder {
            m_context: String::new(),
            m_description: String::new(),
            m_days_to_start: 0,
            m_days_to_end: 0,
            m_weight: Weight::Med,
        }
    }

    pub fn with_context(mut self, context: String) -> TaskBuilder {
        self.m_context = context;
        self
    }

    pub fn with_description(mut self, description: String) -> TaskBuilder {
        self.m_description = description;
        self
    }

    pub fn with_days_to_start(mut self, days_to_start: usize) -> TaskBuilder {
        self.m_days_to_start = days_to_start;
        self
    }

    pub fn with_days_to_end(mut self, days_to_end: usize) -> TaskBuilder {
        self.m_days_to_end = days_to_end;
        self
    }

    pub fn with_weight(mut self, weight: Weight) -> TaskBuilder {
        self.m_weight = weight;
        self
    }

    pub fn build(self) -> Task {
        let days_to_finish = self.m_days_to_end.saturating_sub(self.m_days_to_start);

        let priority = match days_to_finish {
            0 => std::f32::MAX,
            days_to_finish => {
                (match self.m_weight {
                    Weight::High => 3.0,
                    Weight::Med => 2.0,
                    Weight::Low => 1.0,
                }) / days_to_finish as f32
            }
        };

        Task {
            m_context: self.m_context,
            m_description: self.m_description,
            m_days_to_start: self.m_days_to_start,
            m_days_to_end: self.m_days_to_end,
            m_days_to_finish: self.m_days_to_end.saturating_sub(self.m_days_to_start),
            m_weight: self.m_weight,
            m_priority: priority,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Task {
    m_context: String,
    m_description: String,
    m_days_to_start: usize,
    m_days_to_end: usize,
    m_days_to_finish: usize,
    m_weight: Weight,
    m_priority: f32,
}

impl Task {
    pub fn get_context(&self) -> &str {
        &self.m_context
    }

    pub fn get_description(&self) -> &str {
        &self.m_description
    }

    pub fn get_days_to_start(&self) -> usize {
        self.m_days_to_start
    }

    pub fn get_days_to_end(&self) -> usize {
        self.m_days_to_end
    }

    pub fn get_days_to_finish(&self) -> usize {
        self.m_days_to_finish
    }

    pub fn get_weight(&self) -> Weight {
        self.m_weight
    }

    pub fn get_priority(&self) -> f32 {
        self.m_priority
    }

    pub fn edit(self) -> TaskBuilder {
        TaskBuilder {
            m_context: self.m_context,
            m_description: self.m_description,
            m_days_to_start: self.m_days_to_start,
            m_days_to_end: self.m_days_to_end,
            m_weight: self.m_weight,
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.m_priority.partial_cmp(&other.m_priority)
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "| {:10} | {:25} | {:6} | {:6} | {:6} | {:7} |",
            self.m_context,
            self.m_description,
            self.m_days_to_start,
            self.m_days_to_end,
            self.m_days_to_finish,
            self.m_weight,
        )?;

        Ok(())
    }
}
