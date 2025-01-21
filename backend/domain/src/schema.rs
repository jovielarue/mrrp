// @generated automatically by Diesel CLI.

diesel::table! {
    photos (photo_id) {
        photo_id -> Int4,
        post_id -> Int4,
        #[max_length = 128]
        description -> Nullable<Varchar>,
        #[max_length = 128]
        photographer -> Nullable<Varchar>,
        #[max_length = 128]
        photo_path -> Varchar,
        time_taken -> Timestamptz,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Int4,
        #[max_length = 2048]
        description -> Nullable<Varchar>,
        like_count -> Nullable<Int4>,
        #[max_length = 128]
        song -> Nullable<Varchar>,
        time_taken -> Timestamptz,
    }
}

diesel::joinable!(photos -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(photos, posts,);
