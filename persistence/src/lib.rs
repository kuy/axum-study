pub struct InMemory;

impl InMemory {
    pub fn get_all(&self) -> Vec<Beans> {
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

pub struct Beans {
    pub id: i32,
    pub name: String,
    pub country: String,
}
