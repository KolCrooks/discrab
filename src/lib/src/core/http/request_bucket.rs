#[derive(PartialEq)]
pub struct Bucket {
    pub max_requests: i32,
    pub remaining_requests: i32,
    pub reset_at: i64,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            max_requests: 1,
            remaining_requests: 1,
            reset_at: 0,
        }
    }
}
