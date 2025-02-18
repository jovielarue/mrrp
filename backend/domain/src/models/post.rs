use crate::models::user::User;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::{
    form::FromForm,
    serde::{Deserialize, Serialize},
};

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Ord,
    Eq,
    PartialEq,
    PartialOrd,
    Debug,
    Associations,
    Clone,
)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(User))]
pub struct Post {
    pub post_id: i32,
    pub user_id: i32,
    pub text: String,
    pub like_count: Option<i32>,
    pub time: DateTime<Utc>,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            post_id: 0,
            user_id: 0,
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

#[derive(Debug, Serialize)]
pub struct PostReturn {
    pub post: Post,
    pub username: String,
}
