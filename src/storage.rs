use std::fs::{File, OpenOptions};
use std::io::BufReader;
use serde_json::from_reader;
use crate::tasks::Task;
use crate::errors::TodoError;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Result<Vec<Task>, TodoError> {
    let file = OpenOptions::new().read(true).open(FILE_PATH);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let tasks = from_reader(reader).unwrap_or_else(|_| Vec::new());
            Ok(tasks)
        },
        Err(_) => Ok(Vec::new()), // Return an empty list if the file does not exist
    }
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), TodoError> {
    let file = File::create(FILE_PATH)?;
    serde_json::to_writer(file, tasks)?;
    Ok(())
}
