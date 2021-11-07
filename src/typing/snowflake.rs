use bitfield::bitfield;

bitfield! {
    pub struct Snowflake(u64);

    timestamp, _: 63, 22;
    worker_id, _: 21, 17;
    process_id, _: 16, 12;
    increment, _: 11, 0;
}