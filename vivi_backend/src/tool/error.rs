use hyper::StatusCode;
pub struct ErrorMsg {
    pub code: StatusCode,
    pub msg: &'static str,
}

impl ErrorMsg {
    pub fn unknown() -> Self {
        ErrorMsg {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: "Unknown error!",
        }
    }
}

impl std::convert::From<serde_json::Error> for ErrorMsg {
    fn from(error: serde_json::Error) -> ErrorMsg {
        println!(
            "Error {:?} occur while serialize/deserialize",
            error.classify()
        );
        if error.is_io() {
            ErrorMsg {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                msg: "Could not parse body",
            }
        } else {
            ErrorMsg {
                code: StatusCode::BAD_REQUEST,
                msg: "Could not parse body",
            }
        }
    }
}

impl std::convert::From<mongodb::error::Error> for ErrorMsg {
    fn from(error: mongodb::error::Error) -> ErrorMsg {
        println!("Error {:?} occur while processing database", error);
        ErrorMsg {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: "Database error!",
        }
    }
}

impl std::convert::From<mongodb::bson::oid::Error> for ErrorMsg {
    fn from(error: mongodb::bson::oid::Error) -> ErrorMsg {
        println!("Error {:?} occur while translate string to ObjectId", error);
        ErrorMsg {
            code: StatusCode::BAD_REQUEST,
            msg: "ObjectId error!",
        }
    }
}

impl std::convert::From<std::io::Error> for ErrorMsg {
    fn from(error: std::io::Error) -> ErrorMsg {
        println!("Error {:?} occur in an io operation", error);
        ErrorMsg {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: "IO error!",
        }
    }
}
