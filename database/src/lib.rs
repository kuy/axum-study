use async_trait::async_trait;
use repository::ListBeansRepository;

pub struct BeansInMemory;

#[async_trait]
impl ListBeansRepository for BeansInMemory {
    async fn get_all(&self) -> Vec<models::Beans> {
        vec![
            Beans {
                id: 1,
                name: "Kangocho AA".into(),
                country: "Kenya".into(),
            }
            .into(),
            Beans {
                id: 2,
                name: "Amedera".into(),
                country: "Ethiopia".into(),
            }
            .into(),
        ]
    }
}

pub struct Beans {
    pub id: i32,
    pub name: String,
    pub country: String,
}

impl From<Beans> for models::Beans {
    fn from(beans: Beans) -> Self {
        Self {
            id: models::Id(beans.id),
            name: models::Name(beans.name),
            country: models::Country(beans.country),
        }
    }
}
