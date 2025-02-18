// @generated automatically by Diesel CLI.

diesel::table! {
    posts (post_id) {
        post_id -> Int4,
        user_id -> Int4,
        #[max_length = 2048]
        text -> Varchar,
        like_count -> Nullable<Int4>,
        time -> Timestamptz,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 128]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
