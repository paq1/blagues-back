use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::repository::repository_blagues_mongo::RepositoryBlaguesMongo;
use crate::core::repository::Repository;

#[get("/random_joke")]
pub async fn random_joke(repo: &State<RepositoryBlaguesMongo>) -> String {
    //let val: String = repo.create(model: Model)().await.unwrap();
    //val
    "test".to_string()
}