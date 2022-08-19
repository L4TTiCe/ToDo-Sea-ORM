use crate::lib::errors::Error;
use uuid::Uuid;

pub fn parse_str(uuid_str: &str) -> Result<Uuid, Error> {
    Uuid::parse_str(uuid_str).map_err(|_| Error::InvalidUuid(uuid_str.to_string()))
}
