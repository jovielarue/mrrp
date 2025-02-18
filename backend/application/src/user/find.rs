use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use domain::models::user::User;
use infrastructure::establish_connection;

pub fn find_user_id(input_username: &String) -> Option<User> {
    use domain::schema::users;
    use domain::schema::users::dsl::*;

    let found_user = match users::table
        .filter(username.eq(input_username))
        .select(User::as_select())
        .get_result::<User>(&mut establish_connection())
    {
        Ok(user) => Some(user),
        _ => None,
    };

    found_user
}
