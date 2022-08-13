use mongodb::bson;
use mongodb::error::Error as MongoError;

#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    ParseObjectID(#[from] bson::oid::Error),

    #[error("ENV VARIABLE for `{0}` is not set")]
    EnvironmentVariableNotSet(String),

    #[error("{0}")]
    Mongo(#[from] MongoError),

    #[error("{0}")]
    ServerStart(#[from] std::io::Error),
}
