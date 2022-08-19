use std::time::Duration;

use crate::dao::task::TaskDao;
use crate::lib::env::get_env_var;
use crate::lib::errors::Error;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct DbClient {
    pub task_dao: TaskDao,
}

fn construct_db_uri() -> Result<String, Error> {
    let mut uri = String::new();
    info!("Constructing DB URI");

    uri.push_str(get_env_var(String::from("DB_PREFIX"))?.as_str());
    uri.push_str("://");
    uri.push_str(get_env_var(String::from("DB_USER"))?.as_str());
    uri.push(':');

    let mut redacted_uri = uri.clone();
    redacted_uri.push_str("******");

    uri.push_str(get_env_var(String::from("DB_PASSWORD"))?.as_str());

    uri.push('@');
    redacted_uri.push('@');

    uri.push_str(get_env_var(String::from("DB_HOST"))?.as_str());
    redacted_uri.push_str(get_env_var(String::from("DB_HOST"))?.as_str());

    uri.push(':');
    redacted_uri.push(':');

    uri.push_str(get_env_var(String::from("DB_PORT"))?.as_str());
    redacted_uri.push_str(get_env_var(String::from("DB_PORT"))?.as_str());

    uri.push('/');
    redacted_uri.push('/');

    uri.push_str(get_env_var(String::from("POSTGRES_DB"))?.as_str());
    redacted_uri.push_str(get_env_var(String::from("POSTGRES_DB"))?.as_str());

    info!("Constructed DB URI: {}", redacted_uri);
    Ok(uri)
}

impl DbClient {
    pub async fn init() -> Result<Self, Error> {
        let uri = construct_db_uri()?;

        info!("Connecting to PorstgreSQL...");

        let mut options = ConnectOptions::new(uri.clone());
        options.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);
                
        let db_connection: DatabaseConnection = Database::connect(options).await?;

        let task_dao = TaskDao::init(db_connection.clone());

        Ok(DbClient { task_dao })
    }
}
