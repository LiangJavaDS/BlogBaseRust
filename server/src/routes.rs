use crate::models::{PostProduct, Product, ProductJson};
use crate::Pool;

use actix_web::{post, web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

#[post("/add_product")]
pub async fn add_product(
    pool: web::Data<Pool>,
    item: web::Json<ProductJson>,
) -> Result<HttpResponse, Error> {
    Ok(add_single_product(pool, item)
        .await
        .map(|product| HttpResponse::Created().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn add_single_product(
    pool: web::Data<Pool>,
    item: web::Json<ProductJson>,
) -> Result<Product, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();
    match product
        .filter(name.eq(&item.name))
        .first::<Product>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_product = PostProduct {
                name: &item.name,
                title: &item.title,
                data_created: &format!("{}", chrono::Local::now().naive_local()),
            };
            insert_into(product)
                .values(&new_product)
                .execute(&db_connection)
                .expect("Errow saving new product");
            let result = product.order(id.desc()).first(&db_connection).unwrap();
            Ok(result)
        }
    }
}
