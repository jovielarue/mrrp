use crate::utils::date_format::*;
use crate::utils::date_time_utc_form::DateTimeUtcForm;
use diesel::{expression::AsExpression, prelude::*};
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
    #[serde(with = "date_format")]
    pub time_taken: DateTimeUtcForm,
}

#[derive(FromForm, Debug)]
pub struct PostForm {
    pub uuid: String,
    pub description: Option<String>,
    pub song: Option<String>,
}
