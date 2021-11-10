#[macro_use]
extern crate diesel;
pub mod curd;
pub mod schema;
pub mod schema2;
pub mod models;
pub mod to_user;
pub mod to_admin;
pub mod dataio;

pub mod prelude {

    pub use actix_web::{get, post, web, App, HttpRequest, HttpServer, HttpResponse, Responder};
    pub use diesel::{Connection, MysqlConnection};
    pub use crate::{curd, schema, schema2, models, to_user, to_admin};
    pub use anyhow::Result;
    pub use crate::curd::*;
    pub use crate::models::*;
    pub use crate::dataio::*;
    pub use crate::schema::*;
    pub use crate::schema2::*;
    pub use crate::schema::version::dsl::*;
    pub use diesel::prelude::*;

    pub fn establish_connection(database_url: &str) -> MysqlConnection {
        MysqlConnection::establish(database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
    pub const AVAILABLE_PLATFORM: [&str; 2] = ["Android", "IOS"];
    pub const AVAILABLE_CPU_ARCH: [&str; 4] = ["arm64", "x86", "x64", "aarch64"];
    pub const AVAILABLE_CHANNEL: [&str; 2] = ["AppStore", "GooglePlay"];
    pub fn version2int(__version: &str) -> i32 {
        let mut res = 0;
        let mut factor = 1000000;
        for item in __version.split('.') {
            if let Ok(parse) = item.parse::<i32>() {
                res += factor * parse;
                factor /= 100;
            }
        }
        res
    }
    pub fn int2version(mut __int: i32) -> String {
        let mut vt = vec![];
        while __int > 0 {
            vt.push(__int % 100);
            __int /= 100;
        }
        vt.reverse();
        let mut res = String::new();
        for (index, item) in vt.iter().enumerate() {
            res += &*if index > 0 {
                format!(".{}", item)
            } else {
                item.to_string()
            };
        }
        res
    }
    pub fn error(code: i32, message: &str) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(ErrorInfo{
                status: "ERROR!".to_string(),
                code,
                message: message.to_string()
            })
    }


}
