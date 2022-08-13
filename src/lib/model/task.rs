use crate::{
    database::MongoDB,
    model::task::{InsertOneResponse, PublicTask},
};
use actix_web::HttpResponse;
use mongodb::bson::oid::ObjectId;

pub async fn insert_one_response_handler(
    db: actix_web::web::Data<MongoDB>,
    insert_one_reponse: mongodb::results::InsertOneResult,
) -> HttpResponse {
    let verbose = std::env::var("VERBOSE_REST").is_ok();

    if verbose {
        let object_id: ObjectId = insert_one_reponse.inserted_id.as_object_id().unwrap();

        let task = db.task_collection.find_id(object_id.to_hex()).await;
        match task {
            Ok(task) => HttpResponse::Created().json(PublicTask::from(task)),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        HttpResponse::Created().json(InsertOneResponse::from(insert_one_reponse))
    }
}
