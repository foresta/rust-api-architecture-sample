mod handlers;
mod request;
mod response;

use crate::domains::documents::DocumentRepository;
use actix_web::{App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(RequestContext::new())
            .service(handlers::hello)
            .service(handlers::get_documents)
            .service(handlers::get_document)
            .service(handlers::post_document)
            .service(handlers::delete_document)
            .service(handlers::update_document)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }

    pub fn document_repository(&self) -> impl DocumentRepository {
        use crate::infrastructures::repository::documents::DocumentRepositoryImpl;

        DocumentRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
