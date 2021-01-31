#[macro_use]
extern crate diesel;

mod domains;
mod infrastructures;
mod server;
mod usecases;

fn main() -> std::io::Result<()> {
    server::run()
}
