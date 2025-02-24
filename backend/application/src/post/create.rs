use crate::user::create::create_user;
use crate::user::find::find_user_id;
use core::result::Result;
use diesel::{prelude::*, select};
use domain::models::post::{Post, PostForm, PostReturn};
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
        None => create_user(&post_form.username, post_form.password),
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
        return Err(Conflict(
            "Post already exists. Write something unique!".to_string(),
        ));
    }

    match diesel::insert_into(posts::table)
        .values(&post)
        .get_result::<Post>(&mut establish_connection())
    {
        Ok(db_response) => {
            let post_return: PostReturn = PostReturn {
                post: db_response,
                username: post_form.username,
            };
            let response = Response {
                body: ResponseBody::Post(post_return),
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
