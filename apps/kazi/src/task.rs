use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum TaskStage {
    Todo,
    InProgress,
    Done,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub stage: TaskStage,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}
