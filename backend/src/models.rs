use diesel::{data_types::PgTimestamp, prelude::*};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::photo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Photo {
    photo_id: i32,
    post_id: i32,
    description: Option<String>,
    photographer: Option<String>,
    photo_path: String,
    time_taken: PgTimestamp,
}

pub struct Post {
    post_id: i32,
    description: Option<String>,
    like_count: Option<i32>,
    song: Option<String>,
}
