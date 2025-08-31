#![deny(clippy::unwrap_used)]

pub mod api;
pub mod client_core;
pub mod state;

pub(crate) mod domain;
pub(crate) mod utils;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::{
        api::{
            auth::traits::AuthEventBusTrait,
            http::{HttpRequestArgs, HttpResponse, traits::HttpClientTrait},
        },
        client_core::ClientCore,
        state::traits::ConfigRWTrait,
    };

    use super::*;

    struct ConfigRWImpl {}

    impl ConfigRWTrait for ConfigRWImpl {
        async fn set(&self, config: String) {
            println!("saving config {}", config);
        }
        async fn get(&self) -> String {
            "{}".to_string()
        }
    }

    struct AuthBusImpl {}

    impl AuthEventBusTrait for AuthBusImpl {
        async fn push_updated(&self) {
            println!("push updated");
        }
    }
    struct HttpClientImpl {}

    #[async_trait]
    impl HttpClientTrait for HttpClientImpl {
        async fn request(&self, _args: HttpRequestArgs) -> HttpResponse {
            HttpResponse {
                status: todo!(),
                response_code: todo!(),
                payload: todo!(),
            }
        }
    }

    #[tokio::test] // This macro is required for async tests
    async fn test_kv_storage() {
        let conf = ConfigRWImpl {};
        let state_storage = state::state_storage::StateStorage::<ConfigRWImpl>::new(conf.into());
        let auth_bus = AuthBusImpl {};
        let http_client = HttpClientImpl {};

        let core: ClientCore<
            state::state_storage::StateStorage<ConfigRWImpl>,
            AuthBusImpl,
            HttpClientImpl,
        > = ClientCore::new(state_storage, auth_bus, http_client);

        core.init().await;

        let auth_i = core.auth_api();
        let auth_state = auth_i.get_auth_state();
        println!("init auth state: {}", auth_state as i64);
        auth_i.set_url("example.xom".to_string()).await;
    }
}
