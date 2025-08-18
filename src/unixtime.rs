// simple wrapper around a DateTime with preset precision - to the second is more
// than good enough for our use cases and provides efficient storage.

use crate::utils::Utils;
use chrono::{Utc, TimeZone};

use serde::{Serialize, Serializer};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct UnixTime(pub chrono::DateTime<chrono::Utc>);

impl UnixTime {
    pub fn now() -> Self {
        Self(Utils::utc())
    }

    pub fn new(timestamp: u32) -> Self {
        Self(Utc.timestamp_opt(timestamp as i64, 0).unwrap())
    }
}

// default chrono serialization uses RFC3339 with nanosecond precision..
// a bit overkill for our uses. clamp it to seconds.
impl Serialize for UnixTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}
