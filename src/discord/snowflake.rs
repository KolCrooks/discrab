use std::fmt::Display;

use bitfield::bitfield;

bitfield! {
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
