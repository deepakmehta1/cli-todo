use clap::{Arg, Command};

pub struct Args {
    pub command: String,
    pub task_description: Option<String>,
    pub task_id: Option<usize>,
}

pub fn parse_args() -> Args {
    let matches = Command::new("Todo CLI")
        .version("0.1.0")
        .author("Your Name <youremail@example.com>")
        .about("A simple CLI to manage tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds a new task")
                .arg(Arg::new("task")
                     .help("The task description")
                     .required(true)),
        )
        .subcommand(Command::new("list").about("Lists all tasks"))
        .subcommand(
            Command::new("complete")
                .about("Marks a task as completed")
                .arg(Arg::new("task_id")
                     .help("The ID of the task to complete")
                     .required(true)),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes a task")
                .arg(Arg::new("task_id")
                     .help("The ID of the task to delete")
                     .required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => Args {
            command: "add".to_string(),
            task_description: sub_m.get_one::<String>("task").cloned(),
            task_id: None,
        },
        Some(("complete", sub_m)) => Args {
            command: "complete".to_string(),
            task_description: None,
            task_id: sub_m.get_one::<String>("task_id")
                .and_then(|id| id.parse::<usize>().ok()),
        },
        Some(("delete", sub_m)) => Args {
            command: "delete".to_string(),
            task_description: None,
            task_id: sub_m.get_one::<String>("task_id")
                .and_then(|id| id.parse::<usize>().ok()),
        },
        Some(("list", _)) => Args {
            command: "list".to_string(),
            task_description: None,
            task_id: None,
        },
        _ => panic!("Unknown command"),
    }
}
