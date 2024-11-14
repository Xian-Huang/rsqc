use chrono::{DateTime, Utc};

pub mod args;
pub mod reader;
pub mod render;

pub fn get_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.date_naive().to_string()
}
