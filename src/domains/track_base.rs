use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TrackBase {
    pub id: i32,
    pub category: u8,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timespan: DateTime<Utc>,
    pub create_user: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>
}

