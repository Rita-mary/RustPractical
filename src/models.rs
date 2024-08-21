use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
}


pub struct UpdateTask {
    pub description: Option<String>,
    pub completed: Option<bool>,
}
