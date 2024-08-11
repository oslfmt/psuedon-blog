use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostFormData {
    pub title: String,
    pub content: String,
    pub date: chrono::NaiveDate,
    pub tag: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
