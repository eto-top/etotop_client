use std::sync::Arc;

use async_trait::async_trait;
use client_core::api::http::{traits::HttpClientTrait, HttpRequestArgs, HttpResponse};
use flutter_rust_bridge::{frb, DartFnFuture};

use crate::api::core_bridge::core_bridge::{HttpRequestArgsImpl, HttpResponseImpl};

#[frb(ignore)]
#[derive(Clone)]
pub(crate) struct HttpClientImp {
    pub http_request:
        Arc<dyn Fn(HttpRequestArgsImpl) -> DartFnFuture<HttpResponseImpl> + 'static + Send + Sync>,
}

#[frb(ignore)]
#[async_trait]
impl HttpClientTrait for HttpClientImp {
    async fn request(&self, args: HttpRequestArgs) -> HttpResponse {
        let res = (self.http_request)(HttpRequestArgsImpl {
            method: args.method.to_string(),
            endpoint: args.endpoint,
            headers: args.headers,
            body: args.body,
            timeout_secs: args.timeout_secs,
        })
        .await;

        HttpResponse {
            status: res.status,
            response_code: res.response_code,
            payload: res.payload,
        }
    }
}
