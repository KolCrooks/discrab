use std::borrow::BorrowMut;

use hyper::{body, Body, Error, Method, Request};

use crate::core::http::rate_limit_client::{RLClient, RequestRoute};
use serde::Deserialize;
use simd_json;

/**
 * Gateway Object
 * @docs https://discord.com/developers/docs/topics/gateway#get-gateway-bot
 */
#[derive(Deserialize)]

pub struct Gateway {
    /// The WSS URL that can be used for connecting to the gateway
    pub url: String,
    /// The recommended number of shards to use when connecting
    pub shards: u64,
    /// Information on the current session start limit
    pub session_start_limit: SessionStartLimit,
}

/**
 * Session Start Limit Object
 * @docs https://discord.com/developers/docs/topics/gateway#session-start-limit-object
 */
#[derive(Deserialize)]
pub struct SessionStartLimit {
    /// The total number of session starts the current user is allowed
    pub total: u64,
    /// The remaining number of session starts the current user is allowed
    pub remaining: u64,
    /// The number of milliseconds after which the limit resets
    pub reset_after: u64,
    /// The number of identify requests allowed per 5 seconds
    pub max_concurrency: u64,
}

pub async fn get_gateway(http_client: &RLClient) -> Result<Gateway, simd_json::Error> {
    let route = RequestRoute {
        base_route: "/gateway".to_string(),
        major_param: "".to_string(),
    };
    let request_builder = Request::builder()
        .method(Method::GET)
        .uri("https://discord.com/api/gateway/bot")
        .header("content-type", "application/json")
        .body(Body::empty())
        .unwrap();

    let route = http_client
        .send_request(route, request_builder)
        .await
        .unwrap();

    let mut bytes = hyper::body::to_bytes(route).await.unwrap().to_vec();

    simd_json::from_slice::<Gateway>(&mut bytes)
}
