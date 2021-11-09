use crate::prelude::*;

#[get("/getVersionByCondition")]
pub async fn get_version_by_condition(req_body: web::Json<GetVersionByConditionInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = req_body.into_inner();
    if !AVAILABLE_PLATFORM.contains(&input.platform.as_str()) {
        return error(42004, "Platform is error !");
    }
    if !AVAILABLE_CPU_ARCH.contains(&input.cpu_arch.as_str()) {
        return error(42002, "Cpu_arch is error !");
    }
    if !AVAILABLE_CHANNEL.contains(&input.channel.as_str()) {
        return error(42001, "Channel is error !");
    }
    // todo version compare
    let all_filter = version
        .filter(_version.eq(version2int(&input.update_version_code)))
        .filter(_version.le(version2int(&input.max_update_version_code)))
        .filter(_version.ge(version2int(&input.min_update_version_code)))
    // let all_filter = if input.platform == "Android".to_string() {
    //     all_filter
            .filter(max_os_api.le(input.max_os_api.unwrap()))
            .filter(min_os_api.ge(input.min_os_api.unwrap()))
    // } else { all_filter };
    // let all_filter = all_filter
        .filter(platform.eq(input.platform))
        .filter(cpu_arch.eq(input.cpu_arch))
        .filter(channel.eq(input.channel))
        .order_by(_version.desc());
    let version_do_count: i64 = all_filter
        .clone()
        .count()
        .get_result(db_conn.as_ref())
        .unwrap();
    let version_do_list: Vec<Version> = all_filter
        .offset(input.page.unwrap_or(1) as i64)
        .limit(input.page_number.unwrap_or(10) as i64)
        .load(db_conn.as_ref())
        .expect("Error loading posts");
    let result: Vec<VersionVO> = version_do_list
        .iter()
        .map(|x| VersionVO::from_version(x.clone()))
        .collect();

    HttpResponse::Ok()
        .json(GetVersionByConditionOutput {
            list: result,
            count: version_do_count as i32,
        })
}


#[get("/getVersions")]
pub async fn get_versions(req_body: web::Json<GetVersionInput>, db_pool: web::Data<MysqlConnection>) -> impl Responder {
    let input = req_body.into_inner();
    let version_vec: Vec<Version> = version
        .offset(input.page.unwrap_or(1) as i64)
        .limit(input.page_number.unwrap_or(20) as i64)
        .load(db_pool.as_ref())
        .expect("Error loading posts");
    let res: Vec<VersionVO> = version_vec
        .iter()
        .map(|x| VersionVO::from_version(x.clone()))
        .collect();
    HttpResponse::Ok()
        .json(GetVersionOutput {
            result: res
        })
}

