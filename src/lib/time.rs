use chrono::{DateTime, Utc};

pub fn get_current_time() -> DateTime<Utc> {
    chrono::Utc::now()
}
