pub mod typing;
pub mod core;
use hyper;

fn main() {
    let client = core::http::HttpSchedulerClient::new();
    let req = hyper::Request::builder().
    client.send_request()
}