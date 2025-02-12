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
    pub post_id: i32,
    pub photo_id: i32,
    pub photo_path: String,
    pub description: Option<String>,
    pub photographer: Option<String>,
}

#[derive(FromForm, Debug)]
pub struct PhotoForm {
    pub photo_id: i32,
    pub post_id: i32,
    pub uuid: String,
    pub description: Option<String>,
    pub photographer: Option<String>,
}
