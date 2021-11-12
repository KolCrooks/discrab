pub mod core;
pub mod discord;
mod util;

use crate::core::http::{
    rate_limit_client::{RLClient, RequestRoute},
    request_queue::BasicHttpQueue,
};

#[tokio::main]
async fn main() {
    unsafe impl Send for BasicHttpQueue {}

    let client = RLClient::default();

    loop {
        let req = hyper::Request::builder()
            .uri("http://localhost:8000/")
            .body(hyper::Body::empty())
            .unwrap();
        let route = RequestRoute {
            base_route: "".to_string(),
            major_param: "".to_string(),
        };
        client.send_request(route, req).await.unwrap();
    }
    // println!("{:?}", res);
}
