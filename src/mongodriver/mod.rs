use futures::executor;
use futures::stream::TryStreamExt;
use mongodb::bson::{self, doc};
use mongodb::options::FindOptions;
use mongodb::options::{ClientOptions, Credential, ServerAddress};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::*;
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub name: String,
    pub surname: String,
    pub phonenumber: String,
    pub register_date: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Borrows {
    pub client: Client,
    pub borrow_date: String,
    pub end_of_borrow_date: String,
    pub return_date: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Movies {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub title: String,
    pub director: String,
    pub actors: Vec<String>,
    pub genre: Vec<String>,
    pub score: f64,
    pub length: Option<f32>,
    pub short_desc: String,
}
const AUTH_SOURCE: &'static str = "wypozyczalnia";
pub fn get_client(
    username: String,
    password: String,
) -> Result<mongodb::Client, mongodb::error::Error> {
    let srv = ServerAddress::parse("localhost:27017").unwrap();
    let credentials = Credential::builder()
        .username(username.to_owned())
        .password(password.to_owned())
        .source(AUTH_SOURCE.to_owned())
        .build();
    let options = ClientOptions::builder()
        .credential(credentials)
        .hosts(vec![srv])
        .build();
    let client = mongodb::Client::with_options(options);
    client
}

use mongodb::bson::oid::ObjectId;
pub async fn rent_movies(
    db: &mongodb::Database,
    movies: Vec<ObjectId>,
) -> Result<(), mongodb::error::Error> {
    use chrono::{DateTime, Utc}; // 0.4.15
    use std::time::{Duration, SystemTime};

    for movie in movies {
        let now = SystemTime::now();
        let two_weeks = Duration::new(2 * 7 * 24 * 60 * 60, 0);
        let end = now + two_weeks;
        let end: DateTime<Utc> = end.into();
        let end = end.to_rfc3339();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let document = doc! {"_id": movie,"begin_date":now,"end_date":end,"actual_end_date":""};
        let col = db.collection::<mongodb::bson::Document>("wypozyczenia");
        // col.insert_one(document, doc! {});
    }

    Ok(())
}
pub async fn get_movies(db: &mongodb::Database) -> Result<Vec<Movies>, mongodb::error::Error> {
    let movies = db.collection::<Movies>("filmy");
    let mut cursor = movies.find(None, None).await?;
    let mut movies: Vec<Movies> = Vec::new();
    while let Some(doc) = cursor.try_next().await? {
        movies.push(doc);
    }
    Ok(movies)
}
pub fn is_admin(username: &String, db: &Database) -> std::result::Result<bool, Box<dyn Error>> {
    let res: bson::Document = executor::block_on(db.run_command(doc! {"usersInfo": 1}, None))?;
    for user in res.get("users").unwrap().as_array().unwrap() {
        let user_doc = user.as_document().unwrap();
        if username == user_doc.get("user").unwrap().as_str().unwrap() {
            let roles = user_doc.get("roles").unwrap().as_array().unwrap();
            for role in roles {
                if role
                    .as_document()
                    .unwrap()
                    .get("role")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    == "admin"
                {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
    }
    Ok(false)
}
