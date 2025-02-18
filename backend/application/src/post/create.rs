use core::result::Result;
use diesel::{insert_into, prelude::*, select};
use domain::{
    models::{
        post::{Post, PostForm},
        user::User,
    },
    schema::users,
};
use infrastructure::establish_connection;
use rocket::{
    form::Form,
    response::status::{Conflict, Created},
};
use shared::response_models::{Response, ResponseBody};

pub fn create_post<T>(post_form: Form<PostForm>) -> Result<Created<String>, Conflict<String>> {
    use domain::schema::posts;

    let post_form = post_form.into_inner();
    let user_id = match find_user_id(&post_form.username) {
        Some(u) => u.user_id,
        None => create_user(post_form.username, post_form.password),
    };

    let post: Post = Post {
        post_id: 0,
        user_id,
        text: post_form.post,
        like_count: None,
        time: chrono::offset::Utc::now(),
    };
    println!("{:?}", post);

    if post_exists(&post.text) {
        return Err(rocket::response::status::Conflict(
            "Post already exists. Write something unique!".to_string(),
        ));
    }

    match diesel::insert_into(posts::table)
        .values(&post)
        .get_result::<Post>(&mut establish_connection())
    {
        Ok(post) => {
            let response = Response {
                body: ResponseBody::Post(post),
            };
            println!("Db response - {:?}", response);
            Ok(Created::new("").tagged_body(serde_json::to_string(&response).unwrap()))
        }
        // TODO: this should not be a conflict 409. Fix this!!
        Err(_) => Err(Conflict("Something went wrong.".to_string())),
    }
}

fn post_exists(post_text: &str) -> bool {
    use diesel::dsl::exists;
    use domain::schema::posts;
    use domain::schema::posts::dsl::*;

    let exists = match select(exists(posts::table.filter(text.eq(post_text))))
        .get_result::<bool>(&mut establish_connection())
    {
        Ok(boolean) => boolean,
        Err(_) => true,
    };

    return exists;
}

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

fn create_user(input_username: String, input_password: String) -> i32 {
    let user: User = User {
        username: input_username,
        password: input_password,
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
