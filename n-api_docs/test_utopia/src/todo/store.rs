use super::model::Todo;
use tokio::sync::Mutex;

/// In-memory todo store
pub type Store = Mutex<Vec<Todo>>;
