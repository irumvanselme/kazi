use crate::repo::{ListError, Repo, SaveError};
use crate::task::{Task, TaskStage};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct MDRepository {
    tasks_directory: PathBuf,
}

#[derive(Debug)]
pub enum InitError {
    InvalidWorkingDirectory,
}

#[derive(Deserialize, Serialize, Debug)]
struct Header {
    pub id: String,
    pub title: String,
    pub stage: TaskStage,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}

fn get_task_slug(task: &Task) -> String {
    task.title
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

impl Task {
    fn get_header(&self) -> Header {
        return Header {
            id: self.id.clone(),
            title: self.title.clone(),
            stage: self.stage.clone(),
            creation_date: self.creation_date,
        };
    }

    fn get_relative_path(&self, tasks_directory: &PathBuf) -> PathBuf {
        let mut relative_path = tasks_directory.clone();
        relative_path.push(get_task_slug(&self));
        return relative_path;
    }
}

impl MDRepository {
    pub fn new(working_directory: &PathBuf) -> Result<Self, InitError> {
        if working_directory.is_file() {
            return Err(InitError::InvalidWorkingDirectory);
        }

        let mut tasks_directory = working_directory.clone();
        tasks_directory.push(".tasks");

        return Ok(Self {
            tasks_directory: tasks_directory,
        });
    }
}

impl Repo for MDRepository {
    fn list_tasks(&self) -> Result<Vec<Task>, ListError> {
        let mut tasks: Vec<Task> = Vec::new();
        for entry in
            fs::read_dir(&self.tasks_directory).map_err(|_| ListError::FailedToReadCollection)?
        {
            let task_path = entry.unwrap().path();
            let file_name = task_path.iter().last().unwrap();
            if file_name == "meta.yaml" {
                continue;
            }

            tasks.push(parse_task_md_file(&task_path));
        }
        return Ok(tasks);
    }

    fn save(&self, task: Task) -> Result<(), SaveError> {
        let header = task.get_header();
        let header_str = serde_yaml::to_string(&header).unwrap();
        let md_file_content = format!("---\n{}---\n{}", header_str, task.description);
        let target_file_path = format!(
            "{}.md",
            task.get_relative_path(&self.tasks_directory)
                .to_str()
                .unwrap()
        );
        fs::write(target_file_path, md_file_content).unwrap();
        Ok(())
    }
}

fn parse_frontmatter(content: &str) -> [String; 2] {
    let trimmed = content.trim_start();
    if let Some(rest) = trimmed.strip_prefix("---") {
        if let Some(end) = rest.find("\n---") {
            let yaml = rest[..end].trim().to_string();
            let body = rest[end + 4..].trim().to_string();
            return [yaml, body];
        }
    }
    [String::new(), content.to_string()]
}

fn parse_task_md_file(file_path: &PathBuf) -> Task {
    let [yaml, body] = parse_frontmatter(&fs::read_to_string(file_path).unwrap().to_string());
    let parsed_yaml: Header = serde_yaml::from_str(&yaml).unwrap();
    return Task {
        id: parsed_yaml.id,
        title: parsed_yaml.title,
        description: body,
        stage: parsed_yaml.stage,
        creation_date: parsed_yaml.creation_date,
    };
}

#[cfg(test)]
mod md_repo_tests {
    use std::{env, path::PathBuf};

    use crate::{md_repo::MDRepository, repo::Repo, task::Task, task::TaskStage};

    fn get_test_project_path() -> PathBuf {
        let mut test_project_path = env::current_dir().unwrap();
        test_project_path.push("tests");
        test_project_path.push("test-project");
        return test_project_path;
    }

    #[test]
    fn test_list_tasks() {
        let test_project_path = get_test_project_path();
        let repository = MDRepository::new(&test_project_path).unwrap();
        assert_eq!(repository.list_tasks().unwrap().len(), 1)
    }

    #[test]
    fn test_parse_task_md_file() {
        let mut task_one_path = get_test_project_path();
        task_one_path.push(".tasks");
        task_one_path.push("task-one.md");

        let task = super::parse_task_md_file(&task_one_path);
        let expected_task = Task {
            id: "1".to_string(),
            title: "Some cool title".to_string(),
            stage: TaskStage::Todo,
            description: "# some cool content\nsome cool content is here".to_string(),
            creation_date: "2026-03-31T09:13:55.702829Z".parse().unwrap(),
        };
        assert_eq!(task, expected_task);
    }

    macro_rules! test_slug {
        ($test_name:ident, $task_title:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let task = Task {
                    id: "1".to_string(),
                    title: $task_title.to_string(),
                    description: "1".to_string(),
                    stage: TaskStage::Done,
                    creation_date: "2026-03-31T09:13:55.702829Z".parse().unwrap(),
                };
                assert_eq!(super::get_task_slug(&task), $expected);
            }
        };
    }

    test_slug!(single_char, "1", "1");
    test_slug!(single_word, "Apple", "apple");
    test_slug!(two_words, "Two Worlds", "two-worlds");
    test_slug!(
        special_characters,
        "Special Characters !@#$%^&*()",
        "special-characters"
    );
}
