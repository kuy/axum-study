use models::Beans;

pub trait BeansPresenter {
    fn render_list(&mut self, beans: Vec<Beans>);
}
