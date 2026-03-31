use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

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

#[derive(Deserialize, Serialize, Debug)]
struct ProjectConfig {
    title: String,
    tasks_id_prefix: String,
}

struct Project {
    tasks_json_path: PathBuf,
    config: ProjectConfig,
}

struct CreateTaskInput {
    title: String,
}

#[derive(Parser)]
#[command(
    version,
    about = "Kazi, the version manager for tasks",
    long_about = "Git for tasks and issues"
)]
struct KaziCLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize a new project
    Init,
    /// List tasks
    List,
    /// Add a new task
    Add {
        /// Title of the tasks to add
        title: String,
    },
}

fn list_tasks(project: &Project) -> Vec<Task> {
    let data = fs::read_to_string(&project.tasks_json_path).unwrap_or("[]".to_string());
    let tasks: Vec<Task> =
        serde_json::from_str(&data).expect("Failed to parse the test tasks json");
    return tasks;
}

fn get_project(working_directory: PathBuf) -> Project {
    let mut meta_yaml_file = working_directory.clone();
    meta_yaml_file.push(".tasks");
    meta_yaml_file.push("meta.yaml");
    let meta_yaml_str = fs::read_to_string(meta_yaml_file).unwrap();
    let project_config = serde_yaml::from_str(&meta_yaml_str).unwrap();

    let mut tasks_json = working_directory.clone();
    tasks_json.push(".tasks");
    tasks_json.push("tasks.json");

    return Project {
        tasks_json_path: tasks_json,
        config: project_config,
    };
}

fn add_tasks(project: &Project, input: CreateTaskInput) {
    let project_config = &project.config;
    let tasks_json_path = &project.tasks_json_path;

    let mut tasks = list_tasks(project);
    let task_id = format!(
        "{}-{}",
        project_config.tasks_id_prefix,
        (tasks.len() + 1).to_string()
    );
    let task: Task = Task {
        title: input.title,
        id: task_id,
        creation_date: chrono::Utc::now(),
        description: "".to_string(),
        stage: TaskStage::Todo,
    };

    tasks.push(task);
    let tasks_json_string =
        serde_json::to_string_pretty(&tasks).expect("Failed to parse the tasks JSON file");
    fs::write(tasks_json_path, tasks_json_string).expect("Failed to save the JSON file")
}

fn init_project(working_directory: PathBuf) {
    // 1. check if the folder .tasks exists, if not create it.
    let mut dot_tasks_folder = working_directory.clone();
    dot_tasks_folder.push(".tasks");
    if !dot_tasks_folder.exists() {
        fs::create_dir(dot_tasks_folder).expect("Failed to create the .tasks folder")
    }

    // 2. Check if file .tasks/meta.yaml exists,
    let mut meta_yaml_file = working_directory.clone();
    meta_yaml_file.push(".tasks");
    meta_yaml_file.push("meta.yaml");

    // 2.1. If the file exists print File already exists and return.
    if meta_yaml_file.exists() {
        println!("[WARN] Project is already initialized.");
        return;
    } else {
        // 2.2 If the file does not exist create it with the title and the task id prefix.
        let project_title = working_directory
            .iter()
            .last()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let project_config = ProjectConfig {
            title: project_title.clone(),
            tasks_id_prefix: project_title,
        };

        let project_config_yaml_str =
            serde_yaml::to_string(&project_config).expect("Failed to parse the project config");
        fs::write(meta_yaml_file, project_config_yaml_str).expect("Failed to save in a file")
    }
}

fn main() {
    let cli = KaziCLI::parse();
    let cwd = env::current_dir().unwrap();

    if matches!(cli.command, Command::Init) {
        init_project(cwd);
        return;
    }

    let project = get_project(cwd);
    match cli.command {
        Command::Init => unreachable!(),
        Command::Add { title } => {
            add_tasks(&project, CreateTaskInput { title });
        }
        Command::List => {
            let tasks = list_tasks(&project);
            let pretty_list =
                serde_json::to_string_pretty(&tasks).expect("Failed to parse the tasks JSON list");
            println!("{}", pretty_list);
        }
    }
}
