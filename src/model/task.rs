use crate::lib::time::get_current_time;

use chrono::serde::{ts_milliseconds, ts_milliseconds_option};
use chrono::{self, DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

fn default_task_completed_state() -> bool {
    false
}

fn default_dealine() -> Option<DateTime<Utc>> {
    Option::None
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<ObjectId>,

    #[serde(rename = "title")]
    pub task_title: String,

    #[serde(rename = "completed", default = "default_task_completed_state")]
    pub task_state: bool,

    #[serde(
        rename = "created_at",
        default = "get_current_time",
        with = "ts_milliseconds"
    )]
    pub task_created_at: DateTime<Utc>,

    #[serde(
        rename = "deadline",
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option",
        default = "default_dealine"
    )]
    pub task_deadline: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionalTask {
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub task_title: Option<String>,

    #[serde(rename = "completed", skip_serializing_if = "Option::is_none")]
    pub task_state: Option<bool>,

    #[serde(
        rename = "deadline",
        skip_serializing_if = "Option::is_none",
        with = "ts_milliseconds_option",
        default = "default_dealine"
    )]
    pub task_deadline: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(task_title: String, task_state: bool, task_deadline: Option<DateTime<Utc>>) -> Self {
        Task {
            task_id: None,
            task_title,
            task_state,
            task_created_at: get_current_time(),
            task_deadline,
        }
    }
}

#[derive(Serialize)]
pub struct InsertOneResponse {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    inserted_id: ObjectId,
}

impl From<mongodb::results::InsertOneResult> for InsertOneResponse {
    fn from(result: mongodb::results::InsertOneResult) -> Self {
        let inserted_id = result.inserted_id.as_object_id();

        match inserted_id {
            Some(id) => InsertOneResponse { inserted_id: id },
            None => {
                error!("InsertOneResult did not return an ObjectId");
                panic!("InsertOneResult did not return an ObjectId");
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicTask {
    #[serde(
        rename = "_id",
        alias = "_id",
        serialize_with = "serialize_object_id_as_hex_string"
    )]
    pub task_id: ObjectId,

    #[serde(rename = "title")]
    pub task_title: String,

    #[serde(rename = "completed", default = "default_task_completed_state")]
    pub task_state: bool,

    #[serde(
        rename = "created_at",
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
            task_id: task.task_id.unwrap(),
            task_title: task.task_title.clone(),
            task_state: task.task_state,
            task_created_at: task.task_created_at,
            task_deadline: task.task_deadline,
        }
    }
}
