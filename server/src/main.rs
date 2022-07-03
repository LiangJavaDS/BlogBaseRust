use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

#[macro_use]
extern crate diesel;

mod jwtError;
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
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
                //  .supports_credentials(), // Allow the cookie auth.
            )
            // .app_data(web::PayloadConfig::default().limit(10000))
            // .wrap_fn(|req, srv| {
            //     use actix_web::dev::Service;
            //     let targetPath = req.path();
            //     if (targetPath.starts_with("/add_blog")) {
            //         match routes::jwt_from_header(req) {
            //             Ok(result) => srv.call(req).map(|res| res),
            //             Err(e) => unauth,
            //         }
            //     }
            //     srv.call(req).map(|res| res)
            // })
            .app_data(web::Data::new(database_pool.clone()))
            .service(routes::add_product)
            .service(routes::delete_product)
            .service(routes::update_product)
            .service(routes::get_all_product)
            .service(routes::add_blog)
            .service(routes::get_all_blog_titles)
            .service(routes::get_blog)
            .service(routes::update_blog)
            .service(routes::delete_blog)
            .service(routes::add_user)
            .service(routes::login_handler)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
