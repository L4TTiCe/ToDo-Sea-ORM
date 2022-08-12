use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Serialize, Deserialize};
use chrono::{self, DateTime, Utc};
use chrono::serde::{ts_milliseconds, ts_milliseconds_option};

fn default_task_completed_state() -> bool {
    false
}

fn get_current_timestamp() -> DateTime<Utc> {
    chrono::Utc::now()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<ObjectId>,

    #[serde(rename = "title")]
    pub task_title: String,

    #[serde(rename = "completed", default = "default_task_completed_state")]
    pub task_state: bool,

    #[serde(rename = "created_at", default = "get_current_timestamp", with = "ts_milliseconds")]
    pub task_created_at: DateTime<Utc>,

    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none", with = "ts_milliseconds_option")]
    pub task_deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTask {
    #[serde(alias = "_id", serialize_with = "serialize_object_id_as_hex_string")]
    pub task_id: ObjectId,

    #[serde(rename = "title")]
    pub task_title: String,

    #[serde(rename = "completed", default = "default_task_completed_state")]
    pub task_state: bool,

    #[serde(rename = "created_at", default = "get_current_timestamp", with = "ts_milliseconds")]
    pub task_created_at: DateTime<Utc>,
    
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none", with = "ts_milliseconds_option")]
    pub task_deadline: Option<DateTime<Utc>>,
}

impl From<Task> for PublicTask {
    fn from(task: Task) -> Self {
        Self {
            task_id: task.task_id.unwrap(),
            task_title: task.task_title.clone(),
            task_state: task.task_state.clone(),
            task_created_at: task.task_created_at.clone(),
            task_deadline: task.task_deadline.clone(),
        }
    }
}
