use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum TaskStage {
    Todo,
    InProgress,
    Done,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub stage: TaskStage,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}
