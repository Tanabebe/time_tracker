mod application;
mod persistence;
mod domains;
mod schema;

#[macro_use]
extern crate diesel;

fn main() -> std::io::Result<()>{
    application::server::run()
}
