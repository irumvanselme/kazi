use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
enum TaskStage {
    Todo,
    InProgress,
    Done,
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    id: String,
    title: String,
    description: String,
    stage: TaskStage,
    creation_date: chrono::DateTime<chrono::Utc>,
}

struct CreateTaskInput {
    title: String,
}

const TEST_TASKS_JSON_PATH: &'static str = "/Users/anselme/Developer/tech/kazi/tests/tasks.json";

#[derive(Parser)]
#[command(version, about, long_about = "Git for tasks and issues")]
struct KaziCLI {
    name: Option<String>,
}

fn list_tasks() -> Vec<Task> {
    let data = fs::read_to_string(TEST_TASKS_JSON_PATH).expect("TEST TASKS JSON NOT FOUND");
    let tasks: Vec<Task> =
        serde_json::from_str(&data).expect("Failed to parse the test tasks json");
    return tasks;
}

fn add_tasks(input: CreateTaskInput) {
    let mut tasks = list_tasks();
    let task: Task = Task {
        title: input.title,
        id: "1".to_string(),
        creation_date: chrono::Utc::now(),
        description: "".to_string(),
        stage: TaskStage::Todo,
    };

    tasks.push(task);
    let tasks_json_string =
        serde_json::to_string_pretty(&tasks).expect("Failed to parse the tasks JSON file");
    fs::write(TEST_TASKS_JSON_PATH, tasks_json_string).expect("Failed to save the JSON file")
}

fn main() {
    let cli = KaziCLI::parse();
    let name = cli.name.unwrap_or("World".to_string());
    println!("Hello {}!", name);

    add_tasks(CreateTaskInput {
        title: "Some cool task".to_string(),
    });

    let tasks = list_tasks();
    println!("{:?}", tasks);
}
