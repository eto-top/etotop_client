use std::sync::Arc;

use crate::api::auth::auth_bus;
use crate::api::core_bridge::config_rw;
use crate::api::core_bridge::http_client_impl::HttpClientImp;
use client_core::client_core::ClientCore;
use client_core::state::state_storage::StateStorage;
pub use flutter_rust_bridge::{frb, DartFnFuture};

#[derive(Clone)]
#[frb(opaque)]
pub struct CoreBridge {
    pub(crate) client_core: Arc<
        ClientCore<StateStorage<config_rw::ConfigRWImpl>, auth_bus::AuthBusImpl, HttpClientImp>,
    >,
}

impl CoreBridge {
    #[frb(sync)]
    pub fn new(
        get_config: impl Fn() -> DartFnFuture<String> + 'static + Send + Sync,
        save_config: impl Fn(String) -> DartFnFuture<()> + 'static + Send + Sync,
        update_auth: impl Fn() -> DartFnFuture<()> + 'static + Send + Sync,
        http_request: impl Fn(HttpRequestArgsImpl) -> DartFnFuture<(HttpResponseImpl)>
            + 'static
            + Send
            + Sync,
    ) -> Self {
        let config = config_rw::ConfigRWImpl {
            get_config: Arc::new(get_config),
            save_config: Arc::new(save_config),
        };

        let storage = StateStorage::new(config);

        let auth_bus = auth_bus::AuthBusImpl {
            update_auth: Arc::new(update_auth),
        };

        let http_client = HttpClientImp {
            http_request: Arc::new(http_request),
        };

        let client_core: ClientCore<
            StateStorage<config_rw::ConfigRWImpl>,
            auth_bus::AuthBusImpl,
            HttpClientImp,
        > = ClientCore::new(storage, auth_bus, http_client);

        Self {
            client_core: Arc::new(client_core),
        }
    }

    pub async fn init(&mut self) {
        self.client_core.init().await;
    }
}

#[frb(opaque)]
pub struct HttpResponseImpl {
    pub status: u8,
    pub response_code: u16,
    pub payload: Vec<u8>,
}

impl HttpResponseImpl {
    #[frb(sync)]
    pub fn new(status: u8, response_code: u16, payload: Vec<u8>) -> Self {
        Self {
            status,
            response_code,
            payload,
        }
    }
}

pub struct HttpRequestArgsImpl {
    pub method: String,
    pub endpoint: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub timeout_secs: u64,
}
