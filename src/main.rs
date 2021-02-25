#![feature(plugin)]
#[macro_use]
extern crate diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;


use article::Article;
use author::Author;

mod schema;
pub mod article;
pub mod author;

fn main() {
    println!("Hello, world!");
}
