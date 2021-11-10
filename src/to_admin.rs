use chrono::{Local};
use crate::prelude::*;

#[get("/getVersionByCondition")]
pub async fn get_version_by_condition(json: web::Json<GetVersionByConditionInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = json.into_inner();
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
pub async fn get_versions(json: web::Json<GetVersionInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = json.into_inner();
    let version_vec: Vec<Version> = version
        .offset(input.page.unwrap_or(1) as i64)
        .limit(input.page_number.unwrap_or(20) as i64)
        .load(db_conn.as_ref())
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


#[post("/open")]
pub async fn rules_open(form: web::Form<RulesOpenInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = form.into_inner();
    match query_release_state_by_bid(&input.bid, db_conn.as_ref()) {
        Ok(false) =>
            match update_release_state_by_bid(&input.bid, true, db_conn.as_ref()) {
                true => {
                    create_record_version_message(&VersionMessage{
                        bid: "".to_string(),
                        version_bid: "".to_string(),
                        user_bid: "".to_string(),
                        operation: 0,
                        content: "".to_string(),
                        create_time: Local::now().naive_local()
                    }, db_conn.as_ref());
                    HttpResponse::Ok()
                        .json("good")
                },
                false => error(66666, "数据库操作失误！")
            },
        Ok(true) => error(66666, "该规则已上线！"),
        Err(_) => error(66666, "未找到此条记录"),
    }
}

#[post("/close")]
pub async fn rules_close(form: web::Json<RulesCloseInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = form.into_inner();
    match query_release_state_by_bid(&input.bid, db_conn.as_ref()) {
        Ok(true) =>
            match update_release_state_by_bid(&input.bid, false, db_conn.as_ref()) {
                true => HttpResponse::Ok().json(""),
                false => error(66666, "数据库操作失误！")
            },
        Ok(false) => error(66666, "该规则未上线或者已下线！"),
        Err(_) => error(66666, "未找到此条记录"),
    }
}

#[post("/delete")]
pub async fn rules_delete(form: web::Json<RulesDeleteInput>, db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let input = form.into_inner();
    match query_release_state_by_bid(&input.bid, db_conn.as_ref()) {
        Ok(false) =>
            match update_delete_state_by_bid(&input.bid, db_conn.as_ref()) {
                true => HttpResponse::Ok().json(""),
                false => error(66666, "数据库操作失误！")
            },
        Ok(true) => error(66666, "请将规则下线后再进行删除！"),
        Err(_) => error(66666, "未找到此条记录"),
    }
}

#[post("/add")]
pub async fn rules_add(form: web::Json<GetVersionInput>, _db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let _input = form.into_inner();
    HttpResponse::Ok().json("")
}

#[post("/update")]
pub async fn rules_update(form: web::Json<GetVersionInput>, _db_conn: web::Data<MysqlConnection>) -> impl Responder {
    let _input = form.into_inner();
    HttpResponse::Ok().json("")
}
