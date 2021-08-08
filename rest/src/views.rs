use crossbeam_channel::Sender;
use usecases::ports;

pub struct BeansRenderer {
    sender: Sender<String>,
}

impl BeansRenderer {
    pub fn new(sender: Sender<String>) -> Self {
        Self { sender }
    }
}

impl ports::BeansOutputPort for BeansRenderer {
    fn render_list(&self, beans: Vec<models::Beans>) {
        let content = beans
            .into_iter()
            .map(|b| b.name.to_string())
            .reduce(|acc, b| format!("{}, {}", acc, b))
            .unwrap_or("[No Content]".into());
        self.sender.send(content).expect("should be sent");
    }
}
