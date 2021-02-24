#![feature(plugin)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod post;
mod author;
mod schema;

fn main() {
    println!("Hello, world!");
}
