use crate::db;
use crate::ErrorMsg;
use serde::{Deserialize, Serialize};
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    username: String,
    password: String,
    intro: String,
    avatar: String,
}

#[derive(Serialize, Deserialize)]
struct UserReq {
    username: String,
    password: String,
}

pub fn login(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let user_req: UserReq = serde_json::from_slice(&data)?;
    let user = db::find_user(&user_req.username, &user_req.password)?;
    Ok(serde_json::to_vec(&user)?)
}

pub fn register(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    Ok(vec![])
}

pub fn hello_world(data: Vec<u8>, token: String) -> Result<Vec<u8>, ErrorMsg> {
    Ok("you success!".as_bytes().to_vec())
}