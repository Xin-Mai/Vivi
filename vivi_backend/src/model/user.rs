use super::basic::deserialize_object_id_to_string;
use super::basic::BasicRsp;
use super::db;
use crate::tool::error::ErrorMsg;
use crate::tool::sign;
use mongodb::{
    bson::{doc, oid},
    sync::Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(deserialize_with = "deserialize_object_id_to_string")]
    id: String,
    email: String,
    username: String,
    password: String,
    intro: String,
    avatar: String,
}

#[derive(Serialize, Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

lazy_static! {
    static ref USER_TABLE: Collection<User> = db::user_collection();
}

pub fn login(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let user_req: LoginReq = serde_json::from_slice(&data)?;
    let rsp = USER_TABLE
        .find_one(
            doc! {"username": &user_req.username, "password": &user_req.password},
            None,
        )?
        .map_or_else(
            || BasicRsp::err("User not found"),
            |user| BasicRsp::ok(sign::sign(&user.username).as_bytes().to_vec()),
        );
    Ok(serde_json::to_vec(&rsp)?)
}

pub fn register(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    Ok(vec![])
}

pub fn hello_world(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let user = USER_TABLE.find_one(doc! {"_id": &oid::ObjectId::with_string(&id).unwrap()}, None)?;
    Ok(serde_json::to_vec(&user)?)
}
