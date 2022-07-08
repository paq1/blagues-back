#[macro_use] extern crate rocket;

mod core;
mod api;
mod models;

use api::repository::repository_blagues_mongo::RepositoryBlaguesMongo;
use api::controller::blague_controller::random_joke;

#[launch]
async fn rocket() -> _ {
    let repo = RepositoryBlaguesMongo::new().await;

    rocket::build()
        .manage(repo)
        .mount("/", routes![random_joke])
}
