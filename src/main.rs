#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::missing_docs
)]

#![allow(
    clippy::module_name_repetitions,
    clippy::items_after_statements
)]

mod printing;

fn main() {
    println!("Hello world!");
}
