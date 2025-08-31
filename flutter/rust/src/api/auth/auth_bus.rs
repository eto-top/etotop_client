use std::sync::Arc;

use client_core::api::auth::traits::AuthEventBusTrait;
use flutter_rust_bridge::DartFnFuture;

#[derive(Clone)]
pub(crate) struct AuthBusImpl {
    pub update_auth: Arc<dyn Fn() -> DartFnFuture<()> + 'static + Send + Sync>,
}

impl AuthEventBusTrait for AuthBusImpl {
    async fn push_updated(&self) {
        (self.update_auth)().await;
    }
}
