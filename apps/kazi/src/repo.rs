use crate::task::Task;

pub enum ListError {
    FailedToReadCollection,
    InvalidCollectionData,
}

pub enum SaveError {
    ListError(ListError),
    DeserizlizeError,
    FailedToWriteToCollection,
}

pub trait Repo {
    fn list_tasks(&self) -> Result<Vec<Task>, ListError>;
    fn save(&self, task: Task) -> Result<(), SaveError>;
}
