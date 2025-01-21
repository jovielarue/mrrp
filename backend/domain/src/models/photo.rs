use crate::utils::date_format::*;
use crate::utils::date_time_utc_form::DateTimeUtcForm;
use diesel::prelude::*;
use rocket::{
    form::FromForm,
    serde::{Deserialize, Serialize},
};

#[derive(
    Queryable, Insertable, Selectable, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, Debug,
)]
#[diesel(table_name = crate::schema::photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    pub description: Option<String>,
    pub photographer: Option<String>,
    pub photo_path: String,
    #[serde(with = "date_format")]
    pub time_taken: DateTimeUtcForm,
}

#[derive(FromForm, Debug)]
pub struct PhotoForm {
    pub photo_id: i32,
    pub post_id: i32,
    pub uuid: String,
    pub description: Option<String>,
    pub photographer: Option<String>,
    pub time_taken: DateTimeUtcForm,
}
