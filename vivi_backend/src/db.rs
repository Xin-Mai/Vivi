use crate::user::User;
use mongodb::{
    bson::{doc, Bson, Document},
    error::Error,
    sync::{Client, Collection, Database},
};

struct DBManager {
    client: Database,
    user: Collection<User>,
}

lazy_static! {
    static ref DB: DBManager = {
        let cli = Client::with_uri_str("mongodb://localhost:27017").map(|v| { v.database("vivi") }).unwrap();
        let usr = cli.collection("user");
        DBManager {
            client: cli,
            user: usr,
        }
    };
}

pub fn find_user(username: &String, password: &String) -> Result<Option<User>, Error> {
    let c = &DB.user;
    c.find_one(doc! {"username": username, "password": password}, None)
}
