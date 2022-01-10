#[derive(PartialEq)]

/// Contains information on an http bucket so that the route can be rate limited
pub struct Bucket {
    /// The maxiumum number of requests that can be made in the bucket
    pub max_requests: i32,
    /// The number of requests that are remaining in the bucket
    pub remaining_requests: i32,
    /// The time that the buckets rate limit will reset
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
