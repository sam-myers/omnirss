#[macro_use]
extern crate rocket;
extern crate log;
extern crate pretty_env_logger;
extern crate redis;
#[macro_use]
extern crate serde_derive;

pub mod cache;
pub mod error;
mod routes;
mod search_query;
mod server_start;
pub mod settings;
pub mod spotify;
mod response;

pub use server_start::server_start;
