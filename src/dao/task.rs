use crate::dao::utils;
use crate::lib::errors::Error;
use crate::lib::query;
use entity::task;

use chrono::{DateTime, Utc};
use migration::Condition;
use sea_orm::{DatabaseConnection, QueryOrder};
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

    pub async fn create(&self, new_task: task::Model) -> Result<task::Model, Error> {
        debug!("todo: create({:?})", new_task);

        let active_model: task::ActiveModel = new_task.into();
        let task = active_model.insert(&self.db_connection).await?;
        
        Ok(task)
    }

    pub async fn find_id(&self, id: Uuid) -> Result<task::Model, Error> {
        debug!("todo: find_id({:?})", id);

        let task = task::Entity::find_by_id(id).one(&self.db_connection).await?;
        
        match task {
            Some(task) => Ok(task.into()),
            None => Err(Error::NotFound(format!("Task <id: {}>", id))),
        }
    }

    pub async fn find_all(&self, sort_attrib: String, sort_order: query::SortOrder) -> Result<Vec<task::Model>, Error> {
        debug!(
            "to_do: find_all(sort_attrib: {}, sort_order: {})",
            sort_attrib, sort_order
        );

        let tasks = task::Entity::find()
            .order_by(utils::match_task_column(&sort_attrib)?, utils::match_sort_order(&sort_order))
            .all(&self.db_connection)
            .await?;
        
        Ok(tasks.into_iter().map(|task| task.into()).collect())
    }

    pub async fn find_with_params(
        &self,
        attrib: String,
        verb: query::FilterOps,
        date: DateTime<Utc>,
        sort_order: query::SortOrder,
    ) -> Result<Vec<task::Model>, Error> {
        debug!(
            "to_do: find_with_params(attrib: {}, verb: {}, date: {}, sort_order: {})",
            attrib, verb, date, sort_order
        );

        let tasks = task::Entity::find()
            .filter(utils::construct_filter(utils::match_task_column(&attrib)?, verb, date))
            .order_by(utils::match_task_column(&attrib)?, utils::match_sort_order(&sort_order))
            .all(&self.db_connection)
            .await?;

            Ok(tasks.into_iter().map(|task| task.into()).collect())
    }

    pub async fn find_between(
        &self,
        attrib: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        sort_order: query::SortOrder,
    ) -> Result<Vec<task::Model>, Error> {
        debug!(
            "to_do: find_between(attrib: {}, start: {}, end: {}, sort_order: {})",
            attrib, start, end, sort_order
        );

        let tasks = task::Entity::find()
        .filter(
            Condition::all()
                .add(utils::construct_filter(utils::match_task_column(&attrib)?, query::FilterOps::Gte, start))
                .add(utils::construct_filter(utils::match_task_column(&attrib)?, query::FilterOps::Lte, end)),
        )
        .order_by(utils::match_task_column(&attrib)?, utils::match_sort_order(&sort_order))
        .all(&self.db_connection)
        .await?;

        Ok(tasks.into_iter().map(|task| task.into()).collect())
    }

    pub async fn update_task(
            &self,
            id: Uuid,
            new_task: task::Model,
        ) -> Result<task::Model, Error> {
        debug!("todo: update_task({:?}, {:?})", id, new_task);

        let task = task::Entity::find_by_id(id).one(&self.db_connection).await?;
        match task {
            Some(task) => {
                let mut active_model: task::ActiveModel = task.into();

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

        let res: sea_orm::DeleteResult = task::Entity::delete_by_id(id).exec(&self.db_connection).await?;
        Ok(res)
    }

    pub async fn delete_all(&self) -> Result<sea_orm::DeleteResult, Error> {
        debug!("to_do: delete_all()");

        let res: sea_orm::DeleteResult = task::Entity::delete_many()
            .exec(&self.db_connection)
            .await?;

        Ok(res)
    }
}
