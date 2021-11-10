pub mod core;
pub mod typing;
mod util;

use hyper;

use crate::core::HttpClient::RequestRoute;

fn main() {
    let mut client = core::HttpClient::HttpSchedulerClient::new();
    client.spawn_req_thread();

    let req = hyper::Request::builder()
        .uri("http://localhost:8080/")
        .body(hyper::Body::empty())
        .unwrap();

    async {
        let route = RequestRoute {
            base_route: "".to_string(),
            major_param: "".to_string(),
        };
        let res = client.send_request(route, req).await.unwrap();
        println!("{:?}", res);
    };
}
