use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::post().to(create_user)))
    .service(web::resource("/users/find/{name}").route(web::get().to(find_user)))
    .service(web::resource("/users/{id}").route(web::get().to(get_user)));
}

async fn create_user(
    item: web::Json<UserInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    convert(
        web::block(move || {
            let conn = &pool.get().unwrap();
            let username = item.into_inner().username;
            models::create_user(conn, username.as_str())
        })
        .await,
    )
}

async fn find_user(
    name: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    convert(
        web::block(move || {
            let conn = &pool.get().unwrap();
            let name = name.into_inner();
            let key = models::UserKey::Username(name.as_str());
            models::find_user(conn, key)
        })
        .await,
    )
}

async fn get_user(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    convert(
        web::block(move || {
            let conn = &pool.get().unwrap();
            let key = models::UserKey::ID(id.into_inner());
            models::find_user(conn, key)
        })
        .await,
    )
}
