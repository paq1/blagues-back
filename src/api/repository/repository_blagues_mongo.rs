use crate::core::repository::Repository;
use crate::models::error::ErrorMessage;
use crate::models::blague::Blague;
use crate::api::repository::dbo::BlagueDbo;

use mongodb::bson::{doc, Bson, Document};
use mongodb::{options::ClientOptions, options::FindOptions, Client, Collection, Cursor};
use mongodb::bson::oid::ObjectId;
use futures::stream::TryStreamExt;
// todo use it for delete all use futures::future;

const DB_NAME: &str = "blague-db";
const COLLECTION_NAME: &str = "humour-noir";

pub struct RepositoryBlaguesMongo {
    collection: Collection<Document>
}

impl RepositoryBlaguesMongo { 
    pub async fn new() -> Self {
        let client_options_f: mongodb::error::Result<ClientOptions> = ClientOptions::parse("mongodb://localhost:27017").await;
        let client_options: ClientOptions = client_options_f.unwrap();
        let client: Client = Client::with_options(client_options).unwrap();

        // Ping the server to see if you can connect to the server
        check_connection(&client).await.unwrap();

        let db = client.database(DB_NAME);
        let collection = db.collection::<Document>(COLLECTION_NAME);

        RepositoryBlaguesMongo { collection }
    }
}


// todo dbo == dto pour le moment
#[async_trait]
impl Repository<Blague, BlagueDbo, String> for RepositoryBlaguesMongo {
    async fn create(&self, model: &Blague) -> Result<BlagueDbo, ErrorMessage> {
        // on map notre model en document
        let doc_blague: Document = doc! { "description": model.description.clone() };
        let docs = vec![doc_blague];
        let val = self.collection.insert_many(docs, None).await.unwrap();

        let ids_map = val.inserted_ids;
        let mut ids: Vec<String> = vec![];
        // todo: fold
        for (_, value) in ids_map.into_iter() {
            let obj: &Bson = &value;
            let id = obj.as_object_id().unwrap().to_hex();
            ids.push(id);
        }

        let id: String = ids[0].clone();

        self.read(&id).await
    }

    async fn read(&self, id: &String) -> Result<BlagueDbo, ErrorMessage> {
        let blagues: Vec<BlagueDbo> = self.read_all().await?
            .into_iter()
            .filter(|dbo| dbo.id == *id)
            .collect();
        if blagues.len() > 0 {
            Ok(blagues[0].clone())
        } else {
            Err(ErrorMessage::new(format!("l'id {id} n'existe pas")))
        }
        
    }

    async fn read_all(&self) -> Result<Vec<BlagueDbo>, ErrorMessage> {
        let filter = doc! {};
        let find_options = FindOptions::builder().build();
        let mut cursor: Cursor<Document> = self.collection.find(filter, find_options).await.unwrap();
        let mut lst = Vec::new();

        while let Some(task) = cursor.try_next().await.unwrap() {
            let description_bson: &Bson = &task.get("description").unwrap();
            let id_bson: &Bson = &task.get("_id").unwrap();
            let description_str: String = description_bson.as_str().unwrap().to_string();
            
            let obj_id: ObjectId = id_bson.as_object_id().unwrap();
            let id_str: String = obj_id.to_hex();
            
            // contruction du dbo
            let blague_dbo = BlagueDbo {id: id_str, description: description_str};
            lst.push(blague_dbo);
        }
        Ok(lst)
    }
}

async fn check_connection(client: &Client) -> mongodb::error::Result<()> {
    client
        .database(DB_NAME)
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("connection successful");
    Ok(())
}