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
    pub description: Option<String>,
    pub like_count: Option<i32>,
    pub song: Option<String>,
    pub time: DateTime<Utc>,
}

#[derive(FromForm, Debug)]
pub struct PostForm {
    pub uuid: String,
    pub description: Option<String>,
    pub song: Option<String>,
}
