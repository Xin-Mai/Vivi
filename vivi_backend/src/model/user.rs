use super::basic;
use super::basic::deserialize_object_id_to_string;
use super::db;
use crate::tool::error::ErrorMsg;
use crate::tool::sign;
use hyper::StatusCode;
use mongodb::{
    bson::serde_helpers::serialize_hex_string_as_object_id,
    bson::{doc, oid},
    sync::Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(
        deserialize_with = "deserialize_object_id_to_string",
        serialize_with = "serialize_hex_string_as_object_id",
        rename = "_id"
    )]
    id: String,
    email: String,
    username: String,
    password: String,
    intro: Option<String>,
    avatar: Option<String>,
}

#[derive(Deserialize)]
struct LoginReq {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterReq {
    username: String,
    password: String,
    email: String,
}

#[derive(Serialize)]
struct LoginRsp {
    token: String,
    id: String,
}

impl User {
    fn from_reg(req: RegisterReq) -> Self {
        User {
            id: oid::ObjectId::new().to_hex(),
            username: req.username,
            password: req.password,
            email: req.email,
            intro: None,
            avatar: None,
        }
    }
}

impl RegisterReq {
    fn valid(&self) -> bool {
        let lenU = self.username.len();
        let lenP = self.password.len();
        if lenU < 2 || lenU > 10 {
            false
        } else if lenP < 8 || lenP > 16 {
            false
        } else {
            true
        }
    }
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
                let id = user.id;
                let tkn = sign::sign(&id);
                let rsp = LoginRsp { token: tkn, id: id };
                basic::rsp_ok(rsp)
            },
        )
}

pub fn register(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let reg_req: RegisterReq = serde_json::from_slice(&data)?;
    // ensure data length valid
    if !reg_req.valid() {
        return Err(ErrorMsg {
            code: StatusCode::BAD_REQUEST,
            msg: "Register data not valid",
        });
    }
    // check duplicate username
    if let Some(_) = USER_TABLE.find_one(doc! {"username": &reg_req.username}, None)? {
        return Ok(basic::rsp_err("Duplicate username")?);
    }
    let user = User::from_reg(reg_req);
    let id = user.id.clone();
    let rsp = LoginRsp {
        token: sign::sign(&id),
        id: id,
    };
    USER_TABLE.insert_one(user, None)?;
    Ok(basic::rsp_ok(&rsp)?)
}

pub fn hello_world(_: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let oid = &oid::ObjectId::with_string(&id)?;
    let user = USER_TABLE.find_one(doc! {"_id": &oid}, None)?;
    Ok(serde_json::to_vec(&user)?)
}
