use regex::Regex;
use std::fmt::Display;
use std::sync::LazyLock;

pub struct TaskList {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
struct Task {
    name: String,
    completed: bool,
}

impl From<String> for TaskList {
    fn from(value: String) -> Self {
        Self {
            tasks: value
                .lines()
                .filter(|s| !s.trim().is_empty())
                .map(Task::from)
                .collect(),
        }
    }
}

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^- (\[x]|\[ ]) (.*)").unwrap());

impl From<&str> for Task {
    fn from(value: &str) -> Self {
        let captures = RE.captures(&value);
        let Some(cap) = captures else {
            panic!("Invalid task format");
        };

        Task {
            name: cap.get(2).unwrap().as_str().to_owned(),
            completed: cap.get(1).unwrap().as_str() == "[x]",
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let marker = if self.completed { "x" } else { " " };
        write!(f, "- [{}] {}", marker, self.name)
    }
}

impl Display for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for task in &self.tasks {
            writeln!(f, "{}", task)?;
        }
        Ok(())
    }
}

impl TaskList {
    pub fn active_task(&self) -> Option<String> {
        self.tasks
            .iter()
            .find(|t| !t.completed)
            .cloned()
            .map(|t| t.name)
    }

    pub fn advance(&mut self) {
        if let Some(t) = self.tasks.iter_mut().find(|t| !t.completed) {
            t.completed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_from_str() {
        let task = Task::from("- [x] Do something with dragons");
        assert_eq!(task.name, "Do something with dragons");
        assert!(task.completed);
    }
}
