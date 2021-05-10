use super::basic;
use super::basic::deserialize_object_id_to_string;
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
    #[serde(deserialize_with = "deserialize_object_id_to_string", rename = "_id")]
    id: String,
    email: String,
    username: String,
    password: String,
    intro: String,
    avatar: String,
}

// impl User {
//     fn new()
// }

#[derive(Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterReq {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginRsp {
    token: String,
    id: String,
}

lazy_static! {
    static ref USER_TABLE: Collection<User> = db::user_collection();
}

pub fn login(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let user_req: LoginReq = serde_json::from_slice(&data)?;
    USER_TABLE
        .find_one(
            doc! {"username": &user_req.username, "password": &user_req.password},
            None,
        )?
        .map_or_else(
            || basic::rsp_err("User not found"),
            |user| {
                let tkn = sign::sign(&user.id);
                let rsp = LoginRsp {
                    token: tkn,
                    id: user.id,
                };
                basic::rsp_ok(rsp)
            },
        )
}

pub fn register(_: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    Ok(vec![])
}

pub fn hello_world(_: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let oid = &oid::ObjectId::with_string(&id)?;
    let user = USER_TABLE.find_one(doc! {"_id": &oid}, None)?;
    Ok(serde_json::to_vec(&user)?)
}
