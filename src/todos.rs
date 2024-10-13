use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Todo {
    pub name: String,
    pub description: Option<String>,
    pub date_added: DateTime<Local>,
    pub completed: bool,
    pub date_completed: Option<DateTime<Local>>
}

impl Todo {
    pub fn new(name: String, description: Option<String>, completed: bool) -> Self {
        let current = Local::now();
        Self {
            name,
            description,
            date_added: current,
            completed,
            date_completed: None
        }
    }
}

