use serde::{Deserialize, Serialize};
use crate::storage::{save_tasks, load_tasks};
use crate::errors::TodoError;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Task { id, description, completed: false }
    }
}

pub fn add_task(description: &str) -> Result<(), TodoError> {
    let mut tasks = load_tasks()?;
    let task_id = tasks.len() + 1;
    tasks.push(Task::new(task_id, description.to_string()));
    save_tasks(&tasks)?;
    println!("Task added: {}", description);
    Ok(())
}

pub fn list_tasks() -> Result<(), TodoError> {
    let tasks = load_tasks()?;
    for task in tasks {
        println!("{} - {} [{}]", task.id, task.description, if task.completed { "x" } else { " " });
    }
    Ok(())
}

pub fn complete_task(task_id: usize) -> Result<(), TodoError> {
    let mut tasks = load_tasks()?;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.completed = true;
        save_tasks(&tasks)?;
        println!("Task {} marked as completed.", task_id);
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(task_id))
    }
}

pub fn delete_task(task_id: usize) -> Result<(), TodoError> {
    let mut tasks = load_tasks()?;
    if tasks.iter().position(|t| t.id == task_id).is_some() {
        tasks.retain(|t| t.id != task_id);
        save_tasks(&tasks)?;
        println!("Task {} deleted.", task_id);
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(task_id))
    }
}
