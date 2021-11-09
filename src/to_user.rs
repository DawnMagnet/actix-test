use crate::prelude::*;

#[get("/checkUpdate")]
pub async fn check_update(req_body: web::Json<CheckUpdateInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = req_body.into_inner();
    let update_version_code = version2int(&input.version);
    let version_query = version
        .filter(is_release.eq(true))
        .filter(is_delete.eq(false))
        .filter(aid.eq(input.aid))
        .filter(cpu_arch.eq(cpu_arch))
        .filter(min_os_api.le(input.os_api))
        .filter(max_os_api.ge(input.os_api))
        .filter(max_version.ge(update_version_code))
        .filter(min_version.le(update_version_code))
        .filter(channel.eq(input.channel))
        .filter(platform.eq(input.device_platform))
        .order_by(_version.desc())
        .first(db_conn.as_ref());
    if let Ok(version_query_ok) = version_query {
        let result = VersionVO::from_version(version_query_ok);
        HttpResponse::Ok()
            .json(result)
    } else {
        error(40000, "Unable to find suitable version!")
    }

}

#[get("/downloadCount")]
pub async fn download_count(req_body: web::Json<DownloadCountInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = req_body.into_inner();
    let rule = version
        .filter(bid.eq(input.bid.clone()))
        .first::<Version>(db_conn.as_ref());
    if let Ok(rule_ok) = rule {
        let res = diesel::update(version.filter(bid.eq(input.bid.clone())))
            .set(install_count.eq(rule_ok.install_count.unwrap_or(0) + 1))
            .execute(db_conn.as_ref());
        if res.is_ok() {
            HttpResponse::Ok()
                .json(format!("bid:{} install_count+1", input.bid))
        } else {
            error(41001, &format!("数据库操作失败:{}", input.bid))
        }
    } else {
        error(41001, &format!("没有此bid:{}", input.bid))
    }

    // web::Json(ConfOutCome::test_data())
}