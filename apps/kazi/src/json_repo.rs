use std::path::PathBuf;
use std::{fs, io};

use crate::repo::{ListError, Repo, SaveError};
use crate::task::Task;

pub struct JSONRepository {
    tasks_json_path: PathBuf,
}

pub struct FailedToWriteTasksFile(pub io::Error);

impl JSONRepository {
    pub fn new(working_directory: PathBuf) -> Result<Self, FailedToWriteTasksFile> {
        let mut tasks_json_path = working_directory.clone();
        tasks_json_path.push(".tasks");
        tasks_json_path.push("tasks.json");

        if !tasks_json_path.exists() {
            match fs::write(&tasks_json_path, "[]") {
                Ok(_) => {}
                Err(io_err) => return Err(FailedToWriteTasksFile(io_err)),
            }

            println!("[INFO] Creating tasks.json file because it did not exist")
        }

        Ok(Self { tasks_json_path })
    }
}

impl Repo for JSONRepository {
    fn list_tasks(&self) -> Result<Vec<Task>, ListError> {
        let data = fs::read_to_string(&self.tasks_json_path)
            .map_err(|_| ListError::FailedToReadCollection)?;

        serde_json::from_str(&data).map_err(|_| ListError::InvalidCollectionData)
    }

    fn save(&self, task: Task) -> Result<(), SaveError> {
        let mut tasks = self.list_tasks().map_err(SaveError::ListError)?;

        tasks.push(task);
        let tasks_json_string =
            serde_json::to_string_pretty(&tasks).map_err(|_| SaveError::DeserializeError)?;

        fs::write(&self.tasks_json_path, tasks_json_string)
            .map_err(|_| SaveError::FailedToWriteToCollection)?;

        Ok(())
    }
}
