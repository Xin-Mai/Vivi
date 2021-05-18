use super::basic;
use super::db;
use crate::tool::error::ErrorMsg;
use chrono::prelude::Local;
use mongodb::bson::{doc, oid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(rename = "_id")]
    id: String,
    aid: String,
    uid: String,
    quote: String,
    content: String,
    publish_date: String,
}

#[derive(Deserialize)]
struct CommentPublishReq {
    aid: String,
    quote: String,
    content: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CommentPublishRsp {
    cid: String,
    publish_date: String,
}

impl Comment {
    fn from_publish_req(req: CommentPublishReq, uid: &String) -> Self {
        Comment {
            id: oid::ObjectId::new().to_hex(),
            aid: req.aid,
            uid: uid.clone(),
            quote: req.quote,
            content: req.content,
            publish_date: Local::now().to_rfc3339(),
        }
    }
}

pub fn publish(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: CommentPublishReq = serde_json::from_slice(&data)?;
    let comment = Comment::from_publish_req(req, &id);
    let rsp = CommentPublishRsp {
        cid: comment.id.clone(),
        publish_date: comment.publish_date.clone(),
    };
    db::comment_collection().insert_one(comment, None)?;
    basic::rsp_ok(rsp)
}

pub fn get_comments(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;
    let aid = req.id;
    let cursor = db::comment_collection().find(
        doc! {
            "aid": aid,
        },
        None,
    )?;
    let mut results: Vec<Comment> = Vec::new();
    for c in cursor {
        results.push(c?);
    }
    basic::rsp_ok(results)
}

// pub fn delete_comment(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
// let req: basic::SingleStrReq = serde_json::from_slice(&data)?;
// let cid = req.id;

// let collection = db::comment_collection();
// let set: HashSet<&String> = HashSet::new();
// set.insert(&cid);
// let arr: Vec<&String> = Vec::new();

// let options = FindOptions::builder().projection(doc! {
//     "quote": 1,
// }).build();

// loop {
//     let cid = arr.pop();

//     let cursor = collection.find(
//         doc! {
//             "quote": cid,
//         },
//         OptionWrapper(options),
//     )?;
//     for c in cursor {}
// }
// Ok(vec![])
// }
