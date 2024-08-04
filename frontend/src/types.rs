use chrono;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug, Default)]
pub enum Tag {
    // TODO: fix this issue by making tags in db capitalized
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
