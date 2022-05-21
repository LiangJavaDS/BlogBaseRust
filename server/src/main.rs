use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // 读取.env文件变量
    let database_url = std::env::var("DATABASE_URL").expect("can not find dataBase");
    // 连接池
    let database_pool = r2d2::Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Failed to create pool.");
    // 请求
    HttpServer::new(move || {
        App::new()
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("http://localhost:8000")
            //         .al lowed_methods(vec!["GET", "POST", "DELETE"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .max_age(3600), // .supports_credentials(), // Allow the cookie auth.
            // )
            .app_data(web::Data::new(database_pool.clone()))
            .service(routes::add_product)
            .service(routes::delete_product)
            .service(routes::update_product)
            .service(routes::get_all_product)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
