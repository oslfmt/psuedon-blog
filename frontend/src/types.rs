use chrono;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Debug, Default)]
pub enum Tag {
    Blockchain,
    Philosophy,
    #[default]
    Random,
}

#[derive(Clone, PartialEq, Deserialize, Debug, Default)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub tag: Tag,
}

#[derive(Clone, PartialEq, Serialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
