use crate::lib::errors::Error;
use entity::task::Model as Task;
use entity::task::Entity as TaskEntity;
use entity::task::ActiveModel as TaskActiveModel;

use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;

pub struct TaskDao {
    pub db_connection: DatabaseConnection,
}

impl TaskDao {
    pub fn init(db_connection: DatabaseConnection) -> Self {
        TaskDao {
            db_connection,
        }
    }

    pub async fn create(&self, new_task: Task) -> Result<Task, Error> {
        debug!("todo: create({:?})", new_task);

        let active_model: TaskActiveModel = new_task.into();
        let task = active_model.insert(&self.db_connection).await?;
        
        Ok(task)
    }

    pub async fn find_id(&self, id: Uuid) -> Result<Task, Error> {
        debug!("todo: find_id({:?})", id);

        let task = TaskEntity::find_by_id(id).one(&self.db_connection).await?;
        
        match task {
            Some(task) => Ok(task.into()),
            None => Err(Error::NotFound(format!("Task <id: {}>", id))),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<Task>, Error> {
        debug!("to_do: find_all");
        let tasks = TaskEntity::find().all(&self.db_connection).await;
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(err) => {
                error!("{:?}", err);
                Err(Error::Db(err))
            },
        }
    }

    pub async fn update_task(
            &self,
            id: Uuid,
            new_task: Task,
        ) -> Result<Task, Error> {
        debug!("todo: update_task({:?}, {:?})", id, new_task);

        let task = TaskEntity::find_by_id(id).one(&self.db_connection).await?;
        match task {
            Some(task) => {
                let mut active_model: TaskActiveModel = task.into();

                active_model.title = sea_orm::Set(new_task.title);
                active_model.completed = sea_orm::Set(new_task.completed);
                active_model.deadline = sea_orm::Set(new_task.deadline);


                let updated_task = active_model.update(&self.db_connection).await?;
                Ok(updated_task.into())
            },
            None => Err(Error::NotFound(format!("Task <id: {}>", id))),
        }
    }

    pub async fn delete_id(&self, id: Uuid) -> Result<sea_orm::DeleteResult, Error> {
        debug!("to_do: delete_id({})", id);

        let res: sea_orm::DeleteResult = TaskEntity::delete_by_id(id).exec(&self.db_connection).await?;
        Ok(res)
    }

    pub async fn delete_all(&self) -> Result<sea_orm::DeleteResult, Error> {
        debug!("to_do: delete_all()");

        let res: sea_orm::DeleteResult = TaskEntity::delete_many()
            .exec(&self.db_connection)
            .await?;

        Ok(res)
    }

    // pub async fn find_all(&self, sort_attrib: String, sort_order: i32) -> Result<Vec<Task>, Error> {
    //     debug!(
    //         "to_do: find_all(sort_attrib: {}, sort_order: {})",
    //         sort_attrib, sort_order
    //     );

    //     // Using Aggregate
    //     // Reference: https://www.mongodb.com/developer/languages/rust/rust-quickstart-aggregation/

    //     // let sort_stage = doc! {
    //     //     "$sort": {
    //     //         sort_attrib.as_str() : sort_order
    //     //     }
    //     // };

    //     // let pipeline = vec![sort_stage];

    //     // let cursor = self
    //     //     .collection
    //     //     .aggregate(pipeline, None)
    //     //     .await;

    //     // match cursor  {
    //     //     Ok(mut cursor) => {
    //     //         let mut tasks = Vec::new();
    //     //         loop {
    //     //             let result = cursor.try_next().await;

    //     //             match result {
    //     //                 Ok(Some(doc)) => {
    //     //                     let task: Task = mongodb::bson::from_document(doc)?;
    //     //                     tasks.push(task)
    //     //                 },
    //     //                 Ok(None) => break,
    //     //                 Err(err) => return Err(Error::MongoError(err)),
    //     //             }
    //     //         }

    //     //         Ok(tasks)
    //     //     }

    //     //     Err(e) => Err(Error::MongoError(e)),
    //     // }

    //     let sort_options = doc! {sort_attrib.as_str() : sort_order};
    //     let find_options = FindOptions::builder().sort(sort_options).build();

    //     let cursor = self.collection.find(None, find_options).await;

    //     match cursor {
    //         Ok(mut cursor) => {
    //             let mut tasks = Vec::new();
    //             loop {
    //                 let result = cursor.try_next().await;

    //                 match result {
    //                     Ok(Some(task)) => tasks.push(task),
    //                     Ok(None) => break,
    //                     Err(err) => return Err(Error::Mongo(err)),
    //                 }
    //             }

    //             Ok(tasks)
    //         }

    //         Err(e) => Err(Error::Mongo(e)),
    //     }
    // }

    // pub async fn find_with_params(
    //     &self,
    //     attrib: String,
    //     verb: FilterOps,
    //     date: DateTime<Utc>,
    //     sort_order: i32,
    // ) -> Result<Vec<Task>, Error> {
    //     debug!(
    //         "to_do: find_with_params(attrib: {}, verb: {}, date: {}, sort_order: {})",
    //         attrib, verb, date, sort_order
    //     );

    //     let filter: Document =
    //         doc! {attrib.as_str() : { verb.to_string().as_str(): date.timestamp_millis() }};

    //     let sort_options = doc! {attrib.as_str() : sort_order};
    //     let find_options = FindOptions::builder().sort(sort_options).build();

    //     let cursor = self.collection.find(filter, find_options).await;

    //     match cursor {
    //         Ok(mut cursor) => {
    //             let mut tasks = Vec::new();
    //             loop {
    //                 let result = cursor.try_next().await;

    //                 match result {
    //                     Ok(Some(task)) => tasks.push(task),
    //                     Ok(None) => break,
    //                     Err(err) => return Err(Error::Mongo(err)),
    //                 }
    //             }

    //             Ok(tasks)
    //         }

    //         Err(e) => Err(Error::Mongo(e)),
    //     }
    // }

    // pub async fn find_between(
    //     &self,
    //     attrib: String,
    //     start: DateTime<Utc>,
    //     end: DateTime<Utc>,
    //     sort_order: i32,
    // ) -> Result<Vec<Task>, Error> {
    //     debug!(
    //         "to_do: find_between(attrib: {}, start: {}, end: {}, sort_order: {})",
    //         attrib, start, end, sort_order
    //     );

    //     let filter: Document = doc! {
    //         attrib.as_str() :
    //         {
    //             FilterOps::Gte.to_string().as_str() : start.timestamp_millis(),
    //             FilterOps::Lte.to_string().as_str(): end.timestamp_millis()
    //         }
    //     };
    //     let sort_options = doc! {attrib.as_str() : sort_order};
    //     let find_options = FindOptions::builder().sort(sort_options).build();

    //     let cursor = self.collection.find(filter, find_options).await;

    //     match cursor {
    //         Ok(mut cursor) => {
    //             let mut tasks = Vec::new();
    //             loop {
    //                 let result = cursor.try_next().await;

    //                 match result {
    //                     Ok(Some(task)) => tasks.push(task),
    //                     Ok(None) => break,
    //                     Err(err) => return Err(Error::Mongo(err)),
    //                 }
    //             }

    //             Ok(tasks)
    //         }

    //         Err(e) => Err(Error::Mongo(e)),
    //     }
    // }

    // pub async fn delete_id(&self, id: String) -> Result<results::DeleteResult, Error> {
    //     debug!("to_do: delete_id({})", id);

    //     let object_id = parse_object_id_from_str(id.as_str())?;

    //     let filter = mongodb::bson::doc! {"_id": object_id};
    //     let task = self.collection.delete_one(filter, None).await;

    //     match task {
    //         Ok(task) => Ok(task),
    //         Err(err) => Err(Error::Mongo(err)),
    //     }
    // }

    // pub async fn delete_all(&self) -> Result<results::DeleteResult, Error> {
    //     debug!("to_do: delete_all()");

    //     let filter = mongodb::bson::doc! {};
    //     let task = self.collection.delete_many(filter, None).await;

    //     match task {
    //         Ok(task) => Ok(task),
    //         Err(err) => Err(Error::Mongo(err)),
    //     }
    // }
}
