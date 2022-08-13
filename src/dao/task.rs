
use crate::model::task::Task;
use crate::errors::Error;
use crate::lib::object_id::parse_object_id_from_str;

use mongodb::{Collection, Database, results, bson::doc};
// This trait is required to use `try_next()` on the cursor
use futures::stream::TryStreamExt;

pub struct TaskCollection {
    pub collection: Collection<Task>,
}

impl TaskCollection {
    pub fn init(db: Database, collection_name: String) -> Self {
        TaskCollection {
            collection: db.collection(collection_name.as_str()),
        }
    }

    pub async fn create(&self, new_task: Task) -> Result<results::InsertOneResult, Error> {
        let new_doc = Task::new(new_task.task_title.clone(), new_task.task_state.clone(), new_task.task_deadline.clone());
        
        let task = self
            .collection
            .insert_one(new_doc, None)
            .await;

        info!("Created new task: {:?}", task);

        match task {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::MongoError(err)),
        }
            
    }

    pub async fn find_id(&self, id: String) -> Result<Task, Error> {
        let object_id = parse_object_id_from_str(id.as_str())?;
        
        let filter = mongodb::bson::doc! {"_id": object_id};
        let task = self
            .collection
            .find_one(filter, None)
            .await;

        match task {
            Ok(task) => {
                match task {
                    Some(task) => Ok(task),
                    None => Err(Error::NotFound(id)),
                }
            }
            Err(err) => Err(Error::MongoError(err)),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<Task>, Error> {
        let cursor = self
            .collection
            .find(None, None)
            .await;

        match cursor  {
            Ok(mut cursor) => {
                let mut tasks = Vec::new();
                loop {
                    let result = cursor.try_next().await;

                    match result {
                        Ok(Some(task)) => tasks.push(task),
                        Ok(None) => break,
                        Err(err) => return Err(Error::MongoError(err)),
                    }
                }

                Ok(tasks)
            }

            Err(e) => Err(Error::MongoError(e)),
        }
    }

    pub async fn update_task(&self, id: String, new_task: Task) -> Result<results::UpdateResult, Error> {
        let object_id = parse_object_id_from_str(id.as_str())?;

        let new_deadline = new_task.task_deadline.clone();
        let filter = mongodb::bson::doc! {"_id": object_id};

        let new_doc;

        match new_deadline {
            Some(deadline) => {
                new_doc = doc! {
                    "$set":
                        {
                            "completed": new_task.task_state,
                            "deadline": deadline.timestamp_millis(),
                        },
                };
            }
            None => {
                new_doc = doc! {
                    "$set":
                        {
                            "completed": new_task.task_state,
                        },
                };
            }
        }

        let updated_doc = self
            .collection
            .update_one(filter, new_doc, None)
            .await;

        match updated_doc {
            Ok(status) => Ok(status),
            Err(err) => Err(Error::MongoError(err)),
        }
    }

    pub async fn delete_id(&self, id: String) -> Result<results::DeleteResult, Error> {
        let object_id = parse_object_id_from_str(id.as_str())?;

        let filter = mongodb::bson::doc! {"_id": object_id};
        let task = self
            .collection
            .delete_one(filter, None)
            .await;

        match task {
            Ok(task) => Ok(task),
            Err(err) => Err(Error::MongoError(err)),
        }
    }
}
