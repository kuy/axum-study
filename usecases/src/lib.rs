use presenter::BeansPresenter;
use repository::ListBeansRepository;

pub struct BeansUsecase<R, V>
where
    R: ListBeansRepository,
    V: BeansPresenter,
{
    repo: R,
    view: V,
}

impl<R, V> BeansUsecase<R, V>
where
    R: ListBeansRepository,
    V: BeansPresenter,
{
    pub fn new(repo: R, view: V) -> Self {
        Self { repo, view }
    }

    pub async fn list(&self) {
        let beans = self.repo.get_all().await;
        self.view.render_list(beans);
    }
}
