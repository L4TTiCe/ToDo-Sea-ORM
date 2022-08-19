use crate::lib::time::get_current_time;
use entity::task;
use entity::task::Model as Task;

use chrono::serde::{ts_milliseconds, ts_milliseconds_option};
use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OptionalTask {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub task_title: Option<String>,

    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub task_state: Option<bool>,

    #[serde(
        rename = "deadline",
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option",
        default = "task::default_dealine"
    )]
    pub task_deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTask {
    #[serde(rename = "_id", alias = "_id")]
    pub task_id: Option<Uuid>,

    #[serde(rename = "title")]
    pub task_title: String,

    #[serde(rename = "completed", default = "task::default_task_completed_state")]
    pub task_state: bool,

    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "ts_milliseconds"
    )]
    pub task_created_at: DateTime<Utc>,

    #[serde(
        rename = "deadline",
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option"
    )]
    pub task_deadline: Option<DateTime<Utc>>,
}

impl From<Task> for PublicTask {
    fn from(task: Task) -> Self {
        Self {
            task_id: Some(task.id),
            task_title: task.title.clone(),
            task_state: task.completed,
            task_created_at: task.created_at,
            task_deadline: task.deadline,
        }
    }
}
