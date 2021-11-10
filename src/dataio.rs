use crate::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct GetVersionByConditionInput {
    pub platform: String,
    // 选择平台(android/ios)升级
    pub update_version_code: String,
    // 选择制定版本进行升级（有的话将最大版本号最小版本号同时设置为此值）
    pub max_update_version_code: String,
    // 选择可升级的最大版本号进行升级（默认所有中的最大值）
    pub min_update_version_code: String,
    // 选择可升级的最小版本号进行升级（默认所有中的最小值）
    pub max_os_api: Option<i32>,
    // 选择支持的最大操作系统版本进行升级（默认所有中的最大值）
    pub min_os_api: Option<i32>,
    // 选择支持的最小操作系统版本进行升级（默认所有中的最大值）
    pub cpu_arch: String,
    // 选择CPU架构进行升级
    pub channel: String,
    // 选择的渠道
    pub page: Option<i32>,
    // 选择第几页
    pub page_number: Option<i32>,  // 选择每页的版本信息数量
}

#[derive(Serialize)]
pub struct GetVersionByConditionOutput {
    pub list: Vec<VersionVO>,
    pub count: i32,
}

#[derive(Deserialize)]
pub struct GetVersionInput {
    pub page: Option<i32>,
    pub page_number: Option<i32>,
}

#[derive(Serialize)]
pub struct GetVersionOutput {
    pub result: Vec<VersionVO>,
}

#[derive(Deserialize)]
pub struct CheckUpdateInput {
    pub version: String,
    // 版本（未使用）
    pub device_platform: String,
    // 平台(android/ios)
    pub device_id: String,
    // 设备id
    pub os_api: i32,
    // 系统版本【安卓系统下】
    pub channel: String,
    // 不同应用商店
    pub aid: i32,
    // app的唯一标识
    pub cpu_arch: String,  // CPU架构
}
#[derive(Deserialize)]
pub struct DownloadCountInput {
    pub bid: String
}

#[derive(Deserialize)]
pub struct RulesOpenInput {
    pub bid: String
}

pub type RulesDeleteInput = RulesOpenInput;
pub type RulesCloseInput = RulesOpenInput;
