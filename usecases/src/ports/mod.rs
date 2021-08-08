use async_trait::async_trait;
use models::Beans;

#[async_trait]
pub trait ListBeansInputPort {
    async fn get_all(&self) -> Vec<Beans>;
}

#[async_trait]
pub trait BeansOutputPort {
    fn render_list(&self, beans: Vec<Beans>);
}
