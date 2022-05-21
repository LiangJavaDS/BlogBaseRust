use crate::models::{PostProduct, Product, ProductJson, PutProductJson};
use crate::Pool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::{delete, update, RunQueryDsl};

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

#[get("/get_all_product")]
pub async fn get_all_product(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(get_all(pool)
        .await
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn get_all(pool: web::Data<Pool>) -> Result<Vec<Product>, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();
    let result = product.load::<Product>(&db_connection)?;
    Ok(result)
}

#[delete("/delete_product/{id}")]
pub async fn delete_product(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(delete_product_by_id(pool, path)
        .await
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn delete_product_by_id(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();
    let id_string = &path.0;
    let i: i32 = id_string.parse().unwrap();
    let result = delete(product.filter(id.eq(i))).execute(&db_connection)?;
    Ok(result)
}

#[put("/update_product")]
pub async fn update_product(
    pool: web::Data<Pool>,
    path: web::Json<PutProductJson>,
) -> Result<HttpResponse, Error> {
    Ok(update_product_by_id(pool, path)
        .await
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn update_product_by_id(
    pool: web::Data<Pool>,
    path: web::Json<PutProductJson>,
) -> Result<Product, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();
    let updated_product = &path.0;
    let target_id = &path.0.id;
    update(product.filter(id.eq(target_id)))
        .set((
            name.eq(&updated_product.name),
            title.eq(&updated_product.title),
        ))
        .execute(&db_connection)?;
    let result = product.filter(id.eq(&target_id)).first(&db_connection)?;
    Ok(result)
}
