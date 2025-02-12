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
    pub text: Option<String>,
    pub like_count: Option<i32>,
    pub time: DateTime<Utc>,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            post_id: 0,
            text: None,
            like_count: None,
            time: chrono::offset::Utc::now(),
        }
    }
}

#[derive(FromForm, Debug)]
pub struct PostForm {
    pub text: Option<String>,
    pub user: Option<String>,
}
