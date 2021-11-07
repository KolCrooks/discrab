use bitfield::bitfield;

bitfield! {
    pub struct Snowflake(u64);
    pub timestamp, _: 63, 22;
    pub worker_id, _: 21, 17;
    pub process_id, _: 16, 12;
    pub increment, _: 11, 0;
}

impl Snowflake {
    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }
}

