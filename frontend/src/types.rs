use chrono;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug, Default)]
pub enum Tag {
    // TODO: fix this issue by making tags in db capitalized
    #[allow(non_camel_case_types)]
    blockchain,
    #[allow(non_camel_case_types)]
    philosophy,
    #[allow(non_camel_case_types)]
    #[default]
    random,
}

#[derive(Clone, PartialEq, Deserialize, Debug, Default)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub date: chrono::NaiveDate,
    pub tag: Tag,
}
