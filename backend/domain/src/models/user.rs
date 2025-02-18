use diesel::prelude::*;
use rocket::{
    form::FromForm,
    serde::{Deserialize, Serialize},
};

#[derive(
    Queryable, Selectable, Insertable, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd, Debug,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
}

#[derive(FromForm, Debug)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}
