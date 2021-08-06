use repository::ListBeansRepository;

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

    pub async fn list(&self) -> String {
        let beans = self.beans.get_all().await;

        beans
            .into_iter()
            .map(|b| b.name.to_string())
            .reduce(|acc, b| format!("{}, {}", acc, b))
            .unwrap_or("<Nothing>".into())
    }
}
