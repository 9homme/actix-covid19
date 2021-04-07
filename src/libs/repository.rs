use crate::libs::model::NewUser;
use async_trait::async_trait;
use blake2::{Blake2b, Digest};
use bson::Bson;
use color_eyre::{Report, Result};
use mongodb::bson::doc;
use mongodb::{results::InsertOneResult, Database};

use super::model::User;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn add_user(&self, new_user: NewUser) -> Result<InsertOneResult>;
    async fn get_user(&self, username: &String) -> Result<Option<User>>;
}

pub struct RepositoryImpl {
    db: Database,
}

impl RepositoryImpl {
    pub fn new(db: Database) -> Self {
        RepositoryImpl { db }
    }
}

#[async_trait]
impl Repository for RepositoryImpl {
    async fn add_user(&self, new_user: NewUser) -> Result<InsertOneResult> {
        let existing_user = self.get_user(&new_user.username).await;
        match existing_user {
            Ok(Some(_)) => Err(Report::msg("This username is already exists")),
            _ => {
                let collection = self.db.collection("user");
                let hash = Blake2b::digest(new_user.password.as_ref());
                let doc =
                    doc! {"username": &new_user.username, "password_hash":format!("{:x}", hash)};
                let result = collection.insert_one(doc, None).await?;
                Ok(result)
            }
        }
    }

    async fn get_user(&self, username: &String) -> Result<Option<User>> {
        let collection = self.db.collection("user");
        let filter = doc! { "username": username };
        let user_doc = collection.find_one(filter, None).await?;
        match user_doc {
            Some(d) => {
                let user: User = bson::from_bson(Bson::Document(d))?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
