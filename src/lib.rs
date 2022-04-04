#[macro_use]
extern crate rocket;
extern crate redis;
#[macro_use]
extern crate serde_derive;

pub mod cache;
pub mod error;
mod response;
mod routes;
mod search_query;
mod server_start;
pub mod settings;
pub mod spotify;

pub use server_start::server_start;
