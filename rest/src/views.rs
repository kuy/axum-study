use presenter::BeansPresenter;
use std::io::Write;

pub struct BeansRenderer {
    out: Box<dyn Write>,
}

impl BeansRenderer {
    pub fn new<'a, IO>(out: &'a mut IO) -> Self
    where
        IO: Write,
        'a: 'static,
    {
        Self { out: Box::new(out) }
    }
}

impl BeansPresenter for BeansRenderer {
    fn render_list(&mut self, beans: Vec<models::Beans>) {
        let content = beans
            .into_iter()
            .map(|b| b.name.to_string())
            .reduce(|acc, b| format!("{}, {}", acc, b))
            .unwrap_or("[No Content]".into());

        self.out
            .write_all(content.as_bytes())
            .expect("should be written");
    }
}
