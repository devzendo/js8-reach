#[macro_use] 
extern crate rocket;
extern crate log;

use std::env;


#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    match env::var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    rocket::build()
        .mount("/", routes![world])
}

