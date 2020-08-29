extern crate time;
use chrono::{Duration, Utc,DateTime};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1000000000)
}
