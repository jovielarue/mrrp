use diesel::{data_types::PgTimestamp, prelude::*};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::photo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    pub photo_id: i32,
    pub post_id: i32,
    pub description: Option<String>,
    pub photographer: Option<String>,
    pub photo_path: String,
    pub time_taken: PgTimestamp,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::post)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub post_id: i32,
    pub description: Option<String>,
    pub like_count: Option<i32>,
    pub song: Option<String>,
}
