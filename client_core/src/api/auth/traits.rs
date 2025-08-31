use async_trait::async_trait;

use crate::state::auth_state::CAuthState;

#[repr(u8)]
pub enum AuthResult {
    Ok = 0,
    NetworkError = 1,
    ServerError = 2,
}

#[async_trait]
pub trait AuthTrait: Send + Sync {
    async fn login(&self, login: String, password: String) -> AuthResult;

    fn get_auth_state(&self) -> CAuthState;

    async fn set_url(&self, url: String);

    async fn logout(&self);
}

pub trait AuthEventBusTrait: Send + Sync {
    fn push_updated(&self) -> impl std::future::Future<Output = ()> + Send;
}
