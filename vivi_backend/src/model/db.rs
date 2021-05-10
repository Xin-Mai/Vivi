use super::user::User;
use mongodb::sync::{Client, Collection, Database};

lazy_static! {
    static ref DB: Database = Client::with_uri_str("mongodb://localhost:27017")
        .map(|v| { v.database("vivi") })
        .unwrap();
}

pub fn user_collection() -> Collection<User> {
    DB.collection("user")
}
