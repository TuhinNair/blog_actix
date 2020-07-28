use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;


#[derive(Debug, Serialize, Deserialize)]
struct CommentInput {
    user_id: i32,
    body: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/posts/{id}/comments").route(web::post().to(add_comment)).route(web::get().to(get_post_comments)))
    .service(web::resource("/users/{id}/comments").route(web::get().to(get_user_comments)));
}

async fn add_comment(post_id: web::Path<i32>, comment: web::Json<CommentInput>, pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    convert(web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        let data = comment.into_inner();
        let (user_id, body) = (data.user_id, data.body);
        models::create_comment(conn, user_id, post_id.into_inner(), body.as_str())
    }).await)
}

async fn get_post_comments(post_id: web::Path<i32>, pool:web::Data<Pool>) -> Result<HttpResponse, AppError> {
    convert(web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::post_comments(conn, post_id.into_inner())
    }).await)
}

async fn get_user_comments(user_id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    convert(web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::user_comments(conn, user_id.into_inner())
    }).await)
}