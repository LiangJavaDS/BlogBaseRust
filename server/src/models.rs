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
