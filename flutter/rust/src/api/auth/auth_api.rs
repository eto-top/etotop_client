use std::sync::Arc;

use client_core::api::auth::traits::AuthTrait;
use flutter_rust_bridge::frb;

use crate::api::core_bridge::core_bridge::CoreBridge;

struct CoreAuth(Arc<dyn AuthTrait>);

#[frb(opaque)]
pub struct AuthApi {
    #[frb(skip)]
    auth: CoreAuth,
}

impl AuthApi {
    pub async fn login(&self, login: String, password: String) -> i64 {
        let res = self.auth.0.login(login, password).await;
        res as i64
    }

    pub fn get_auth_state(&self) -> i64 {
        self.auth.0.get_auth_state() as i64
    }

    pub async fn set_url(&self, url: String) {
        self.auth.0.set_url(url).await
    }

    pub async fn logout(&self) {
        self.auth.0.logout().await
    }
}

#[frb(sync)]
pub fn get_auth_api(bridge: CoreBridge) -> AuthApi {
    AuthApi {
        auth: CoreAuth(bridge.client_core.auth_api()),
    }
}
