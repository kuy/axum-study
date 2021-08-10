#[cfg(feature = "memory")]
mod memory;

#[cfg(feature = "memory")]
pub use memory::Memory;

#[cfg(feature = "postgres")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "postgres")]
extern crate dotenv;

#[cfg(feature = "postgres")]
mod postgres;

#[cfg(feature = "postgres")]
pub use postgres::PostgresDatabase;

#[cfg(feature = "postgres")]
mod schema;

pub struct Beans {
    pub id: i32,
    pub name: String,
    pub country: String,
}
