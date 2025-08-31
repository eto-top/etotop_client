use std::sync::Arc;

use crate::{
    api::auth::{auth_api::AuthApi, traits::AuthEventBusTrait},
    api::http::traits::HttpClientTrait,
    state::traits::StateTrait,
};

pub(crate) struct Di<S: StateTrait, A: AuthEventBusTrait, H: HttpClientTrait> {
    pub state: Arc<S>,
    pub auth_api: Arc<AuthApi<S, A, H>>,
    pub http_client: Arc<H>,
}

impl<S: StateTrait + 'static, A: AuthEventBusTrait + 'static, H: HttpClientTrait + 'static>
    Di<S, A, H>
{
    pub fn new(a: Arc<A>, s: Arc<S>, h: Arc<H>) -> Self {
        let auth_api = Arc::new(AuthApi {
            state: s.clone(),
            auth_bus: a.clone(),
            http_client: h.clone(),
        });

        Self {
            state: s,
            auth_api,
            http_client: h,
        }
    }
}
