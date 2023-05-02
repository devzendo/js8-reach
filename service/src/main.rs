#[macro_use] 
extern crate rocket;
extern crate log;

use std::env;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use std::path::PathBuf;

// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

//  Create a route for any url that is a path from the /
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../ui/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

#[get("/data/<path..>")]
async fn data(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("./data/").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}


// Return the index when the url is /
#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[launch]
fn rocket() -> _ {
    match env::var("RUST_LOG") {
        Ok(_) => {}
        Err(_) => env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    // You must mount the static_files route
    rocket::build()
        .mount("/", routes![index, static_files, data])
}

