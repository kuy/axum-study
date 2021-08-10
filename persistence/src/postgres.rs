use chrono::NaiveDateTime;
use diesel::*;
use std::env;

pub struct PostgresDatabase;

impl PostgresDatabase {
    pub async fn get_all(&self) -> Vec<crate::Beans> {
        use crate::schema::beans::dsl::*;

        let database_url = env::var("DATABASE_URL").expect("should put '.env' file");
        let conn = PgConnection::establish(&database_url).expect("should be established");

        beans
            .load::<Beans>(&conn)
            .unwrap()
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

#[derive(Queryable)]
pub struct Beans {
    pub id: i32,
    pub name: String,
    pub country: Option<String>,
    pub created_on: NaiveDateTime,
}

impl From<Beans> for crate::Beans {
    fn from(beans: Beans) -> Self {
        crate::Beans {
            id: beans.id,
            name: beans.name,
            country: beans.country.unwrap_or_default(),
        }
    }
}
