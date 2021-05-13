use super::basic;
use super::db;
use crate::tool::error::ErrorMsg;
use crate::tool::sign;
use hyper::StatusCode;
use mongodb::{
    bson::{doc, oid},
    options::UpdateModifications,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    id: String,
    email: String,
    username: String,
    password: String,
    intro: String,
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

#[derive(Deserialize)]
struct UserInfoUpdateReq {
    username: String,
    intro: String,
    email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserPwdUpdateRed {
    old_pwd: String,
    new_pwd: String,
}

#[derive(Serialize)]
struct FindUserRsp {
    username: String,
    email: String,
    intro: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserInfoRsp {
    read_num: i64,
    like_num: i64,
    count: i64,
}

impl User {
    fn from_reg(req: RegisterReq) -> Self {
        User {
            id: oid::ObjectId::new().to_hex(),
            username: req.username,
            password: req.password,
            email: req.email,
            intro: "".to_string(),
        }
    }
}

impl RegisterReq {
    fn valid(&self) -> bool {
        let len_user = self.username.len();
        let len_pwd = self.password.len();
        if len_user < 2 || len_user > 10 {
            false
        } else if len_pwd < 8 || len_pwd > 16 {
            false
        } else {
            true
        }
    }
}

impl Into<UpdateModifications> for UserInfoUpdateReq {
    fn into(self) -> UpdateModifications {
        UpdateModifications::Document(doc! { "$set": {
            "username": self.username,
            "email": self.email,
            "intro": self.intro,
        }})
    }
}

impl From<User> for FindUserRsp {
    fn from(user: User) -> FindUserRsp {
        FindUserRsp {
            username: user.username,
            email: user.email,
            intro: user.intro,
        }
    }
}

const MAX_AVATAR_LEN: usize = 1_000_000;

pub fn login(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let user_req: LoginReq = serde_json::from_slice(&data)?;
    db::user_collection()
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
    let collection = db::user_collection();
    // check duplicate username
    if let Some(_) = collection.find_one(doc! {"username": &reg_req.username}, None)? {
        return Ok(basic::rsp_err("Duplicate username")?);
    }
    let user = User::from_reg(reg_req);
    let id = user.id.clone();
    let rsp = LoginRsp {
        token: sign::sign(&id),
        id: id,
    };
    collection.insert_one(user, None)?;
    Ok(basic::rsp_ok(&rsp)?)
}

pub fn find_user(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;
    let uid = req.id;
    db::user_collection()
        .find_one(doc! {"_id": uid}, None)?
        .map_or_else(
            || basic::rsp_err("User not found"),
            |user| basic::rsp_ok(FindUserRsp::from(user)),
        )
}

pub fn user_info(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;
    let uid = req.id;
    let pipeline = vec![
        doc! {"$match": {"uid": &uid}},
        doc! {"$group": { "_id": "$uid",
             "reads": { "$sum": "$readNum" },
            "likes": { "$sum": { "$size": "$likeList"} },
            "counts": { "$sum": 1 },
        }},
    ];
    let cursor = db::article_collection().aggregate(pipeline, None)?;
    let mut like_num = 0;
    let mut read_num = 0;
    let mut count = 0;
    for c in cursor {
        let _ = c.map(|c| {
            count += c.get_i32("counts").unwrap_or_default() as i64;
            like_num += c.get_i32("likes").unwrap_or_default() as i64;
            read_num += c.get_i64("reads").unwrap_or_default();
        });
    }
    basic::rsp_ok(UserInfoRsp {
        like_num: like_num,
        read_num: read_num,
        count: count,
    })
}

pub fn update_user_info(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: UserInfoUpdateReq = serde_json::from_slice(&data)?;
    let res = db::user_collection().update_one(doc! {"_id": id}, req, None)?;
    if res.modified_count == 1 {
        basic::rsp_ok("")
    } else {
        basic::rsp_err("User not found")
    }
}

pub fn update_user_pwd(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    let req: UserPwdUpdateRed = serde_json::from_slice(&data)?;
    let res = db::user_collection().update_one(
        doc! {"_id": id, "password": req.old_pwd},
        doc! {"$set": {"password": req.new_pwd} },
        None,
    )?;
    if res.modified_count == 1 {
        basic::rsp_ok("")
    } else {
        basic::rsp_err("Username & password not match!")
    }
}

pub fn update_user_avatar(data: Vec<u8>, id: String) -> Result<Vec<u8>, ErrorMsg> {
    if data.len() > MAX_AVATAR_LEN {
        return Err(ErrorMsg {
            code: StatusCode::BAD_REQUEST,
            msg: "Avatar too big",
        });
    }
    let path = format!("/root/avatar/{}", id);
    std::fs::write(path, data)?;
    basic::rsp_ok("")
}

pub fn download_avatar(data: Vec<u8>) -> Result<Vec<u8>, ErrorMsg> {
    let req: basic::SingleStrReq = serde_json::from_slice(&data)?;
    let path = format!("/root/avatar/{}", req.id);
    if std::fs::metadata(&path).is_err() {
        basic::rsp_err("Avatar not exist")
    } else {
        Ok(std::fs::read(path)?)
    }
}
