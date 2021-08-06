use async_trait::async_trait;
use repository::*;

pub struct BeansInMemory;

#[async_trait]
impl ListBeansRepository for BeansInMemory {
    async fn get_all(&self) -> Vec<Beans> {
        vec![
            Beans {
                id: 1,
                name: "Kangocho AA".into(),
                country: "Kenya".into(),
            },
            Beans {
                id: 2,
                name: "Amedera".into(),
                country: "Ethiopia".into(),
            },
        ]
    }
}
