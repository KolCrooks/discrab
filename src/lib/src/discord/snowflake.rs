use std::fmt::Display;

use bitfield::bitfield;
use serde::{de, Deserialize, Deserializer, Serialize};

bitfield! {
    #[derive(Serialize, Clone, PartialEq, Eq, Hash)]
    pub struct Snowflake(u64);
    pub timestamp, _: 63, 22;
    pub worker_id, _: 21, 17;
    pub process_id, _: 16, 12;
    pub increment, _: 11, 0;
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        match s.parse() {
            Ok(sf) => Ok(Snowflake(sf)),
            Err(_) => Err(de::Error::custom("invalid snowflake")),
        }
    }
}
