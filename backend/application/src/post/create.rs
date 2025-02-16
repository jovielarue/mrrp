use core::result::Result;
use diesel::{expression::exists, prelude::*, select};
use domain::models::post::{Post, PostForm};
use infrastructure::establish_connection;
use rocket::{form::Form, response::status::Conflict, response::status::Created};
use shared::response_models::{Response, ResponseBody};

pub fn create_post<T>(post_form: Form<PostForm>) -> Result<Created<String>, Conflict<String>> {
    use domain::schema::posts;

    let post_form = post_form.into_inner();
    let post: Post = Post {
        post_id: 0,
        username: post_form.username,
        text: post_form.post,
        like_count: None,
        time: chrono::offset::Utc::now(),
    };
    println!("{:?}", post);

    if post_exists(&post.text) {
        return Err(rocket::response::status::Conflict(
            "Post already exists.".to_string(),
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
