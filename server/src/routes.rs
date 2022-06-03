use crate::models::{
    Blog, BlogCatalogue, BlogJson, PostBlog, PostProduct, PostUser, Product, ProductJson,
    PutBlogJson, PutProductJson, PutUserJson, User, UserJson,
};
use crate::Pool;

use actix_web::{delete, error, get, post, put, web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::{delete, update, RunQueryDsl};
use uuid::Uuid;

#[post("/add_product")]
pub async fn add_product(
    pool: web::Data<Pool>,
    item: web::Json<ProductJson>,
) -> Result<HttpResponse, Error> {
    Ok(add_single_product(pool, item)
        .await
        .map(|product| HttpResponse::Created().json(product))
        .map_err(|e| error::ErrorBadRequest(e))?)
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
                .expect("Error saving new product");
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
        .map_err(|e| error::ErrorBadRequest(e))?)
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
        .map_err(|e| error::ErrorBadRequest(e))?)
}

async fn delete_product_by_id(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();
    let id_string = &path.into_inner();
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
        .map_err(|e| error::ErrorBadRequest(e))?)
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

/** 新增博客 */
#[post("add_blog")]
pub async fn add_blog(
    pool: web::Data<Pool>,
    item: web::Json<BlogJson>,
) -> Result<HttpResponse, Error> {
    Ok(add_single_blog(pool, item)
        .await
        .map(|product| HttpResponse::Created().json(product))
        .map_err(|e| error::ErrorBadRequest(e))?)
}

async fn add_single_blog(
    pool: web::Data<Pool>,
    item: web::Json<BlogJson>,
) -> Result<Blog, diesel::result::Error> {
    use crate::schema::blogs::dsl::*;
    let db_connection = pool.get().unwrap();
    match blogs
        .filter(title.eq(&item.title))
        .first::<Blog>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_blog = PostBlog {
                id: &Uuid::new_v4().as_hyphenated().to_string(),
                user_id: "fakeUserId",
                title: &item.title,
                content: &item.content,
                tag: &item.tag,
                created_at: &format!("{}", chrono::Local::now().naive_local()),
                updated_at: &format!("{}", chrono::Local::now().naive_local()),
            };
            insert_into(blogs)
                .values(&new_blog)
                .execute(&db_connection)
                .expect("Error saving new blog");
            let result = blogs.order(id.desc()).first(&db_connection).unwrap();
            Ok(result)
        }
    }
}

/** 获取所有博客标题 */
#[get("/get_all_blog_titles")]
pub async fn get_all_blog_titles(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(get_blog_titles(pool)
        .await
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| error::ErrorBadRequest(e))?)
}

async fn get_blog_titles(
    pool: web::Data<Pool>,
) -> Result<Vec<BlogCatalogue>, diesel::result::Error> {
    use crate::schema::blogs::dsl::*;
    let db_connection = pool.get().unwrap();
    let all_blogs = blogs
        .order(created_at.desc())
        .load::<Blog>(&db_connection)?;
    let result = all_blogs
        .into_iter()
        .map(|b| BlogCatalogue {
            id: b.id,
            title: b.title,
            tag: b.tag,
            created_at: b.created_at,
            updated_at: b.updated_at,
        })
        .collect();
    Ok(result)
}

/** 获取一篇博客 */
#[get("/get_blog/{id}")]
pub async fn get_blog(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(get_blog_by_id(pool, path)
        .await
        .map(|blog| HttpResponse::Ok().json(blog))
        .map_err(|e| error::ErrorBadRequest(e))?)
}

async fn get_blog_by_id(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<Blog, diesel::result::Error> {
    use crate::schema::blogs::dsl::*;
    let db_connection = pool.get().unwrap();
    let id_string = &path.into_inner();
    println!("id={}", id_string);
    let result = blogs.filter(id.eq(id_string)).first(&db_connection)?;
    Ok(result)
}

/** 更新博客 */
#[put("/update_blog")]
pub async fn update_blog(
    pool: web::Data<Pool>,
    path: web::Json<PutBlogJson>,
) -> Result<HttpResponse, Error> {
    Ok(update_blog_by_id(pool, path)
        .await
        .map(|blog| HttpResponse::Ok().json(blog))
        .map_err(|e| error::ErrorBadRequest(e))?)
}

async fn update_blog_by_id(
    pool: web::Data<Pool>,
    path: web::Json<PutBlogJson>,
) -> Result<Blog, diesel::result::Error> {
    use crate::schema::blogs::dsl::*;
    let db_connection = pool.get().unwrap();
    let updated_blog = &path.0;
    update(blogs.filter(id.eq(&updated_blog.id)))
        .set((
            title.eq(&updated_blog.title),
            content.eq(&updated_blog.content),
            tag.eq(&updated_blog.tag),
        ))
        .execute(&db_connection)?;
    let result = blogs
        .filter(id.eq(&updated_blog.id))
        .first(&db_connection)?;
    Ok(result)
}

/** 删除博客 */
#[delete("delete_blog/{id}")]
pub async fn delete_blog(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(delete_blog_by_id(pool, path)
        .await
        .map(|blog| HttpResponse::Ok().json(blog))
        .map_err(|e| error::ErrorBadRequest(e))?)
}

async fn delete_blog_by_id(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::blogs::dsl::*;
    let db_connection = pool.get().unwrap();
    let id_string = &path.into_inner();
    let result = delete(blogs.filter(id.eq(id_string))).execute(&db_connection)?;
    Ok(result)
}

#[post("/add_user")]
pub async fn add_user(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>,
) -> Result<HttpResponse, Error> {
    Ok(add_single_user(pool, item)
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|e| error::ErrorBadRequest(e))?)
}

pub async fn add_single_user(
    pool: web::Data<Pool>,
    item: web::Json<UserJson>,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    match users
        .filter(username.eq(&item.username))
        .first::<User>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_user = PostUser {
                id: &Uuid::new_v4().as_hyphenated().to_string(),
                username: &item.username,
                password: &item.password,
                email: &item.email,
                // phone:&item.phone,
                // avatar:&item.avatar,
                // avatar_url:&item.avatar_url,
                // slogan:&item.slogan,
                is_deleted: &false,
                created_at: &format!("{}", chrono::Local::now().naive_local()),
                updated_at: &format!("{}", chrono::Local::now().naive_local()),
            };
            insert_into(users)
                .values(&new_user)
                .execute(&db_connection)
                .expect("Error saving new user");
            let result = users
                .order(created_at.desc())
                .first(&db_connection)
                .unwrap();
            Ok(result)
            // if result {
            //     Ok("保存成功")
            // }
            // Err("保存失败")
        }
    }
}

#[put("/update_user")]
pub async fn update_user(
    pool: web::Data<Pool>,
    item: web::Json<PutUserJson>,
) -> Result<HttpResponse, Error> {
    Ok(update_user_by_id(pool, item)
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| error::ErrorBadRequest(e))?)
}

async fn update_user_by_id(
    pool: web::Data<Pool>,
    path: web::Json<PutUserJson>,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    let updated_user = &path.0;
    update(users.filter(id.eq(&updated_user.id)))
        .set((
            password.eq(&updated_user.password),
            email.eq(&updated_user.email),
        ))
        .execute(&db_connection)?;
    let result = users
        .filter(id.eq(&updated_user.id))
        .first(&db_connection)?;
    Ok(result)
}
