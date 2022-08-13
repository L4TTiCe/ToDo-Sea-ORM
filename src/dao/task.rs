
use crate::model::task::Task;
use crate::errors::Error;
use crate::lib::object_id::parse_object_id_from_str;

use mongodb::{Collection, Database, results, bson::doc};
// This trait is required to use `try_next()` on the cursor
use futures::stream::TryStreamExt;

pub struct TaskCollection {
    pub collection: Collection<Task>,
}

impl TaskCollection {
    pub fn init(db: Database, collection_name: String) -> Self {
        TaskCollection {
            collection: db.collection(collection_name.as_str()),
        }
    }
}
