use super::article::Article;
use super::comment::Comment;
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

pub fn article_collection() -> Collection<Article> {
    DB.collection("article")
}

pub fn comment_collection() -> Collection<Comment> {
    DB.collection("comment")
}
