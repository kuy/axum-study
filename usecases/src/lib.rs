use repository::{Beans, ListBeansRepository};

pub struct BeansUsecase<R>
where
    R: ListBeansRepository,
{
    beans: R,
}

impl<R> BeansUsecase<R>
where
    R: ListBeansRepository,
{
    pub fn new(beans: R) -> Self {
        Self { beans }
    }

    pub async fn list(&self) -> Vec<Beans> {
        self.beans.get_all().await
    }
}
