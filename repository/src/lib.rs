use async_trait::async_trait;
use models::Beans;

#[async_trait]
pub trait ListBeansRepository {
    async fn get_all(&self) -> Vec<Beans>;
}
