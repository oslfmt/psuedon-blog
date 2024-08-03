use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormPostData {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub tag: String,
}
