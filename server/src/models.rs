use crate::schema::*;
// 序列化、反序列化
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub data_created: String,
}

#[derive(Debug, Insertable)]
#[table_name = "product"]
pub struct PostProduct<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub data_created: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PutProductJson {
    pub id: i32,
    pub name: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductJson {
    pub name: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogJson {
    pub title: String,
    pub content: String,
    pub tag: String,
}

#[derive(Debug, Insertable)]
#[table_name = "blogs"]
pub struct PostBlog<'a> {
    pub id: &'a str,
    pub user_id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub tag: &'a str,
    pub created_at: &'a str,
    pub updated_at: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Blog {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub tag: Option<String>,
    pub image: Option<Vec<u8>>,
    pub image_url: Option<String>,
    pub likes: Option<i32>,
    pub page_view_num: Option<i32>,
    pub commnet_id: Option<String>,
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct BlogCatalogue {
    pub id: String,
    pub title: String,
    pub tag: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PutBlogJson {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tag: String,
}
