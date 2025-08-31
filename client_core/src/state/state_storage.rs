use async_trait::async_trait;

use crate::{
    lock_field,
    state::{
        client_state::ClientState,
        traits::{ConfigRWTrait, StateTrait},
    },
};
use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

pub struct StateStorage<T: ConfigRWTrait> {
    pub config_rw: T,
    pub client_state: Arc<RwLock<ClientState>>,
}

impl<T: ConfigRWTrait> StateStorage<T> {
    pub fn new(config_rw: T) -> Self {
        StateStorage {
            config_rw: config_rw,
            client_state: Arc::default(),
        }
    }

    async fn save(&self) {
        self.config_rw.set(self.to_string()).await;
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(
            self.client_state
                .read()
                .expect("StateStorage lock poisoned")
                .deref(),
        )
        .expect("StateStorage to_string serde_json error")
    }
}

#[async_trait]
impl<T> StateTrait for StateStorage<T>
where
    T: ConfigRWTrait,
{
    async fn erase(&self) {
        *self.client_state.write().expect("lock poisoned") = ClientState::default();
    }

    async fn init(&self) {
        let config: String = self.config_rw.get().await;

        let p = match serde_json::from_str::<ClientState>(&config) {
            Ok(value) => value,
            Err(_) => {
                #[cfg(debug_assertions)]
                println!("Incorrect config!");
                self.config_rw.set("".to_string()).await;
                ClientState::default()
            }
        };
        *self.client_state.write().expect("lock poisoned") = p;
    }

    fn get_url(&self) -> String {
        lock_field!(read self.client_state, url).unwrap_or_default()
    }
    async fn set_url(&self, v: String) {
        lock_field!(write self.client_state, url = Some(v.to_string()));
        self.save().await
    }

    async fn set_user_id(&self, id: u64) {
        lock_field!(write self.client_state, user_id = Some(id));
        self.save().await
    }

    fn get_user_id(&self) -> u64 {
        lock_field!(read self.client_state, user_id).unwrap_or_default()
    }
}
