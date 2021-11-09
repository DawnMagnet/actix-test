use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Clone)]
pub struct Version {
    pub id: i32,
    pub bid: String,
    pub version: i32,
    pub md5: Option<String>,
    pub download_url : Option<String>,
    pub max_os_api : i32,
    pub min_os_api : i32,
    pub max_version : i32,
    pub min_version : i32,
    pub cpu_arch : String,
    pub channel : Option<String>,
    pub platform : Option<String>,
    pub pop_count : Option<i32>,
    pub install_count : Option<i32>,
    pub aid : Option<i32>,
    // 存疑
    pub create_time : NaiveDateTime,
    pub delete_time : Option<NaiveDateTime>,
    pub is_delete : Option<bool>,
    pub is_release : Option<bool>,
}

#[derive(Queryable)]
pub struct VersionMessage {

}

use serde::{Deserialize, Serialize};
use crate::prelude::int2version;

#[derive(Deserialize, Debug)]
pub struct ConfInCome {

}

#[derive(Serialize)]
pub struct ConfOutCome {

}

impl ConfOutCome {
    pub fn test_data() -> Self {
        ConfOutCome {
            // platform: "Android".to_string(),
            // download_url: "www.baidu.com".to_string(),
            // update_version_code: "6.1.2".to_string(),
            // md5: "expand".to_string(),
            // device_id_list: "[1,2,3]".to_string(),
            // max_update_version_code: "6.1.4".to_string(),
            // min_update_version_code: "6.1.0".to_string(),
            // max_os_api: 6,
            // min_os_api: 14,
            // cpu_arch: "aarch64".to_string(),
            // channel: "unknown".to_string(),
            // title: "good".to_string(),
            // update_tips: "none".to_string(),
        }
    }
}
#[derive(Serialize)]
pub struct ErrorInfo {
    pub status: String,
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Default)]
pub struct VersionVO {
    pub bid: String,
    pub version: String,
    pub download_url: String,
    pub max_os_api: String,
    pub min_os_api: String,
    pub max_version: String,
    pub min_version: String,
    pub cpu_arch: String,
    pub channel: String,
    pub platform: String,
    pub pop_count: String,
    pub install_count: String,
    pub aid: String,
    pub create_time: String,
    pub delete_time: String,
    pub is_delete: String,
    pub is_release: String,
}

impl VersionVO {
    pub fn from_version(i: Version) -> Self {
        let mut one_vo = VersionVO::default();
        one_vo.version = int2version(i.version);
        one_vo.bid = i.bid.to_string();
        one_vo.download_url = i.download_url.unwrap_or(String::new());
        one_vo.max_version = int2version(i.max_version);
        one_vo.min_version = int2version(i.min_version);
        one_vo.is_delete = i.is_delete.unwrap_or(false).to_string();
        one_vo.is_release = i.is_release.unwrap_or(false).to_string();
        one_vo.platform = i.platform.unwrap_or(String::new());
        one_vo.channel = i.channel.unwrap_or(String::new());
        one_vo.max_os_api = i.max_os_api.to_string();
        one_vo.min_os_api = i.min_os_api.to_string();
        one_vo.cpu_arch = i.cpu_arch.to_string();
        one_vo.aid = i.aid.unwrap_or(0).to_string();
        one_vo.pop_count = i.pop_count.unwrap_or(0).to_string();
        one_vo.install_count = i.install_count.unwrap_or(0).to_string();
        one_vo.create_time = i.create_time.to_string();
        one_vo.delete_time = if let Some(t) = i.delete_time {
            t.to_string()
        } else {
            String::new()
        };
        one_vo
    }
}


