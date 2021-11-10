pub mod core;
pub mod typing;
mod util;

use crate::core::http::RequestRoute;

#[tokio::main]
async fn main() {
    let mut client = core::http::HttpSchedulerClient::new();
    client.spawn_req_thread();

    let req = hyper::Request::builder()
        .uri("http://localhost:8000/")
        .body(hyper::Body::empty())
        .unwrap();

    let route = RequestRoute {
        base_route: "".to_string(),
        major_param: "".to_string(),
    };
    let res = client.send_request(route, req).await.unwrap();
    println!("{:?}", res);
}
