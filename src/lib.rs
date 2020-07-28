#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod models;
mod routes;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16) -> Self {
        Blog { port }
    }

    pub async fn run(&self, db_url: String) -> std::io::Result<()> {
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");
        println!("Starting HTTP Server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
                .configure(routes::posts::configure)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}
