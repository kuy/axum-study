use crate::Beans;

pub struct Memory;

impl Memory {
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
            Beans {
                id: 2,
                name: "La Bolsa!".into(),
                country: "Guatemala".into(),
            },
        ]
    }
}
