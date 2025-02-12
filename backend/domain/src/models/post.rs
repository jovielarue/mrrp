use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::{
    form::FromForm,
    serde::{Deserialize, Serialize},
};

#[derive(
    Queryable, Selectable, Insertable, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, Debug,
)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub post_id: i32,
    pub username: String,
    pub text: String,
    pub like_count: Option<i32>,
    pub time: DateTime<Utc>,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            post_id: 0,
            username: "Jovie".to_string(),
            text: "".to_string(),
            like_count: None,
            time: chrono::offset::Utc::now(),
        }
    }
}

#[derive(FromForm, Debug)]
pub struct PostForm {
    pub username: String,
    pub password: String,
    pub post: String,
}
