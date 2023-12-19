#[macro_use]
extern crate rocket;

use rocket::Config;
use std::net::Ipv4Addr;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    let config: rocket::Config = Config {
        port: 9000,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::debug_default()
    };
    rocket::custom(config).mount("/", routes![hello])
}
