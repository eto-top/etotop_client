use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ClientState {
    pub url: Option<String>,
    pub user_id: Option<u64>,
}
