use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Bson};
use mongodb::options::FindOptions;
use std::error::Error;
use std::result::Result;
use tokio;
mod gui;
use gui::*;
mod mongodriver;
use mongodriver::*;
async fn amain() -> Result<(), Box<dyn Error>> {
    let client = get_client("superUser".to_owned(), "12345".to_owned())?;
    let db = client.database("wypozyczalnia");
    // for col_name in db.list_collection_names(None).await? {
    let find_options = FindOptions::builder().projection(doc! { "_id": 0 }).build();
    let col = db.collection::<Bson>("admin");
    // println!("{}:", &col_name);
    let mut cursor = col.find(None, find_options).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }
    prompt_login(db);
    // }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    amain().await
}
