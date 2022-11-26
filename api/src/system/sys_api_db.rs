use app_service::system;
use axum::{extract::Query, Json};
use db::{
    common::res::Res,
    db_conn,
    system::{
        models::sys_api_db::{SysApiDbAddEditReq, SysApiDbSearchReq},
        prelude::SysApiDbModel,
    },
    DB,
};

#[utoipa::path(
    post,
    path = "/system/api_db/add",
    tag = "SysApiDb",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增api与数据库表对应关系", body = String)
    ),
    request_body = SysApiDbAddEditReq,
)]
/// 新增api与数据库表对应关系
pub async fn add(Json(req): Json<SysApiDbAddEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_api_db::add(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/api_db/get_by_id",
    tag = "SysApiDb",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取对应关系", body = [SysApiDbModel])
    ),
    params(
        ("params" = SysApiDbSearchReq, Query, description = "查询参数")
    ),
)]
/// 按id获取对应关系
pub async fn get_by_id(Query(req): Query<SysApiDbSearchReq>) -> Res<Vec<SysApiDbModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_api_db::get_by_id(db, &req.api_id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}