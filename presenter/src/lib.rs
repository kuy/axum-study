use models::Beans;

pub trait BeansPresenter {
    fn render_list(&self, beans: Vec<Beans>);
}
