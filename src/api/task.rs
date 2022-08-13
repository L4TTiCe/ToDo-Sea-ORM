use crate::errors::Error;
use crate::{database::MongoDB, model::task::PublicTask};
use crate::model::task::{Task, OptionalTask};
use crate::lib;

use actix_web::web::Query;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use chrono::serde::ts_milliseconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize};

pub fn attach_service(app: &mut actix_web::web::ServiceConfig) {
    app
        .service(create_task)
        .service(get_task)
        .service(get_all_tasks)
        .service(update_task)
        .service(delete_task);
}

#[post("/todo")]
pub async fn create_task(db: Data<MongoDB>, new_task: Json<Task>) -> HttpResponse {
    let data: Task = Task::new(new_task.task_title.clone(), new_task.task_state.clone(), new_task.task_deadline.clone());

    let status = db.task_collection.create(data).await;

    match status {
        Ok(success_result) =>  lib::model::task::insert_one_response_handler(db, success_result).await,
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct TaskIdentifier {
    task_id: String,
}

#[get("/todo/{task_id}")]
pub async fn get_task(db: Data<MongoDB>, path: Path<TaskIdentifier>) -> HttpResponse {
    let id = path.into_inner().task_id;

    if id.is_empty() {
        info!("Invalid ID");
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let task = db.task_collection.find_id(id).await;

    match task {
        Ok(task) => HttpResponse::Ok().json(PublicTask::from(task)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

fn get_default_query_param_option<T>() -> Option<T> {
    Option::None
}

#[derive(Deserialize)]
pub struct GetAllQueryParams {
    #[serde(rename = "attrib")]
    attribute: Option<String>,

    sort: Option<String>,

    #[serde(with = "ts_milliseconds_option", default = "get_default_query_param_option")]
    before: Option<DateTime<Utc>>,

    #[serde(with = "ts_milliseconds_option", default = "get_default_query_param_option")]
    after: Option<DateTime<Utc>>,

    #[serde(with = "ts_milliseconds_option", default = "get_default_query_param_option")]
    start: Option<DateTime<Utc>>,

    #[serde(with = "ts_milliseconds_option", default = "get_default_query_param_option")]
    end: Option<DateTime<Utc>>,
}

fn send_data(data: Result<Vec<Task>, Error>) -> HttpResponse {
    match data {
        Ok(tasks) => {
            let public_tasks: Vec<PublicTask> = tasks.into_iter().map(|task| PublicTask::from(task)).collect();
            HttpResponse::Ok().json(public_tasks)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todo")]
pub async fn get_all_tasks(db: Data<MongoDB>, params: Query<GetAllQueryParams>) -> HttpResponse {
    let sort_attrib: String;
    let sort_order: i32; // 1 for ascending, -1 for descending

    match &params.sort {
        Some(sort) => {
            match sort.to_lowercase().as_ref() {
                "asc" | "1" => {
                    sort_order = 1;
                },
                "desc" | "-1" => {
                    sort_order = -1;
                },
                _ => {
                    return HttpResponse::BadRequest().body("Invalid sort order. Must be either 1 or -1")
                }
            }
        }
        None => {
            sort_order = -1;
        }
    }

    match &params.attribute {
        Some(attribute) => {
            match attribute.as_str() {
                "title" | "created_at" | "deadline" => {
                    sort_attrib = attribute.to_string();
                }
                _ => {
                    info!("Invalid attribute: {}", attribute);
                    return HttpResponse::BadRequest().body(format!("Invalid attribute: {}. Valid attributes are: title, created_at, deadline", attribute));
                }
            }

            match attribute.as_str() {
                "title" => {
                    if params.before.is_some() || params.after.is_some() || params.start.is_some() || params.end.is_some() {
                        return HttpResponse::BadRequest().body("Cannot use before, after, start, or end with title");
                    }
                },
                _ => {
                    match params.before {
                        Some(date) => {
                            if params.after.is_some() {
                                return HttpResponse::BadRequest().body("Cannot use 'before' and 'after'");
                            }
                            if params.start.is_some() || params.end.is_some() {
                                return HttpResponse::BadRequest().body("Cannot use 'before' with 'start' or 'end'");
                            }
                            let data = db.task_collection.find_with_params(attribute.to_string(), lib::mongodb::FilterOps::LTE, date.clone(), sort_order).await;
                            return send_data(data)
                        }
                        None => {}
                    }
                
                    match params.after {
                        Some(date) => {
                            if params.before.is_some() {
                                return HttpResponse::BadRequest().body("Cannot use before and after");
                            }
                            if params.start.is_some() || params.end.is_some() {
                                return HttpResponse::BadRequest().body("Cannot use 'after' with 'start' or 'end'");
                            }
                            let data = db.task_collection.find_with_params(attribute.to_string(), lib::mongodb::FilterOps::GTE, date.clone(), sort_order).await;
                            return send_data(data)
                        }
                        None => {}
                    }

                    match params.start {
                        Some(start_date) => {

                            match params.end {
                                Some(end_date) => {
                                    if start_date > end_date {
                                        return HttpResponse::BadRequest().body("'start' must be before 'end'");
                                    }
                                    let data = db.task_collection.find_between(attribute.to_string(), start_date, end_date, sort_order).await;
                                    return send_data(data)
                                }
                                None => {
                                    return HttpResponse::BadRequest().body("No 'end' specified. 'start' requires 'end'. Try using 'after' instead");
                                }
                            }

                        }
                        None => {}
                    }
                }
            }
        },
        None => {
            if params.before.is_some() || params.after.is_some() {
                info!("'attrib' is required when using before or after");
                return HttpResponse::BadRequest().body("'attrib' is required when using before or after");
            } else {
                sort_attrib = "created_at".to_string();
            }
        }
    }

    let data = db.task_collection.find_all(sort_attrib, sort_order).await;
    send_data(data)
}

#[put("/todo/{task_id}")]
pub async fn update_task(db: Data<MongoDB>, path: Path<TaskIdentifier>, new_task: Json<OptionalTask>) -> HttpResponse {
    let id = path.into_inner().task_id;

    if id.is_empty() {
        info!("Invalid ID");
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let data = db.task_collection.find_id(id.clone()).await;
    
    match data {
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
        Ok(task) => {
            let mut new_data = Task::new(task.task_title.clone(), task.task_state.clone(), task.task_deadline.clone());
            
            match new_task.task_title.clone() {
                Some(title) => new_data.task_title = title,
                None => (),
            }
            match new_task.task_state {
                Some(state) => new_data.task_state = state,
                None => (),
            }
            match new_task.task_deadline {
                Some(deadline) => new_data.task_deadline = Option::from(deadline),
                None => (),
            }

            let status = db.task_collection.update_task(id, new_data).await;

            match status {
                Ok(success_result) =>  HttpResponse::Ok().json(success_result),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
    }
}

#[delete("/todo/{task_id}")]
pub async fn delete_task(db: Data<MongoDB>, path: Path<TaskIdentifier>) -> HttpResponse {  
    let id = path.into_inner().task_id;

    if id.is_empty() {
        info!("Invalid ID");
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let task = db.task_collection.delete_id(id).await;

    match task {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
