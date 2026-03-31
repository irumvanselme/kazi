mod json_repo;
mod md_repo;
mod project;
mod repo;
mod task;
mod tasks_table;

use clap::{Parser, Subcommand, ValueEnum};
use project::{Project, init_project};
use std::env;

use crate::{json_repo::JSONRepository, md_repo::MDRepository, repo::Repo};

#[derive(Debug, Clone, ValueEnum)]
enum RepoType {
    JSON,
    MD,
}

#[derive(Parser)]
#[command(
    version,
    about = "Kazi, the version manager for tasks",
    long_about = "Git for tasks and issues"
)]
struct KaziCLI {
    #[arg(short, long, value_enum, default_value_t = RepoType::MD)]
    storage_type: RepoType,

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

fn main() {
    let cli = KaziCLI::parse();
    let cwd = env::current_dir().unwrap();

    if matches!(cli.command, Command::Init) {
        return match init_project(cwd) {
            Ok(_) => {
                println!("[INFO] Project initiailized successfully")
            }
            Err(err) => {
                println!("[ERROR] Failed to init the project {:?}", err);
            }
        };
    }

    let repository: Box<dyn Repo> = match cli.storage_type {
        RepoType::MD => Box::new(MDRepository::new(&cwd).expect("TODO: Fix me")),
        RepoType::JSON => Box::new(match JSONRepository::new(cwd.clone()) {
            Ok(repo) => repo,
            Err(err) => {
                println!(
                    "[ERROR] Failed to write to tasks.json file, reason = {}. Run init command first",
                    err.0.to_string()
                );

                return;
            }
        }),
    };

    let project = match Project::load(repository, cwd) {
        Ok(project) => project,
        Err(err) => {
            println!("[ERROR] Failed to load project {:?}", err);
            return;
        }
    };

    match cli.command {
        Command::Init => unreachable!(),
        Command::Add { title } => {
            match project.add_task(title) {
                Ok(_) => {
                    println!("[INFO] Task added successfully")
                }
                Err(err) => {
                    println!("[ERROR] Failed to add a new command, reason={:?}", err);
                }
            };
        }
        Command::List => {
            let tasks = project.list_tasks();
            let pretty_list =
                serde_json::to_string_pretty(&tasks).expect("Failed to parse the tasks JSON list");
            println!("{}", pretty_list);
        }
    }
}
