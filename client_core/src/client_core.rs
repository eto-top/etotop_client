use std::sync::Arc;

use crate::{
    api::auth::traits::{AuthEventBusTrait, AuthTrait},
    api::http::traits::HttpClientTrait,
    domain::di::Di,
    state::traits::StateTrait,
};

pub struct ClientCore<S: StateTrait, A: AuthEventBusTrait, H: HttpClientTrait> {
    di: Arc<Di<S, A, H>>,
}

impl<S: StateTrait + 'static, A: AuthEventBusTrait + 'static, H: HttpClientTrait + 'static>
    ClientCore<S, A, H>
{
    pub fn new(storage: S, auth_bus: A, http_client: H) -> Self {
        Self {
            di: Arc::new(Di::new(
                Arc::new(auth_bus),
                Arc::new(storage),
                Arc::new(http_client),
            )),
        }
    }

    pub async fn init(&self) {
        self.di.state.init().await;
    }

    pub fn auth_api(&self) -> Arc<dyn AuthTrait> {
        self.di.auth_api.clone()
    }
}
