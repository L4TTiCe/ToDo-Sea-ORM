use mongodb::bson::oid::ObjectId;
use crate::errors::Error;

pub fn parse_object_id_from_str(object_id: &str) -> Result<ObjectId, Error> {
    mongodb::bson::oid::ObjectId::parse_str(object_id)
        .map_err(|err| {
            error!("{} is not a valid ObjectId", object_id);
            Error::ParseObjectID(err)
        })
}
