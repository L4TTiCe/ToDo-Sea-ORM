use crate::errors::Error;
use crate::lib::object_id::parse_object_id_from_str;
use crate::{lib::mongodb::FilterOps, model::task::Task};

use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    results, Collection, Database,
};
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
        debug!("todo: create({:?})", new_task);

        let new_doc = Task::new(
            new_task.task_title.clone(),
            new_task.task_state,
            new_task.task_deadline,
        );

        let task = self.collection.insert_one(new_doc, None).await;

        info!("Created new task: {:?}", task);

        match task {
            Ok(result) => Ok(result),
            Err(err) => Err(Error::Mongo(err)),
        }
    }

    pub async fn find_id(&self, id: String) -> Result<Task, Error> {
        debug!("to_do: find_id({})", id);
        let object_id = parse_object_id_from_str(id.as_str())?;

        let filter = mongodb::bson::doc! {"_id": object_id};
        let task = self.collection.find_one(filter, None).await;

        match task {
            Ok(task) => match task {
                Some(task) => Ok(task),
                None => Err(Error::NotFound(id)),
            },
            Err(err) => Err(Error::Mongo(err)),
        }
    }

    pub async fn find_all(&self, sort_attrib: String, sort_order: i32) -> Result<Vec<Task>, Error> {
        debug!(
            "to_do: find_all(sort_attrib: {}, sort_order: {})",
            sort_attrib, sort_order
        );

        // Using Aggregate
        // Reference: https://www.mongodb.com/developer/languages/rust/rust-quickstart-aggregation/

        // let sort_stage = doc! {
        //     "$sort": {
        //         sort_attrib.as_str() : sort_order
        //     }
        // };

        // let pipeline = vec![sort_stage];

        // let cursor = self
        //     .collection
        //     .aggregate(pipeline, None)
        //     .await;

        // match cursor  {
        //     Ok(mut cursor) => {
        //         let mut tasks = Vec::new();
        //         loop {
        //             let result = cursor.try_next().await;

        //             match result {
        //                 Ok(Some(doc)) => {
        //                     let task: Task = mongodb::bson::from_document(doc)?;
        //                     tasks.push(task)
        //                 },
        //                 Ok(None) => break,
        //                 Err(err) => return Err(Error::MongoError(err)),
        //             }
        //         }

        //         Ok(tasks)
        //     }

        //     Err(e) => Err(Error::MongoError(e)),
        // }

        let sort_options = doc! {sort_attrib.as_str() : sort_order};
        let find_options = FindOptions::builder().sort(sort_options).build();

        let cursor = self.collection.find(None, find_options).await;

        match cursor {
            Ok(mut cursor) => {
                let mut tasks = Vec::new();
                loop {
                    let result = cursor.try_next().await;

                    match result {
                        Ok(Some(task)) => tasks.push(task),
                        Ok(None) => break,
                        Err(err) => return Err(Error::Mongo(err)),
                    }
                }

                Ok(tasks)
            }

            Err(e) => Err(Error::Mongo(e)),
        }
    }

    pub async fn find_with_params(
        &self,
        attrib: String,
        verb: FilterOps,
        date: DateTime<Utc>,
        sort_order: i32,
    ) -> Result<Vec<Task>, Error> {
        debug!(
            "to_do: find_with_params(attrib: {}, verb: {}, date: {}, sort_order: {})",
            attrib, verb, date, sort_order
        );

        let filter: Document =
            doc! {attrib.as_str() : { verb.to_string().as_str(): date.timestamp_millis() }};

        let sort_options = doc! {attrib.as_str() : sort_order};
        let find_options = FindOptions::builder().sort(sort_options).build();

        let cursor = self.collection.find(filter, find_options).await;

        match cursor {
            Ok(mut cursor) => {
                let mut tasks = Vec::new();
                loop {
                    let result = cursor.try_next().await;

                    match result {
                        Ok(Some(task)) => tasks.push(task),
                        Ok(None) => break,
                        Err(err) => return Err(Error::Mongo(err)),
                    }
                }

                Ok(tasks)
            }

            Err(e) => Err(Error::Mongo(e)),
        }
    }

    pub async fn find_between(
        &self,
        attrib: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        sort_order: i32,
    ) -> Result<Vec<Task>, Error> {
        debug!(
            "to_do: find_between(attrib: {}, start: {}, end: {}, sort_order: {})",
            attrib, start, end, sort_order
        );

        let filter: Document = doc! {
            attrib.as_str() :
            {
                FilterOps::Gte.to_string().as_str() : start.timestamp_millis(),
                FilterOps::Lte.to_string().as_str(): end.timestamp_millis()
            }
        };
        let sort_options = doc! {attrib.as_str() : sort_order};
        let find_options = FindOptions::builder().sort(sort_options).build();

        let cursor = self.collection.find(filter, find_options).await;

        match cursor {
            Ok(mut cursor) => {
                let mut tasks = Vec::new();
                loop {
                    let result = cursor.try_next().await;

                    match result {
                        Ok(Some(task)) => tasks.push(task),
                        Ok(None) => break,
                        Err(err) => return Err(Error::Mongo(err)),
                    }
                }

                Ok(tasks)
            }

            Err(e) => Err(Error::Mongo(e)),
        }
    }

    pub async fn update_task(
        &self,
        id: String,
        new_task: Task,
    ) -> Result<results::UpdateResult, Error> {
        debug!("to_do: update_task({}, {:?})", id, new_task);

        let object_id = parse_object_id_from_str(id.as_str())?;

        let new_deadline = new_task.task_deadline;
        let filter = mongodb::bson::doc! {"_id": object_id};

        let new_doc = match new_deadline {
            Some(deadline) => {
                doc! {
                    "$set":
                        {
                            "completed": new_task.task_state,
                            "deadline": deadline.timestamp_millis(),
                        },
                }
            }
            None => {
                doc! {
                    "$set":
                        {
                            "completed": new_task.task_state,
                        },
                }
            }
        };

        let updated_doc = self.collection.update_one(filter, new_doc, None).await;

        match updated_doc {
            Ok(status) => Ok(status),
            Err(err) => Err(Error::Mongo(err)),
        }
    }

    pub async fn delete_id(&self, id: String) -> Result<results::DeleteResult, Error> {
        debug!("to_do: delete_id({})", id);

        let object_id = parse_object_id_from_str(id.as_str())?;

        let filter = mongodb::bson::doc! {"_id": object_id};
        let task = self.collection.delete_one(filter, None).await;

        match task {
            Ok(task) => Ok(task),
            Err(err) => Err(Error::Mongo(err)),
        }
    }
}
