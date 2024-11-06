use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Serialize, EnumString, Display, Eq, PartialEq)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed,
}

#[derive(Serialize)]
pub struct Task {
    pub user_uuid: String,
    pub task_uuid: String,
    pub task_type: String,
    pub state: TaskState,
    pub source_file: String,
    pub result_file: Option<String>,
}

impl Task {
    pub fn new(user_uuid: String, task_type: String, source_file: String) -> Task {
        Task {
            user_uuid,
            task_uuid: Uuid::new_v4().to_string(),
            task_type,
            state: TaskState::NotStarted,
            source_file,
            result_file: None,
        }
    }

    pub fn get_global_id(&self) -> String {
        return format!("{}_{}", self.user_uuid, self.task_uuid);
    }

    pub fn can_transition_to(&self, state: &TaskState) -> bool {
        self.state != *state
    }
}


// resdata data models

#[derive(Serialize, Deserialize)]
pub struct Dataset<K, V> {
    pub dataset_uuid: String,
    pub name: String,
    pub data_type: String,
    pub author: Vec<K, V>,
    pub data_headings: Vec<String>
}

impl Dataset {
    pub fn new(name: String, data_type: String) -> Task {
        Task {
            dataset_uuid: Uuid::new_v4().to_string(),
            name: name, 
            data_type: 
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct 
