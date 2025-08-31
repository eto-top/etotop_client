use std::sync::Arc;

use client_core::state::traits::ConfigRWTrait;
use flutter_rust_bridge::DartFnFuture;

#[derive(Clone)]
pub(crate) struct ConfigRWImpl {
    pub get_config: Arc<dyn Fn() -> DartFnFuture<String> + 'static + Send + Sync>,
    pub save_config: Arc<dyn Fn(String) -> DartFnFuture<()> + 'static + Send + Sync>,
}

impl ConfigRWTrait for ConfigRWImpl {
    async fn set(&self, config: String) {
        (self.save_config)(config).await;
    }
    async fn get(&self) -> String {
        async { (self.get_config)().await.into() }.await
    }
}
