use crate::task::Task;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ListError {
    #[error("Failed to read the collection (data store)")]
    FailedToReadCollection,

    #[error("Failed to parse data in the collection. App unable to format it.")]
    InvalidCollectionData,
}

#[derive(Error, Debug)]
pub enum SaveError {
    #[error("Failed to save because save can not read all the tasks")]
    ListError(ListError),

    #[error("Failed to deserialize the input task")]
    DeserializeError,

    #[error("Failed to write the inputs into the data store")]
    FailedToWriteToCollection,
}

pub trait Repo {
    fn list_tasks(&self) -> Result<Vec<Task>, ListError>;
    fn save(&self, task: Task) -> Result<(), SaveError>;
}
