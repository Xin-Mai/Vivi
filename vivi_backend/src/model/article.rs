use super::basic;
use super::db;
use crate::tool::error::ErrorMsg;
use chrono::prelude::Utc;
use mongodb::{
    bson::serde_helpers::chrono_datetime_as_bson_datetime,
    bson::{doc, oid},
    options::UpdateModifications,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    #[serde(rename = "_id")]
    id: String,
    title: String,
    content: String,
    tag: String,
    uid: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    publish_date: chrono::DateTime<chrono::Utc>,
    read_num: usize,
    like_list: HashSet<String>,
}

#[derive(Deserialize)]
struct PublishReq {
    id: String,
    title: String,
    content: String,
    tag: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetArticleRsp {
    title: String,
    content: String,
    tag: String,
    uid: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    publish_date: chrono::DateTime<chrono::Utc>,
    read_num: usize,
    like_num: usize,
    like: bool,
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
            publish_date: Utc::now(),
            read_num: 0,
            like_list: HashSet::new(),
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

impl Into<GetArticleRsp> for Article {
    fn into(self) -> GetArticleRsp {
        let set = &self.like_list;
        let i_like = set.contains(&self.uid);
        GetArticleRsp {
            title: self.title,
            content: self.content,
            tag: self.tag,
            uid: self.uid,
            publish_date: self.publish_date,
            read_num: self.read_num,
            like_num: set.len(),
            like: i_like,
        }
    }
}

pub fn publish(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: PublishReq = serde_json::from_slice(&data)?;
    let aid = &req.id;
    let collection = db::article_collection();
    if aid.len() == 0 {
        let article = Article::from_publish(req, &id);
        collection.insert_one(article, None)?;
    } else {
        let res = collection.update_one(doc! {"_id": &aid}, req, None)?;
        if res.modified_count != 1 {
            return Err(ErrorMsg::unknown());
        }
    }
    Ok(vec![])
}

pub fn get_article(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;

    db::article_collection()
        .find_one_and_update(doc! {"_id": req.id}, doc! {"$inc": {"readNum": 1}}, None)?
        .map_or_else(
            || basic::rsp_err("Article not found"),
            |article| basic::rsp_ok::<GetArticleRsp>(article.into()),
        )
}

pub fn like(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;

    let collection = db::article_collection();
    collection
        .find_one(doc! {"_id": req.id}, None)?
        .map_or_else(
            || basic::rsp_err("Article not found"),
            |article| {
                let mut set = article.like_list;
                if set.contains(&id) {
                    set.remove(&id);
                } else {
                    set.insert(id);
                }
                let res = collection.update_one(
                    doc! {"_id": &article.id},
                    doc! {"$set": {
                        "like_list": set.into_iter().collect::<Vec<String>>(),
                    }},
                    None,
                )?;
                if res.modified_count != 1 {
                    Err(ErrorMsg::unknown())
                } else {
                    Ok(vec![])
                }
            },
        )
}

pub fn delete_article(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;
    let res = db::article_collection().delete_one(doc! {"_id": req.id, "uid": id}, None)?;
    if res.deleted_count != 1 {
        basic::rsp_err("Delete failed")
    } else {
        Ok(vec![])
    }
}
