use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    api::{
        auth::traits::{AuthEventBusTrait, AuthResult, AuthTrait},
        http::{ProtobufRequestBuilder, ResponseType, traits::HttpClientTrait},
    },
    state::{auth_state::CAuthState, traits::StateTrait},
};

pub struct AuthApi<S: StateTrait, A: AuthEventBusTrait, H: HttpClientTrait> {
    pub(crate) state: Arc<S>,
    pub(crate) auth_bus: Arc<A>,
    pub(crate) http_client: Arc<H>,
}

#[async_trait]
impl<S: StateTrait + 'static, A: AuthEventBusTrait + 'static, H: HttpClientTrait + 'static>
    AuthTrait for AuthApi<S, A, H>
{
    async fn login(&self, _login: String, _password: String) -> AuthResult {
        let req = ProtobufRequestBuilder::new(self.state.get_url() + "/login", [].to_vec()).build();
        let resp = self.http_client.request(req).await;
        println!(
            "login response code: {}, payload: {}",
            resp.response_code,
            String::from_utf8_lossy(&resp.payload)
        );

        if resp.status == ResponseType::Ok as u8 {
            return AuthResult::Ok;
        } else if resp.status == ResponseType::Timeout as u8
            || resp.status == ResponseType::NetworkError as u8
        {
            return AuthResult::NetworkError;
        }
        AuthResult::ServerError
    }

    fn get_auth_state(&self) -> CAuthState {
        let mut answer = CAuthState::Login;
        let url = self.state.get_url();
        let user_id = self.state.get_user_id();
        if url.is_empty() {
            answer = CAuthState::EnterUrl;
        } else if user_id != u64::default() {
            answer = CAuthState::Main;
        }
        println!(
            "url: {}\nuser_id: {}\nauth state: {}",
            url, user_id, answer as i64
        );
        answer
    }

    async fn set_url(&self, url: String) {
        self.state.set_url(url).await;
        self.push_state_updated().await
    }

    async fn logout(&self) {
        // TODO close session on server
        let url = self.state.get_url();
        self.state.erase().await;
        self.state.set_url(url).await;
        self.push_state_updated().await;
    }
}

impl<S: StateTrait + 'static, A: AuthEventBusTrait + 'static, H: HttpClientTrait + 'static>
    AuthApi<S, A, H>
{
    pub async fn push_state_updated(&self) {
        self.auth_bus.push_updated().await;
    }
}
