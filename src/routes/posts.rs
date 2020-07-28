use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

#[derive(Deserialize, Debug, Serialize)]
struct PostInput {
    title: String,
    body: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/posts")
            .route(web::post().to(add_post))
            .route(web::get().to(get_user_posts)),
    )
    .service(web::resource("/posts/{id}/publish").route(web::post().to(publish_post)))
    .service(web::resource("/posts").route(web::get().to(get_all_posts)));
}

async fn add_post(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    convert(
        web::block(move || {
            let conn: &SqliteConnection = &pool.get().unwrap();
            let key = models::UserKey::ID(user_id.into_inner());
            models::find_user(conn, key).and_then(|user| {
                let post = post.into_inner();
                let title = post.title;
                let body = post.body;
                models::create_post(conn, &user, title.as_str(), body.as_str())
            })
        })
        .await,
    )
}

async fn publish_post(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    convert(
        web::block(move || {
            let conn: &SqliteConnection = &pool.get().unwrap();
            models::publish_post(conn, post_id.into_inner())
        })
        .await,
    )
}

async fn get_all_posts(pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    convert(
        web::block(move || {
            let conn: &SqliteConnection = &pool.get().unwrap();
            models::all_posts(conn)
        })
        .await,
    )
}

async fn get_user_posts(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    convert(
        web::block(move || {
            let conn: &SqliteConnection = &pool.get().unwrap();
            models::user_posts(conn, user_id.into_inner())
        })
        .await,
    )
}
