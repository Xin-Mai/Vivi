use super::basic;
use super::basic::deserialize_object_id_to_string;
use super::db;
use crate::tool::error::ErrorMsg;
use chrono::prelude::Utc;
use mongodb::{
    bson::serde_helpers::{chrono_datetime_as_bson_datetime, serialize_hex_string_as_object_id},
    bson::{doc, oid},
    options::UpdateModifications,
    sync::Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    #[serde(
        deserialize_with = "deserialize_object_id_to_string",
        serialize_with = "serialize_hex_string_as_object_id",
        rename = "_id"
    )]
    id: String,
    title: String,
    content: String,
    tag: String,
    uid: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    date: chrono::DateTime<chrono::Utc>,
    read_num: i32,
    like_num: i32,
}

#[derive(Deserialize)]
struct PublishReq {
    id: String,
    title: String,
    content: String,
    tag: String,
}

impl Article {
    fn from_publish(req: PublishReq, uid: &String) -> Self {
        let mut id = req.id;
        if id.len() == 0 {
            id = oid::ObjectId::new().to_hex();
        }
        Article {
            id: id,
            title: req.title,
            content: req.content,
            tag: req.tag,
            uid: uid.clone(),
            date: Utc::now(),
            read_num: 0,
            like_num: 0,
        }
    }
}

impl Into<UpdateModifications> for PublishReq {
    fn into(self) -> UpdateModifications {
        UpdateModifications::Document(doc! { "$set": {
            "title": self.title,
            "content": self.content,
            "tag": self.tag,
            "date": Utc::now(),
        }})
    }
}

lazy_static! {
    static ref ARTICLE_TABLE: Collection<Article> = db::article_collection();
}

pub fn publish(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: PublishReq = serde_json::from_slice(&data)?;
    let aid = &req.id;
    if aid.len() == 0 {
        let article = Article::from_publish(req, &id);
        ARTICLE_TABLE.insert_one(article, None)?;
    } else {
        let res = ARTICLE_TABLE.update_one(doc! {"_id": &aid}, req, None)?;
        if res.modified_count != 1 {
            return Err(ErrorMsg::unknown());
        }
    }
    Ok(vec![])
}

pub fn get_article(data: Vec<u8>, _: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;
    let oid = oid::ObjectId::with_string(&req.id)?;

    ARTICLE_TABLE
        .find_one_and_update(doc! {"_id": oid}, doc! {"$inc": {"read_num": 1}}, None)?
        .map_or_else(
            || basic::rsp_err("Article not found"),
            |article| basic::rsp_ok(article),
        )
}
