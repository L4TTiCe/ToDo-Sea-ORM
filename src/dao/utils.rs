use crate::lib::{query, errors::Error};
use entity::task;

use sea_orm::ColumnTrait;

pub fn match_sort_order(sort_order: &query::SortOrder) -> sea_orm::Order {
    match sort_order {
        query::SortOrder::Asc => sea_orm::Order::Asc,
        query::SortOrder::Desc => sea_orm::Order::Desc,
    }
}

pub fn match_task_column(column_name: &str) -> Result<task::Column, Error> {
    match column_name {
        "title" => Ok(task::Column::Title),
        "created_at" => Ok(task::Column::CreatedAt),
        "deadline" => Ok(task::Column::Deadline),
        &_ => Err(Error::NotFound(format!("Column `{}` not found", column_name))),
    }
}

pub fn construct_filter<T>(col: task::Column, op: query::FilterOps, val: T) -> migration::SimpleExpr where sea_orm::Value: std::convert::From<T>{
    match op {
        query::FilterOps::Gte => col.gte(val),
        query::FilterOps::Lte => col.lte(val),
    }
}
