use async_trait::async_trait;

#[async_trait]
pub trait StateTrait: Send + Sync {
    async fn init(&self);
    async fn erase(&self);

    async fn set_url(&self, url: String);
    fn get_url(&self) -> String;

    async fn set_user_id(&self, id: u64);
    fn get_user_id(&self) -> u64;
}

pub trait ConfigRWTrait: Send + Sync {
    fn set(&self, config: String) -> impl std::future::Future<Output = ()> + Send;
    fn get(&self) -> impl std::future::Future<Output = String> + Send;
}
