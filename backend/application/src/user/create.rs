use diesel::{dsl::insert_into, RunQueryDsl};
use domain::{models::user::User, schema::users};
use infrastructure::establish_connection;

pub fn create_user(input_username: &str, input_password: String) -> i32 {
    let user: User = User {
        username: input_username.to_string(),
        password: input_password.to_string(),
        user_id: 0,
    };
    let user_id = match insert_into(users::table)
        .values(&user)
        .get_result::<User>(&mut establish_connection())
    {
        Ok(u) => u.user_id,
        _ => -1,
    };

    user_id
}
