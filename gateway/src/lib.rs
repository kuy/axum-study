use async_trait::async_trait;
use persistence::InMemory;
use usecases::ports::ListBeansInputPort;

pub struct BeansGateway {
    db: InMemory,
}

impl BeansGateway {
    pub fn new() -> Self {
        Self { db: InMemory {} }
    }
}

#[async_trait]
impl ListBeansInputPort for BeansGateway {
    async fn get_all(&self) -> Vec<models::Beans> {
        self.db.get_all().into_iter().map(convert).collect()
    }
}

fn convert(beans: persistence::Beans) -> models::Beans {
    models::Beans {
        id: models::Id(beans.id),
        name: models::Name(beans.name),
        country: models::Country(beans.country),
    }
}
