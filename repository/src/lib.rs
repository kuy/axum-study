use async_trait::async_trait;

#[async_trait]
pub trait ListBeansRepository {
    async fn get_all(&self) -> Vec<Beans>;
}

pub struct Beans {
    pub id: i32,
    pub name: String,
    pub country: String,
}
