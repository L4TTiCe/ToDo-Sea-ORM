use std::fmt::Display;
use crate::errors::Error;
use mongodb::bson::oid::ObjectId;

pub fn parse_object_id_from_str(object_id: &str) -> Result<ObjectId, Error> {
    mongodb::bson::oid::ObjectId::parse_str(object_id).map_err(|err| {
        error!("{} is not a valid ObjectId", object_id);
        Error::ParseObjectIDFailed(err)
    })
}

// Only contains Operations that are in use. Not exhaustive.
pub enum FilterOps {
    Gte,
    Lte,
}

impl FilterOps {
    pub fn as_str(&self) -> &'static str {
        match self {
            FilterOps::Gte => "$gte",
            FilterOps::Lte => "$lte",
        }
    }
}

impl Display for FilterOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
