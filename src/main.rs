mod tasks;
mod storage;
mod cli;
mod errors;

use cli::parse_args;

use tasks::{add_task, list_tasks, complete_task, delete_task};

fn main() {
    let args = parse_args();

    // Handle commands based on parsed arguments
    let result = match args.command.as_str() {
        "add" => add_task(args.task_description.as_deref().expect("Task description is required")),
        "list" => list_tasks(),
        "complete" => complete_task(args.task_id.expect("Task ID is required")),
        "delete" => delete_task(args.task_id.expect("Task ID is required")),
        _ => {
            eprintln!("Unknown command");
            Ok(())
        }
    };

    // Handle errors if any
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

