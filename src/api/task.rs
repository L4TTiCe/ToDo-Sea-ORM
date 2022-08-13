use crate::{database::MongoDB, model::task::PublicTask};
use crate::model::task::{Task, OptionalTask};
use crate::lib;

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

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
    info!("POST /todo {:?}", new_task);

    let data: Task = Task::new(new_task.task_title.clone(), new_task.task_state.clone(), new_task.task_deadline.clone());

    let status = db.task_collection.create(data).await;

    match status {
        Ok(success_result) =>  lib::model::task::insert_one_response_handler(db, success_result).await,
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_id: String,
}

#[get("/todo/{task_id}")]
pub async fn get_task(db: Data<MongoDB>, path: Path<TaskIdentifier>) -> HttpResponse {
    info!("GET /todo/{}", path.task_id);
    
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

#[get("/todo")]
pub async fn get_all_tasks(db: Data<MongoDB>) -> HttpResponse {
    info!("GET /todo");
    let data = db.task_collection.find_all().await;

    match data {
        Ok(tasks) => {
            let public_tasks: Vec<PublicTask> = tasks.into_iter().map(|task| PublicTask::from(task)).collect();
            HttpResponse::Ok().json(public_tasks)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/todo/{task_id}")]
pub async fn update_task(db: Data<MongoDB>, path: Path<TaskIdentifier>, new_task: Json<OptionalTask>) -> HttpResponse {
    info!("PUT /todo/{}", path.task_id);
    
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
    info!("DEL /todo/{}", path.task_id);
    
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
