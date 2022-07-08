use async_trait::async_trait;

use crate::models::error::ErrorMessage;

#[async_trait]
pub trait Repository<Model, Dbo, Id> {
    async fn create(&self, model: &Model) -> Result<Dbo, ErrorMessage>;
    async fn read(&self, id: &String) -> Result<Dbo, ErrorMessage>;
    async fn read_all(&self) -> Result<Vec<Dbo>, ErrorMessage>;
}