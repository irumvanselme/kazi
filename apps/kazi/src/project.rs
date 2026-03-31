use crate::repo::{Repo, SaveError};
use crate::task::{Task, TaskStage};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectConfig {
    pub title: String,
    pub tasks_id_prefix: String,
}

pub struct Project {
    pub config: ProjectConfig,
    pub repo: Box<dyn Repo>,
}

pub enum LoadError {
    ProjectNotInitialized,
    InvalidMetaYamlFile,
    UnexpectedCase(io::Error),
}

impl Project {
    pub fn load(repo: Box<dyn Repo>, working_directory: PathBuf) -> Result<Self, LoadError> {
        let mut meta_yaml_file = working_directory.clone();
        meta_yaml_file.push(".tasks");
        meta_yaml_file.push("meta.yaml");

        if !meta_yaml_file.exists() {
            return Err(LoadError::ProjectNotInitialized);
        }

        let meta_yaml_str =
            fs::read_to_string(meta_yaml_file).map_err(LoadError::UnexpectedCase)?;
        let project_config =
            serde_yaml::from_str(&meta_yaml_str).map_err(|_| LoadError::InvalidMetaYamlFile)?;

        return Ok(Project {
            repo,
            config: project_config,
        });
    }

    pub fn list_tasks(&self) -> Vec<Task> {
        // TODO: Handle if the listing of tasks fails.
        return self.repo.list_tasks().unwrap_or(Vec::new());
    }

    pub fn add_task(&self, title: String) -> Result<(), SaveError> {
        let tasks = self.list_tasks();
        let project_config = &self.config;
        let task_id = format!(
            "{}-{}",
            project_config.tasks_id_prefix,
            (tasks.len() + 1).to_string()
        );

        let task: Task = Task {
            title: title,
            id: task_id,
            creation_date: chrono::Utc::now(),
            description: "".to_string(),
            stage: TaskStage::Todo,
        };

        self.repo.save(task)
    }
}

pub enum InitProjectError {
    FailedToCreateDotTasksFolder(io::Error),
    FailedToSaveMetaYamlFile(io::Error),
}

pub fn init_project(working_directory: PathBuf) -> Result<(), InitProjectError> {
    // 1. check if the folder .tasks exists, if not create it.
    let mut dot_tasks_folder = working_directory.clone();
    dot_tasks_folder.push(".tasks");
    if !dot_tasks_folder.exists() {
        match fs::create_dir(dot_tasks_folder) {
            Err(io_error) => return Err(InitProjectError::FailedToCreateDotTasksFolder(io_error)),
            Ok(_) => {
                println!("[INFO] Created the .tasks folder");
            }
        }
    } else {
        println!("[DEBUG] .tasks folder already exists");
    }

    // 2. Check if file .tasks/meta.yaml exists,
    let mut meta_yaml_file = working_directory.clone();
    meta_yaml_file.push(".tasks");
    meta_yaml_file.push("meta.yaml");

    // 2.1. If the file exists print File already exists and return.
    let meta_file_exists = meta_yaml_file.exists();
    if meta_file_exists {
        println!("[WARN] Project is already initialized.");
        Ok(())
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
        return match fs::write(meta_yaml_file, project_config_yaml_str) {
            Ok(_) => Ok(()),
            Err(io_error) => Err(InitProjectError::FailedToSaveMetaYamlFile(io_error)),
        };
    }
}
