use crate::errors::Error;
use crate::lib::env::get_env_var;
use crate::dao::task::TaskCollection;

use mongodb::{Client, options::ClientOptions, bson::doc, Database};
use tokio::time::timeout;

pub struct MongoDB {
    pub task_collection: TaskCollection,
}

fn construct_db_uri() -> Result<String, Error> {
    let mut uri = String::new();
    info!("Constructing DB URI");

    uri.push_str(get_env_var(String::from("DB_PREFIX"))?.as_str());
    uri.push_str("://");
    uri.push_str(get_env_var(String::from("DB_USER"))?.as_str());
    uri.push(':');
    uri.push_str(get_env_var(String::from("DB_PASSWORD"))?.as_str());
    uri.push('@');
    uri.push_str(get_env_var(String::from("DB_HOST"))?.as_str());
    uri.push(':');
    uri.push_str(get_env_var(String::from("DB_PORT"))?.as_str());

    info!("Sucessfully constructed DB URI");
    Ok(uri)
}

async fn ping_db(client: Client) {
    let timeout_duration = std::time::Duration::from_secs(5);

    if let Err(_) = timeout(
        timeout_duration, 
        client.database("admin")
            .run_command(doc! {"ping": 1}, None)
        ).await {
            warn!("Failed to recieve response fron Database wihin {} s", timeout_duration.as_secs());
    } else {
        info!("Sucessfuly connected to MongoDB.");
    }
}

impl MongoDB {
    pub async fn init() -> Result<Self, Error> {
        let uri = construct_db_uri()?;

        info!("Connecting to MongoDB...");

        let options = ClientOptions::parse(uri.as_str()).await?;
        let client = Client::with_options(options)?;
        let database_name = get_env_var(String::from("DB_DATABASE"))?;

        ping_db(client.clone()).await;

        let db: Database = client.database(database_name.as_str());

        info!("Linking to Task Collection...");
        let task_collection = TaskCollection::init(db.clone(), String::from("tasks"));
        
        Ok(
            MongoDB {
                task_collection,
            }
        )
    }
}
