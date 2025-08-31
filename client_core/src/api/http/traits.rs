use async_trait::async_trait;

use crate::api::http::{HttpRequestArgs, HttpResponse};

#[async_trait]
pub trait HttpClientTrait: Send + Sync {
    async fn request(&self, args: HttpRequestArgs) -> HttpResponse;
}
